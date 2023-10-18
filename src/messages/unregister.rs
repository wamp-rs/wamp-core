use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Unregister - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-unregister-2)
/// Represents an Unregister message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Unregister;
/// use wamp_core::unregister;
/// use serde_json::json;
///
/// # let mut unregister_message2 = unregister!(2);
///
/// let unregister_message = Unregister {
///     request_id: 1,
///     registration: 2
/// };
///
/// # assert_eq!(unregister_message, unregister_message2);
/// ```
/// ### Serializer
/// Implements serde Serialize trait for Unregister
/// ```
/// use wamp_core::messages::Unregister;
/// use serde_json::{json, to_string};
///
/// // Create an unregister message
/// let unregister = Unregister {
///     request_id: 1,
///     registration: 2
/// };
///
/// // Establish raw json data string
/// let data = r#"[66,1,2]"#;
///
/// // Here we convert it from an `Unregister` frame, to a string representation.
/// let unregister = to_string(&unregister).unwrap();
///
/// // Confirm that our Unregister frame strings are equal to each other
/// assert_eq!(unregister, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for Unregister
/// ```
/// use wamp_core::messages::Unregister;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[66,1,2]"#;
///
/// // Here we convert it to an `Unregister` frame
/// let unregister = from_str::<Unregister>(data).unwrap();
///
/// // Confirm that our request_id and registration deserialized
/// assert_eq!(unregister.request_id, 1);
/// assert_eq!(unregister.registration, 2);
/// ```

pub struct Unregister {
    pub request_id: u64,
    pub registration: u64,
}

#[macro_export]
/// # unregister Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-unregister-2)
/// Macro that allows for creating unregister wamp message with auto incrementing request id.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Unregister};
/// use wamp_core::unregister;
/// use serde_json::json;
///
/// let mut unregister_message = unregister!(2);
/// let unregister_message2 = unregister!(3);
///
/// assert_ne!(unregister_message, unregister_message2);
///
/// // These macro invocations are the same as the following:
/// let unregister_message3 = Unregister {
///     request_id: 1,
///     registration: 2
/// };
///
/// assert_eq!(unregister_message, unregister_message3);
/// assert_ne!(unregister_message2, unregister_message3);
/// ```
macro_rules! unregister {
    ($registration:expr) => {
        Unregister {
            request_id: $crate::factories::increment(),
            registration: $registration,
        }
    };
}

impl WampMessage for Unregister {
    const ID: u64 = 66;

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

impl Serialize for Unregister {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (Self::ID, &self.request_id, &self.registration).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Unregister {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UnregisterVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);

        impl<'vi> Visitor<'vi> for UnregisterVisitor {
            type Value = Unregister;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Unregister components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Unregister, A, _>(&message_id, "Unregister")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "request_id must be present and type u64.",
                )?;
                let registration: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "registration must be present and object like.",
                )?;
                Ok(Unregister {
                    request_id,
                    registration,
                })
            }
        }

        deserializer.deserialize_struct(
            "Unregister",
            &["request_id", "registration"],
            UnregisterVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Unregister;

    #[test]
    fn test() {
        let d1 = r#"[66,788923562,2103333224]"#;
        let p1 = Unregister {
            request_id: 788923562,
            registration: 2103333224,
        };
        assert_eq!(d1, to_string(&p1).unwrap());
        assert_eq!(from_str::<Unregister>(d1).unwrap(), p1);
    }
}
