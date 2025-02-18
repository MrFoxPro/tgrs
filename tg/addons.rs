use std::fmt;
use derive_more::derive::From;
use futures_util::AsyncRead;
use serde::{Deserialize, Serialize, Serializer};
use crate::{client::FormParts, *};

// Common things
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum EditMessageResult {
	EditedMessage(Message),
	MessageIsInline(bool),
}

// File attachments
pub trait Attachable: Serialize + Sized {
	fn attach(self, key: &'static str, parts: &mut FormParts);
}

pub const FILE_ANCHOR: &str = "<FILE>";

#[derive(Debug, Clone, From)]
pub struct InputFile {
	pub filename: String,
	pub inner: InputFileInner,
}
#[derive(From)]
pub enum InputFileInner {
	Bytes(bytes::Bytes),
	Stream(Option<Box<dyn AsyncRead + Send + Sync + Unpin>>),
}
impl InputFile {
	pub fn new(inner: impl Into<InputFileInner>) -> Self {
		InputFile { inner: inner.into(), filename: String::from(itoa::Buffer::new().format(fastrand::u16(..))) }
	}
	pub fn with_filename(inner: impl Into<InputFileInner>, filename: impl Into<String>) -> Self {
		InputFile { inner: inner.into(), filename: filename.into() }
	}
}
impl Serialize for InputFile {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		format!("attach://{}", self.filename).serialize(serializer)
	}
}
impl Clone for InputFileInner {
	fn clone(&self) -> Self {
		match self {
			Self::Bytes(data) => Self::Bytes(data.clone()),
			Self::Stream(_) => Self::Stream(None)
		}
	}
}
impl fmt::Debug for InputFileInner {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Bytes(_) => writeln!(f, "InputFile::Bytes(<data>)"),
			Self::Stream(_) => writeln!(f, "InputFile::Stream(<stream>)"),
		}
	}
}
impl From<Vec<u8>> for InputFile {
	fn from(value: Vec<u8>) -> Self { Self::new(bytes::Bytes::from(value)) }
}
impl From<bytes::Bytes> for InputFile {
	fn from(value: bytes::Bytes) -> Self { Self::new(value) }
}
impl From<Option<Box<dyn AsyncRead + Send + Sync + Unpin>>> for InputFile {
	fn from(value: Option<Box<dyn AsyncRead + Send + Sync + Unpin>>) -> Self { Self::new(value) }
}
impl Attachable for InputFile {
	fn attach(self, key: &'static str, parts: &mut FormParts) {
		parts.add_string(key, format!("attach://{}", self.filename));
		parts.add_file(self);
	}
}

/** Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using `multipart/form-data` under `file_attach_name` name */
#[derive(Clone, Debug, From)]
pub enum Asset {
	File(InputFile),
	Url(String),
}
impl Asset {
	pub fn file(input: impl Into<InputFile>) -> Self { Self::File(input.into()) }
	pub fn url(input: impl Into<String>) -> Self { Self::Url(input.into()) }
}
impl From<Vec<u8>> for Asset {
	fn from(value: Vec<u8>) -> Self { Self::File(InputFile::new(bytes::Bytes::from(value))) }
}
impl From<bytes::Bytes> for Asset {
	fn from(value: bytes::Bytes) -> Self { Self::File(InputFile::new(value)) }
}

impl Serialize for Asset {
	fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
		match self {
			Self::Url(url) => url.serialize(s),
			Self::File(file) => file.serialize(s)
		}
	}
}

impl Attachable for Asset {
	fn attach(self, key: &'static str, parts: &mut FormParts) {
		match self {
			Asset::Url(url) => parts.add_string(key, url),
			Asset::File(file) => file.attach(key, parts),
		}
	}
}
impl Attachable for schema::InputMedia {
	fn attach(self, key: &'static str, parts: &mut FormParts) {
		parts.add_string(key, serde_json::to_string(&self).unwrap());
		match self {
			InputMedia::Animation(InputMediaAnimation { media: Asset::File(file), .. }) 
			| InputMedia::Document(InputMediaDocument { media: Asset::File(file), .. })
			| InputMedia::Photo(InputMediaPhoto { media: Asset::File(file), .. })
			| InputMedia::Audio(InputMediaAudio { media: Asset::File(file), .. })
			| InputMedia::Video(InputMediaVideo { media: Asset::File(file), .. }) => {
				parts.add_file(file);
			}
			_ => {}
		}
	}
}
impl Attachable for Vec<schema::Media> {
	fn attach(self, key: &'static str, parts: &mut FormParts) {
		parts.add_string(key, serde_json::to_string(&self).unwrap());
		for item in self.into_iter() {
			match item {
				Media::InputMediaDocument(InputMediaDocument { media: Asset::File(file), .. })
				| Media::InputMediaPhoto(InputMediaPhoto { media: Asset::File(file), .. })
				| Media::InputMediaAudio(InputMediaAudio { media: Asset::File(file), .. })
				| Media::InputMediaVideo(InputMediaVideo { media: Asset::File(file), .. }) => {
					parts.add_file(file);
				}
				_ => {}
			}
		}
	}
}
impl Attachable for schema::InputSticker {
	fn attach(self, key: &'static str, parts: &mut FormParts) {
		parts.add_string(key, serde_json::to_string(&self).unwrap());
		if let Asset::File(file) = self.sticker { parts.add_file(file); }
	}
}
impl Attachable for Vec<schema::InputSticker> {
	fn attach(self, key: &'static str, parts: &mut FormParts) {
		parts.add_string(key, serde_json::to_string(&self).unwrap());
		for item in self.into_iter() {
			if let Asset::File(file) = item.sticker { parts.add_file(file); }
		}
	}
}
impl Attachable for schema::InputPaidMedia {
	fn attach(self, key: &'static str, parts: &mut FormParts) {
		parts.add_string(key, serde_json::to_string(&self).unwrap());
		match self {
			Self::Photo(InputPaidMediaPhoto { media: Asset::File(file), .. }) 
			| Self::Video(InputPaidMediaVideo { media: Asset::File(file), .. }) => {
				parts.add_file(file);
			}
			_ => {}
		}
	}
}
impl Attachable for Vec<schema::InputPaidMedia> {
	fn attach(self, key: &'static str, parts: &mut FormParts) {
		parts.add_string(key, serde_json::to_string(&self).unwrap());
		for item in self.into_iter() {
			match item {
				InputPaidMedia::Photo(InputPaidMediaPhoto { media: Asset::File(file), .. }) 
				| InputPaidMedia::Video(InputPaidMediaVideo { media: Asset::File(file), .. }) => {
					parts.add_file(file);
				}
				_ => {}
			}
		}
	}
}