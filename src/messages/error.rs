use super::{
    Call, Cancel, Invocation, MessageDirection, Publish, Register, Subscribe, Unregister,
    Unsubscribe, WampMessage,
};
use crate::{messages::helpers, roles::Roles};
use serde::{de::Visitor, Deserialize, Serialize};
use serde_json::{json, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::marker::PhantomData;

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u64)]
pub enum WampErrorEvent {
    Unsubscribe = Unsubscribe::ID,
    Subscribe = Subscribe::ID,
    Publish = Publish::ID,
    Register = Register::ID,
    Unregister = Unregister::ID,
    Invocation = Invocation::ID,
    Cancel = Cancel::ID,
    Call = Call::ID,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Error
/// Represents an Error message in WAMP protocol.
/// ## Wamp Docs
/// > [Protocol Errors](https://wamp-proto.org/wamp_latest_ietf.html#name-protocol-errors)
/// >
/// > [Subscribe Error](https://wamp-proto.org/wamp_latest_ietf.html#name-subscribe-error)
/// >
/// > [Unsubscribe Error](https://wamp-proto.org/wamp_latest_ietf.html#name-unsubscribe-error)
/// >
/// > [Publish Error](https://wamp-proto.org/wamp_latest_ietf.html#name-publish-error)
/// >
/// > [Register Error](https://wamp-proto.org/wamp_latest_ietf.html#name-register-error)
/// >
/// > [Unregister Error](https://wamp-proto.org/wamp_latest_ietf.html#name-unregister-error)
/// >
/// > [Invocation Error](https://wamp-proto.org/wamp_latest_ietf.html#name-invocation-error)
/// >
/// > [Call Error](https://wamp-proto.org/wamp_latest_ietf.html#name-call-error)
/// >
/// ## Examples
/// ```
/// use wamp_core::messages::{WampError, WampErrorEvent};
/// use serde_json::{json, from_value};
///
/// let error = WampError{
///     event: WampErrorEvent::Call,
///     request_id: 1,
///     details: json!({}),
///     error: "com.myapp.error.object_write_protected".to_string(),
///     args: json!(["Object is write protected."]),
///     kwargs: json!({"severity": 3})
/// };
/// ```
/// ### Serializer
/// Implementation of Serializer for WAMP Error
/// ```
/// use wamp_core::messages::{WampError, WampErrorEvent};
/// use serde_json::{json, to_string};
///
/// let error = WampError{
///     event: WampErrorEvent::Call,
///     request_id: 7814135,
///     details: json!({}),
///     error: "com.myapp.error.object_write_protected".to_string(),
///     args: json!(["Object is write protected."]),
///     kwargs: json!({"severity": 3})
/// };
///
/// let data = r#"[8,48,7814135,{},"com.myapp.error.object_write_protected",["Object is write protected."],{"severity":3}]"#;
///
/// let data2 = to_string(&error).unwrap();
///
/// assert_eq!(data, data2)
/// ```
/// ### Deserializer
/// Implementation of serde Deserialize for WAMP Error
/// ```
/// use wamp_core::messages::{WampError, WampErrorEvent};
/// use serde_json::{json, from_str};
///
/// let error = WampError {
///     event: WampErrorEvent::Call,
///     request_id: 7814135,
///     details: json!({}),
///     error: "com.myapp.error.object_write_protected".to_string(),
///     args: json!(["Object is write protected."]),
///     kwargs: json!({"severity": 3})
/// };
///
/// let data = r#"[8,48,7814135,{},"com.myapp.error.object_write_protected",["Object is write protected."],{"severity":3}]"#;
///
/// let error2 = from_str::<WampError>(data).unwrap();
///
/// assert_eq!(error, error2);
/// ```
pub struct WampError {
    pub event: WampErrorEvent,
    pub request_id: u64,
    pub details: Value,
    pub error: String,
    pub args: Value,
    pub kwargs: Value,
}

#[macro_export]
/// # Error Macro
/// This macro is used for constructing wamp errors with default empty or custom details, args, and kwargs.
/// ## Examples
/// ```
/// use wamp_core::messages::{WampErrorEvent, WampError};
/// use wamp_core::error;
/// use serde_json::{json, Value};
///
/// // Create an error with default values
/// let error = error!(WampErrorEvent::Call, 1, "wamp.error.unknown_realm");
///
/// // Which is the same as creating this
/// let error2 = WampError {
///     event: WampErrorEvent::Call,
///     request_id: 1,
///     details: json!({}),
///     error: "wamp.error.unknown_realm".to_string(),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// assert_eq!(error, error2);
///
/// // Some other ways to use the macro
///
/// // Create error with custom details
/// let _ = error!(WampErrorEvent::Call, 1, "wamp.error.unknown", json!({ "key": "value" }));
///
/// // create error with empty default details and custom args or kwargs
/// let _ = error!(WampErrorEvent::Call, 1, "wamp.error.unknown", args: json!([ 1, 2, 3 ]));
/// let _ = error!(WampErrorEvent::Call, 1, "wamp.error.unknown", kwargs: json!({ "key": "value" }));
///
/// // create error with empty default details and custom args and kwargs
/// let _ = error!(WampErrorEvent::Call, 1, "wamp.error.unknown", args: json!([ 1, 2, 3 ]), kwargs: json!({ "key": "value" }));
///
/// // note that when you use all values, you do not need keyword arguments for args and kwargs
/// let _ = error!(WampErrorEvent::Call, 1, "wamp.error.unknown", json!({}), json!([1, 2, 3]), json!({ "key": "value" }));
/// ```
macro_rules! error {
    ($event:expr, $request_id:expr, $error:expr) => {
        error! {$event, $request_id, $error, serde_json::json!({})}
    };

    ($event:expr, $request_id:expr, $error:expr, args: $args:expr, kwargs: $kwargs:expr) => {
        error! {$event, $request_id, $error, serde_json::json!({}), $args, $kwargs}
    };

    ($event:expr, $request_id:expr, $error:expr, args: $args:expr) => {
        error! {$event, $request_id, $error, serde_json::json!({}), $args, serde_json::Value::Null}
    };

    ($event:expr, $request_id:expr, $error:expr, kwargs: $kwargs:expr) => {
        error! {$event, $request_id, $error, serde_json::json!({}), serde_json::Value::Null, $kwargs}
    };

    ($event:expr, $request_id:expr, $error:expr, $details:expr) => {
        error! {$event, $request_id, $error, $details, serde_json::Value::Null, serde_json::Value::Null}
    };

    ($event:expr, $request_id:expr, $error:expr, $details:expr, args: $args:expr) => {
        error! {$event, $request_id, $error, $details, $args, serde_json::Value::Null}
    };

    ($event:expr, $request_id:expr, $error:expr, $details:expr, kwargs: $kwargs:expr) => {
        error! {$event, $request_id, $error, $details, serde_json::Value::Null, $kwargs}
    };

    ($event:expr, $request_id:expr, $error:expr, $details:expr, $args:expr, $kwargs:expr) => {
        WampError {
            event: $event,
            request_id: $request_id,
            details: $details,
            error: $error.to_string(),
            args: $args,
            kwargs: $kwargs,
        }
    };
}

impl WampMessage for WampError {
    const ID: u64 = 8;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &true,
                sends: &true,
            },
            Roles::Caller => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Publisher => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Subscriber => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Dealer => &MessageDirection {
                receives: &true,
                sends: &true,
            },
            Roles::Broker => &MessageDirection {
                receives: &false,
                sends: &true,
            },
        }
    }
}

