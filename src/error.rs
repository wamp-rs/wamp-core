use crate::messages::{
    Abort, Authenticate, Call, Cancel, Challenge, Event, Goodbye, Hello, Interrupt, Invocation,
    Messages, Publish, Published, Register, Registered, Subscribe, Subscribed, Unregister,
    Unregistered, Unsubscribe, Unsubscribed, WampError, WampResult, Welcome, Yield,
};
use tungstenite::http::header::{InvalidHeaderValue, ToStrError};

#[derive(Debug)]
pub enum Error {
    InvalidURI,
    ToStrError(ToStrError),
    InvalidHeaderValue(InvalidHeaderValue),
    TungsteniteError(tungstenite::Error),
    SerdeJsonError(serde_json::Error),
    InvalidMessageEnumMember,
    Error(&'static str),
    InvalidFrameReceived(Messages),
    Close,
    Abort(Abort),
    NoSuchWampErrorType(Messages),
    NoSuchMessage,
}

macro_rules! message_to_from {
    ($typ: ident) => {
        impl TryFrom<$typ> for tungstenite::Message {
            type Error = serde_json::Error;

            fn try_from(value: $typ) -> Result<tungstenite::Message, Self::Error> {
                Ok(tungstenite::Message::Text(serde_json::to_string(&value)?))
            }
        }
    };
}

//message_to_from!(Abort);
message_to_from!(Abort);
message_to_from!(Authenticate);
message_to_from!(Call);
message_to_from!(Cancel);
message_to_from!(Challenge);
message_to_from!(WampError);
message_to_from!(WampResult);
message_to_from!(Event);
message_to_from!(Goodbye);
message_to_from!(Hello);
message_to_from!(Interrupt);
message_to_from!(Invocation);
message_to_from!(Publish);
message_to_from!(Published);
message_to_from!(Register);
message_to_from!(Registered);
message_to_from!(Subscribe);
message_to_from!(Subscribed);
message_to_from!(Unregister);
message_to_from!(Unregistered);
message_to_from!(Unsubscribe);
message_to_from!(Unsubscribed);
message_to_from!(Welcome);
message_to_from!(Yield);

//impl<M: WampMessage + Serialize> TryFrom<M> for crate::error::Error {
//    type Error = Error;
//
//    fn try_from(value: M) -> Result<Self, Self::Error> {
//
//    }
//}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJsonError(value)
    }
}

impl From<tungstenite::Error> for Error {
    fn from(value: tungstenite::Error) -> Self {
        Self::TungsteniteError(value)
    }
}

#[derive(Debug)]
/// # [TODO]: WampErrorUri
/// Unimplemented, unfortunately this does absolutely nothing in the current moment. The reasons are described below.
/// 
/// ## The Problem
/// Wamp URI's have a variable amount of error URIs that get sent with different enabled features on wamp routers.
/// This leads to the possibility of also running into "unknown errors". This is running with the assumption that we add
/// in each string manually to serde to parse the error to the enum variant.
/// 
/// Which, also isnt how the wamp protocol defines how to parse URIs. While I understand from the documents that URIs are
/// parsed using Regex, I have gotten extremely inconsistent results while testing with errors using Regex to parse URIs.
/// 
/// To further explain, while there is some level of structure to the Regex they use in reference to what type of URI it takes,
/// and I have modeled that into a rust like structure, using the Regex on actual URI's from the wamp protocol returns very mixed 
/// (and almost always wrong on edge cases) results.
/// 
/// I will stop documenting here to cite myself, more investigation is needed.
pub enum WampErrorUri {
    NotAuthorized,
    ProcedureAlreadyExists,
    NoSuchRealm,
    ProtocolViolation,
    NoSuchSubscription,
    NoSuchRegistration,
    InvalidUri,
    NoSuchProcedure,
    InvalidArgument,
    Canceled,
    PayloadSizeExceeded,
    FeatureNotSupported,
    Timeout,
    Unavailable,
    NoAvailableCallee,
    DiscloseMeNotAllowed,
    OptionDisallowedDiscloseMe,
    NoMatchingAuthMethod,
    NoSuchRole,
    NoSuchPrincipal,
    AuthenticationDenied,
    AuthenticationFailed,
    AuthenticationRequired,
    AuthorizationDenied,
    AuthorizationFailed,
    AuthorizationRequired,
    NetworkFailure,
    OptionNotAllowed,
}
/// [TODO]: See WampErrorUri Structure for more details.
pub enum CloseUri {
    SystemShutdown,
    CloseRealm,
    GoodbyeAndOut,
    Killed,
}
