use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Unsubscribe - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-unsubscribe-2)
/// Represents an Unsubscribe message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Unsubscribe;
/// use wamp_core::unsubscribe;
/// use serde_json::json;
///
/// # let mut unsubscribe_message2 = unsubscribe!(2);
///
/// let unsubscribe_message = Unsubscribe {
///     request_id: 1,
///     subscription: 2
/// };
///
/// # assert_eq!(unsubscribe_message, unsubscribe_message2);
/// ```
///
///
/// ### Serializer
/// Implements serde Serialize trait for Unsubscribe
/// ```
/// use wamp_core::messages::Unsubscribe;
/// use serde_json::{json, to_string};
///
/// // Create an unsubscribe message
/// let unsubscribe = Unsubscribe {
///     request_id: 1,
///     subscription: 2
/// };
///
/// // Establish raw json data string
/// let data = r#"[34,1,2]"#;
///
/// // Here we convert it from an `Unsubscribe` frame, to a string representation.
/// let unsubscribe = to_string(&unsubscribe).unwrap();
///
/// // Confirm that our unsubscribe frame strings are equal to each other
/// assert_eq!(unsubscribe, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for Unsubscribe
/// ```
/// use wamp_core::messages::Unsubscribe;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[34,1,2]"#;
///
/// // Here we convert it to an `Unsubscribe` frame
/// let unsubscribe = from_str::<Unsubscribe>(data).unwrap();
///
/// // Confirm that our request_id and subscription deserialized
/// assert_eq!(unsubscribe.request_id, 1);
/// assert_eq!(unsubscribe.subscription, 2);
/// ```
pub struct Unsubscribe {
    pub request_id: u64,
    pub subscription: u64,
}

#[macro_export]
/// # unsubscribe Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-unsubscribe-2)
/// Macro that allows for creating auto incrementing Unsubscribe wamp message.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Unsubscribe};
/// use wamp_core::unsubscribe;
/// use serde_json::json;
///
/// let mut unsubscribe_message = unsubscribe!(2);
/// let unsubscribe_message2 = unsubscribe!(3);
///
/// assert_ne!(unsubscribe_message, unsubscribe_message2);
///
/// // These macro invocations are the same as the following:
/// let unsubscribe_message3 = Unsubscribe {
///     request_id: 1,
///     subscription: 2
/// };
///
/// assert_eq!(unsubscribe_message, unsubscribe_message3);
/// assert_ne!(unsubscribe_message2, unsubscribe_message3);
/// ```
macro_rules! unsubscribe {
    ($subscription:expr) => {
        Unsubscribe {
            request_id: $crate::factories::increment(),
            subscription: $subscription,
        }
    };
}

impl WampMessage for Unsubscribe {
    const ID: u64 = 34;

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
                receives: &false,
                sends: &true,
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

impl Serialize for Unsubscribe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (Self::ID, &self.request_id, &self.subscription).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Unsubscribe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UnsubscribeVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);

        impl<'vi> Visitor<'vi> for UnsubscribeVisitor {
            type Value = Unsubscribe;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Unsubscribe components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Unsubscribe, A, _>(&message_id, "Unsubscribe")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "request_id must be present and type u64.",
                )?;
                let subscription: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "subscription must be present and object like.",
                )?;
                Ok(Unsubscribe {
                    request_id,
                    subscription,
                })
            }
        }

        deserializer.deserialize_struct(
            "Unsubscribe",
            &["request_id", "subscription"],
            UnsubscribeVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Unsubscribe;

    #[test]
    fn test() {
        let d1 = r#"[34,85346237,5512315355]"#;
        let p1 = Unsubscribe {
            request_id: 85346237,
            subscription: 5512315355,
        };
        assert_eq!(d1, to_string(&p1).unwrap());
        assert_eq!(from_str::<Unsubscribe>(d1).unwrap(), p1);
    }
}
