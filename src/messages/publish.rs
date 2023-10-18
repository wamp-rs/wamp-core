use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use std::fmt::Formatter;
use std::marker::PhantomData;

use super::{helpers, MessageDirection, WampMessage};

#[derive(Debug, PartialEq, Eq, Clone)]
/// # Publish - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-publish-2)
///  Represents an publish message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Publish;
/// use wamp_core::publish;
/// use serde_json::{json, Value};
///
/// let publish = Publish {
///     request_id: 1,
///     options: json!({ }),
///     topic: "topic".to_string(),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// let publish2 = publish!("topic");
///
/// assert_eq!(publish, publish2);
/// ```
/// ### Serializer
/// Serde Serialize trait implementation for publish.
/// ```
/// use wamp_core::messages::Publish;
/// use serde_json::{json, to_string};
///
/// let publish = Publish {
///     request_id: 7814135,
///     options: json!({}),
///     topic: "com.myapp.user.new".to_string(),
///     args: json!(["johnny"]),
///     kwargs: json!({"firstname":"John","surname":"Doe"})
/// };
///
/// let publish2_string = r#"[16,7814135,{},"com.myapp.user.new",["johnny"],{"firstname":"John","surname":"Doe"}]"#;
///
/// let publish_string = to_string(&publish).unwrap();
/// assert_eq!(publish_string, publish2_string);
/// ```
/// ### Deserializer
/// Serde Deserialize trait implementation for publish.
/// ```
/// use wamp_core::messages::Publish;
/// use serde_json::{json, from_str};
///
/// let publish = Publish {
///     request_id: 7814135,
///     options: json!({}),
///     topic: "com.myapp.user.new".to_string(),
///     args: json!(["johnny"]),
///     kwargs: json!({"firstname":"John","surname":"Doe"})
/// };
///
/// let publish2_string = r#"[16,7814135,{},"com.myapp.user.new",["johnny"],{"firstname":"John","surname":"Doe"}]"#;
///
/// let publish2 = from_str::<Publish>(publish2_string).unwrap();
/// assert_eq!(publish, publish2);
/// ```
pub struct Publish {
    pub request_id: u64,
    pub options: Value,
    pub topic: String,
    pub args: Value,
    pub kwargs: Value,
}

#[macro_export]
/// ## Publish Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-publish-2)
/// Publish message builder with thread safe auto-incrementing request-ids.
/// ### Examples
/// ```
/// use wamp_core::publish;
/// use wamp_core::messages::Publish;
/// use serde_json::{json, Value};
///
/// // Create a Publish message with default values
/// let publish = publish!("topic");
///
/// // Which is the same as creating this:
/// let publish2 = Publish {
///     topic: "topic".to_string(),
///     request_id: 1,
///     options: json!({}),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// assert_eq!(publish, publish2);
///
/// // Some other ways you can construct it using the macro
///
/// // Create a publish with custom options but empty args and kwargs
/// let _ = publish!("topic", json!( { "key": "value" } ));
///
/// // Create a publish with custom args or kwargs, but empty options
/// let _ = publish!("topic", args: json!( [ 1, 2, 3 ] ));
/// let _ = publish!("topic", kwargs: json!( { "key": "value" } ));
///
/// // Create a publish with custom args and kwargs, but empty options
/// let _ = publish!("topic", args: json!([ 1, 2, 3 ]), kwargs: json!({ "key": "value" }));
///
/// // Create a publish with custom options, and either custom args OR custom kwargs
/// let _ = publish!("topic", json!( { "key": "value" } ), args: json!( [ 1, 2, 3 ] ));
/// let _ = publish!("topic", json!( { "key": "value" } ), kwargs: json!( { "key": "value" } ));
///
/// // Create a publish with custom options, and both custom args and kwargs
/// // Note that when you use all "required" arguments for the struuct, keyword arguments should not be used for args and kwargs
/// let _ = publish!("topic", json!({}), json!([]), json!({}));
/// ```
macro_rules! publish {
    ($topic:expr) => {
        publish! {$topic, serde_json::json!({}), serde_json::Value::Null, serde_json::Value::Null}
    };

    ($topic:expr, $options:expr) => {
        publish! {$topic, $options, serde_json::Value::Null, serde_json::Value::Null}
    };

    ($topic:expr, args: $args:expr) => {
        publish! {$topic, serde_json::json!({}), $args, serde_json::Value::Null}
    };

    ($topic:expr, kwargs: $kwargs:expr) => {
        publish! {$topic, serde_json::json!({}), serde_json::Value::Null, $kwargs}
    };

    ($topic:expr, args: $args:expr, kwargs: $kwargs:expr) => {
        publish! {$topic, serde_json::json!({}), $args, $kwargs}
    };

    ($topic:expr, $options:expr, args: $args:expr) => {
        publish! {$topic, $options, $args, serde_json::Value::Null}
    };

    ($topic:expr, $options:expr, kwargs: $kwargs:expr) => {
        publish! {$topic, $options, serde_json::Value::Null, $kwargs}
    };

    ($topic:expr, $options:expr, $args:expr, $kwargs:expr) => {{
        Publish {
            request_id: $crate::factories::increment(),
            options: $options,
            topic: $topic.to_string(),
            args: $args,
            kwargs: $kwargs,
        }
    }};
}

impl WampMessage for Publish {
    const ID: u64 = 16;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Caller => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Callee => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Publisher => &MessageDirection {
                receives: &false,
                sends: &true,
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
                receives: &true,
                sends: &false,
            },
        }
    }
}

impl Serialize for Publish {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let options =
            helpers::ser_value_is_object::<S, _>(&self.options, "Options must be object like.")?;
        let args =
            helpers::ser_value_is_args::<S, _>(&self.args, "Args must be Array like or Null.")?;
        let kwargs = helpers::ser_value_is_kwargs::<S, _>(
            &self.kwargs,
            "Kwargs must be Object like or Null.",
        )?;
        if args.is_null() {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, options, &self.topic).serialize(serializer)
            } else {
                (
                    Self::ID,
                    &self.request_id,
                    options,
                    &self.topic,
                    json!([]),
                    kwargs,
                )
                    .serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, options, &self.topic, args).serialize(serializer)
            } else {
                (
                    Self::ID,
                    &self.request_id,
                    options,
                    &self.topic,
                    args,
                    kwargs,
                )
                    .serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Publish {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PublishVisitor(
            PhantomData<u8>,
            PhantomData<u64>,
            PhantomData<Value>,
            PhantomData<String>,
            PhantomData<Value>,
            PhantomData<Value>,
        );

        impl<'vi> Visitor<'vi> for PublishVisitor {
            type Value = Publish;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Publish components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Publish, A, _>(&message_id, "Publish")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Request ID must be present and type u64.",
                )?;
                let options: Value = helpers::deser_seq_element(
                    &mut seq,
                    "Options must be present and object like.",
                )?;
                helpers::deser_value_is_object::<A, _>(&options, "Options must be object like.")?;
                let topic: String =
                    helpers::deser_seq_element(&mut seq, "topic must be present and object like.")?;
                let args: Value = helpers::deser_args_kwargs_element(
                    &mut seq,
                    "Args must be array like or null.",
                )?;
                let kwargs: Value = helpers::deser_args_kwargs_element(
                    &mut seq,
                    "Kwargs must be object like or null.",
                )?;
                Ok(Publish {
                    request_id,
                    options,
                    topic,
                    args,
                    kwargs,
                })
            }
        }

        deserializer.deserialize_struct(
            "Publish",
            &["request_id", "options", "topic", "args", "kwargs"],
            PublishVisitor(
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
