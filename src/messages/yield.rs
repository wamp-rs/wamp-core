use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Yield - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-yield-2)
///  Represents an Yield message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Yield;
/// use wamp_core::r#yield;
/// use serde_json::{json, Value};
///
/// let r#yield = Yield {
///     request_id: 2,
///     options: json!({}),
///     args: serde_json::Value::Null,
///     kwargs: serde_json::Value::Null
/// };
///
/// let mut r#yield2 = r#yield!{1, json!({})};
///
/// assert_ne!(r#yield, r#yield2);
///
/// r#yield2.request_id = 2;
///
/// assert_eq!(r#yield, r#yield2);
/// ```
/// ### Serializer
/// Serde Serialize trait implementation for Yield.
/// ```
/// use wamp_core::messages::Yield;
/// use serde_json::{json, to_string};
///
/// let r#yield = Yield {
///     request_id: 2,
///     options: json!({}),
///     args: json!([1, 2, 3]),
///     kwargs: json!({"key": "value"})
/// };
///
/// let r#yield2_string = r#"[70,2,{},[1,2,3],{"key":"value"}]"#;
///
/// let r#yield_string = to_string(&r#yield).unwrap();
/// assert_eq!(r#yield_string, r#yield2_string);
/// ```
/// ### Deserializer
/// Serde Deserialize trait implementation for Yield.
/// ```
/// use wamp_core::messages::Yield;
/// use serde_json::{json, from_str};
///
/// let r#yield = Yield {
///     request_id: 2,
///     options: json!({}),
///     args: json!([1, 2, 3]),
///     kwargs: json!({"key": "value"})
/// };
///
/// let r#yield2_string = r#"[70,2,{},[1,2,3],{"key":"value"}]"#;
///
/// let r#yield2 = from_str::<Yield>(r#yield2_string).unwrap();
/// assert_eq!(r#yield, r#yield2);
/// ```
pub struct Yield {
    pub request_id: u64,
    pub options: Value,
    pub args: Value,
    pub kwargs: Value,
}

#[macro_export]
/// ## Yield Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-yield-2)
/// Yield macro for easy creation with default values
/// ### Examples
/// ```
/// use wamp_core::r#yield;
/// use wamp_core::messages::Yield;
/// use serde_json::{json, Value};
///
/// // Create a r#yield message with default values
/// let mut r#yield = r#yield!(1, json!({}));
///
/// // Which is the same as creating this:
/// let r#yield2 = Yield {
///     request_id: 2,
///     options: json!({}),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// assert_ne!(r#yield, r#yield2);
///
/// r#yield.request_id = 2;
///
/// assert_eq!(r#yield, r#yield2);
///
/// // Some other ways you can construct it using the macro
///
/// // Create a r#yield with custom options but empty args and kwargs
/// let _ = r#yield!(2, json!( { "key": "value" } ));
///
/// // Create a r#yield with custom args or kwargs, but empty options
/// let _ = r#yield!(2, args: json!( [ 1, 2, 3 ] ));
/// let _ = r#yield!(2, kwargs: json!( { "key": "value" } ));
///
/// // Create a r#yield with custom args and kwargs, but empty options
/// let _ = r#yield!(2, args: json!([ 1, 2, 3 ]), kwargs: json!({ "key": "value" }));
///
/// // Create a r#yield with custom options, and either custom args OR custom kwargs
/// let _ = r#yield!(2, json!( { "key": "value" } ), args: json!( [ 1, 2, 3 ] ));
/// let _ = r#yield!(2, json!( { "key": "value" } ), kwargs: json!( { "key": "value" } ));
///
/// // Create a r#yield with custom options, and both custom args and kwargs
/// // Note that when you use all "required" arguments for the struuct, keyword arguments should not be used for args and kwargs
/// let _ = r#yield!(2, json!({}), json!([]), json!({}));
/// ```
macro_rules! r#yield {
    ($request_id:expr) => {
        r#yield! {$request_id, serde_json::json!({}), serde_json::Value::Null, serde_json::Value::Null}
    };
    ($request_id:expr, $options:expr) => {
        r#yield! {$request_id, $options, serde_json::Value::Null, serde_json::Value::Null}
    };
    ($request_id:expr, args: $args:expr) => {
        r#yield! {$request_id, serde_json::json!({}), $args, serde_json::Value::Null}
    };
    ($request_id:expr, kwargs: $kwargs:expr) => {
        r#yield! {$request_id, serde_json::json!({}), serde_json::Value::Null, $kwargs}
    };
    ($request_id:expr, args: $args:expr, kwargs: $kwargs:expr) => {
        r#yield! {$request_id, serde_json::json!({}), $args, $kwargs}
    };
    ($request_id:expr, $options:expr, args: $args:expr) => {
        r#yield! {$request_id, $options, $args, serde_json::Value::Null}
    };
    ($request_id:expr, $options:expr, kwargs: $kwargs:expr) => {
        r#yield! {$request_id, $options, serde_json::Value::Null, $kwargs}
    };
    ($request_id:expr, $options:expr, $args:expr, $kwargs:expr) => {
        Yield {
            args: $args,
            request_id: $request_id,
            options: $options,
            kwargs: $kwargs,
        }
    };
}

impl WampMessage for Yield {
    const ID: u64 = 70;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &false,
                sends: &true,
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
                receives: &true,
                sends: &false,
            },
            Roles::Broker => &MessageDirection {
                receives: &false,
                sends: &false,
            },
        }
    }
}

impl Serialize for Yield {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let options =
            helpers::ser_value_is_object::<S, _>(&self.options, "options must be object like.")?;
        let args =
            helpers::ser_value_is_args::<S, _>(&self.args, "Args must be Array like or Null.")?;
        let kwargs = helpers::ser_value_is_kwargs::<S, _>(
            &self.kwargs,
            "Kwargs must be Object like or Null.",
        )?;
        if args.is_null() {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, options).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, options, json!([]), kwargs).serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, options, args).serialize(serializer)
            } else {
                (Self::ID, &self.request_id, options, args, kwargs).serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Yield {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct YieldVisitor(
            PhantomData<u64>,
            PhantomData<u64>,
            PhantomData<Value>,
            PhantomData<Value>,
            PhantomData<Value>,
        );

        impl<'vi> Visitor<'vi> for YieldVisitor {
            type Value = Yield;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Yield components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Yield, A, _>(&message_id, "Yield")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Request ID must be present and type u64.",
                )?;
                let options: Value = helpers::deser_seq_element(
                    &mut seq,
                    "options must be present and object like.",
                )?;
                helpers::deser_value_is_object::<A, _>(&options, "options must be object like.")?;
                let args: Value = helpers::deser_args_kwargs_element(
                    &mut seq,
                    "Args must be array like or null.",
                )?;
                let kwargs: Value = helpers::deser_args_kwargs_element(
                    &mut seq,
                    "Kwargs must be object like or null.",
                )?;
                Ok(Yield {
                    request_id,
                    options,
                    args,
                    kwargs,
                })
            }
        }

        deserializer.deserialize_struct(
            "Yield",
            &["request_id", "options", "args", "kwargs"],
            YieldVisitor(
                PhantomData,
                PhantomData,
                PhantomData,
                PhantomData,
                PhantomData,
            ),
        )
    }
}
