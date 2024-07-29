use serde::{Deserialize, Serialize};
use std::fmt::{self};
use ts_rs::TS;

/// data structure of response to return from Tauri Core
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommandData<T> {
    pub data: Option<T>,
    pub message: String,
    pub status: bool,
}

pub type ApiResponse<D, E> = Result<CommandData<D>, CommandData<E>>;

impl<T: fmt::Display + fmt::Debug> fmt::Display for CommandData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "data: {:?}
            message: {},
            status: {}
            ",
            self.data, self.message, self.status,
        )
    }
}

impl<T> Default for CommandData<T> {
    fn default() -> Self {
        Self {
            data: None::<T>,
            message: String::from("returned data form core"),
            status: true,
        }
    }
}

impl<T> CommandData<T> {
    /// only retun the data
    pub fn new(data: T) -> Self {
        Self {
            data: Some(data),
            ..Default::default()
        }
    }
    /// if the response is ok
    /// returns a CommandData struct
    /// with the data, message, and status
    pub fn ok(message: &str, data: T) -> Self {
        Self {
            data: Some(data),
            message: message.to_string(),
            ..Default::default()
        }
    }

    /// if the response is an error
    /// returns a CommandData struct
    /// with the data, message, and status
    pub fn err(message: &str, data: T) -> Self {
        Self {
            data: Some(data),
            message: message.to_string(),
            status: false,
        }
    }
}
