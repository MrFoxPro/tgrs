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
	pub file_name: Option<String>,
	pub inner: InputFileInner,
}
impl InputFile {
	pub fn new(inner: impl Into<InputFileInner>) -> Self {
		InputFile { file_name: None, inner: inner.into() }
	}
	pub fn with_filename(inner: impl Into<InputFileInner>, filename: impl Into<String>) -> Self {
		InputFile { file_name: Some(filename.into()), inner: inner.into() }
	}
}
impl From<Vec<u8>> for InputFile {
	fn from(value: Vec<u8>) -> Self { Self { file_name: None, inner: value.into() } }
}
impl From<Option<Box<dyn AsyncRead + Send + Sync + Unpin>>> for InputFile {
	fn from(value: Option<Box<dyn AsyncRead + Send + Sync + Unpin>>) -> Self { Self { file_name: None, inner: value.into() } }
}

#[derive(From)]
pub enum InputFileInner {
	Bytes(Vec<u8>),
	Stream(Option<Box<dyn AsyncRead + Send + Sync + Unpin>>),
}

impl Serialize for InputFile {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		FILE_ANCHOR.serialize(serializer)
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
impl Attachable for InputFile {
	fn attach(self, key: &'static str, parts: &mut FormParts) {
		let id = self.file_name.clone().unwrap_or_else(|| parts.inner.len().to_string());
		parts.add_string(key, format!("attach://{id}"));
		parts.add_file(id, self);
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
	fn from(value: Vec<u8>) -> Self { Self::File(InputFile { file_name: None, inner: value.into() }) }
}

impl Serialize for Asset {
	fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
		match self {
			Self::Url(url) => url.serialize(s),
			Self::File(_) => FILE_ANCHOR.serialize(s)
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

impl Attachable for schema::InputSticker {
	fn attach(self, key: &'static str, parts: &mut FormParts) {
		let mut as_json = serde_json::to_string(&self).unwrap();
		if let Asset::File(file) = self.sticker {
			let file_id = file.file_name.clone().unwrap_or_else(|| parts.inner.len().to_string());
			as_json = as_json.replacen(FILE_ANCHOR, &format!("attach://{file_id}"), 1);
			parts.add_file(file_id.to_string(), file);
		}
		parts.add_string(key, as_json);
	}
}

impl Attachable for schema::InputMedia {
	fn attach(self, key: &'static str, parts: &mut FormParts) {
		let mut as_json = serde_json::to_string(&self).unwrap();
		match self {
			Self::Animation(InputMediaAnimation { media: Asset::File(file), .. }) 
			| Self::Document(InputMediaDocument { media: Asset::File(file), .. })
			| Self::Photo(InputMediaPhoto { media: Asset::File(file), .. })
			| Self::Audio(InputMediaAudio { media: Asset::File(file), .. })
			| Self::Video(InputMediaVideo { media: Asset::File(file), .. }) => {
				let file_id = file.file_name.clone().unwrap_or_else(|| parts.inner.len().to_string());
				as_json = as_json.replacen(FILE_ANCHOR, &format!("attach://{file_id}"), 1);
				parts.add_file(file_id.to_string(), file);
			}
			_ => {}
		}
		parts.add_string(key, as_json);
	}
}


// let as_json = as_json.
// while let Some(index) = as_json[start..].find(FILE_ANCHOR) {
// 	let actual_index = start + index;
// 	result.push_str(&as_json[start..actual_index]);
// 	start = actual_index + FILE_ANCHOR.len();

// 	let file_key = format!("{base_id}-{rel_id}");
// 	result.push_str(format!("attach://{file_key}").as_str());
// 	parts.inner.push((file_key.into(), self_files.remove(rel_id).unwrap().into()));

// 	rel_id += 1;
// }

// result.push_str(&as_json[start..]);
// parts.add_string(key, result);

// self.sticker.attach(key, parts);