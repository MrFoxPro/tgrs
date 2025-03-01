use std::{
    convert::TryFrom,
    error::Error,
    fmt,
    ops::{Index, IndexMut, Range},
};
use serde::{Deserialize, Serialize};
use crate::User;

/// Represents a collection of text entities.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(into = "Vec<MessageEntity>", try_from = "Vec<RawMessageEntity>")]
pub struct MessageEntities {
    pub items: Vec<MessageEntity>,
}

impl MessageEntities {
    pub fn push(&mut self, value: MessageEntity) {
        self.items.push(value);
    }
}

impl TryFrom<Vec<RawMessageEntity>> for MessageEntities {
    type Error = MessageEntityError;

    fn try_from(entities: Vec<RawMessageEntity>) -> Result<Self, Self::Error> {
        entities
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<Vec<MessageEntity>, _>>()
            .map(|items| Self { items })
    }
}

impl FromIterator<MessageEntity> for MessageEntities {
    fn from_iter<T: IntoIterator<Item = MessageEntity>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for MessageEntities {
    type Item = MessageEntity;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a> IntoIterator for &'a MessageEntities {
    type Item = &'a MessageEntity;
    type IntoIter = std::slice::Iter<'a, MessageEntity>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.as_slice().iter()
    }
}

impl<'a> IntoIterator for &'a mut MessageEntities {
    type Item = &'a mut MessageEntity;
    type IntoIter = std::slice::IterMut<'a, MessageEntity>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.as_mut_slice().iter_mut()
    }
}

