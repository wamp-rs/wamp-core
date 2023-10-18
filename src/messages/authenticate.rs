use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::{de::Visitor, Deserialize, Serialize};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Authenticate - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-authenticate)
/// Represents an Authentication message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Authenticate;
/// use wamp_core::authenticate;
/// use serde_json::json;
/// # let mut auth_message2 = authenticate!("signature");
///
/// let auth_message = Authenticate {
///     signature: "signature".to_string(),
///     details: json!({})
/// };
///
/// # assert_eq!(auth_message, auth_message2);
/// ```
/// ### Serializer
/// Implements serde Serialize trait for Authenticate
/// ```
/// use wamp_core::messages::Authenticate;
/// use serde_json::{json, to_string};
///
/// // Create an Authenticate message
/// let auth = Authenticate {
///     signature: "signature".to_string(),
///     details: json!({ "key": "value" })
/// };
///
/// // Establish raw json data string
/// let data = r#"[5,"signature",{"key":"value"}]"#;
///
/// // Here we convert it from an `Authenticate` frame, to a string representation.
/// let auth = to_string(&auth).unwrap();
///
/// // Confirm that our auth frame strings are equal to each other
/// assert_eq!(auth, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for Authenticate
/// ```
/// use wamp_core::messages::Authenticate;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[5,"signature",{}]"#;
///
/// // Here we convert it to an `Authenticate` frame
/// let auth = from_str::<Authenticate>(data).unwrap();
///
/// // Confirm that our signature and details deserialized
/// assert_eq!(auth.signature, "signature");
/// assert_eq!(auth.details, serde_json::json!({}));
/// ```
pub struct Authenticate {
    pub signature: String,
    pub details: Value,
}

#[macro_export]
/// # Authenticate Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-authenticate)
/// Macro that allows for default empty implementation of details object on Authenticate.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Authenticate};
/// use wamp_core::authenticate;
/// use serde_json::json;
///
/// // Construct with default empty details object
/// let mut auth_message = authenticate!("signature");
/// assert_eq!(auth_message.details, json!({}));
///
/// // Construct with custom details
/// let auth_message2 = authenticate!("signature", json!({
///     "key": "value"
/// }));
///
/// assert_ne!(auth_message, auth_message2);
/// auth_message.details = json!({ "key": "value" });
/// assert_eq!(auth_message, auth_message2);
///
/// // These macro invocations are the same as the following:
/// let auth_message3 = Authenticate {
///     signature: "signature".to_string(),
///     details: json!({
///         "key": "value"
///     })
/// };
///
/// assert_eq!(auth_message, auth_message3);
/// assert_eq!(auth_message2, auth_message3);
/// ```
macro_rules! authenticate {
    ($signature:expr) => {
        authenticate! {$signature, serde_json::json!({})}
    };

    ($signature:expr, $details:expr) => {
        $crate::messages::Authenticate {
            signature: $signature.to_string(),
            details: $details,
        }
    };
}

impl WampMessage for Authenticate {
    const ID: u64 = 5;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Caller => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Publisher => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Subscriber => &MessageDirection {
                receives: &false,
                sends: &true,
            },
            Roles::Dealer => &MessageDirection {
                receives: &true,
                sends: &false,
            },
            Roles::Broker => &MessageDirection {
                receives: &true,
                sends: &false,
            },
        }
    }
}

impl Serialize for Authenticate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let details =
            helpers::ser_value_is_object::<S, _>(&self.details, "Details must be object like.")?;
        (Self::ID, &self.signature, details).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Authenticate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AuthenticateVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for AuthenticateVisitor {
            type Value = Authenticate;
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
                helpers::validate_id::<Authenticate, A, _>(&message_id, "Authenticate")?;
                let signature: String =
                    helpers::deser_seq_element(&mut seq, "Signature must be type String.")?;
                let details: Value = helpers::deser_seq_element(
                    &mut seq,
                    "Details must be present and object like.",
                )?;
                helpers::deser_value_is_object::<A, _>(&details, "Value must be object like")?;
                Ok(Authenticate { signature, details })
            }
        }

        deserializer.deserialize_struct(
            "Authenticate",
            &["signature", "details"],
            AuthenticateVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}
