use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Registered - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-registered-2)
/// Represents an Registered message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Registered;
/// use wamp_core::registered;
/// use serde_json::json;
///
/// # let mut registered_message2 = registered!(1, 2);
///
/// let registered_message = Registered {
///     request_id: 1,
///     registration: 2
/// };
///
/// # assert_eq!(registered_message, registered_message2);
/// ```
/// ### Serializer
/// Implements serde Serialize trait for registered
/// ```
/// use wamp_core::messages::Registered;
/// use serde_json::{json, to_string};
///
/// // Create an registered message
/// let registered = Registered {
///     request_id: 1,
///     registration: 2
/// };
///
/// // Establish raw json data string
/// let data = r#"[65,1,2]"#;
///
/// // Here we convert it from an `registered` frame, to a string representation.
/// let registered = to_string(&registered).unwrap();
///
/// // Confirm that our registered frame strings are equal to each other
/// assert_eq!(registered, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for registered
/// ```
/// use wamp_core::messages::Registered;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[65,1,2]"#;
///
/// // Here we convert it to an `Registered` frame
/// let registered = from_str::<Registered>(data).unwrap();
///
/// // Confirm that our request_id and registration deserialized
/// assert_eq!(registered.request_id, 1);
/// assert_eq!(registered.registration, 2);
/// ```
pub struct Registered {
    pub request_id: u64,
    pub registration: u64,
}

#[macro_export]
/// # Registered Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-registered-2)
/// Macro that allows for creating Registered wamp message.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Registered};
/// use wamp_core::registered;
/// use serde_json::json;
///
/// let mut registered_message = registered!(1, 2);
/// let registered_message2 = registered!(1, 3);
///
/// assert_ne!(registered_message, registered_message2);
///
/// // These macro invocations are the same as the following:
/// let registered_message3 = Registered {
///     request_id: 1,
///     registration: 2
/// };
///
/// assert_eq!(registered_message, registered_message3);
/// assert_ne!(registered_message2, registered_message3);
/// ```
macro_rules! registered {
    ($request_id:expr, $registration:expr) => {
        Registered {
            request_id: $request_id,
            registration: $registration,
        }
    };
}

impl WampMessage for Registered {
    const ID: u64 = 65;

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

impl Serialize for Registered {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (Self::ID, &self.request_id, &self.registration).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Registered {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RegisteredVisitor(PhantomData<u64>, PhantomData<u64>, PhantomData<u64>);

        impl<'vi> Visitor<'vi> for RegisteredVisitor {
            type Value = Registered;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Registered components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Registered, A, _>(&message_id, "Registered")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "request_id must be present and type u64.",
                )?;
                let registration: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "registration must be present and object like.",
                )?;
                Ok(Registered {
                    request_id,
                    registration,
                })
            }
        }

        deserializer.deserialize_struct(
            "Registered",
            &["request_id", "registration"],
            RegisteredVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Registered;

    #[test]
    fn test() {
        let d1 = r#"[65,25349185,2103333224]"#;
        let p1 = Registered {
            request_id: 25349185,
            registration: 2103333224,
        };
        assert_eq!(d1, to_string(&p1).unwrap());
        assert_eq!(from_str::<Registered>(d1).unwrap(), p1);
    }
}
