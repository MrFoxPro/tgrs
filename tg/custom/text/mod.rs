mod entities;

use std::str::EncodeUtf16;
use serde::{Deserialize, Serialize};
pub use entities::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Text {
	pub data: String,
	pub entities: Option<MessageEntities>,
}

impl Text {
	pub fn with_entities(mut self, value: MessageEntities) -> Self {
		self.entities = Some(value);
		self
	}
	pub fn get_bot_commands(&self) -> Option<Vec<MessageEntityBotCommand>> {
		self.entities.as_ref().map(|entities| {
			let repr = TextRepr::from(self);
			entities.into_iter().filter_map(|entity| {
				let MessageEntity::BotCommand(position) = entity else { return None };
				let entity_data = repr.get_entity_content(*position);
				let parts = entity_data.as_str().splitn(2, '@').collect::<Vec<&str>>();
				let len = parts.len();
				assert!(len >= 1);
				let command = parts[0].to_string();
				let bot_name = if len == 2 { Some(parts[1].to_string()) } else { None };
				Some(MessageEntityBotCommand { command, bot_name })
			}).collect::<Vec<MessageEntityBotCommand>>()
		})
		.filter(|entities| !entities.is_empty())
	}
}

impl PartialEq<str> for Text {
	fn eq(&self, other: &str) -> bool {
		self.data == other
	}
}

impl PartialEq<String> for Text {
	fn eq(&self, other: &String) -> bool {
		self.data == *other
	}
}

impl AsRef<str> for Text {
	fn as_ref(&self) -> &str {
		self.data.as_str()
	}
}

impl From<String> for Text {
	fn from(s: String) -> Self {
		Self {
			data: s,
			entities: None,
		}
	}
}

impl<'a> From<&'a str> for Text {
	fn from(value: &'a str) -> Self {
		Self {
			data: String::from(value),
			entities: None,
		}
	}
}

struct TextRepr<'a> {
	iter: EncodeUtf16<'a>,
}

impl<'a> From<&'a Text> for TextRepr<'a> {
	fn from(text: &'a Text) -> Self {
		Self {
			iter: text.data.encode_utf16(),
		}
	}
}

impl<'a> TextRepr<'a> {
	fn get_entity_content(&self, position: TextEntityPosition) -> String {
		let (offset, length) = (position.offset as usize, position.length as usize);
		let bytes = self.iter.clone().skip(offset).take(length).collect::<Vec<u16>>();
		String::from_utf16_lossy(&bytes)
	}
}
