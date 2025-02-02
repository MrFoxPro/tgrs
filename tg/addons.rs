use serde::Deserialize;
use crate::*;

/** Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using `multipart/form-data` under `file_attach_name` name */
pub type InputFile = String;

#[derive(Debug, Clone, Deserialize)]
pub enum EditMessageResult {
	EditedMessage(Message),
	MessageIsInline(bool),
}

impl Into<String> for ChatId {
	fn into(self) -> String {
		match self { Self::Id(id) => id.to_string(), Self::Username(username) => username }
	}
}
