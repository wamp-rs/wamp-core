use super::{helpers, MessageDirection, WampMessage};
use crate::roles::Roles;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Interrupt - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-interrupt)
/// Represents an interrupt message in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Interrupt;
/// use wamp_core::interrupt;
/// use serde_json::json;
/// # let mut interrupt_message2 = interrupt!(1);
///
/// let interrupt_message = Interrupt {
///     request_id: 1,
///     options: json!({})
/// };
///
/// # assert_eq!(interrupt_message, interrupt_message2);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for interrupt
/// ```
/// use wamp_core::messages::Interrupt;
/// use serde_json::from_str;
///
/// // Here is our raw json data string
/// let data = r#"[69,1,{}]"#;
///
/// // Here we convert it to an `interrupt` frame
/// let interrupt = from_str::<Interrupt>(data).unwrap();
///
/// // Confirm that our request_id and options deserialized
/// assert_eq!(interrupt.request_id, 1);
/// assert_eq!(interrupt.options, serde_json::json!({}));
/// ```
/// ### Serializer
/// Implements serde Serialize trait for interrupt
/// ```
/// use wamp_core::messages::Interrupt;
/// use serde_json::{json, to_string};
///
/// // Create an interrupt message
/// let interrupt = Interrupt {
///     request_id: 1,
///     options: json!({ "key": "value" })
/// };
///
/// // Establish raw json data string
/// let data = r#"[69,1,{"key":"value"}]"#;
///
/// // Here we convert it from an `interrupt` frame, to a string representation.
/// let interrupt = to_string(&interrupt).unwrap();
///
/// // Confirm that our interrupt frame strings are equal to each other
/// assert_eq!(interrupt, data);
/// ```
pub struct Interrupt {
    pub request_id: u64,
    pub options: Value,
}

#[macro_export]
/// # Interrupt Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-interrupt)
/// Macro that allows for default empty implementation of options object on Cabcel.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Interrupt};
/// use wamp_core::interrupt;
/// use serde_json::json;
///
/// // Construct with default empty options object
/// let request_id = 1;
/// let mut interrupt_message = interrupt!(request_id);
/// assert_eq!(interrupt_message.options, json!({}));
///
/// // Construct with custom options
/// let interrupt_message2 = interrupt!(1, json!({
///     "key": "value"
/// }));
///
/// assert_ne!(interrupt_message, interrupt_message2);
/// interrupt_message.options = json!({ "key": "value" });
/// assert_eq!(interrupt_message, interrupt_message2);
///
/// // These macro invocations are the same as the following:
/// let interrupt_message3 = Interrupt {
///     request_id: 1,
///     options: json!({
///         "key": "value"
///     })
/// };
///
/// assert_eq!(interrupt_message, interrupt_message3);
/// assert_eq!(interrupt_message2, interrupt_message3);
/// ```
macro_rules! interrupt {
    ($request_id:expr) => {
        interrupt! {$request_id, serde_json::json!({})}
    };
    ($request_id:expr, $options:expr) => {
        Interrupt {
            request_id: $request_id,
            options: $options,
        }
    };
}

impl WampMessage for Interrupt {
    const ID: u64 = 69;

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

impl Serialize for Interrupt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let options =
            helpers::ser_value_is_object::<S, _>(&self.options, "Options must be object like.")?;
        (Self::ID, &self.request_id, options).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Interrupt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct InterruptVisitor(PhantomData<u8>, PhantomData<String>, PhantomData<Value>);

        impl<'vi> Visitor<'vi> for InterruptVisitor {
            type Value = Interrupt;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("WAMP Interrupt frame, expressed as a sequence.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'vi>,
            {
                let message_id: u64 =
                    helpers::deser_seq_element(&mut seq, "Message ID must be type u64.")?;
                helpers::validate_id::<Interrupt, A, _>(&message_id, "Interrupt")?;
                let request_id: u64 =
                    helpers::deser_seq_element(&mut seq, "Request ID must be a u64.")?;
                let options: Value =
                    helpers::deser_seq_element(&mut seq, "Options must be a JSON value.")?;
                helpers::deser_value_is_object::<A, _>(&options, "Options must be object like.")?;
                Ok(Interrupt {
                    request_id,
                    options,
                })
            }
        }

        deserializer.deserialize_struct(
            "Interrupt",
            &["request_id", "options"],
            InterruptVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}
/*
#[cfg(test)]
mod tests {
    use serde_json::{to_string, from_str};

    //use crate::messages::interrupt::Interrupt;


    #[test]
    fn test() {
        let d1 = r#"[69,3,{}]"#;
        let g1 = Interrupt {
            options: serde_json::json!({}),
            request_id: 3
        };
        let d2 = to_string(&g1).unwrap();
        assert_eq!(d1, d2);
        let g2: Interrupt = from_str(d1).unwrap();
        assert_eq!(g1, g2);
    }
}
*/
