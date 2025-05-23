use crate::User;
use std::ops::{Range};
use serde::{Deserialize, Serialize};
use serde_with::apply;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageEntity {
	#[serde(flatten)]
	pub pos: TextEntityPosition,
	#[serde(flatten)]
	pub kind: MessageEntityType,
}
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct TextEntityPosition {
	/// Offset in UTF-16 code units to the start of the entity.
	pub offset: usize,
	pub length: usize,
}

impl From<Range<usize>> for TextEntityPosition {
	fn from(range: Range<usize>) -> Self {
		Self { offset: range.start, length: range.end - range.start }
	}
}

#[apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum MessageEntityType {
	Blockquote,
	Bold,
	BotCommand,
	Cashtag,
	Code,
	CustomEmoji { custom_emoji_id: Option<String> },
	Email,
	ExpandableBlockquote,
	Hashtag,
	Italic,
	Mention,
	PhoneNumber,
	Pre { language: Option<String> },
	Spoiler,
	Strikethrough,
	TextLink { url: Option<String> },
	TextMention { user: Option<User> },
	Underline,
	Url,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MessageEntityBotCommand {
	pub command: String,
	/// Username of a bot.
	pub bot_name: Option<String>,
}