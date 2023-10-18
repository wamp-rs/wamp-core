pub mod abort;
pub mod authenticate;
pub mod call;
pub mod cancel;
pub mod challenge;
pub mod error;
pub mod event;
pub mod goodbye;
pub mod hello;
pub mod interrupt;
pub mod invocation;
pub mod publish;
pub mod published;
pub mod register;
pub mod registered;
pub mod result;
pub mod subscribe;
pub mod subscribed;
pub mod unregister;
pub mod unregistered;
pub mod unsubscribe;
pub mod unsubscribed;
pub mod welcome;
pub mod r#yield;

pub use abort::Abort;
pub use authenticate::Authenticate;
pub use call::Call;
pub use cancel::Cancel;
pub use challenge::Challenge;
pub use error::{WampError, WampErrorEvent};
pub use event::Event;
pub use goodbye::Goodbye;
pub use hello::Hello;
pub use interrupt::Interrupt;
pub use invocation::Invocation;
pub use publish::Publish;
pub use published::Published;
pub use r#yield::Yield;
pub use register::Register;
pub use registered::Registered;
pub use result::WampResult;
pub use subscribe::Subscribe;
pub use subscribed::Subscribed;
use tungstenite::Message;
pub use unregister::Unregister;
pub use unregistered::Unregistered;
pub use unsubscribe::Unsubscribe;
pub use unsubscribed::Unsubscribed;
pub use welcome::Welcome;

use serde::{de, Deserialize, Deserializer};
use serde_json::{from_str, from_value, json, Value};

use crate::roles::Roles;

/// # Message parsing helpers
///
/// These helpers are internal methods for parsing different aspects of each message.
/// This is very unorganized and could use some clarity.
///
/// The plan for current future releases is to make a macro that automatically
/// creates wamp message parsers, there are two main reasons for this.
///
/// > 1. Macros will allow for a "consistent" state of messages that does not change per instance.
/// >
/// > 2. Macros will allow for easy creation of WAMP "Extension" messages.
///
/// With that said, these helpers, while clear in definition, can be moved into the rest of the "macro"
/// code when the time comes, as that will be the primary implentation of the macro, is to create
/// the Serializer and Deserializer (as well as implement a couple other helper traits for the library)
///
/// There is a good change that at the point I am talking about, this will become its own proc macro crate.
pub(crate) mod helpers {

    use serde::{
        de::{self, SeqAccess},
        ser::Error,
        Deserialize, Serializer,
    };
    use serde_json::Value;
    use std::fmt::Display;

    use super::WampMessage;

