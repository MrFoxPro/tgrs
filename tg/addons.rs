use std::fmt;
use derive_more::derive::From;
use futures_util::AsyncRead;
use serde::{Deserialize, Serialize};
use crate::*;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Asset {
	File(#[serde(skip)] InputFile),
	Url(String),
}

/** Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using `multipart/form-data` under `file_attach_name` name */
#[derive(From)]
pub enum InputFile {
	Bytes(Vec<u8>),
	Stream(Option<Box<dyn AsyncRead + Send + Sync + Unpin>>),
}
impl Clone for InputFile {
	fn clone(&self) -> Self {
	    match self {
	    	Self::Bytes(r) => Self::Bytes(r.clone()),
	    	Self::Stream(_) => Self::Stream(None)
	    }
	}
}
impl fmt::Debug for InputFile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	    match self {
	    	Self::Bytes(_) => writeln!(f, "InputFile::Bytes(<data>)"),
			// #[cfg(feature = "stream-file")]
	    	Self::Stream(_) => writeln!(f, "InputFile::Stream(<stream>)"),
	    }
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum EditMessageResult {
	EditedMessage(Message),
	MessageIsInline(bool),
}