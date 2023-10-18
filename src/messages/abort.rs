use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Abort - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-abort-2)
/// Represents an Abort message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Abort;
/// use wamp_core::abort;
/// use serde_json::json;
/// # let mut abort_message2 = abort!("wamp.error.no_such_realm");
///
/// let abort_message = Abort {
///     reason: "wamp.error.no_such_realm".to_string(),
///     details: json!({})
/// };
///
/// # assert_eq!(abort_message, abort_message2);
/// ```
/// ### Serializer
/// Implements serde Serialize trait for Abort
/// ```
/// use wamp_core::messages::Abort;
/// use serde_json::{json, to_string};
///
/// // Create an Abort message
/// let abort = Abort {
///     details: json!({ "message": "The realm does not exist." }),
///     reason: "wamp.error.no_such_realm".to_string()
/// };
///
/// // Establish raw json data string
/// let data = r#"[3,{"message":"The realm does not exist."},"wamp.error.no_such_realm"]"#;
///
/// // Here we convert it from an `Abort` frame, to a string representation.
/// let abort = to_string(&abort).unwrap();
///
/// // Confirm that our abort frame strings are equal to eachother
/// assert_eq!(abort, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for Abort
/// ```
/// use wamp_core::messages::Abort;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[3,{"message":"The realm does not exist."},"wamp.error.no_such_realm"]"#;
///
/// // Here we convert it to an `Abort` frame
/// let abort = from_str::<Abort>(data).unwrap();
///
/// // Confirm that our error type deserialized
/// assert_eq!(abort.reason, "wamp.error.no_such_realm");
/// ```
pub struct Abort {
    pub details: Value,
    pub reason: String,
}

#[macro_export]
/// # Abort Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-abort-2)
/// Abort macro allows for default empty implementation of details object on Abort.
/// ## Examples
/// ```
/// use wamp_core::messages::Abort;
/// use wamp_core::abort;
/// use serde_json::json;
///
/// // Construct with default empty details object
/// let mut abort_message = abort!("wamp.error.no_such_realm");
/// assert_eq!(abort_message.details, json!({}));
///
/// // Construct with custom details
/// let abort_message2 = abort!("wamp.error.no_such_realm", json!({
///     "message": "The realm does not exist."
/// }));
///
/// assert_ne!(abort_message, abort_message2);
/// abort_message.details = json!({ "message": "The realm does not exist." });
/// assert_eq!(abort_message, abort_message2);
///
/// // These macro invocations are the same as the following:
/// let abort_message3 = Abort {
///     reason: "wamp.error.no_such_realm".to_string(),
///     details: json!({
///         "message": "The realm does not exist."
///     })
/// };
///
/// assert_eq!(abort_message, abort_message3);
/// assert_eq!(abort_message2, abort_message3);
/// ```
macro_rules! abort {
    ($reason:expr) => {
        abort! {$reason, serde_json::json!({})}
    };

    ($reason:expr, $details:expr) => {
        Abort {
            details: $details,
            reason: $reason.to_string(),
        }
    };
}

impl WampMessage for Abort {
    const ID: u64 = 3;

    fn direction(role: crate::roles::Roles) -> &'static super::MessageDirection {
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

impl Serialize for Abort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let details =
            helpers::ser_value_is_object::<S, _>(&self.details, "Details must be object like.")?;
        (Self::ID, &details, &self.reason).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Abort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AbortVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for AbortVisitor {
            type Value = Abort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Abort frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>,
            {
                let message_id: u64 =
                    helpers::deser_seq_element(&mut seq, "Message ID must be type u8.")?;
                helpers::validate_id::<Abort, A, _>(&message_id, "Abort")?;
                let details: Value =
                    helpers::deser_seq_element(&mut seq, "Details must be a JSON value.")?;
                let reason: String =
                    helpers::deser_seq_element(&mut seq, "Reason must be a String.")?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                Ok(Abort { reason, details })
            }
        }

        deserializer.deserialize_struct(
            "Abort",
            &["reason", "details"],
            AbortVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}