    pub(crate) fn deser_seq_element<
        'de,
        T: PartialEq + Deserialize<'de>,
        E: Display,
        A: SeqAccess<'de>,
    >(
        seq: &mut A,
        error: E,
    ) -> Result<T, <A as SeqAccess<'de>>::Error> {
        let element: Option<T> = seq.next_element()?;
        if element != None {
            Ok(element.unwrap())
        } else {
            Err(serde::de::Error::custom(error))
        }
    }

    pub(crate) fn deser_args_kwargs_element<'de, E: Display, A: SeqAccess<'de>>(
        seq: &mut A,
        error: E,
    ) -> Result<Value, <A as SeqAccess<'de>>::Error> {
        let element: Option<Value> = seq.next_element()?;
        if let Some(element) = element {
            if element.is_object() || element.is_array() {
                Ok(element)
            } else {
                Err(serde::de::Error::custom(error))
            }
        } else {
            Ok(Value::Null)
        }
    }

    pub(crate) fn validate_id<'de, M: WampMessage, A: SeqAccess<'de>, E: Display>(
        id: &u64,
        name: E,
    ) -> Result<(), A::Error> {
        if &M::ID == id {
            Ok(())
        } else {
            Err(de::Error::custom(format!(
                "{name} has invalid ID {id}. The ID for {name} must be {}",
                M::ID
            )))
        }
    }

    pub(crate) fn deser_value_is_object<'de, A: SeqAccess<'de>, E: Display>(
        v: &Value,
        e: E,
    ) -> Result<(), A::Error> {
        if v.is_object() {
            Ok(())
        } else {
            Err(de::Error::custom(e))
        }
    }

    pub(crate) fn ser_value_is_object<S: Serializer, T: Display>(
        v: &Value,
        e: T,
    ) -> Result<&Value, S::Error> {
        if v.is_object() {
            Ok(v)
        } else {
            Err(S::Error::custom(e))
        }
    }

    pub(crate) fn ser_value_is_args<S: Serializer, T: Display>(
        v: &Value,
        e: T,
    ) -> Result<&Value, S::Error> {
        if v.is_array() || v.is_null() {
            Ok(v)
        } else {
            Err(S::Error::custom(e))
        }
    }

    pub(crate) fn ser_value_is_kwargs<S: Serializer, T: Display>(
        v: &Value,
        e: T,
    ) -> Result<&Value, S::Error> {
        if v.is_object() || v.is_null() {
            Ok(v)
        } else {
            Err(S::Error::custom(e))
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
/// # Message Direction
/// Indicates the Message Direction for a specified Role.
///
/// Receives means that the specified Role is allowed to receive the message.
/// Sends means that the specified Role allowed to send the message.
pub struct MessageDirection {
    pub receives: &'static bool,
    pub sends: &'static bool,
}

pub trait WampMessage {
    const ID: u64;

    /// # Direction method
    /// Indicates the Message Direction for a specified Role.
    ///
    /// Receives means that the specified Role is allowed to receive the message.
    /// Sends means that the specified Role allowed to send the message.
    fn direction(role: Roles) -> &'static MessageDirection;
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// # Messages Enum
/// This represents each of the messages described in the WAMP protocol.
///
/// This includes its own deserializer (you should serialize using the inner struct always).
///
/// It also implements `From<*n> for Messages` where n = each WAMP message.
/// # Examples
/// ```
/// use wamp_core::messages::{Call, Messages};
/// use wamp_core::call;
/// use serde_json::{Value, json, from_str};
///
/// let message = Messages::from(call!("topic"));
///
/// // Which is the same as this:
/// let mut message2 = Messages::Call(Call {
///     request_id: 1,
///     options: json!({}),
///     procedure: "topic".to_string(),
///     args: Value::Null,
///     kwargs: Value::Null
/// });
/// assert_eq!(message, message2);
///
/// // Lets make a raw string to pass to the deserializer (this is a Call message)
/// let data = r#"[48,1,{},"topic"]"#;
///
/// // Deserialize the raw string
/// let message3 = from_str::<Messages>(data).unwrap();
///
/// assert_eq!(message2, message3);
/// ```
pub enum Messages {
    Abort(Abort),
    Authenticate(Authenticate),
    Call(Call),
    Cancel(Cancel),
    Challenge(Challenge),
    Error(WampError),
    Event(Event),
    Goodbye(Goodbye),
    Hello(Hello),
    Interrupt(Interrupt),
    Invocation(Invocation),
    Publish(Publish),
    Published(Published),
    Register(Register),
    Registered(Registered),
    Result(WampResult),
    Subscribe(Subscribe),
    Subscribed(Subscribed),
    Unregister(Unregister),
    Unregistered(Unregistered),
    Unsubscribe(Unsubscribe),
    Unsubscribed(Unsubscribed),
    Welcome(Welcome),
    Yield(Yield),
    Extension(Vec<Value>),
}

impl Messages {
    /// # Get Message ID
    ///
    /// Get the message ID of a WAMP message. This uses the static u64 for any known WAMP messages.
    ///
    /// For Extension messages, it attempts to get the ID and returns None otherwise.
    ///
    /// ## Examples
    /// ```
    /// use wamp_core::call;
    /// use wamp_core::messages::Messages;
    ///
    /// let message = Messages::from(call!("topic"));
    ///
    /// let message_id = message.id();
    ///
    /// assert_eq!(message_id, Some(48));
    /// ```
    pub fn id(&self) -> Option<u64> {
        match self {
            Messages::Authenticate(_) => Some(Authenticate::ID),
            Messages::Abort(_) => Some(Abort::ID),
            Messages::Call(_) => Some(Call::ID),
            Messages::Cancel(_) => Some(Cancel::ID),
            Messages::Challenge(_) => Some(Authenticate::ID),
            Messages::Error(_) => Some(WampError::ID),
            Messages::Event(_) => Some(Event::ID),
            Messages::Goodbye(_) => Some(Goodbye::ID),
            Messages::Hello(_) => Some(Hello::ID),
            Messages::Interrupt(_) => Some(Interrupt::ID),
            Messages::Invocation(_) => Some(Invocation::ID),
            Messages::Publish(_) => Some(Publish::ID),
            Messages::Published(_) => Some(Published::ID),
            Messages::Register(_) => Some(Register::ID),
            Messages::Registered(_) => Some(Registered::ID),
            Messages::Result(_) => Some(WampResult::ID),
            Messages::Subscribe(_) => Some(Subscribe::ID),
            Messages::Subscribed(_) => Some(Subscribed::ID),
            Messages::Unregister(_) => Some(Unregister::ID),
            Messages::Unregistered(_) => Some(Unregistered::ID),
            Messages::Unsubscribe(_) => Some(Unsubscribe::ID),
            Messages::Unsubscribed(_) => Some(Unsubscribed::ID),
            Messages::Welcome(_) => Some(Welcome::ID),
            Messages::Yield(_) => Some(Yield::ID),
            Messages::Extension(values) => {
                if let Some(value) = values.first() {
                    value.as_u64()
                } else {
                    None
                }
            }
        }
    }
}

macro_rules! try_from_messages {
    ($i: ident) => {
        impl From<$i> for Messages {
            fn from(v: $i) -> Messages {
                Messages::$i(v)
            }
        }

        impl From<Messages> for $i {
            fn from(v: Messages) -> $i {
                v.into()
            }
        }
    };
}

try_from_messages!(Abort);
try_from_messages!(Authenticate);
try_from_messages!(Call);
try_from_messages!(Cancel);
try_from_messages!(Challenge);

// Created manually because the enum member name is not the same as struct name.
impl From<WampError> for Messages {
    fn from(v: WampError) -> Self {
        Messages::Error(v)
    }
}

impl TryFrom<Messages> for WampError {
    type Error = crate::error::Error;
    fn try_from(v: Messages) -> Result<WampError, Self::Error> {
        if let Messages::Error(v) = v {
            Ok(v)
        } else {
            Err(crate::error::Error::InvalidMessageEnumMember)
        }
    }
}

impl TryFrom<tungstenite::Message> for Messages {
    type Error = crate::error::Error;

    fn try_from(value: Message) -> Result<Self, crate::error::Error> {
        Ok(from_str(value.to_text()?)?)
    }
}

impl From<WampResult> for Messages {
    fn from(v: WampResult) -> Self {
        Messages::Result(v)
    }
}

impl TryFrom<Messages> for WampResult {
    type Error = crate::error::Error;
    fn try_from(v: Messages) -> Result<WampResult, Self::Error> {
        if let Messages::Result(v) = v {
            Ok(v)
        } else {
            Err(crate::error::Error::InvalidMessageEnumMember)
        }
    }
}

try_from_messages!(Event);
try_from_messages!(Goodbye);
try_from_messages!(Hello);
try_from_messages!(Interrupt);
try_from_messages!(Invocation);
try_from_messages!(Publish);
try_from_messages!(Published);
try_from_messages!(Register);
try_from_messages!(Registered);
try_from_messages!(Subscribe);
try_from_messages!(Subscribed);
try_from_messages!(Unregister);
try_from_messages!(Unregistered);
try_from_messages!(Unsubscribe);
try_from_messages!(Unsubscribed);
try_from_messages!(Welcome);
try_from_messages!(Yield);

impl<'de> Deserialize<'de> for Messages {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wamp_components: Vec<Value> = Deserialize::deserialize(deserializer)?;
        let wamp_message_id = match wamp_components.first() {
            Some(v) => match v.as_u64() {
                Some(v) => Ok(v),
                None => Err(de::Error::custom("")),
            },
            None => Err(de::Error::custom("value")),
        }?;

        fn helper<'d, T, D>(wamp_components: Vec<Value>) -> Result<T, D::Error>
        where
            T: for<'de> Deserialize<'de>,
            D: Deserializer<'d>,
        {
            let value: T = from_value(json!(wamp_components)).map_err(de::Error::custom)?;
            Ok(value)
        }

        match wamp_message_id {
            Abort::ID => Ok(Self::Abort(helper::<Abort, D>(wamp_components)?)),
            Authenticate::ID => Ok(Self::Authenticate(helper::<Authenticate, D>(
                wamp_components,
            )?)),
            Call::ID => Ok(Self::Call(helper::<Call, D>(wamp_components)?)),
            Cancel::ID => Ok(Self::Cancel(helper::<Cancel, D>(wamp_components)?)),
            Challenge::ID => Ok(Self::Challenge(helper::<Challenge, D>(wamp_components)?)),
            WampError::ID => Ok(Self::Error(helper::<WampError, D>(wamp_components)?)),
            Event::ID => Ok(Self::Event(helper::<Event, D>(wamp_components)?)),
            Goodbye::ID => Ok(Self::Goodbye(helper::<Goodbye, D>(wamp_components)?)),
            Hello::ID => Ok(Self::Hello(helper::<Hello, D>(wamp_components)?)),
            Interrupt::ID => Ok(Self::Interrupt(helper::<Interrupt, D>(wamp_components)?)),
            Invocation::ID => Ok(Self::Invocation(helper::<Invocation, D>(wamp_components)?)),
            Publish::ID => Ok(Self::Publish(helper::<Publish, D>(wamp_components)?)),
            Published::ID => Ok(Self::Published(helper::<Published, D>(wamp_components)?)),
            Register::ID => Ok(Self::Register(helper::<Register, D>(wamp_components)?)),
            Registered::ID => Ok(Self::Registered(helper::<Registered, D>(wamp_components)?)),
            WampResult::ID => Ok(Self::Result(helper::<WampResult, D>(wamp_components)?)),
            Subscribe::ID => Ok(Self::Subscribe(helper::<Subscribe, D>(wamp_components)?)),
            Subscribed::ID => Ok(Self::Subscribed(helper::<Subscribed, D>(wamp_components)?)),
            Unregister::ID => Ok(Self::Unregister(helper::<Unregister, D>(wamp_components)?)),
            Unregistered::ID => Ok(Self::Unregistered(helper::<Unregistered, D>(
                wamp_components,
            )?)),
            Unsubscribe::ID => Ok(Self::Unsubscribe(helper::<Unsubscribe, D>(
                wamp_components,
            )?)),
            Unsubscribed::ID => Ok(Self::Unsubscribed(helper::<Unsubscribed, D>(
                wamp_components,
            )?)),
            Welcome::ID => Ok(Self::Welcome(helper::<Welcome, D>(wamp_components)?)),
            Yield::ID => Ok(Self::Yield(helper::<Yield, D>(wamp_components)?)),
            _ => Ok(Self::Extension(wamp_components)),
        }
    }
}
