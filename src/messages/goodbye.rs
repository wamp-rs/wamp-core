use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Goodbye - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-goodbye-2)
/// represenets an goodbye message in wamp protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Goodbye;
/// use wamp_core::goodbye;
/// use serde_json::json;
/// # let mut goodbye_message2 = goodbye!("wamp.close.system_shutdown");
///
/// let goodbye_message = Goodbye {
///     reason: "wamp.close.system_shutdown".to_string(),
///     details: json!({})
/// };
///
/// # assert_eq!(goodbye_message, goodbye_message2);
/// ```
///
/// ### Serializer
/// Implements serde Serialize trait for Goodbye
/// ```
/// use wamp_core::messages::Goodbye;
/// use serde_json::{json, to_string};
///
/// // Create an goodbye message
/// let goodbye = Goodbye {
///     details: json!({"message": "The host is shutting down now."}),
///     reason: "wamp.close.system_shutdown".to_string()
/// };
///
/// // Establish raw json data string
/// let data = r#"[6,{"message":"The host is shutting down now."},"wamp.close.system_shutdown"]"#;
///
/// // Here we convert it from an `Goodbye` frame, to a string representation.
/// let goodbye = to_string(&goodbye).unwrap();
///
/// // Confirm that our Goodbye frame strings are equal to eachother
/// assert_eq!(goodbye, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for Goodbye
/// ```
/// use wamp_core::messages::Goodbye;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[6,{"message": "The host is shutting down now."},"wamp.close.system_shutdown"]"#;
///
/// // Here we convert it to an `goodbye` frame
/// let goodbye = from_str::<Goodbye>(data).unwrap();
///
/// // Confirm that our error type deserialized
/// assert_eq!(goodbye.reason, "wamp.close.system_shutdown");
/// ```
pub struct Goodbye {
    pub details: Value,
    pub reason: String,
}

#[macro_export]
/// # Goodbye Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-goodbye-2)
/// Macro that allows for default empty implementation of details object on Goodbye.
/// ## Examples
/// ```
/// use wamp_core::messages::Goodbye;
/// use wamp_core::goodbye;
/// use serde_json::json;
///
/// // Construct with default empty details object
/// let mut goodbye_message = goodbye!("wamp.close.system_shutdown");
/// assert_eq!(goodbye_message.details, json!({}));
///
/// // Construct with custom details
/// let goodbye_message2 = goodbye!("wamp.close.system_shutdown", json!({
///     "message": "The host is shutting down now."
/// }));
///
/// assert_ne!(goodbye_message, goodbye_message2);
/// goodbye_message.details = json!({ "message": "The host is shutting down now." });
/// assert_eq!(goodbye_message, goodbye_message2);
///
/// // These macro invocations are the same as the following:
/// let goodbye_message3 = Goodbye {
///     reason: "wamp.close.system_shutdown".to_string(),
///     details: json!({
///         "message": "The host is shutting down now."
///     })
/// };
///
/// assert_eq!(goodbye_message, goodbye_message3);
/// assert_eq!(goodbye_message2, goodbye_message3);
/// ```
macro_rules! goodbye {
    ($reason:expr) => {
        goodbye! {$reason, serde_json::json!({})}
    };

    ($reason:expr, $details:expr) => {
        Goodbye {
            details: $details,
            reason: $reason.to_string(),
        }
    };
}

impl WampMessage for Goodbye {
    const ID: u64 = 6;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &true,
                sends: &true,
            },
            Roles::Caller => &MessageDirection {
                receives: &true,
                sends: &true,
            },
            Roles::Publisher => &MessageDirection {
                receives: &true,
                sends: &true,
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
                receives: &true,
                sends: &true,
            },
        }
    }
}

impl Serialize for Goodbye {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let details =
            helpers::ser_value_is_object::<S, _>(&self.details, "Details must be object like.")?;
        (Self::ID, &details, &self.reason).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Goodbye {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct GoodbyeVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for GoodbyeVisitor {
            type Value = Goodbye;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Goodbye frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>,
            {
                let message_id: u64 =
                    helpers::deser_seq_element(&mut seq, "Message ID must be type u8.")?;
                helpers::validate_id::<Goodbye, A, _>(&message_id, "Goodbye")?;
                let details: Value =
                    helpers::deser_seq_element(&mut seq, "Details must be a JSON value.")?;
                let reason: String =
                    helpers::deser_seq_element(&mut seq, "Reason must be a String.")?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                Ok(Goodbye { reason, details })
            }
        }

        deserializer.deserialize_struct(
            "Goodbye",
            &["reason", "details"],
            GoodbyeVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::Goodbye;

    #[test]
    fn test() {
        let d1 = r#"[6,{"message":"The host is shutting down now."},"wamp.close.system_shutdown"]"#;
        let g1 = Goodbye {
            details: serde_json::json!({"message":"The host is shutting down now."}),
            reason: "wamp.close.system_shutdown".to_string(),
        };
        let d2 = to_string(&g1).unwrap();
        let g2: Goodbye = from_str(d1).unwrap();
        assert_eq!(d1, d2);
        assert_eq!(g1, g2);
    }
}
