use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Call - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-call-2)
///  Represents an Call message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Call;
/// use wamp_core::call;
/// use serde_json::{json, Value};
///
/// let call = Call {
///     request_id: 1,
///     options: json!({ }),
///     procedure: "procedure".to_string(),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// let call2 = call!("procedure");
///
/// assert_eq!(call, call2);
/// ```
/// ### Serializer
/// Serde Serialize trait implementation for Call.
/// ```
/// use wamp_core::messages::Call;
/// use serde_json::{json, to_string};
///
/// let call = Call {
///     request_id: 7814135,
///     options: json!({}),
///     procedure: "com.myapp.user.new".to_string(),
///     args: json!(["johnny"]),
///     kwargs: json!({"firstname":"John","surname":"Doe"})
/// };
///
/// let call2_string = r#"[48,7814135,{},"com.myapp.user.new",["johnny"],{"firstname":"John","surname":"Doe"}]"#;
///
/// let call_string = to_string(&call).unwrap();
/// assert_eq!(call_string, call2_string);
/// ```
/// ### Deserializer
/// Serde Deserialize trait implementation for Call.
/// ```
/// use wamp_core::messages::Call;
/// use serde_json::{json, from_str};
///
/// let call = Call {
///     request_id: 7814135,
///     options: json!({}),
///     procedure: "com.myapp.user.new".to_string(),
///     args: json!(["johnny"]),
///     kwargs: json!({"firstname":"John","surname":"Doe"})
/// };
///
/// let call2_string = r#"[48,7814135,{},"com.myapp.user.new",["johnny"],{"firstname":"John","surname":"Doe"}]"#;
///
/// let call2 = from_str::<Call>(call2_string).unwrap();
/// assert_eq!(call, call2);
/// ```
pub struct Call {
    pub request_id: u64,
    pub options: Value,
    pub procedure: String,
    pub args: Value,
    pub kwargs: Value,
}

#[macro_export]
/// ## Call Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-call-2)
/// Call message builder with thread safe auto-incrementing request-ids.
/// ### Examples
/// ```
/// use wamp_core::call;
/// use wamp_core::messages::Call;
/// use serde_json::{json, Value};
///
/// // Create a call message with default values
/// let call = call!("procedure");
///
/// // Which is the same as creating this:
/// let call2 = Call {
///     procedure: "procedure".to_string(),
///     request_id: 1,
///     options: json!({}),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// assert_eq!(call, call2);
///
/// // Some other ways you can construct it using the macro
///
/// // Create a call with custom options but empty args and kwargs
/// let _ = call!("procedure", json!( { "key": "value" } ));
///
/// // Create a call with custom args or kwargs, but empty options
/// let _ = call!("procedure", args: json!( [ 1, 2, 3 ] ));
/// let _ = call!("procedure", kwargs: json!( { "key": "value" } ));
///
/// // Create a call with custom args and kwargs, but empty options
/// let _ = call!("procedure", args: json!([ 1, 2, 3 ]), kwargs: json!({ "key": "value" }));
///
/// // Create a call with custom options, and either custom args OR custom kwargs
/// let _ = call!("procedure", json!( { "key": "value" } ), args: json!( [ 1, 2, 3 ] ));
/// let _ = call!("procedure", json!( { "key": "value" } ), kwargs: json!( { "key": "value" } ));
///
/// // Create a call with custom options, and both custom args and kwargs
/// // Note that when you use all "required" arguments for the struuct, keyword arguments should not be used for args and kwargs
/// let _ = call!("procedure", json!({}), json!([]), json!({}));
/// ```
macro_rules! call {
    ($procedure:expr) => {
        call! {$procedure, serde_json::json!({}), serde_json::Value::Null, serde_json::Value::Null}
    };

    ($procedure:expr, $options:expr) => {
        call! {$procedure, $options, serde_json::Value::Null, serde_json::Value::Null}
    };

    ($procedure:expr, args: $args:expr) => {
        call! {$procedure, serde_json::json!({}), $args, serde_json::Value::Null}
    };

    ($procedure:expr, kwargs: $kwargs:expr) => {
        call! {$procedure, serde_json::json!({}), serde_json::Value::Null, $kwargs}
    };

    ($procedure:expr, args: $args:expr, kwargs: $kwargs:expr) => {
        call! {$procedure, serde_json::json!({}), $args, $kwargs}
    };

    ($procedure:expr, $options:expr, args: $args:expr) => {
        call! {$procedure, $options, $args, serde_json::Value::Null}
    };

    ($procedure:expr, $options:expr, kwargs: $kwargs:expr) => {
        call! {$procedure, $options, serde_json::Value::Null, $kwargs}
    };

    ($procedure:expr, $options:expr, $args:expr, $kwargs:expr) => {{
        $crate::messages::Call {
            request_id: $crate::factories::increment(),
            options: $options,
            procedure: $procedure.to_string(),
            args: $args,
            kwargs: $kwargs,
        }
    }};
}

impl WampMessage for Call {
    const ID: u64 = 48;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Caller => &MessageDirection {
                receives: &false,
                sends: &true,
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

impl Serialize for Call {
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
                (Self::ID, &self.request_id, options, &self.procedure).serialize(serializer)
            } else {
                (
                    Self::ID,
                    &self.request_id,
                    options,
                    &self.procedure,
                    json!([]),
                    kwargs,
                )
                    .serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (Self::ID, &self.request_id, options, &self.procedure, args).serialize(serializer)
            } else {
                (
                    Self::ID,
                    &self.request_id,
                    options,
                    &self.procedure,
                    args,
                    kwargs,
                )
                    .serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Call {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CallVisitor(
            PhantomData<u8>,
            PhantomData<u64>,
            PhantomData<Value>,
            PhantomData<String>,
            PhantomData<Value>,
            PhantomData<Value>,
        );

        impl<'vi> Visitor<'vi> for CallVisitor {
            type Value = Call;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Call components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Call, A, _>(&message_id, "Call")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Request ID must be present and type u64.",
                )?;
                let options: Value = helpers::deser_seq_element(
                    &mut seq,
                    "Options must be present and object like.",
                )?;
                helpers::deser_value_is_object::<A, _>(&options, "Options must be object like.")?;
                let procedure: String = helpers::deser_seq_element(
                    &mut seq,
                    "Procedure must be present and object like.",
                )?;
                let args: Value = helpers::deser_args_kwargs_element(
                    &mut seq,
                    "Args must be array like or null.",
                )?;
                let kwargs: Value = helpers::deser_args_kwargs_element(
                    &mut seq,
                    "Kwargs must be object like or null.",
                )?;
                Ok(Call {
                    request_id,
                    options,
                    procedure,
                    args,
                    kwargs,
                })
            }
        }

        deserializer.deserialize_struct(
            "Call",
            &[
                "request_id",
                "message_id",
                "options",
                "procedure",
                "args",
                "kwargs",
            ],
            CallVisitor(
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
