use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Cancel - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-cancel)
/// Represents an Cancel message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Cancel;
/// use wamp_core::cancel;
/// use serde_json::json;
/// # let mut cancel_message2 = cancel!(1);
///
/// let cancel_message = Cancel {
///     request_id: 1,
///     options: json!({})
/// };
///
/// # assert_eq!(cancel_message, cancel_message2);
/// ```
/// ### Serializer
/// Implements serde Serialize trait for Cancel
/// ```
/// use wamp_core::messages::Cancel;
/// use serde_json::{json, to_string};
///
/// // Create an Cancel message
/// let cancel = Cancel {
///     request_id: 1,
///     options: json!({ "key": "value" })
/// };
///
/// // Establish raw json data string
/// let data = r#"[49,1,{"key":"value"}]"#;
///
/// // Here we convert it from an `Cancel` frame, to a string representation.
/// let cancel = to_string(&cancel).unwrap();
///
/// // Confirm that our Cancel frame strings are equal to each other
/// assert_eq!(cancel, data);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for Cancel
/// ```
/// use wamp_core::messages::Cancel;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[49,1,{}]"#;
///
/// // Here we convert it to an `Cancel` frame
/// let cancel = from_str::<Cancel>(data).unwrap();
///
/// // Confirm that our request_id and options deserialized
/// assert_eq!(cancel.request_id, 1);
/// assert_eq!(cancel.options, serde_json::json!({}));
/// ```
pub struct Cancel {
    pub request_id: u64,
    pub options: Value,
}

#[macro_export]
/// # Cancel Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-cancel)
/// Macro that allows for default empty implementation of options object on Cabcel.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Cancel};
/// use wamp_core::cancel;
/// use serde_json::json;
///
/// // Construct with default empty options object
/// let request_id = 1;
/// let mut cancel_message = cancel!(request_id);
/// assert_eq!(cancel_message.options, json!({}));
///
/// // Construct with custom options
/// let cancel_message2 = cancel!(1, json!({
///     "key": "value"
/// }));
///
/// assert_ne!(cancel_message, cancel_message2);
/// cancel_message.options = json!({ "key": "value" });
/// assert_eq!(cancel_message, cancel_message2);
///
/// // These macro invocations are the same as the following:
/// let cancel_message3 = Cancel {
///     request_id: 1,
///     options: json!({
///         "key": "value"
///     })
/// };
///
/// assert_eq!(cancel_message, cancel_message3);
/// assert_eq!(cancel_message2, cancel_message3);
/// ```
macro_rules! cancel {
    ($request_id: expr) => {
        cancel!($request_id, serde_json::json!({}))
    };
    ($request_id: expr, $options:expr) => {
        Cancel {
            request_id: $request_id,
            options: $options,
        }
    };
}

impl WampMessage for Cancel {
    const ID: u64 = 49;

    fn direction(role: Roles) -> &'static MessageDirection {
        match role {
            Roles::Callee => &MessageDirection {
                receives: &false,
                sends: &false,
            },
            Roles::Caller => &MessageDirection {
                receives: &false,
                sends: &true,
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

impl Serialize for Cancel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let options =
            helpers::ser_value_is_object::<S, _>(&self.options, "Options must be object like.")?;
        (Self::ID, &self.request_id, options).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Cancel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CancelVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for CancelVisitor {
            type Value = Cancel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Cancel frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>,
            {
                let message_id: u64 =
                    helpers::deser_seq_element(&mut seq, "Message ID must be type u64.")?;
                helpers::validate_id::<Cancel, A, _>(&message_id, "Cancel")?;
                let request_id: u64 =
                    helpers::deser_seq_element(&mut seq, "Request ID must be a u64.")?;
                let options: Value =
                    helpers::deser_seq_element(&mut seq, "Options must be a JSON value.")?;
                helpers::deser_value_is_object::<A, _>(&options, "Options must be object like.")?;
                Ok(Cancel {
                    request_id,
                    options,
                })
            }
        }

        deserializer.deserialize_struct(
            "Cancel",
            &["request_id", "options"],
            CancelVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}
