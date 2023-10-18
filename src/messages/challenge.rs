use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::{de::Visitor, Deserialize, Serialize};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Challenge - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-challenge)
/// Represents an Challenge message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Challenge;
/// use wamp_core::challenge;
/// use serde_json::json;
/// # let mut challenge_message2 = challenge!("authmethod");
///
/// let challenge_message = Challenge {
///     authmethod: "authmethod".to_string(),
///     details: json!({})
/// };
///
/// # assert_eq!(challenge_message, challenge_message2);
/// ```
/// ### Serializer
/// Implements serde Serialize trait for Challenge
/// ```
/// use wamp_core::messages::Challenge;
/// use serde_json::{json, to_string};
///
/// // Create an Challenge message
/// let challenge = Challenge {
///     authmethod: "authmethod".to_string(),
///     details: json!({ "key": "value" })
/// };
///
/// // Establish raw json data string
/// let data = r#"[4,"authmethod",{"key":"value"}]"#;
///
/// // Here we convert it from an `Challenge` frame, to a string representation.
/// let challenge = to_string(&challenge).unwrap();
///
/// // Confirm that our challenge frame strings are equal to each other
/// assert_eq!(challenge, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for Challenge
/// ```
/// use wamp_core::messages::Challenge;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[4,"authmethod",{}]"#;
///
/// // Here we convert it to an `Challenge` frame
/// let challenge = from_str::<Challenge>(data).unwrap();
///
/// // Confirm that our authmethod and details deserialized
/// assert_eq!(challenge.authmethod, "authmethod");
/// assert_eq!(challenge.details, serde_json::json!({}));
/// ```
pub struct Challenge {
    pub authmethod: String,
    pub details: Value,
}

#[macro_export]
/// # Challenge Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-challenge)
/// Macro that allows for default empty implementation of details object on Challenge.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Challenge};
/// use wamp_core::challenge;
/// use serde_json::json;
///
/// // Construct with default empty details object
/// let mut challenge_message = challenge!("authmethod");
/// assert_eq!(challenge_message.details, json!({}));
///
/// // Construct with custom details
/// let challenge_message2 = challenge!("authmethod", json!({
///     "key": "value"
/// }));
///
/// assert_ne!(challenge_message, challenge_message2);
/// challenge_message.details = json!({ "key": "value" });
/// assert_eq!(challenge_message, challenge_message2);
///
/// // These macro invocations are the same as the following:
/// let challenge_message3 = Challenge {
///     authmethod: "authmethod".to_string(),
///     details: json!({
///         "key": "value"
///     })
/// };
///
/// assert_eq!(challenge_message, challenge_message3);
/// assert_eq!(challenge_message2, challenge_message3);
/// ```
macro_rules! challenge {
    ($authmethod:expr) => {
        challenge! {$authmethod, serde_json::json!({})}
    };
    ($authmethod:expr, $details:expr) => {
        Challenge {
            authmethod: $authmethod.to_string(),
            details: $details,
        }
    };
}

impl WampMessage for Challenge {
    const ID: u64 = 4;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &true,
                sends: &false,
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
                receives: &false,
                sends: &true,
            },
            Roles::Broker => &MessageDirection {
                receives: &false,
                sends: &true,
            },
        }
    }
}

impl Serialize for Challenge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let details =
            helpers::ser_value_is_object::<S, _>(&self.details, "Details must be object like.")?;
        (Self::ID, &self.authmethod, details).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Challenge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ChallengeVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for ChallengeVisitor {
            type Value = Challenge;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Wamp message containing authentication details")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message ID must be present and type u8.",
                )?;
                helpers::validate_id::<Challenge, A, _>(&message_id, "Challenge")?;
                let authmethod: String =
                    helpers::deser_seq_element(&mut seq, "authmethod must be type String.")?;
                let details: Value = helpers::deser_seq_element(
                    &mut seq,
                    "Details must be present and object like.",
                )?;
                helpers::deser_value_is_object::<A, _>(&details, "Value must be object like")?;
                Ok(Challenge {
                    authmethod,
                    details,
                })
            }
        }

        deserializer.deserialize_struct(
            "Challenge",
            &["authmethod", "details"],
            ChallengeVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}
