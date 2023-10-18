use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Welcome - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-welcome-2)
/// Represents an Welcome message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Welcome;
/// use wamp_core::welcome;
/// use serde_json::{json, Value};
/// # let mut welcome_message2 = welcome!(1);
///
/// let welcome_message = Welcome {
///     session: 1,
///     details: Value::Null
/// };
///
/// # assert_eq!(welcome_message, welcome_message2);
/// ```
/// ### Serializer
/// Implements serde Serialize trait for welcome
/// ```
/// use wamp_core::messages::Welcome;
/// use serde_json::{json, to_string};
///
/// // Create an welcome message
/// let welcome = Welcome {
///     session: 1,
///     details: json!({ "key": "value" })
/// };
///
/// // Establish raw json data string
/// let data = r#"[2,1,{"key":"value"}]"#;
///
/// // Here we convert it from an `welcome` frame, to a string representation.
/// let welcome = to_string(&welcome).unwrap();
///
/// // Confirm that our welcome frame strings are equal to each other
/// assert_eq!(welcome, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for welcome
/// ```
/// use wamp_core::messages::Welcome;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[2,1,{}]"#;
///
/// // Here we convert it to an `Welcome` frame
/// let welcome = from_str::<Welcome>(data).unwrap();
///
/// // Confirm that our session and details deserialized
/// assert_eq!(welcome.session, 1);
/// assert_eq!(welcome.details, serde_json::json!({}));
/// ```
pub struct Welcome {
    pub session: u64,
    pub details: Value,
}

#[macro_export]
/// # welcome Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-welcome)
/// Macro that allows for default empty implementation of details object on Cabcel.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Welcome};
/// use wamp_core::welcome;
/// use serde_json::{json, Value};
///
/// // Construct with default empty details object
/// let session = 1;
/// let mut welcome_message = welcome!(session);
/// assert_eq!(welcome_message.details, Value::Null);
///
/// // Construct with custom details
/// let welcome_message2 = welcome!(1, json!({
///     "key": "value"
/// }));
///
/// assert_ne!(welcome_message, welcome_message2);
/// welcome_message.details = json!({ "key": "value" });
/// assert_eq!(welcome_message, welcome_message2);
///
/// // These macro invocations are the same as the following:
/// let welcome_message3 = Welcome {
///     session: 1,
///     details: json!({
///         "key": "value"
///     })
/// };
///
/// assert_eq!(welcome_message, welcome_message3);
/// assert_eq!(welcome_message2, welcome_message3);
/// ```
macro_rules! welcome {
    ($session:expr) => {
        welcome!($session, serde_json::Value::Null)
    };
    ($session:expr, $details:expr) => {
        Welcome {
            session: $session,
            details: $details,
        }
    };
}

impl WampMessage for Welcome {
    const ID: u64 = 2;

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

impl Serialize for Welcome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let details =
            helpers::ser_value_is_object::<S, _>(&self.details, "details must be object like.")?;
        (Self::ID, &self.session, details).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Welcome {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct WelcomeVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for WelcomeVisitor {
            type Value = Welcome;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Welcome frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>,
            {
                let message_id: u64 =
                    helpers::deser_seq_element(&mut seq, "Message ID must be type u64.")?;
                helpers::validate_id::<Welcome, A, _>(&message_id, "Welcome")?;
                let session: u64 =
                    helpers::deser_seq_element(&mut seq, "Request ID must be a u64.")?;
                let details: Value =
                    helpers::deser_seq_element(&mut seq, "details must be a JSON value.")?;
                helpers::deser_value_is_object::<A, _>(&details, "details must be object like.")?;
                Ok(Welcome { session, details })
            }
        }

        deserializer.deserialize_struct(
            "Welcome",
            &["session", "details"],
            WelcomeVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, json, to_string};

    use super::*;

    #[test]
    fn test() {
        let d1 = r#"[2,9129137332,{"roles":{"broker":{}}}]"#;
        let w1 = Welcome {
            session: 9129137332,
            details: json!({"roles": {
                "broker": {}
            }}),
        };
        assert_eq!(w1, from_str(d1).unwrap());
        assert_eq!(d1, to_string(&w1).unwrap());
    }
}
