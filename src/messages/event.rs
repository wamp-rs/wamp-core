use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use std::fmt::Formatter;
use std::marker::PhantomData;

use super::{helpers, MessageDirection, WampMessage};

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Event - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-event-2)
///  Represents an Event message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Event;
/// use wamp_core::event;
/// use serde_json::{json, Value};
///
/// let event = Event {
///     subscription: 1,
///     publication: 2,
///     details: json!({}),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// let event2 = event!(1, 2);
///
/// assert_eq!(event, event2);
/// ```
/// ### Serializer
/// Serde Serialize trait implementation for event.
/// ```
/// use wamp_core::messages::Event;
/// use serde_json::{json, to_string};
///
/// let event = Event {
///     subscription: 1,
///     publication: 2,
///     details: json!({}),
///     args: json!([1, 2, 3]),
///     kwargs: json!({"key": "value"})
/// };
///
/// let event2_string = r#"[36,1,2,{},[1,2,3],{"key":"value"}]"#;
///
/// let event_string = to_string(&event).unwrap();
/// assert_eq!(event_string, event2_string);
/// ```
/// ### Deserializer
/// Serde Deserialize trait implementation for event.
/// ```
/// use wamp_core::messages::Event;
/// use serde_json::{json, from_str};
///
/// let event = Event {
///     subscription: 1,
///     publication: 2,
///     details: json!({}),
///     args: json!([1, 2, 3]),
///     kwargs: json!({"key": "value"})
/// };
///
/// let event2_string = r#"[36,1,2,{},[1,2,3],{"key":"value"}]"#;
///
/// let event2 = from_str::<Event>(event2_string).unwrap();
/// assert_eq!(event, event2);
/// ```
pub struct Event {
    pub subscription: u64,
    pub publication: u64,
    pub details: Value,
    pub args: Value,
    pub kwargs: Value,
}

#[macro_export]
/// ## Event Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-event-2)
///
/// ### Examples
/// ```
/// use wamp_core::event;
/// use wamp_core::messages::Event;
/// use serde_json::{json, Value};
///
/// // Create a event message with default values
/// let event = event!(1, 2);
///
/// // Which is the same as creating this:
/// let event2 = Event {
///     subscription: 1,
///     publication: 2,
///     details: json!({}),
///     args: Value::Null,
///     kwargs: Value::Null
/// };
///
/// assert_eq!(event, event2);
///
/// // Some other ways you can construct it using the macro
///
/// // Create a event with custom details but empty args and kwargs
/// let _ = event!(1, 2, json!( { "key": "value" } ));
///
/// // Create a event with custom args or kwargs, but empty details
/// let _ = event!(1, 2, args: json!( [ 1, 2, 3 ] ));
/// let _ = event!(1, 2, kwargs: json!( { "key": "value" } ));
///
/// // Create a event with custom args and kwargs, but empty details
/// let _ = event!(1, 2, args: json!([ 1, 2, 3 ]), kwargs: json!({ "key": "value" }));
///
/// // Create a event with custom details, and either custom args OR custom kwargs
/// let _ = event!(1, 2, json!( { "key": "value" } ), args: json!( [ 1, 2, 3 ] ));
/// let _ = event!(1, 2, json!( { "key": "value" } ), kwargs: json!( { "key": "value" } ));
///
/// // Create a event with custom details, and both custom args and kwargs
/// // Note that when you use all "required" arguments for the struuct, keyword arguments should not be used for args and kwargs
/// let _ = event!(1, 2, json!({}), json!([]), json!({}));
/// ```
macro_rules! event {
    ($subscription:expr, $publication:expr) => {
        event! {$subscription, $publication, json!({}), serde_json::Value::Null, Value::Null}
    };

    ($subscription:expr, $publication:expr, $details:expr) => {
        event! {$subscription, $publication, $details, serde_json::Value::Null, Value::Null}
    };

    ($subscription:expr, $publication:expr, args:$args:expr) => {
        event! {$subscription, $publication, json!({}), $args, Value::Null}
    };

    ($subscription:expr, $publication:expr, kwargs:$kwargs:expr) => {
        event! {$subscription, $publication, json!({}), serde_json::Value::Null, $kwargs }
    };

    ($subscription:expr, $publication:expr, args:$args:expr, kwargs:$kwargs:expr) => {
        event! {$subscription, $publication, json!({}), $args, $kwargs }
    };

    ($subscription:expr, $publication:expr, $details:expr, args:$args:expr) => {
        event! {$subscription, $publication, $details, $args, serde_json::Value::Null}
    };

    ($subscription:expr, $publication:expr, $details:expr, kwargs:$kwargs:expr) => {
        event! {$subscription, $publication, $details, serde_json::Value::Null, $kwargs}
    };

    ($subscription:expr, $publication:expr, $details:expr, $args:expr, $kwargs:expr) => {{
        Event {
            subscription: $subscription,
            publication: $publication,
            details: $details,
            args: $args,
            kwargs: $kwargs,
        }
    }};
}

impl WampMessage for Event {
    const ID: u64 = 36;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &false,
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
                receives: &true,
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

impl Serialize for Event {
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
            if kwargs.is_null() {}
        } else {
            if kwargs.is_null() {}
        }

        if args.is_null() {
            if kwargs.is_null() {
                (
                    Self::ID,
                    &self.subscription,
                    &self.publication,
                    &self.details,
                )
                    .serialize(serializer)
            } else {
                (
                    Self::ID,
                    &self.subscription,
                    &self.publication,
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
                    &self.subscription,
                    &self.publication,
                    &self.details,
                    args,
                )
                    .serialize(serializer)
            } else {
                (
                    Self::ID,
                    &self.subscription,
                    &self.publication,
                    &self.details,
                    args,
                    kwargs,
                )
                    .serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventVisitor(
            PhantomData<u8>,
            PhantomData<u64>,
            PhantomData<u64>,
            PhantomData<Value>,
            PhantomData<Value>,
            PhantomData<Value>,
        );

        impl<'vi> Visitor<'vi> for EventVisitor {
            type Value = Event;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Event components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Event, A, _>(&message_id, "Event")?;
                let subscription: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Subscription must be present and type u64.",
                )?;
                let publication: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Publication must be present and object like.",
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
                Ok(Event {
                    subscription,
                    publication,
                    details,
                    args,
                    kwargs,
                })
            }
        }

        deserializer.deserialize_struct(
            "Event",
            &["subscription", "publication", "details", "args", "kwargs"],
            EventVisitor(
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

    use super::Event;

    #[test]
    fn test() {
        let d = r#"[36,5512315355,4429313566,{},[],{"color":"orange","sizes":[23,42,7]}]"#;
        let mut ed = Event {
            subscription: 5512315355,
            publication: 4429313566,
            details: serde_json::json!({}),
            args: serde_json::Value::Null,
            kwargs: serde_json::json!({"color":"orange","sizes":[23,42,7]}),
        };
        let ed2: Event = from_str(d).unwrap();
        let d2 = to_string(&ed).unwrap();
        assert_ne!(ed, ed2);
        ed.args = serde_json::json!([]);
        assert_eq!(ed, ed2);
        assert_eq!(d, d2);
    }
}