impl Serialize for WampError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let details =
            helpers::ser_value_is_object::<S, _>(&self.details, "Details must be Object like.")?;
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
                    &self.event,
                    &self.request_id,
                    details,
                    &self.error,
                )
                    .serialize(serializer)
            } else {
                (
                    Self::ID,
                    &self.event,
                    &self.request_id,
                    details,
                    &self.error,
                    json!([]),
                    kwargs,
                )
                    .serialize(serializer)
            }
        } else {
            if kwargs.is_null() {
                (
                    Self::ID,
                    &self.event,
                    &self.request_id,
                    details,
                    &self.error,
                    args,
                )
                    .serialize(serializer)
            } else {
                (
                    Self::ID,
                    &self.event,
                    &self.request_id,
                    details,
                    &self.error,
                    args,
                    kwargs,
                )
                    .serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for WampError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct WampErrorVisitor(
            PhantomData<u64>,
            PhantomData<WampErrorEvent>,
            PhantomData<u64>,
            PhantomData<Value>,
            PhantomData<String>,
            PhantomData<Value>,
            PhantomData<Value>,
        );

        impl<'vi> Visitor<'vi> for WampErrorVisitor {
            type Value = WampError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of WampError components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message id must be present and type u64.",
                )?;
                helpers::validate_id::<WampError, A, _>(&message_id, "WampError")?;
                let event: WampErrorEvent = helpers::deser_seq_element(
                    &mut seq,
                    "Message type of error must be present and type u64",
                )?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Request ID must be present and type u64",
                )?;
                let details: Value = helpers::deser_seq_element(
                    &mut seq,
                    "Details must be present and object like",
                )?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                let error: String = helpers::deser_seq_element(
                    &mut seq,
                    "Error URI must be present and type String",
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
                Ok(WampError {
                    event,
                    request_id,
                    details,
                    error,
                    args,
                    kwargs,
                })
            }
        }

        deserializer.deserialize_struct(
            "WampError",
            &["event", "request_id", "details", "error", "args", "kwargs"],
            WampErrorVisitor(
                PhantomData,
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
