use std::marker::PhantomData;

use serde::{de::Visitor, Deserialize, Serialize};
use serde_json::Value;

use crate::{messages::helpers, roles::Roles};

use super::{MessageDirection, WampMessage};

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Register - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-register-2)
/// Represents an Register frame in the WAMP protocol.
/// ## Examples
/// ```
/// use wamp_core::messages::Register;
/// use wamp_core::register;
/// use serde_json::json;
///
/// # let mut registerd1 = register!("procedure");
///
/// let registerd = Register {
///     request_id: 1,
///     procedure: "procedure".to_string(),
///     options: json!({})
/// };
/// # registerd1.options = json!({});
/// # assert_eq!(registerd, registerd1);
/// ```
/// ### Serializer
/// Implements serde Serialize trait for Register
/// ```
/// use wamp_core::messages::Register;
/// use serde_json::{json, to_string};
/// use wamp_core::register;
///
/// let data = r#"[64,1,{},"com.myapp.myprocedure1"]"#;
///
/// let register1 = register!("com.myapp.myprocedure1");
///
/// let register = Register {
///     request_id: 1,
///     procedure: "com.myapp.myprocedure1".to_string(),
///     options: json!({})
/// };
///
/// let data2 = to_string(&register1).unwrap();
/// let data3 = to_string(&register).unwrap();
///
/// assert_eq!(data, data2);
/// assert_eq!(data2, data3);
/// ```
/// ### Deserializer
/// Implements serde Deserialize trait for register
/// ```
/// use wamp_core::messages::Register;
/// use serde_json::from_str;
/// use wamp_core::register;
///
/// let data = r#"[64,1,{},"com.myapp.myprocedure1"]"#;
///
/// let register = from_str::<Register>(data).unwrap();
///
/// let register2 = register!("com.myapp.myprocedure1");
///
/// assert_eq!(register, register2);
/// ```
pub struct Register {
    pub request_id: u64,
    pub options: Value,
    pub procedure: String,
}

#[macro_export]
/// # register Macro - [wamp-proto](https://wamp-proto.org/wamp_latest_ietf.html#name-register-2)
/// Macro that allows for default implementations of Register with empty or custom options and auto incremented request id.
/// ## Examples
/// ```
/// use wamp_core::messages::{self, Register};
/// use wamp_core::register;
/// use serde_json::json;
///
/// let procedure = "procedure";
///
/// // Construct with default empty options object
/// let register = register!(procedure);
///
/// let register2 = Register {
///     request_id: 1,
///     options: json!({}),
///     procedure: procedure.to_string()
/// };
///
/// assert_eq!(register, register2);
/// ```
macro_rules! register {
    ($procedure:expr) => {
        register! {$procedure, serde_json::json!({})}
    };
    ($procedure:expr, $options:expr) => {
        Register {
            procedure: $procedure.to_string(),
            options: $options,
            request_id: $crate::factories::increment(),
        }
    };
}

impl WampMessage for Register {
    const ID: u64 = 64;

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

impl Serialize for Register {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (Self::ID, &self.request_id, &self.options, &self.procedure).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Register {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RegisterVisitor(
            PhantomData<u64>,
            PhantomData<u64>,
            PhantomData<Value>,
            PhantomData<String>,
        );

        impl<'vi> Visitor<'vi> for RegisterVisitor {
            type Value = Register;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A sequence of Register components.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'vi>,
            {
                let message_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Message id must be present and type u64.",
                )?;
                helpers::validate_id::<Register, A, _>(&message_id, "Register")?;
                let request_id: u64 = helpers::deser_seq_element(
                    &mut seq,
                    "Request ID must be present and type u64",
                )?;
                let options: Value = helpers::deser_seq_element(
                    &mut seq,
                    "options must be present and object like",
                )?;
                helpers::deser_value_is_object::<A, _>(&options, "options must be object like.")?;
                let procedure: String = helpers::deser_seq_element(
                    &mut seq,
                    "procedure URI must be present and type String",
                )?;
                helpers::deser_value_is_object::<A, _>(&options, "options must be object like.")?;
                Ok(Register {
                    request_id,
                    options,
                    procedure,
                })
            }
        }

        deserializer.deserialize_struct(
            "Register",
            &["request_id", "options", "procedure"],
            RegisterVisitor(PhantomData, PhantomData, PhantomData, PhantomData),
        )
    }
}
