use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Result - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-result-2)
///  Represents an Result message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::WampResult;
/// use wamp_core::result;
/// use serde_json::{json, Value};
///
/// let mut result = WampResult {
///     request_id: 1,
///     details: json!({}),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// let result2 = result!(2);
///
/// assert_ne!(result, result2);
///
/// result.request_id = 2;
///
/// assert_eq!(result, result2);
/// ```
/// ### Serializer
/// Serde Serialize trait implementation for Result.
/// ```
/// use wamp_core::messages::WampResult;
/// use serde_json::{json, to_string};
///
/// let result = WampResult {
///     request_id: 1,
///     details: json!({}),
///     args: json!([1, 2, 3]),
///     kwargs: json!({"key": "value"})
/// };
///
/// let result2_string = r#"[50,1,{},[1,2,3],{"key":"value"}]"#;
///
/// let result_string = to_string(&result).unwrap();
/// assert_eq!(result_string, result2_string);
/// ```
/// ### Deserializer
/// Serde Deserialize trait implementation for Result.
/// ```
/// use wamp_core::messages::WampResult;
/// use serde_json::{json, from_str};
///
/// let result = WampResult {
///     request_id: 1,
///     details: json!({}),
///     args: json!([1, 2, 3]),
///     kwargs: json!({"key": "value"})
/// };
///
/// let result2_string = r#"[50,1,{},[1,2,3],{"key":"value"}]"#;
///
/// let result2 = from_str::<WampResult>(result2_string).unwrap();
/// assert_eq!(result, result2);
/// ```
pub struct WampResult {
    pub request_id: u64,
    pub details: Value,
    pub args: Value,
    pub kwargs: Value,
}

#[macro_export]
/// ## Result Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-result-2)
/// Macro for creating Result messages easily with auto incrementing request id.
/// ### Examples
/// ```
/// use wamp_core::result;
/// use wamp_core::messages::WampResult;
/// use serde_json::{json, Value};
///
/// // Create a result message with default values
/// let mut result = result!(2);
///
/// // Which is the same as creating this:
/// let result2 = WampResult {
///     request_id: 1,
///     details: json!({}),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// assert_ne!(result, result2);
///
/// result.request_id = 1;
///
/// assert_eq!(result, result2);
///
/// // Some other ways you can construct it using the macro
///
/// // Create a result with custom details but empty args and kwargs
/// let _ = result!(2, json!( { "key": "value" } ));
///
/// // Create a result with custom args or kwargs, but empty details
/// let _ = result!(2, args: json!( [ 1, 2, 3 ] ));
/// let _ = result!(2, kwargs: json!( { "key": "value" } ));
///
/// // Create a result with custom args and kwargs, but empty details
/// let _ = result!(2, args: json!([ 1, 2, 3 ]), kwargs: json!({ "key": "value" }) );
///
/// // Create a result with custom details, and either custom args OR custom kwargs
/// let _ = result!(2, json!( { "key": "value" } ), args: json!( [ 1, 2, 3 ] ));
/// let _ = result!(2, json!( { "key": "value" } ), kwargs: json!( { "key": "value" } ));
///
/// // Create a result with custom details, and both custom args and kwargs
/// // Note that when you use all "required" arguments for the struuct, keyword arguments should not be used for args and kwargs
/// let _ = result!(2, json!({}), json!([]), json!({}));
/// ```
macro_rules! result {
    ($request_id:expr) => {
        result! {$request_id, json!({}), Value::Null, Value::Null}
    };
    ($request_id:expr, $details:expr) => {
        result! {$request_id, $details, Value::Null, Value::Null}
    };
    ($request_id:expr, args: $args:expr) => {
        result! {$request_id, serde_json::json!({}), $args, Value::Null}
    };
    ($request_id:expr, kwargs: $kwargs:expr) => {
        result! {$request_id, serde_json::json!({}), Value::Null, $kwargs}
    };
    ($request_id:expr, args: $args:expr, kwargs: $kwargs:expr) => {
        result! {$request_id, serde_json::json!({}), $args, $kwargs}
    };
    ($request_id:expr, $details:expr, args: $args:expr) => {
        result! {$request_id, $details, $args, Value::Null}
    };
    ($request_id:expr, $details:expr, kwargs: $kwargs:expr) => {
        result! {$request_id, $details, Value::Null, $kwargs}
    };
    ($request_id:expr, $details:expr, $args:expr, $kwargs:expr) => {
        WampResult {
            args: $args,
            request_id: $request_id,
            details: $details,
            kwargs: $kwargs,
        }
    };
}

impl WampMessage for WampResult {
    const ID: u64 = 50;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Caller => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Publisher => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Subscriber => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Dealer => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Broker => &MessageDirection {
                receives: &false,
                sends: &true,
            },
        }
    }
}

impl Serialize for WampResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let details =
            helpers::ser_value_is_object::<S, _>(&self.details, "details must be object like.")?;
        let args =
            helpers::ser_value_is_args::<S, _>(&self.args, "Args must be Array like or Null.")?;
        let kwargs = helpers::ser_value_is_kwargs::<S, _>(
            &self.kwargs,
            "Kwargs must be Object like or Null.",
        )?;
        if args.is_null() {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, details).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, details, json!([]), kwargs).serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, details, args).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, details, args, kwargs).serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for WampResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WampResultVisitor(
            PhantomData<u64>,
            PhantomData<u64>,
            PhantomData<Value>,
            PhantomData<Value>,
            PhantomData<Value>,
        );

        impl<'vi> Visitor<'vi> for WampResultVisitor {
            type Value = WampResult;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of WampResult components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<WampResult, A, _>(&message_id, "WampResult")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Request ID must be present and type u64.",
                )?;
                let details: Value = helpers::deser_seq_element(
                    &mut seq,
                    "details must be present and object like.",
                )?;
                helpers::deser_value_is_object::<A, _>(&details, "details must be object like.")?;
                let args: Value = helpers::deser_args_kwargs_element(
                    &mut seq,
                    "Args must be array like or null.",
                )?;
                let kwargs: Value = helpers::deser_args_kwargs_element(
                    &mut seq,
                    "Kwargs must be object like or null.",
                )?;
                Ok(WampResult {
                    request_id,
                    details,
                    args,
                    kwargs,
                })
            }
        }

        deserializer.deserialize_struct(
            "WampResult",
            &["request_id", "details", "args", "kwargs"],
            WampResultVisitor(
                PhantomData,
                PhantomData,
                PhantomData,
                PhantomData,
                PhantomData,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, json, to_string};

    use super::WampResult;

    #[test]
    fn test() {
        let d1 = r#"[50,7814135,{},[],{"karma":10,"userid":123}]"#;
        let mut w1 = WampResult {
            request_id: 7814135,
            details: json!({}),
            args: serde_json::Value::Null,
            kwargs: json!({"userid":123,"karma":10}),
        };
        assert_ne!(from_str::<WampResult>(d1).unwrap(), w1);
        w1.args = json!([]);
        assert_eq!(from_str::<WampResult>(d1).unwrap(), w1);
        assert_eq!(to_string(&w1).unwrap(), d1);
    }
}
