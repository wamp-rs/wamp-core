use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Subscribed - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-subscribed-2)
/// Represents an subscribed message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Subscribed;
/// use wamp_core::subscribed;
/// use serde_json::json;
///
/// # let mut subscribed_message2 = subscribed!(1, 2);
///
/// let subscribed_message = Subscribed {
///     request_id: 1,
///     subscription: 2
/// };
///
/// # assert_eq!(subscribed_message, subscribed_message2);
/// ```
/// ### Serializer
/// Implements serde Serialize trait for subscribed
/// ```
/// use wamp_core::messages::Subscribed;
/// use serde_json::{json, to_string};
///
/// // Create an subscribed message
/// let subscribed = Subscribed {
///     request_id: 1,
///     subscription: 2
/// };
///
/// // Establish raw json data string
/// let data = r#"[33,1,2]"#;
///
/// // Here we convert it from an `Subscribed` frame, to a string representation.
/// let subscribed = to_string(&subscribed).unwrap();
///
/// // Confirm that our Subscribed frame strings are equal to each other
/// assert_eq!(subscribed, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for Subscribed
/// ```
/// use wamp_core::messages::Subscribed;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[33,1,2]"#;
///
/// // Here we convert it to an `subscribed` frame
/// let subscribed = from_str::<Subscribed>(data).unwrap();
///
/// // Confirm that our request_id and subscription deserialized
/// assert_eq!(subscribed.request_id, 1);
/// assert_eq!(subscribed.subscription, 2);
/// ```
pub struct Subscribed {
    pub request_id: u64,
    pub subscription: u64,
}

#[macro_export]
/// # Subscribed Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-subscribed-2)
/// Macro that allows for creating subscribed wamp message.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Subscribed};
/// use wamp_core::subscribed;
/// use serde_json::json;
///
/// let mut subscribed_message = subscribed!(1, 2);
/// let subscribed_message2 = subscribed!(1, 3);
///
/// assert_ne!(subscribed_message, subscribed_message2);
///
/// // These macro invocations are the same as the following:
/// let subscribed_message3 = Subscribed {
///     request_id: 1,
///     subscription: 2
/// };
///
/// assert_eq!(subscribed_message, subscribed_message3);
/// assert_ne!(subscribed_message2, subscribed_message3);
/// ```
macro_rules! subscribed {
    ($request_id:expr, $subscription:expr) => {
        Subscribed {
            request_id: $request_id,
            subscription: $subscription,
        }
    };
}

impl WampMessage for Subscribed {
    const ID: u64 = 33;

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

impl Serialize for Subscribed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (Self::ID, &self.request_id, &self.subscription).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Subscribed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SubscribedVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);

        impl<'vi> Visitor<'vi> for SubscribedVisitor {
            type Value = Subscribed;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Subscribed components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Subscribed, A, _>(&message_id, "Subscribed")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "request_id must be present and type u64.",
                )?;
                let subscription: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "subscription must be present and object like.",
                )?;
                Ok(Subscribed {
                    request_id,
                    subscription,
                })
            }
        }

        deserializer.deserialize_struct(
            "Subscribed",
            &["request_id", "subscription"],
            SubscribedVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}
