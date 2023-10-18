use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Invocation - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-invocation-2)
///  Represents an invocation message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Invocation;
/// use wamp_core::invocation;
/// use serde_json::{json, Value};
///
/// let invocation = Invocation {
///     request_id: 1,
///     registration: 2,
///     details: json!({}),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// let invocation2 = invocation!(2);
///
/// assert_eq!(invocation, invocation2);
/// ```
/// ### Serializer
/// Serde Serialize trait implementation for invocation.
/// ```
/// use wamp_core::messages::Invocation;
/// use serde_json::{json, to_string};
///
/// let invocation = Invocation {
///     request_id: 1,
///     registration: 2,
///     details: json!({}),
///     args: json!([1, 2, 3]),
///     kwargs: json!({"key": "value"})
/// };
///
/// let invocation2_string = r#"[68,1,2,{},[1,2,3],{"key":"value"}]"#;
///
/// let invocation_string = to_string(&invocation).unwrap();
/// assert_eq!(invocation_string, invocation2_string);
/// ```
/// ### Deserializer
/// Serde Deserialize trait implementation for invocation.
/// ```
/// use wamp_core::messages::Invocation;
/// use serde_json::{json, from_str};
///
/// let invocation = Invocation {
///     request_id: 1,
///     registration: 2,
///     details: json!({}),
///     args: json!([1, 2, 3]),
///     kwargs: json!({"key": "value"})
/// };
///
/// let invocation2_string = r#"[68,1,2,{},[1,2,3],{"key":"value"}]"#;
///
/// let invocation2 = from_str::<Invocation>(invocation2_string).unwrap();
/// assert_eq!(invocation, invocation2);
/// ```
pub struct Invocation {
    pub request_id: u64,
    pub registration: u64,
    pub details: Value,
    pub args: Value,
    pub kwargs: Value,
}

#[macro_export]
/// ## invocation Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-invocation-2)
/// Macro for creating invocation messages easily with auto incrementing request id.
/// ### Examples
/// ```
/// use wamp_core::invocation;
/// use wamp_core::messages::Invocation;
/// use serde_json::{json, Value};
///
/// // Create a invocation message with default values
/// let invocation = invocation!(2);
///
/// // Which is the same as creating this:
/// let invocation2 = Invocation {
///     request_id: 1,
///     registration: 2,
///     details: json!({}),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// assert_eq!(invocation, invocation2);
///
/// // Some other ways you can construct it using the macro
///
/// // Create a invocation with custom details but empty args and kwargs
/// let _ = invocation!(2, json!( { "key": "value" } ));
///
/// // Create a invocation with custom args or kwargs, but empty details
/// let _ = invocation!(2, args: json!( [ 1, 2, 3 ] ));
/// let _ = invocation!(2, kwargs: json!( { "key": "value" } ));
///
/// // Create a invocation with custom args and kwargs, but empty details
/// let _ = invocation!(2, args: json!([ 1, 2, 3 ]), kwargs: json!({ "key": "value" }));
///
/// // Create a invocation with custom details, and either custom args OR custom kwargs
/// let _ = invocation!(2, json!( { "key": "value" } ), args: json!( [ 1, 2, 3 ] ));
/// let _ = invocation!(2, json!( { "key": "value" } ), kwargs: json!( { "key": "value" } ));
///
/// // Create a invocation with custom details, and both custom args and kwargs
/// // Note that when you use all "required" arguments for the struuct, keyword arguments should not be used for args and kwargs
/// let _ = invocation!(2, json!({}), json!([]), json!({}));
/// ```
macro_rules! invocation {
    ($registration:expr) => {
        invocation!{$registration, serde_json::json!({}), serde_json::Value::Null, serde_json::Value::Null}
    };

    ($registration:expr, $details:expr) => {
        invocation!{$registration, $details, serde_json::Value::Null, serde_json::Value::Null}
    };

    ($registration:expr, args: $args:expr) => {
        invocation!{$registration, serde_json::json!({}), $args, serde_json::Value::Null}
    };

    ($registration:expr, kwargs: $kwargs:expr) => {
        invocation!{$registration, serde_json::json!({}), serde_json::Value::Null, $kwargs}
    };

    ($registration:expr, args: $args:expr, kwargs: $kwargs:expr) => {
        invocation!{$registration, serde_json::json!({}), $args, $kwargs}
    };

    ($registration:expr, $details:expr, args: $args:expr) => {
        invocation!{$registration, $details, $args, serde_json::Value::Null}
    };

    ($registration:expr, $details:expr, kwargs: $kwargs:expr) => {
        invocation!{$registration, $details, serde_json::Value::Null, $kwargs}
    };

    ($registration:expr, $details:expr, $args:expr, $kwargs:expr) => {{
        Invocation {
            request_id: $crate::factories::increment(),
            details: $details,
            registration: $registration,
            args: $args,
            kwargs: $kwargs
        }
    }};
}

