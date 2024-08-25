extern crate alloc;

use crate::actions::NotificationAction;
use alloc::ffi::NulError;

///
/// Catchall error type for converting errors from between different libraries as needed
///
/// <3
///
#[derive(Debug)]
pub enum RoslError {
    ZbusFdo(zbus::fdo::Error),
    NotificationSend(tokio::sync::mpsc::error::SendError<NotificationAction>),
    CStr(NulError),
    Other { message: String },
}

impl From<zbus::Error> for RoslError {
    fn from(err: zbus::Error) -> Self {
        match err {
            zbus::Error::FDO(e) => Self::ZbusFdo(*e),
            _ => Self::Other {
                message: err.to_string(),
            },
        }
    }
}

impl From<tokio::sync::mpsc::error::SendError<NotificationAction>> for RoslError {
    fn from(err: tokio::sync::mpsc::error::SendError<NotificationAction>) -> Self {
        RoslError::NotificationSend(err)
    }
}

impl From<NulError> for RoslError {
    fn from(err: NulError) -> Self {
        RoslError::CStr(err)
    }
}

impl From<RoslError> for zbus::fdo::Error {
    fn from(err: RoslError) -> Self {
        match err {
            RoslError::ZbusFdo(e) => e,
            RoslError::Other { message } => zbus::fdo::Error::Failed(message),
            RoslError::NotificationSend(e) => zbus::fdo::Error::Failed(e.to_string()),
            RoslError::CStr(e) => zbus::fdo::Error::Failed(e.to_string()),
        }
    }
}
