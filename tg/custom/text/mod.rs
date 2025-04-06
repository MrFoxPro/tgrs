mod entities;
pub use entities::*;

use std::str::EncodeUtf16;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Text {
	pub data: String,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub entities: Vec<MessageEntity>,
}

impl Text {
	pub fn entities(mut self, value: Vec<MessageEntity>) -> Self {
		self.entities = value;
		self
	}
	pub fn get_bot_commands(&self) -> Vec<MessageEntityBotCommand> {
		if self.entities.is_empty() { return Vec::new(); }

		let repr = TextRepr::from(self);
		let mut commands = Vec::with_capacity(self.entities.len());
		for entity in self.entities.iter() {
			let MessageEntityType::BotCommand = entity.kind else { continue };

			let entity_data = repr.get_entity_content(entity.pos);
			let parts = entity_data.as_str().splitn(2, '@').collect::<Vec<&str>>();
			let len = parts.len();
			
			let command = parts[0].to_string();
			let bot_name = if len == 2 { Some(parts[1].to_string()) } else { None };
			
			commands.push(MessageEntityBotCommand { command, bot_name });
		}

		return commands;
	}
}

impl PartialEq<str> for Text {
	fn eq(&self, other: &str) -> bool { self.data == other }
}
impl PartialEq<String> for Text {
	fn eq(&self, other: &String) -> bool { self.data == *other }
}
impl AsRef<str> for Text {
	fn as_ref(&self) -> &str { self.data.as_str() }
}
impl From<String> for Text {
	fn from(s: String) -> Self { Self { data: s, entities: Vec::new() } }
}
impl<'a> From<&'a str> for Text {
	fn from(value: &'a str) -> Self {
		Self { data: String::from(value), entities: Vec::new() }
	}
}

struct TextRepr<'a> { iter: EncodeUtf16<'a> }
impl<'a> From<&'a Text> for TextRepr<'a> {
	fn from(text: &'a Text) -> Self { Self { iter: text.data.encode_utf16() } }
}
impl<'a> TextRepr<'a> {
	fn get_entity_content(&self, position: TextEntityPosition) -> String {
		let (offset, length) = (position.offset as usize, position.length as usize);
		let bytes = self.iter.clone().skip(offset).take(length).collect::<Vec<u16>>();
		String::from_utf16_lossy(&bytes)
	}
}
