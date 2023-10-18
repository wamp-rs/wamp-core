use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Unsubscribed - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-unsubscribed-2)
/// Represents an Unsubscribed message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Unsubscribed;
/// use wamp_core::unsubscribed;
/// # let unsubscribed_message1 = unsubscribed!(1);
///
/// let unsubscribed_message = Unsubscribed {
///     request_id: 1
/// };
///
/// # assert_eq!(unsubscribed_message, unsubscribed_message1);
/// ```
///
/// ### Serializer
/// Implements serde Serialize trait for Unsubscribed
/// ```
/// use wamp_core::messages::Unsubscribed;
/// use serde_json::{json, to_string};
///
/// // Create an Unsubscribe message
/// let unsubscribed = Unsubscribed {
///     request_id: 1
/// };
///
/// // Establish raw json data string
/// let data = r#"[35,1]"#;
///
/// // Here we convert it from an `Unsubscribed` frame, to a string representation.
/// let unsubscribed = to_string(&unsubscribed).unwrap();
///
/// // Confirm that our Unsubscribed frame strings are equal to each other
/// assert_eq!(unsubscribed, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for Unsubscribed
/// ```
/// use wamp_core::messages::Unsubscribed;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[35,1]"#;
///
/// // Here we convert it to an `Unsubscribed` frame
/// let unsubscribed = from_str::<Unsubscribed>(data).unwrap();
///
/// // Confirm that our request_id and subscription deserialized
/// assert_eq!(unsubscribed.request_id, 1);
/// ```
pub struct Unsubscribed {
    pub request_id: u64,
}

#[macro_export]
/// # Unsubscribed Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-unsubscribed-2)
/// Quicly create Unsubscribed message with this macro.
/// ## Examples
/// ```
/// use wamp_core::messages::Unsubscribed;
/// use wamp_core::unsubscribed;
///
/// # let unsubscribed_message1 = unsubscribed!(1);
///
/// let unsubscribed_message = Unsubscribed {
///     request_id: 1
/// };
///
/// # assert_eq!(unsubscribed_message, unsubscribed_message1);
/// ```
macro_rules! unsubscribed {
    ($request_id:expr) => {
        Unsubscribed {
            request_id: $request_id,
        }
    };
}

impl WampMessage for Unsubscribed {
    const ID: u64 = 35;

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

impl Serialize for Unsubscribed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (Self::ID, &self.request_id).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Unsubscribed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UnsubscribedVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);

        impl<'vi> Visitor<'vi> for UnsubscribedVisitor {
            type Value = Unsubscribed;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Unsubscribed components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Unsubscribed, A, _>(&message_id, "Unsubscribed")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "request_id must be present and type u64.",
                )?;
                Ok(Unsubscribed { request_id })
            }
        }

        deserializer.deserialize_struct(
            "Unsubscribed",
            &["request_id", "registration"],
            UnsubscribedVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}