impl Index<usize> for MessageEntities {
    type Output = MessageEntity;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl IndexMut<usize> for MessageEntities {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl From<MessageEntities> for Vec<MessageEntity> {
    fn from(entities: MessageEntities) -> Self {
        entities.items
    }
}

/// Represents an entity in a text.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "RawMessageEntity", into = "RawMessageEntity")]
pub enum MessageEntity {
    Blockquote(TextEntityPosition),
    Bold(TextEntityPosition),
    BotCommand(TextEntityPosition),
    Cashtag(TextEntityPosition),
    Code(TextEntityPosition),
    CustomEmoji {
        /// Use [`crate::types::GetCustomEmojiStickers`] to get full information about the sticker.
        custom_emoji_id: String,
        position: TextEntityPosition,
    },
    Email(TextEntityPosition),
    ExpandableBlockquote(TextEntityPosition),
    Hashtag(TextEntityPosition),
    Italic(TextEntityPosition),
    /// A user mention (e.g. @username).
    Mention(TextEntityPosition),
    PhoneNumber(TextEntityPosition),
    Pre {
        position: TextEntityPosition,
        language: Option<String>,
    },
    Spoiler(TextEntityPosition),
    Strikethrough(TextEntityPosition),
    TextLink {
        position: TextEntityPosition,
        url: String,
    },
    TextMention {
        position: TextEntityPosition,
        user: User,
    },
    Underline(TextEntityPosition),
    Url(TextEntityPosition),
}

macro_rules! message_entity_factory {
    ($($method_name:ident => $enum_variant: ident),*) => {
        $(
            pub fn $method_name<T: Into<TextEntityPosition>>(pos: T) -> MessageEntity {
                MessageEntity::$enum_variant(pos.into())
            }
        )*
    };
}

impl MessageEntity {
    message_entity_factory!(
        blockquote => Blockquote,
        bold => Bold,
        bot_command => BotCommand,
        cashtag => Cashtag,
        code => Code,
        email => Email,
        expandable_blockquote => ExpandableBlockquote,
        hashtag => Hashtag,
        italic => Italic,
        mention => Mention,
        phone_number => PhoneNumber,
        spoiler => Spoiler,
        strikethrough => Strikethrough,
        underline => Underline
    );

    pub fn custom_emoji<P: Into<TextEntityPosition>, I: Into<String>>(
        pos: P,
        custom_emoji_id: I,
    ) -> MessageEntity {
        MessageEntity::CustomEmoji {
            position: pos.into(),
            custom_emoji_id: custom_emoji_id.into(),
        }
    }

    pub fn pre<P: Into<TextEntityPosition>, L: Into<String>>(
        pos: P,
        language: Option<L>,
    ) -> MessageEntity {
        MessageEntity::Pre {
            position: pos.into(),
            language: language.map(|x| x.into()),
        }
    }

    pub fn text_link<P: Into<TextEntityPosition>, U: Into<String>>(pos: P, url: U) -> MessageEntity {
        MessageEntity::TextLink {
            position: pos.into(),
            url: url.into(),
        }
    }

    pub fn text_mention<P: Into<TextEntityPosition>>(pos: P, user: User) -> MessageEntity {
        MessageEntity::TextMention {
            position: pos.into(),
            user,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RawMessageEntity {
    offset: u32,
    length: u32,
    #[serde(flatten)]
    entity_type: RawMessageEntityType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum RawMessageEntityType {
    Blockquote,
    Bold,
    BotCommand,
    Cashtag,
    Code,
    CustomEmoji {
        #[serde(skip_serializing_if = "Option::is_none")]
        custom_emoji_id: Option<String>,
    },
    Email,
    ExpandableBlockquote,
    Hashtag,
    Italic,
    Mention,
    PhoneNumber,
    Pre {
        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<String>,
    },
    Spoiler,
    Strikethrough,
    TextLink {
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
    },
    TextMention {
        #[serde(skip_serializing_if = "Option::is_none")]
        user: Option<User>,
    },
    Underline,
    Url,
}

/// Represents an error when parsing/serializing entities.
#[derive(Debug)]
pub enum MessageEntityError {
    /// Custom emoji is required for custom_emoji entity.
    NoCustomEmoji,
    /// URL is required for `text_link` entity.
    NoUrl,
    /// User is required for `text_mention` entity.
    NoUser,
}

impl Error for MessageEntityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // Self::Serialize(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for MessageEntityError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::MessageEntityError::*;
        write!(
            out,
            "{}",
            match self {
                NoCustomEmoji => String::from("Custom emoji is required for custom_emoji entity"),
                NoUrl => String::from("URL is required for text_link entity"),
                NoUser => String::from("user is required for text_mention entity"),
                // Serialize(err) => format!("failed to serialize text entities: {}", err),
            }
        )
    }
}

impl TryFrom<RawMessageEntity> for MessageEntity {
    type Error = MessageEntityError;

    fn try_from(raw: RawMessageEntity) -> Result<Self, Self::Error> {
        let position = TextEntityPosition {
            offset: raw.offset,
            length: raw.length,
        };

        Ok(match raw.entity_type {
            RawMessageEntityType::Blockquote => MessageEntity::Blockquote(position),
            RawMessageEntityType::Bold => MessageEntity::Bold(position),
            RawMessageEntityType::BotCommand => MessageEntity::BotCommand(position),
            RawMessageEntityType::Cashtag => MessageEntity::Cashtag(position),
            RawMessageEntityType::Code => MessageEntity::Code(position),
            RawMessageEntityType::CustomEmoji { custom_emoji_id } => MessageEntity::CustomEmoji {
                position,
                custom_emoji_id: custom_emoji_id.ok_or(MessageEntityError::NoCustomEmoji)?,
            },
            RawMessageEntityType::Email => MessageEntity::Email(position),
            RawMessageEntityType::ExpandableBlockquote => MessageEntity::ExpandableBlockquote(position),
            RawMessageEntityType::Hashtag => MessageEntity::Hashtag(position),
            RawMessageEntityType::Italic => MessageEntity::Italic(position),
            RawMessageEntityType::Mention => MessageEntity::Mention(position),
            RawMessageEntityType::PhoneNumber => MessageEntity::PhoneNumber(position),
            RawMessageEntityType::Pre { language } => MessageEntity::Pre { position, language },
            RawMessageEntityType::Spoiler => MessageEntity::Spoiler(position),
            RawMessageEntityType::Strikethrough => MessageEntity::Strikethrough(position),
            RawMessageEntityType::TextLink { url } => MessageEntity::TextLink {
                position,
                url: url.ok_or(MessageEntityError::NoUrl)?,
            },
            RawMessageEntityType::TextMention { user } => MessageEntity::TextMention {
                position,
                user: user.ok_or(MessageEntityError::NoUser)?,
            },
            RawMessageEntityType::Underline => MessageEntity::Underline(position),
            RawMessageEntityType::Url => MessageEntity::Url(position),
        })
    }
}

impl From<MessageEntity> for RawMessageEntity {
    fn from(entity: MessageEntity) -> Self {
        macro_rules! raw {
            ($entity_type:ident($position:ident $( , $item:ident )?)) => {
                RawMessageEntity {
                    entity_type: RawMessageEntityType::$entity_type $( { $item: $item.into() } )?,
                    offset: $position.offset as _,
                    length: $position.length as _,
                }
            };
        }
        match entity {
            MessageEntity::Blockquote(p) => raw!(Blockquote(p)),
            MessageEntity::Bold(p) => raw!(Bold(p)),
            MessageEntity::BotCommand(p) => raw!(BotCommand(p)),
            MessageEntity::Cashtag(p) => raw!(Cashtag(p)),
            MessageEntity::Code(p) => raw!(Code(p)),
            MessageEntity::CustomEmoji {
                position: p,
                custom_emoji_id,
            } => raw!(CustomEmoji(p, custom_emoji_id)),
            MessageEntity::Email(p) => raw!(Email(p)),
            MessageEntity::ExpandableBlockquote(p) => raw!(ExpandableBlockquote(p)),
            MessageEntity::Hashtag(p) => raw!(Hashtag(p)),
            MessageEntity::Italic(p) => raw!(Italic(p)),
            MessageEntity::Mention(p) => raw!(Mention(p)),
            MessageEntity::PhoneNumber(p) => raw!(PhoneNumber(p)),
            MessageEntity::Pre {
                position: p,
                language,
            } => raw!(Pre(p, language)),
            MessageEntity::Spoiler(p) => raw!(Spoiler(p)),
            MessageEntity::Strikethrough(p) => raw!(Strikethrough(p)),
            MessageEntity::TextLink { position: p, url } => raw!(TextLink(p, url)),
            MessageEntity::TextMention { position: p, user } => raw!(TextMention(p, user)),
            MessageEntity::Underline(p) => raw!(Underline(p)),
            MessageEntity::Url(p) => raw!(Url(p)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MessageEntityBotCommand {
    pub command: String,
    /// Username of a bot.
    pub bot_name: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct TextEntityPosition {
    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: u32,
    pub length: u32,
}

impl From<Range<u32>> for TextEntityPosition {
    fn from(range: Range<u32>) -> Self {
        Self {
            offset: range.start,
            length: range.end - range.start,
        }
    }
}
