use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub enum DisconnectReason {
    IoServerDisconnect,
    IoClientDisconnect,
    PingTimeout,
    TransportClose,
    TransportError,
}

impl AsRef<str> for DisconnectReason {
    fn as_ref(&self) -> &'static str {
        match self {
            DisconnectReason::IoServerDisconnect => "io server disconnect",
            DisconnectReason::IoClientDisconnect => "io client disconnect",
            DisconnectReason::PingTimeout => "ping timeout",
            DisconnectReason::TransportClose => "transport close",
            DisconnectReason::TransportError => "transport error",
        }
    }
}

impl FromStr for DisconnectReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "io server disconnect" => Ok(DisconnectReason::IoServerDisconnect),
            "io client disconnect" => Ok(DisconnectReason::IoClientDisconnect),
            "ping timeout" => Ok(DisconnectReason::PingTimeout),
            "transport close" => Ok(DisconnectReason::TransportClose),
            "transport error" => Ok(DisconnectReason::TransportError),
            _ => Err(()),
        }
    }
}

impl Display for DisconnectReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DisconnectReason::IoServerDisconnect => {
                write!(f, "io server disconnect")
            }
            DisconnectReason::IoClientDisconnect => {
                write!(f, "io client disconnect")
            }
            DisconnectReason::PingTimeout => {
                write!(f, "ping timeout")
            }
            DisconnectReason::TransportClose => {
                write!(f, "transport close")
            }
            DisconnectReason::TransportError => {
                write!(f, "transport error")
            }
        }
    }
}
