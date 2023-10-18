use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;
use std::marker::PhantomData;

use super::{helpers, MessageDirection, WampMessage};

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Published - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-published-2)
/// Represents an published message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Published;
/// use wamp_core::published;
/// use serde_json::json;
///
/// # let mut published_message2 = published!(1, 2);
///
/// let published_message = Published {
///     request_id: 1,
///     publication: 2
/// };
///
/// # assert_eq!(published_message, published_message2);
/// ```
/// ### Serializer
/// Implements serde Serialize trait for published
/// ```
/// use wamp_core::messages::Published;
/// use serde_json::{json, to_string};
///
/// // Create an published message
/// let published = Published {
///     request_id: 1,
///     publication: 2
/// };
///
/// // Establish raw json data string
/// let data = r#"[17,1,2]"#;
///
/// // Here we convert it from an `published` frame, to a string representation.
/// let published = to_string(&published).unwrap();
///
/// // Confirm that our published frame strings are equal to each other
/// assert_eq!(published, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for published
/// ```
/// use wamp_core::messages::Published;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[17,1,2]"#;
///
/// // Here we convert it to an `published` frame
/// let published = from_str::<Published>(data).unwrap();
///
/// // Confirm that our request_id and publication deserialized
/// assert_eq!(published.request_id, 1);
/// assert_eq!(published.publication, 2);
/// ```
pub struct Published {
    pub request_id: u64,
    pub publication: u64,
}

#[macro_export]
/// # Published Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-published-2)
/// Macro that allows for default empty implementation of publication object on Published.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Published};
/// use wamp_core::published;
/// use serde_json::json;
///
/// let mut published_message = published!(1, 2);
/// let published_message2 = published!(1, 3);
///
/// assert_ne!(published_message, published_message2);
///
/// // These macro invocations are the same as the following:
/// let published_message3 = Published {
///     request_id: 1,
///     publication: 2
/// };
///
/// assert_eq!(published_message, published_message3);
/// assert_ne!(published_message2, published_message3);
/// ```
macro_rules! published {
    ($request_id:expr, $publication:expr) => {
        Published {
            request_id: $request_id,
            publication: $publication,
        }
    };
}

impl WampMessage for Published {
    const ID: u64 = 17;

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
                receives: &true,
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

impl Serialize for Published {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (Self::ID, &self.request_id, &self.publication).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Published {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PublishedVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);

        impl<'vi> Visitor<'vi> for PublishedVisitor {
            type Value = Published;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Published components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Published, A, _>(&message_id, "Published")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "request_id must be present and type u64.",
                )?;
                let publication: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "publication must be present and object like.",
                )?;
                Ok(Published {
                    request_id,
                    publication,
                })
            }
        }

        deserializer.deserialize_struct(
            "Published",
            &["request_id", "publication"],
            PublishedVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Published;

    #[test]
    fn test() {
        let d1 = r#"[17,239714735,4429313566]"#;
        let p1 = Published {
            request_id: 239714735,
            publication: 4429313566,
        };
        assert_eq!(d1, to_string(&p1).unwrap());
        assert_eq!(from_str::<Published>(d1).unwrap(), p1);
    }
}
