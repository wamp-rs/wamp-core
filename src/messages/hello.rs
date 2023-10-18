use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Hello - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-hello-2)
/// Represents an Hello message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Hello;
/// use wamp_core::hello;
/// use serde_json::json;
/// # let mut hello_message2 = hello!("realm");
///
/// let hello_message = Hello {
///     realm: "realm".to_string(),
///     details: json!({})
/// };
///
/// # assert_eq!(hello_message, hello_message2);
/// ```
/// ### Serializer
/// ```
/// use wamp_core::messages::Hello;
/// use serde_json::{json, to_string};
///
/// // Create an Hello message
/// let hello = Hello {
///     realm: "realm".to_string(),
///     details: json!({ "key": "value" })
/// };
///
/// // Establish raw json data string
/// let data = r#"[1,"realm",{"key":"value"}]"#;
///
/// // Here we convert it from an `Hello` frame, to a string representation.
/// let hello = to_string(&hello).unwrap();
///
/// // Confirm that our hello frame strings are equal to each other
/// assert_eq!(hello, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for hello
/// ```
/// use wamp_core::messages::Hello;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[1,"realm",{}]"#;
///
/// // Here we convert it to an `Hello` frame
/// let hello = from_str::<Hello>(data).unwrap();
///
/// // Confirm that our realm and details deserialized
/// assert_eq!(hello.realm, "realm");
/// assert_eq!(hello.details, serde_json::json!({}));
/// ```
/// Implements serde Serialize trait for hello
pub struct Hello {
    pub realm: String,
    pub details: Value,
}

#[macro_export]
/// # Hello Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-hello-2)
/// Macro that allows for default empty implementation of details object on hello.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Hello};
/// use wamp_core::hello;
/// use serde_json::json;
///
/// // Construct with default empty details object
/// let mut hello_message = hello!("realm");
/// assert_eq!(hello_message.details, json!({}));
///
/// // Construct with custom details
/// let hello_message2 = hello!("realm", json!({
///     "key": "value"
/// }));
///
/// assert_ne!(hello_message, hello_message2);
/// hello_message.details = json!({ "key": "value" });
/// assert_eq!(hello_message, hello_message2);
///
/// // These macro invocations are the same as the following:
/// let hello_message3 = Hello {
///     realm: "realm".to_string(),
///     details: json!({
///         "key": "value"
///     })
/// };
///
/// assert_eq!(hello_message, hello_message3);
/// assert_eq!(hello_message2, hello_message3);
/// ```
macro_rules! hello {
    ($realm:expr) => {
        hello! {$realm, {serde_json::json!({})}}
    };

    ($realm:expr, $details:expr) => {
        Hello {
            realm: $realm.to_string(),
            details: $details,
        }
    };
}

impl WampMessage for Hello {
    const ID: u64 = 1;

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

impl Serialize for Hello {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let details =
            helpers::ser_value_is_object::<S, _>(&self.details, "Details must be object like.")?;
        (Self::ID, &self.realm, &details).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Hello {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HelloVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for HelloVisitor {
            type Value = Hello;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Hello frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>,
            {
                let message_id: u64 =
                    helpers::deser_seq_element(&mut seq, "Message ID must be type u8.")?;
                helpers::validate_id::<Hello, A, _>(&message_id, "Hello")?;
                let realm: String =
                    helpers::deser_seq_element(&mut seq, "realm must be a String.")?;
                let details: Value =
                    helpers::deser_seq_element(&mut seq, "Details must be a JSON value.")?;
                helpers::deser_value_is_object::<A, _>(&details, "Details must be object like.")?;
                Ok(Hello { realm, details })
            }
        }

        deserializer.deserialize_struct(
            "Hello",
            &["realm", "details"],
            HelloVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}