impl WampMessage for Invocation {
    const ID: u64 = 68;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Caller => &MessageDirection {
                receives: &false,
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
                sends: &true,
            },
            Roles::Broker => &MessageDirection {
                receives: &false,
                sends: &false,
            },
        }
    }
}

impl Serialize for Invocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let args =
            helpers::ser_value_is_args::<S, _>(&self.args, "Args must be Array like or Null.")?;
        let kwargs = helpers::ser_value_is_kwargs::<S, _>(
            &self.kwargs,
            "Kwargs must be Object like or Null.",
        )?;

        if args.is_null() {
            if kwargs.is_null() {
                (
                    Self::ID,
                    &self.request_id,
                    &self.registration,
                    &self.details,
                )
                    .serialize(serializer)
            } else {
                (
                    Self::ID,
                    &self.request_id,
                    &self.registration,
                    &self.details,
                    json!([]),
                    kwargs,
                )
                    .serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (
                    Self::ID,
                    &self.request_id,
                    &self.registration,
                    &self.details,
                    args,
                )
                    .serialize(serializer)
            } else {
                (
                    Self::ID,
                    &self.request_id,
                    &self.registration,
                    &self.details,
                    args,
                    kwargs,
                )
                    .serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Invocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InvocationVisitor(
            PhantomData<u8>,
            PhantomData<u64>,
            PhantomData<u64>,
            PhantomData<Value>,
            PhantomData<Value>,
            PhantomData<Value>,
        );

        impl<'vi> Visitor<'vi> for InvocationVisitor {
            type Value = Invocation;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Invocation components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Invocation, A, _>(&message_id, "Invocation")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "request_id must be present and type u64.",
                )?;
                let registration: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "registration must be present and object like.",
                )?;
                let details: Value = helpers::deser_seq_element(
                    &mut seq,
                    "Details must be present and object like.",
                )?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                let args: Value = helpers::deser_args_kwargs_element(
                    &mut seq,
                    "Args must be array like or null.",
                )?;
                let kwargs: Value = helpers::deser_args_kwargs_element(
                    &mut seq,
                    "Kwargs must be object like or null.",
                )?;
                Ok(Invocation {
                    request_id,
                    registration,
                    details,
                    args,
                    kwargs,
                })
            }
        }

        deserializer.deserialize_struct(
            "Invocation",
            &["request_id", "registration", "details", "args", "kwargs"],
            InvocationVisitor(
                PhantomData,
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
    use serde_json::{from_str, to_string};

    use super::Invocation;

    #[test]
    fn test() {
        let d = r#"[68,6131533,9823529,{},["johnny"],{"firstname":"John","surname":"Doe"}]"#;
        let mut ed = Invocation {
            request_id: 6131533,
            registration: 9823529,
            details: serde_json::json!({}),
            args: serde_json::Value::Null,
            kwargs: serde_json::json!({"firstname":"John","surname":"Doe"}),
        };
        let ed2: Invocation = from_str(d).unwrap();
        assert_ne!(ed, ed2);
        ed.args = serde_json::json!(["johnny"]);
        assert_eq!(ed, ed2);
        let d2 = to_string(&ed).unwrap();
        assert_eq!(d, d2);
    }
}
