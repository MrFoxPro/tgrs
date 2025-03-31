/* @generated */

use serde::{Serialize, Deserialize};
use serde_with::apply;
use derive_more::{From, Display};
use crate::{addons::*, custom::*, client::{Executable, FormParts}, InputFile};

pub const SCHEMA_VERSION: &str = "8.3.0";

/**Contains information about the affiliate that received a commission via this transaction.

https://core.telegram.org/bots/api/#affiliateinfo*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct AffiliateInfo {
	/**The chat that received an affiliate commission if it was received by a chat*/
	pub affiliate_chat: Option<Chat>,
	/**The bot or the user that received an affiliate commission if it was received by a bot or a user*/
	pub affiliate_user: Option<User>,
	/**Integer amount of Telegram Stars received by the affiliate from the transaction, rounded to 0; can be negative for refunds*/
	pub amount: i64,
	/**The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the bot from referred users*/
	pub commission_per_mille: i64,
	/**The number of 1/1000000000 shares of Telegram Stars received by the affiliate; from -999999999 to 999999999; can be negative for refunds*/
	pub nanostar_amount: Option<i64>,
}
impl AffiliateInfo {
	pub fn new(amount: impl Into<i64>, commission_per_mille: impl Into<i64>) -> Self {
		Self {
			affiliate_chat: None,
			affiliate_user: None,
			amount: amount.into(),
			commission_per_mille: commission_per_mille.into(),
			nanostar_amount: None,
		}
	}
	/** *Optional*. The chat that received an affiliate commission if it was received by a chat*/
	pub fn affiliate_chat(mut self, affiliate_chat: impl Into<Chat>) -> Self {
		self.affiliate_chat = Some(affiliate_chat.into());
		self
	}
	/** *Optional*. The bot or the user that received an affiliate commission if it was received by a bot or a user*/
	pub fn affiliate_user(mut self, affiliate_user: impl Into<User>) -> Self {
		self.affiliate_user = Some(affiliate_user.into());
		self
	}
	/** *Optional*. The number of 1/1000000000 shares of Telegram Stars received by the affiliate; from -999999999 to 999999999; can be negative for refunds*/
	pub fn nanostar_amount(mut self, nanostar_amount: impl Into<i64>) -> Self {
		self.nanostar_amount = Some(nanostar_amount.into());
		self
	}
}
/**This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).

https://core.telegram.org/bots/api/#animation*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Animation {
	/**Duration of the video in seconds as defined by the sender*/
	pub duration: i64,
	/**Identifier for this file, which can be used to download or reuse the file*/
	pub file_id: String,
	/**Original animation filename as defined by the sender*/
	pub file_name: Option<String>,
	/**File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub file_size: Option<i64>,
	/**Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub file_unique_id: String,
	/**Video height as defined by the sender*/
	pub height: i64,
	/**MIME type of the file as defined by the sender*/
	pub mime_type: Option<String>,
	/**Animation thumbnail as defined by the sender*/
	pub thumbnail: Option<PhotoSize>,
	/**Video width as defined by the sender*/
	pub width: i64,
}
impl Animation {
	pub fn new(duration: impl Into<i64>, file_id: impl Into<String>, file_unique_id: impl Into<String>, height: impl Into<i64>, width: impl Into<i64>) -> Self {
		Self {
			duration: duration.into(),
			file_id: file_id.into(),
			file_name: None,
			file_size: None,
			file_unique_id: file_unique_id.into(),
			height: height.into(),
			mime_type: None,
			thumbnail: None,
			width: width.into(),
		}
	}
	/** *Optional*. Original animation filename as defined by the sender*/
	pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
		self.file_name = Some(file_name.into());
		self
	}
	/** *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub fn file_size(mut self, file_size: impl Into<i64>) -> Self {
		self.file_size = Some(file_size.into());
		self
	}
	/** *Optional*. MIME type of the file as defined by the sender*/
	pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
		self.mime_type = Some(mime_type.into());
		self
	}
	/** *Optional*. Animation thumbnail as defined by the sender*/
	pub fn thumbnail(mut self, thumbnail: impl Into<PhotoSize>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
}
/**This object represents an audio file to be treated as music by the Telegram clients.

https://core.telegram.org/bots/api/#audio*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Audio {
	/**Duration of the audio in seconds as defined by the sender*/
	pub duration: i64,
	/**Identifier for this file, which can be used to download or reuse the file*/
	pub file_id: String,
	/**Original filename as defined by the sender*/
	pub file_name: Option<String>,
	/**File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub file_size: Option<i64>,
	/**Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub file_unique_id: String,
	/**MIME type of the file as defined by the sender*/
	pub mime_type: Option<String>,
	/**Performer of the audio as defined by the sender or by audio tags*/
	pub performer: Option<String>,
	/**Thumbnail of the album cover to which the music file belongs*/
	pub thumbnail: Option<PhotoSize>,
	/**Title of the audio as defined by the sender or by audio tags*/
	pub title: Option<String>,
}
impl Audio {
	pub fn new(duration: impl Into<i64>, file_id: impl Into<String>, file_unique_id: impl Into<String>) -> Self {
		Self {
			duration: duration.into(),
			file_id: file_id.into(),
			file_name: None,
			file_size: None,
			file_unique_id: file_unique_id.into(),
			mime_type: None,
			performer: None,
			thumbnail: None,
			title: None,
		}
	}
	/** *Optional*. Original filename as defined by the sender*/
	pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
		self.file_name = Some(file_name.into());
		self
	}
	/** *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub fn file_size(mut self, file_size: impl Into<i64>) -> Self {
		self.file_size = Some(file_size.into());
		self
	}
	/** *Optional*. MIME type of the file as defined by the sender*/
	pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
		self.mime_type = Some(mime_type.into());
		self
	}
	/** *Optional*. Performer of the audio as defined by the sender or by audio tags*/
	pub fn performer(mut self, performer: impl Into<String>) -> Self {
		self.performer = Some(performer.into());
		self
	}
	/** *Optional*. Thumbnail of the album cover to which the music file belongs*/
	pub fn thumbnail(mut self, thumbnail: impl Into<PhotoSize>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
	/** *Optional*. Title of the audio as defined by the sender or by audio tags*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
}
/**This object describes the way a background is filled based on the selected colors. Currently, it can be one of

* [BackgroundFillSolid](https://core.telegram.org/bots/api/#backgroundfillsolid)
* [BackgroundFillGradient](https://core.telegram.org/bots/api/#backgroundfillgradient)
* [BackgroundFillFreeformGradient](https://core.telegram.org/bots/api/#backgroundfillfreeformgradient)

https://core.telegram.org/bots/api/#backgroundfill*/
#[derive(Clone, Debug, Deserialize, From)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum BackgroundFill {
	FreeformGradient(BackgroundFillFreeformGradient),
	Gradient(BackgroundFillGradient),
	Solid(BackgroundFillSolid),
}
/**The background is a freeform gradient that rotates after every message in the chat.

https://core.telegram.org/bots/api/#backgroundfillfreeformgradient*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BackgroundFillFreeformGradient {
	/**A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format*/
	pub colors: Vec<i64>,
	/**Type of the background fill, always “freeform\_gradient”
	Default: freeform_gradient*/
	pub r#type: String,
}
impl BackgroundFillFreeformGradient {
	pub fn new(colors: impl Into<Vec<i64>>, r#type: impl Into<String>) -> Self {
		Self {
			colors: colors.into(),
			r#type: r#type.into(),
		}
	}
	pub fn add_color(mut self, color: impl Into<i64>) -> Self {
		self.colors.push(color.into());
		self
	}
}
/**The background is a gradient fill.

https://core.telegram.org/bots/api/#backgroundfillgradient*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BackgroundFillGradient {
	/**Bottom color of the gradient in the RGB24 format*/
	pub bottom_color: i64,
	/**Type of the background fill, always “gradient”
	Default: gradient*/
	pub r#type: String,
	/**Clockwise rotation angle of the background fill in degrees; 0-359*/
	pub rotation_angle: i64,
	/**Top color of the gradient in the RGB24 format*/
	pub top_color: i64,
}
impl BackgroundFillGradient {
	pub fn new(bottom_color: impl Into<i64>, r#type: impl Into<String>, rotation_angle: impl Into<i64>, top_color: impl Into<i64>) -> Self {
		Self {
			bottom_color: bottom_color.into(),
			r#type: r#type.into(),
			rotation_angle: rotation_angle.into(),
			top_color: top_color.into(),
		}
	}
}
/**The background is filled using the selected color.

https://core.telegram.org/bots/api/#backgroundfillsolid*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BackgroundFillSolid {
	/**The color of the background fill in the RGB24 format*/
	pub color: i64,
	/**Type of the background fill, always “solid”
	Default: solid*/
	pub r#type: String,
}
impl BackgroundFillSolid {
	pub fn new(color: impl Into<i64>, r#type: impl Into<String>) -> Self {
		Self {
			color: color.into(),
			r#type: r#type.into(),
		}
	}
}
/**This object describes the type of a background. Currently, it can be one of

* [BackgroundTypeFill](https://core.telegram.org/bots/api/#backgroundtypefill)
* [BackgroundTypeWallpaper](https://core.telegram.org/bots/api/#backgroundtypewallpaper)
* [BackgroundTypePattern](https://core.telegram.org/bots/api/#backgroundtypepattern)
* [BackgroundTypeChatTheme](https://core.telegram.org/bots/api/#backgroundtypechattheme)

https://core.telegram.org/bots/api/#backgroundtype*/
#[derive(Clone, Debug, Deserialize, From)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum BackgroundType {
	ChatTheme(BackgroundTypeChatTheme),
	Fill(BackgroundTypeFill),
	Pattern(BackgroundTypePattern),
	Wallpaper(BackgroundTypeWallpaper),
}
/**The background is taken directly from a built-in chat theme.

https://core.telegram.org/bots/api/#backgroundtypechattheme*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BackgroundTypeChatTheme {
	/**Type of the background, always “chat\_theme”
	Default: chat_theme*/
	pub r#type: String,
	/**Name of the chat theme, which is usually an emoji*/
	pub theme_name: String,
}
impl BackgroundTypeChatTheme {
	pub fn new(r#type: impl Into<String>, theme_name: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
			theme_name: theme_name.into(),
		}
	}
}
/**The background is automatically filled based on the selected colors.

https://core.telegram.org/bots/api/#backgroundtypefill*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BackgroundTypeFill {
	/**Dimming of the background in dark themes, as a percentage; 0-100*/
	pub dark_theme_dimming: i64,
	/**The background fill*/
	pub fill: BackgroundFill,
	/**Type of the background, always “fill”
	Default: fill*/
	pub r#type: String,
}
impl BackgroundTypeFill {
	pub fn new(dark_theme_dimming: impl Into<i64>, fill: impl Into<BackgroundFill>, r#type: impl Into<String>) -> Self {
		Self {
			dark_theme_dimming: dark_theme_dimming.into(),
			fill: fill.into(),
			r#type: r#type.into(),
		}
	}
}
/**The background is a .PNG or .TGV (gzipped subset of SVG with MIME type “application/x-tgwallpattern”) pattern to be combined with the background fill chosen by the user.

https://core.telegram.org/bots/api/#backgroundtypepattern*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BackgroundTypePattern {
	/**Document with the pattern*/
	pub document: Document,
	/**The background fill that is combined with the pattern*/
	pub fill: BackgroundFill,
	/**Intensity of the pattern when it is shown above the filled background; 0-100*/
	pub intensity: i64,
	/**if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
	Default value: true*/
	pub is_inverted: Option<bool>,
	/**if the background moves slightly when the device is tilted
	Default value: true*/
	pub is_moving: Option<bool>,
	/**Type of the background, always “pattern”
	Default: pattern*/
	pub r#type: String,
}
impl BackgroundTypePattern {
	pub fn new(document: impl Into<Document>, fill: impl Into<BackgroundFill>, intensity: impl Into<i64>, r#type: impl Into<String>) -> Self {
		Self {
			document: document.into(),
			fill: fill.into(),
			intensity: intensity.into(),
			is_inverted: None,
			is_moving: None,
			r#type: r#type.into(),
		}
	}
	/** *Optional*. *True*, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
	Default value: true*/
	pub fn is_inverted(mut self, is_inverted: bool) -> Self {
		self.is_inverted = Some(is_inverted);
		self
	}
	/** *Optional*. *True*, if the background moves slightly when the device is tilted
	Default value: true*/
	pub fn is_moving(mut self, is_moving: bool) -> Self {
		self.is_moving = Some(is_moving);
		self
	}
}
/**The background is a wallpaper in the JPEG format.

https://core.telegram.org/bots/api/#backgroundtypewallpaper*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BackgroundTypeWallpaper {
	/**Dimming of the background in dark themes, as a percentage; 0-100*/
	pub dark_theme_dimming: i64,
	/**Document with the wallpaper*/
	pub document: Document,
	/**if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
	Default value: true*/
	pub is_blurred: Option<bool>,
	/**if the background moves slightly when the device is tilted
	Default value: true*/
	pub is_moving: Option<bool>,
	/**Type of the background, always “wallpaper”
	Default: wallpaper*/
	pub r#type: String,
}
impl BackgroundTypeWallpaper {
	pub fn new(dark_theme_dimming: impl Into<i64>, document: impl Into<Document>, r#type: impl Into<String>) -> Self {
		Self {
			dark_theme_dimming: dark_theme_dimming.into(),
			document: document.into(),
			is_blurred: None,
			is_moving: None,
			r#type: r#type.into(),
		}
	}
	/** *Optional*. *True*, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
	Default value: true*/
	pub fn is_blurred(mut self, is_blurred: bool) -> Self {
		self.is_blurred = Some(is_blurred);
		self
	}
	/** *Optional*. *True*, if the background moves slightly when the device is tilted
	Default value: true*/
	pub fn is_moving(mut self, is_moving: bool) -> Self {
		self.is_moving = Some(is_moving);
		self
	}
}
/**Describes the birthdate of a user.

https://core.telegram.org/bots/api/#birthdate*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Birthdate {
	/**Day of the user's birth; 1-31*/
	pub day: i64,
	/**Month of the user's birth; 1-12*/
	pub month: i64,
	/**Year of the user's birth*/
	pub year: Option<i64>,
}
impl Birthdate {
	pub fn new(day: impl Into<i64>, month: impl Into<i64>) -> Self {
		Self {
			day: day.into(),
			month: month.into(),
			year: None,
		}
	}
	/** *Optional*. Year of the user's birth*/
	pub fn year(mut self, year: impl Into<i64>) -> Self {
		self.year = Some(year.into());
		self
	}
}
/**This object represents a bot command.

https://core.telegram.org/bots/api/#botcommand*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotCommand {
	/**Text of the command; 1-32 characters. Can contain only lowercase English letters, digits and underscores.
	Min len: 1
	Max len: 32*/
	pub command: String,
	/**Description of the command; 1-256 characters.
	Min len: 1
	Max len: 256*/
	pub description: String,
}
impl BotCommand {
	pub fn new(command: impl Into<String>, description: impl Into<String>) -> Self {
		Self {
			command: command.into(),
			description: description.into(),
		}
	}
}
/**This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:

* [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault)
* [BotCommandScopeAllPrivateChats](https://core.telegram.org/bots/api/#botcommandscopeallprivatechats)
* [BotCommandScopeAllGroupChats](https://core.telegram.org/bots/api/#botcommandscopeallgroupchats)
* [BotCommandScopeAllChatAdministrators](https://core.telegram.org/bots/api/#botcommandscopeallchatadministrators)
* [BotCommandScopeChat](https://core.telegram.org/bots/api/#botcommandscopechat)
* [BotCommandScopeChatAdministrators](https://core.telegram.org/bots/api/#botcommandscopechatadministrators)
* [BotCommandScopeChatMember](https://core.telegram.org/bots/api/#botcommandscopechatmember)

https://core.telegram.org/bots/api/#botcommandscope*/
#[derive(Clone, Debug, Serialize, From)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum BotCommandScope {
	AllChatAdministrators(BotCommandScopeAllChatAdministrators),
	AllGroupChats(BotCommandScopeAllGroupChats),
	AllPrivateChats(BotCommandScopeAllPrivateChats),
	Chat(BotCommandScopeChat),
	ChatAdministrators(BotCommandScopeChatAdministrators),
	ChatMember(BotCommandScopeChatMember),
	Default(BotCommandScopeDefault),
}
/**Represents the [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands, covering all group and supergroup chat administrators.

https://core.telegram.org/bots/api/#botcommandscopeallchatadministrators*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct BotCommandScopeAllChatAdministrators {
	/**Scope type, must be *all\_chat\_administrators*
	Default: all_chat_administrators*/
	pub r#type: String,
}
impl BotCommandScopeAllChatAdministrators {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**Represents the [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands, covering all group and supergroup chats.

https://core.telegram.org/bots/api/#botcommandscopeallgroupchats*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct BotCommandScopeAllGroupChats {
	/**Scope type, must be *all\_group\_chats*
	Default: all_group_chats*/
	pub r#type: String,
}
impl BotCommandScopeAllGroupChats {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**Represents the [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands, covering all private chats.

https://core.telegram.org/bots/api/#botcommandscopeallprivatechats*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct BotCommandScopeAllPrivateChats {
	/**Scope type, must be *all\_private\_chats*
	Default: all_private_chats*/
	pub r#type: String,
}
impl BotCommandScopeAllPrivateChats {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**Represents the [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands, covering a specific chat.

https://core.telegram.org/bots/api/#botcommandscopechat*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct BotCommandScopeChat {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**Scope type, must be *chat*
	Default: chat*/
	pub r#type: String,
}
impl BotCommandScopeChat {
	pub fn new(chat_id: impl Into<ChatId>, r#type: impl Into<String>) -> Self {
		Self {
			chat_id: chat_id.into(),
			r#type: r#type.into(),
		}
	}
}
/**Represents the [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands, covering all administrators of a specific group or supergroup chat.

https://core.telegram.org/bots/api/#botcommandscopechatadministrators*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct BotCommandScopeChatAdministrators {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**Scope type, must be *chat\_administrators*
	Default: chat_administrators*/
	pub r#type: String,
}
impl BotCommandScopeChatAdministrators {
	pub fn new(chat_id: impl Into<ChatId>, r#type: impl Into<String>) -> Self {
		Self {
			chat_id: chat_id.into(),
			r#type: r#type.into(),
		}
	}
}
/**Represents the [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands, covering a specific member of a group or supergroup chat.

https://core.telegram.org/bots/api/#botcommandscopechatmember*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct BotCommandScopeChatMember {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**Scope type, must be *chat\_member*
	Default: chat_member*/
	pub r#type: String,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl BotCommandScopeChatMember {
	pub fn new(chat_id: impl Into<ChatId>, r#type: impl Into<String>, user_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			r#type: r#type.into(),
			user_id: user_id.into(),
		}
	}
}
/**Represents the default [scope](https://core.telegram.org/bots/api/#botcommandscope) of bot commands. Default commands are used if no commands with a [narrower scope](https://core.telegram.org/bots/api/#determining-list-of-commands) are specified for the user.

https://core.telegram.org/bots/api/#botcommandscopedefault*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct BotCommandScopeDefault {
	/**Scope type, must be *default*
	Default: default*/
	pub r#type: String,
}
impl BotCommandScopeDefault {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**This object represents the bot's description.

https://core.telegram.org/bots/api/#botdescription*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BotDescription {
	/**The bot's description*/
	pub description: String,
}
impl BotDescription {
	pub fn new(description: impl Into<String>) -> Self {
		Self {
			description: description.into(),
		}
	}
}
/**This object represents the bot's name.

https://core.telegram.org/bots/api/#botname*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BotName {
	/**The bot's name*/
	pub name: String,
}
impl BotName {
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
		}
	}
}
/**This object represents the bot's short description.

https://core.telegram.org/bots/api/#botshortdescription*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BotShortDescription {
	/**The bot's short description*/
	pub short_description: String,
}
impl BotShortDescription {
	pub fn new(short_description: impl Into<String>) -> Self {
		Self {
			short_description: short_description.into(),
		}
	}
}
/**Describes the connection of the bot with a business account.

https://core.telegram.org/bots/api/#businessconnection*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BusinessConnection {
	/**True, if the bot can act on behalf of the business account in chats that were active in the last 24 hours*/
	pub can_reply: bool,
	/**Date the connection was established in Unix time*/
	pub date: i64,
	/**Unique identifier of the business connection*/
	pub id: String,
	/**True, if the connection is active*/
	pub is_enabled: bool,
	/**Business account user that created the business connection*/
	pub user: User,
	/**Identifier of a private chat with the user who created the business connection. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.*/
	pub user_chat_id: i64,
}
impl BusinessConnection {
	pub fn new(can_reply: bool, date: impl Into<i64>, id: impl Into<String>, is_enabled: bool, user: impl Into<User>, user_chat_id: impl Into<i64>) -> Self {
		Self {
			can_reply: can_reply,
			date: date.into(),
			id: id.into(),
			is_enabled: is_enabled,
			user: user.into(),
			user_chat_id: user_chat_id.into(),
		}
	}
}
/**Contains information about the start page settings of a Telegram Business account.

https://core.telegram.org/bots/api/#businessintro*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BusinessIntro {
	/**Message text of the business intro*/
	pub message: Option<String>,
	/**Sticker of the business intro*/
	pub sticker: Option<Sticker>,
	/**Title text of the business intro*/
	pub title: Option<String>,
}
impl BusinessIntro {
	pub fn new() -> Self {
		Self {
			message: None,
			sticker: None,
			title: None,
		}
	}
	/** *Optional*. Message text of the business intro*/
	pub fn message(mut self, message: impl Into<String>) -> Self {
		self.message = Some(message.into());
		self
	}
	/** *Optional*. Sticker of the business intro*/
	pub fn sticker(mut self, sticker: impl Into<Sticker>) -> Self {
		self.sticker = Some(sticker.into());
		self
	}
	/** *Optional*. Title text of the business intro*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
}
/**Contains information about the location of a Telegram Business account.

https://core.telegram.org/bots/api/#businesslocation*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BusinessLocation {
	/**Address of the business*/
	pub address: String,
	/**Location of the business*/
	pub location: Option<Location>,
}
impl BusinessLocation {
	pub fn new(address: impl Into<String>) -> Self {
		Self {
			address: address.into(),
			location: None,
		}
	}
	/** *Optional*. Location of the business*/
	pub fn location(mut self, location: impl Into<Location>) -> Self {
		self.location = Some(location.into());
		self
	}
}
/**This object is received when messages are deleted from a connected business account.

https://core.telegram.org/bots/api/#businessmessagesdeleted*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BusinessMessagesDeleted {
	/**Unique identifier of the business connection*/
	pub business_connection_id: String,
	/**Information about a chat in the business account. The bot may not have access to the chat or the corresponding user.*/
	pub chat: Chat,
	/**The list of identifiers of deleted messages in the chat of the business account*/
	pub message_ids: Vec<i64>,
}
impl BusinessMessagesDeleted {
	pub fn new(business_connection_id: impl Into<String>, chat: impl Into<Chat>, message_ids: impl Into<Vec<i64>>) -> Self {
		Self {
			business_connection_id: business_connection_id.into(),
			chat: chat.into(),
			message_ids: message_ids.into(),
		}
	}
	pub fn add_message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_ids.push(message_id.into());
		self
	}
}
/**Describes the opening hours of a business.

https://core.telegram.org/bots/api/#businessopeninghours*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BusinessOpeningHours {
	/**List of time intervals describing business opening hours*/
	pub opening_hours: Vec<BusinessOpeningHoursInterval>,
	/**Unique name of the time zone for which the opening hours are defined*/
	pub time_zone_name: String,
}
impl BusinessOpeningHours {
	pub fn new(opening_hours: impl Into<Vec<BusinessOpeningHoursInterval>>, time_zone_name: impl Into<String>) -> Self {
		Self {
			opening_hours: opening_hours.into(),
			time_zone_name: time_zone_name.into(),
		}
	}
	pub fn add_opening_hour(mut self, opening_hour: impl Into<BusinessOpeningHoursInterval>) -> Self {
		self.opening_hours.push(opening_hour.into());
		self
	}
}
/**Describes an interval of time during which a business is open.

https://core.telegram.org/bots/api/#businessopeninghoursinterval*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct BusinessOpeningHoursInterval {
	/**The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 0 - 8 \* 24 \* 60*/
	pub closing_minute: i64,
	/**The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0 - 7 \* 24 \* 60*/
	pub opening_minute: i64,
}
impl BusinessOpeningHoursInterval {
	pub fn new(closing_minute: impl Into<i64>, opening_minute: impl Into<i64>) -> Self {
		Self {
			closing_minute: closing_minute.into(),
			opening_minute: opening_minute.into(),
		}
	}
}
/**A placeholder, currently holds no information. Use [BotFather](https://t.me/botfather) to set up your game.

https://core.telegram.org/bots/api/#callbackgame*/
pub type CallbackGame = ();
/**This object represents an incoming callback query from a callback button in an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If the button that originated the query was attached to a message sent by the bot, the field *message* will be present. If the button was attached to a message sent via the bot (in [inline mode](https://core.telegram.org/bots/api/#inline-mode)), the field *inline\_message\_id* will be present. Exactly one of the fields *data* or *game\_short\_name* will be present.

https://core.telegram.org/bots/api/#callbackquery*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct CallbackQuery {
	/**Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in [games](https://core.telegram.org/bots/api/#games).*/
	pub chat_instance: String,
	/**Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.*/
	pub data: Option<String>,
	/**Sender*/
	pub from: User,
	/**Short name of a [Game](https://core.telegram.org/bots/api/#games) to be returned, serves as the unique identifier for the game*/
	pub game_short_name: Option<String>,
	/**Unique identifier for this query*/
	pub id: String,
	/**Identifier of the message sent via the bot in inline mode, that originated the query.*/
	pub inline_message_id: Option<String>,
	/**Message sent by the bot with the callback button that originated the query*/
	pub message: Option<MaybeInaccessibleMessage>,
}
impl CallbackQuery {
	pub fn new(chat_instance: impl Into<String>, from: impl Into<User>, id: impl Into<String>) -> Self {
		Self {
			chat_instance: chat_instance.into(),
			data: None,
			from: from.into(),
			game_short_name: None,
			id: id.into(),
			inline_message_id: None,
			message: None,
		}
	}
	/** *Optional*. Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.*/
	pub fn data(mut self, data: impl Into<String>) -> Self {
		self.data = Some(data.into());
		self
	}
	/** *Optional*. Short name of a [Game](https://core.telegram.org/bots/api/#games) to be returned, serves as the unique identifier for the game*/
	pub fn game_short_name(mut self, game_short_name: impl Into<String>) -> Self {
		self.game_short_name = Some(game_short_name.into());
		self
	}
	/** *Optional*. Identifier of the message sent via the bot in inline mode, that originated the query.*/
	pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
		self.inline_message_id = Some(inline_message_id.into());
		self
	}
	/** *Optional*. Message sent by the bot with the callback button that originated the query*/
	pub fn message(mut self, message: impl Into<MaybeInaccessibleMessage>) -> Self {
		self.message = Some(message.into());
		self
	}
}
/**This object represents a chat.

https://core.telegram.org/bots/api/#chat*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Chat {
	/**First name of the other party in a private chat*/
	pub first_name: Option<String>,
	/**Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.*/
	pub id: i64,
	/**if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
	Default value: true*/
	pub is_forum: Option<bool>,
	/**Last name of the other party in a private chat*/
	pub last_name: Option<String>,
	/**Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
	One of: private, group, supergroup, channel*/
	pub r#type: String,
	/**Title, for supergroups, channels and group chats*/
	pub title: Option<String>,
	/**Username, for private chats, supergroups and channels if available*/
	pub username: Option<String>,
}
impl Chat {
	pub fn new(id: impl Into<i64>, r#type: impl Into<String>) -> Self {
		Self {
			first_name: None,
			id: id.into(),
			is_forum: None,
			last_name: None,
			r#type: r#type.into(),
			title: None,
			username: None,
		}
	}
	/** *Optional*. First name of the other party in a private chat*/
	pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
		self.first_name = Some(first_name.into());
		self
	}
	/** *Optional*. *True*, if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
	Default value: true*/
	pub fn is_forum(mut self, is_forum: bool) -> Self {
		self.is_forum = Some(is_forum);
		self
	}
	/** *Optional*. Last name of the other party in a private chat*/
	pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
		self.last_name = Some(last_name.into());
		self
	}
	/** *Optional*. Title, for supergroups, channels and group chats*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
	/** *Optional*. Username, for private chats, supergroups and channels if available*/
	pub fn username(mut self, username: impl Into<String>) -> Self {
		self.username = Some(username.into());
		self
	}
}
/**Represents the rights of an administrator in a chat.

https://core.telegram.org/bots/api/#chatadministratorrights*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatAdministratorRights {
	/**if the user is allowed to change the chat title, photo and other settings*/
	pub can_change_info: bool,
	/**if the administrator can delete messages of other users*/
	pub can_delete_messages: bool,
	/**if the administrator can delete stories posted by other users*/
	pub can_delete_stories: bool,
	/**if the administrator can edit messages of other users and can pin messages; for channels only*/
	pub can_edit_messages: Option<bool>,
	/**if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive*/
	pub can_edit_stories: bool,
	/**if the user is allowed to invite new users to the chat*/
	pub can_invite_users: bool,
	/**if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages and ignore slow mode. Implied by any other administrator privilege.*/
	pub can_manage_chat: bool,
	/**if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only*/
	pub can_manage_topics: Option<bool>,
	/**if the administrator can manage video chats*/
	pub can_manage_video_chats: bool,
	/**if the user is allowed to pin messages; for groups and supergroups only*/
	pub can_pin_messages: Option<bool>,
	/**if the administrator can post messages in the channel, or access channel statistics; for channels only*/
	pub can_post_messages: Option<bool>,
	/**if the administrator can post stories to the chat*/
	pub can_post_stories: bool,
	/**if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)*/
	pub can_promote_members: bool,
	/**if the administrator can restrict, ban or unban chat members, or access supergroup statistics*/
	pub can_restrict_members: bool,
	/**if the user's presence in the chat is hidden*/
	pub is_anonymous: bool,
}
impl ChatAdministratorRights {
	pub fn new(can_change_info: bool, can_delete_messages: bool, can_delete_stories: bool, can_edit_stories: bool, can_invite_users: bool, can_manage_chat: bool, can_manage_video_chats: bool, can_post_stories: bool, can_promote_members: bool, can_restrict_members: bool, is_anonymous: bool) -> Self {
		Self {
			can_change_info: can_change_info,
			can_delete_messages: can_delete_messages,
			can_delete_stories: can_delete_stories,
			can_edit_messages: None,
			can_edit_stories: can_edit_stories,
			can_invite_users: can_invite_users,
			can_manage_chat: can_manage_chat,
			can_manage_topics: None,
			can_manage_video_chats: can_manage_video_chats,
			can_pin_messages: None,
			can_post_messages: None,
			can_post_stories: can_post_stories,
			can_promote_members: can_promote_members,
			can_restrict_members: can_restrict_members,
			is_anonymous: is_anonymous,
		}
	}
	/** *Optional*. *True*, if the administrator can edit messages of other users and can pin messages; for channels only*/
	pub fn can_edit_messages(mut self, can_edit_messages: bool) -> Self {
		self.can_edit_messages = Some(can_edit_messages);
		self
	}
	/** *Optional*. *True*, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only*/
	pub fn can_manage_topics(mut self, can_manage_topics: bool) -> Self {
		self.can_manage_topics = Some(can_manage_topics);
		self
	}
	/** *Optional*. *True*, if the user is allowed to pin messages; for groups and supergroups only*/
	pub fn can_pin_messages(mut self, can_pin_messages: bool) -> Self {
		self.can_pin_messages = Some(can_pin_messages);
		self
	}
	/** *Optional*. *True*, if the administrator can post messages in the channel, or access channel statistics; for channels only*/
	pub fn can_post_messages(mut self, can_post_messages: bool) -> Self {
		self.can_post_messages = Some(can_post_messages);
		self
	}
}
/**This object represents a chat background.

https://core.telegram.org/bots/api/#chatbackground*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatBackground {
	/**Type of the background*/
	pub r#type: BackgroundType,
}
impl ChatBackground {
	pub fn new(r#type: impl Into<BackgroundType>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**This object contains information about a chat boost.

https://core.telegram.org/bots/api/#chatboost*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatBoost {
	/**Point in time (Unix timestamp) when the chat was boosted*/
	pub add_date: i64,
	/**Unique identifier of the boost*/
	pub boost_id: String,
	/**Point in time (Unix timestamp) when the boost will automatically expire, unless the booster's Telegram Premium subscription is prolonged*/
	pub expiration_date: i64,
	/**Source of the added boost*/
	pub source: ChatBoostSource,
}
impl ChatBoost {
	pub fn new(add_date: impl Into<i64>, boost_id: impl Into<String>, expiration_date: impl Into<i64>, source: impl Into<ChatBoostSource>) -> Self {
		Self {
			add_date: add_date.into(),
			boost_id: boost_id.into(),
			expiration_date: expiration_date.into(),
			source: source.into(),
		}
	}
}
/**This object represents a service message about a user boosting a chat.

https://core.telegram.org/bots/api/#chatboostadded*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatBoostAdded {
	/**Number of boosts added by the user*/
	pub boost_count: i64,
}
impl ChatBoostAdded {
	pub fn new(boost_count: impl Into<i64>) -> Self {
		Self {
			boost_count: boost_count.into(),
		}
	}
}
/**This object represents a boost removed from a chat.

https://core.telegram.org/bots/api/#chatboostremoved*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatBoostRemoved {
	/**Unique identifier of the boost*/
	pub boost_id: String,
	/**Chat which was boosted*/
	pub chat: Chat,
	/**Point in time (Unix timestamp) when the boost was removed*/
	pub remove_date: i64,
	/**Source of the removed boost*/
	pub source: ChatBoostSource,
}
impl ChatBoostRemoved {
	pub fn new(boost_id: impl Into<String>, chat: impl Into<Chat>, remove_date: impl Into<i64>, source: impl Into<ChatBoostSource>) -> Self {
		Self {
			boost_id: boost_id.into(),
			chat: chat.into(),
			remove_date: remove_date.into(),
			source: source.into(),
		}
	}
}
/**This object describes the source of a chat boost. It can be one of

* [ChatBoostSourcePremium](https://core.telegram.org/bots/api/#chatboostsourcepremium)
* [ChatBoostSourceGiftCode](https://core.telegram.org/bots/api/#chatboostsourcegiftcode)
* [ChatBoostSourceGiveaway](https://core.telegram.org/bots/api/#chatboostsourcegiveaway)

https://core.telegram.org/bots/api/#chatboostsource*/
#[derive(Clone, Debug, Deserialize, From)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum ChatBoostSource {
	GiftCode(ChatBoostSourceGiftCode),
	Giveaway(ChatBoostSourceGiveaway),
	Premium(ChatBoostSourcePremium),
}
/**The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.

https://core.telegram.org/bots/api/#chatboostsourcegiftcode*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatBoostSourceGiftCode {
	/**Source of the boost, always “gift\_code”
	Default: gift_code*/
	pub source: String,
	/**User for which the gift code was created*/
	pub user: User,
}
impl ChatBoostSourceGiftCode {
	pub fn new(source: impl Into<String>, user: impl Into<User>) -> Self {
		Self {
			source: source.into(),
			user: user.into(),
		}
	}
}
/**The boost was obtained by the creation of a Telegram Premium or a Telegram Star giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription for Telegram Premium giveaways and *prize\_star\_count* / 500 times for one year for Telegram Star giveaways.

https://core.telegram.org/bots/api/#chatboostsourcegiveaway*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatBoostSourceGiveaway {
	/**Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.*/
	pub giveaway_message_id: i64,
	/**True, if the giveaway was completed, but there was no user to win the prize
	Default value: true*/
	pub is_unclaimed: Option<bool>,
	/**The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only*/
	pub prize_star_count: Option<i64>,
	/**Source of the boost, always “giveaway”
	Default: giveaway*/
	pub source: String,
	/**User that won the prize in the giveaway if any; for Telegram Premium giveaways only*/
	pub user: Option<User>,
}
impl ChatBoostSourceGiveaway {
	pub fn new(giveaway_message_id: impl Into<i64>, source: impl Into<String>) -> Self {
		Self {
			giveaway_message_id: giveaway_message_id.into(),
			is_unclaimed: None,
			prize_star_count: None,
			source: source.into(),
			user: None,
		}
	}
	/** *Optional*. True, if the giveaway was completed, but there was no user to win the prize
	Default value: true*/
	pub fn is_unclaimed(mut self, is_unclaimed: bool) -> Self {
		self.is_unclaimed = Some(is_unclaimed);
		self
	}
	/** *Optional*. The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only*/
	pub fn prize_star_count(mut self, prize_star_count: impl Into<i64>) -> Self {
		self.prize_star_count = Some(prize_star_count.into());
		self
	}
	/** *Optional*. User that won the prize in the giveaway if any; for Telegram Premium giveaways only*/
	pub fn user(mut self, user: impl Into<User>) -> Self {
		self.user = Some(user.into());
		self
	}
}
/**The boost was obtained by subscribing to Telegram Premium or by gifting a Telegram Premium subscription to another user.

https://core.telegram.org/bots/api/#chatboostsourcepremium*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatBoostSourcePremium {
	/**Source of the boost, always “premium”
	Default: premium*/
	pub source: String,
	/**User that boosted the chat*/
	pub user: User,
}
impl ChatBoostSourcePremium {
	pub fn new(source: impl Into<String>, user: impl Into<User>) -> Self {
		Self {
			source: source.into(),
			user: user.into(),
		}
	}
}
/**This object represents a boost added to a chat or changed.

https://core.telegram.org/bots/api/#chatboostupdated*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatBoostUpdated {
	/**Information about the chat boost*/
	pub boost: ChatBoost,
	/**Chat which was boosted*/
	pub chat: Chat,
}
impl ChatBoostUpdated {
	pub fn new(boost: impl Into<ChatBoost>, chat: impl Into<Chat>) -> Self {
		Self {
			boost: boost.into(),
			chat: chat.into(),
		}
	}
}
/**This object contains full information about a chat.

https://core.telegram.org/bots/api/#chatfullinfo*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatFullInfo {
	/**Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See [accent colors](https://core.telegram.org/bots/api/#accent-colors) for more details.*/
	pub accent_color_id: i64,
	/**If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames#collectible-usernames); for private chats, supergroups and channels*/
	pub active_usernames: Vec<String>,
	/**List of available reactions allowed in the chat. If omitted, then all [emoji reactions](https://core.telegram.org/bots/api/#reactiontypeemoji) are allowed.*/
	pub available_reactions: Vec<ReactionType>,
	/**Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background*/
	pub background_custom_emoji_id: Option<String>,
	/**Bio of the other party in a private chat*/
	pub bio: Option<String>,
	/**For private chats, the date of birth of the user*/
	pub birthdate: Option<Birthdate>,
	/**For private chats with business accounts, the intro of the business*/
	pub business_intro: Option<BusinessIntro>,
	/**For private chats with business accounts, the location of the business*/
	pub business_location: Option<BusinessLocation>,
	/**For private chats with business accounts, the opening hours of the business*/
	pub business_opening_hours: Option<BusinessOpeningHours>,
	/**if gifts can be sent to the chat
	Default value: true*/
	pub can_send_gift: Option<bool>,
	/**if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
	Default value: true*/
	pub can_send_paid_media: Option<bool>,
	/**if the bot can change the group sticker set
	Default value: true*/
	pub can_set_sticker_set: Option<bool>,
	/**For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.*/
	pub custom_emoji_sticker_set_name: Option<String>,
	/**Description, for groups, supergroups and channel chats*/
	pub description: Option<String>,
	/**Custom emoji identifier of the emoji status of the chat or the other party in a private chat*/
	pub emoji_status_custom_emoji_id: Option<String>,
	/**Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any*/
	pub emoji_status_expiration_date: Option<i64>,
	/**First name of the other party in a private chat*/
	pub first_name: Option<String>,
	/**if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
	Default value: true*/
	pub has_aggressive_anti_spam_enabled: Option<bool>,
	/**if non-administrators can only get the list of bots and administrators in the chat
	Default value: true*/
	pub has_hidden_members: Option<bool>,
	/**if privacy settings of the other party in the private chat allows to use `tg://user?id=<user_id>` links only in chats with the user
	Default value: true*/
	pub has_private_forwards: Option<bool>,
	/**if messages from the chat can't be forwarded to other chats
	Default value: true*/
	pub has_protected_content: Option<bool>,
	/**if the privacy settings of the other party restrict sending voice and video note messages in the private chat
	Default value: true*/
	pub has_restricted_voice_and_video_messages: Option<bool>,
	/**if new chat members will have access to old messages; available only to chat administrators
	Default value: true*/
	pub has_visible_history: Option<bool>,
	/**Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.*/
	pub id: i64,
	/**Primary invite link, for groups, supergroups and channel chats*/
	pub invite_link: Option<String>,
	/**if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
	Default value: true*/
	pub is_forum: Option<bool>,
	/**if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
	Default value: true*/
	pub join_by_request: Option<bool>,
	/**if users need to join the supergroup before they can send messages
	Default value: true*/
	pub join_to_send_messages: Option<bool>,
	/**Last name of the other party in a private chat*/
	pub last_name: Option<String>,
	/**Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.*/
	pub linked_chat_id: Option<i64>,
	/**For supergroups, the location to which the supergroup is connected*/
	pub location: Option<ChatLocation>,
	/**The maximum number of reactions that can be set on a message in the chat*/
	pub max_reaction_count: i64,
	/**The time after which all messages sent to the chat will be automatically deleted; in seconds*/
	pub message_auto_delete_time: Option<i64>,
	/**Default chat member permissions, for groups and supergroups*/
	pub permissions: Option<ChatPermissions>,
	/**For private chats, the personal channel of the user*/
	pub personal_chat: Option<Chat>,
	/**Chat photo*/
	pub photo: Option<ChatPhoto>,
	/**The most recent pinned message (by sending date)*/
	pub pinned_message: Option<Message>,
	/**Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api/#profile-accent-colors) for more details.*/
	pub profile_accent_color_id: Option<i64>,
	/**Custom emoji identifier of the emoji chosen by the chat for its profile background*/
	pub profile_background_custom_emoji_id: Option<String>,
	/**Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
	One of: private, group, supergroup, channel*/
	pub r#type: String,
	/**For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds*/
	pub slow_mode_delay: Option<i64>,
	/**For supergroups, name of the group sticker set*/
	pub sticker_set_name: Option<String>,
	/**Title, for supergroups, channels and group chats*/
	pub title: Option<String>,
	/**For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions*/
	pub unrestrict_boost_count: Option<i64>,
	/**Username, for private chats, supergroups and channels if available*/
	pub username: Option<String>,
}
impl ChatFullInfo {
	pub fn new(accent_color_id: impl Into<i64>, id: impl Into<i64>, max_reaction_count: impl Into<i64>, r#type: impl Into<String>) -> Self {
		Self {
			accent_color_id: accent_color_id.into(),
			active_usernames: Vec::new(),
			available_reactions: Vec::new(),
			background_custom_emoji_id: None,
			bio: None,
			birthdate: None,
			business_intro: None,
			business_location: None,
			business_opening_hours: None,
			can_send_gift: None,
			can_send_paid_media: None,
			can_set_sticker_set: None,
			custom_emoji_sticker_set_name: None,
			description: None,
			emoji_status_custom_emoji_id: None,
			emoji_status_expiration_date: None,
			first_name: None,
			has_aggressive_anti_spam_enabled: None,
			has_hidden_members: None,
			has_private_forwards: None,
			has_protected_content: None,
			has_restricted_voice_and_video_messages: None,
			has_visible_history: None,
			id: id.into(),
			invite_link: None,
			is_forum: None,
			join_by_request: None,
			join_to_send_messages: None,
			last_name: None,
			linked_chat_id: None,
			location: None,
			max_reaction_count: max_reaction_count.into(),
			message_auto_delete_time: None,
			permissions: None,
			personal_chat: None,
			photo: None,
			pinned_message: None,
			profile_accent_color_id: None,
			profile_background_custom_emoji_id: None,
			r#type: r#type.into(),
			slow_mode_delay: None,
			sticker_set_name: None,
			title: None,
			unrestrict_boost_count: None,
			username: None,
		}
	}
	pub fn add_active_username(mut self, active_username: impl Into<String>) -> Self {
		self.active_usernames.push(active_username.into());
		self
	}
	/** *Optional*. If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames#collectible-usernames); for private chats, supergroups and channels*/
	pub fn active_usernames(mut self, active_usernames: impl Into<Vec<String>>) -> Self {
		self.active_usernames = active_usernames.into();
		self
	}
	pub fn add_available_reaction(mut self, available_reaction: impl Into<ReactionType>) -> Self {
		self.available_reactions.push(available_reaction.into());
		self
	}
	/** *Optional*. List of available reactions allowed in the chat. If omitted, then all [emoji reactions](https://core.telegram.org/bots/api/#reactiontypeemoji) are allowed.*/
	pub fn available_reactions(mut self, available_reactions: impl Into<Vec<ReactionType>>) -> Self {
		self.available_reactions = available_reactions.into();
		self
	}
	/** *Optional*. Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background*/
	pub fn background_custom_emoji_id(mut self, background_custom_emoji_id: impl Into<String>) -> Self {
		self.background_custom_emoji_id = Some(background_custom_emoji_id.into());
		self
	}
	/** *Optional*. Bio of the other party in a private chat*/
	pub fn bio(mut self, bio: impl Into<String>) -> Self {
		self.bio = Some(bio.into());
		self
	}
	/** *Optional*. For private chats, the date of birth of the user*/
	pub fn birthdate(mut self, birthdate: impl Into<Birthdate>) -> Self {
		self.birthdate = Some(birthdate.into());
		self
	}
	/** *Optional*. For private chats with business accounts, the intro of the business*/
	pub fn business_intro(mut self, business_intro: impl Into<BusinessIntro>) -> Self {
		self.business_intro = Some(business_intro.into());
		self
	}
	/** *Optional*. For private chats with business accounts, the location of the business*/
	pub fn business_location(mut self, business_location: impl Into<BusinessLocation>) -> Self {
		self.business_location = Some(business_location.into());
		self
	}
	/** *Optional*. For private chats with business accounts, the opening hours of the business*/
	pub fn business_opening_hours(mut self, business_opening_hours: impl Into<BusinessOpeningHours>) -> Self {
		self.business_opening_hours = Some(business_opening_hours.into());
		self
	}
	/** *Optional*. *True*, if gifts can be sent to the chat
	Default value: true*/
	pub fn can_send_gift(mut self, can_send_gift: bool) -> Self {
		self.can_send_gift = Some(can_send_gift);
		self
	}
	/** *Optional*. *True*, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
	Default value: true*/
	pub fn can_send_paid_media(mut self, can_send_paid_media: bool) -> Self {
		self.can_send_paid_media = Some(can_send_paid_media);
		self
	}
	/** *Optional*. *True*, if the bot can change the group sticker set
	Default value: true*/
	pub fn can_set_sticker_set(mut self, can_set_sticker_set: bool) -> Self {
		self.can_set_sticker_set = Some(can_set_sticker_set);
		self
	}
	/** *Optional*. For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.*/
	pub fn custom_emoji_sticker_set_name(mut self, custom_emoji_sticker_set_name: impl Into<String>) -> Self {
		self.custom_emoji_sticker_set_name = Some(custom_emoji_sticker_set_name.into());
		self
	}
	/** *Optional*. Description, for groups, supergroups and channel chats*/
	pub fn description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}
	/** *Optional*. Custom emoji identifier of the emoji status of the chat or the other party in a private chat*/
	pub fn emoji_status_custom_emoji_id(mut self, emoji_status_custom_emoji_id: impl Into<String>) -> Self {
		self.emoji_status_custom_emoji_id = Some(emoji_status_custom_emoji_id.into());
		self
	}
	/** *Optional*. Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any*/
	pub fn emoji_status_expiration_date(mut self, emoji_status_expiration_date: impl Into<i64>) -> Self {
		self.emoji_status_expiration_date = Some(emoji_status_expiration_date.into());
		self
	}
	/** *Optional*. First name of the other party in a private chat*/
	pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
		self.first_name = Some(first_name.into());
		self
	}
	/** *Optional*. *True*, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
	Default value: true*/
	pub fn has_aggressive_anti_spam_enabled(mut self, has_aggressive_anti_spam_enabled: bool) -> Self {
		self.has_aggressive_anti_spam_enabled = Some(has_aggressive_anti_spam_enabled);
		self
	}
	/** *Optional*. *True*, if non-administrators can only get the list of bots and administrators in the chat
	Default value: true*/
	pub fn has_hidden_members(mut self, has_hidden_members: bool) -> Self {
		self.has_hidden_members = Some(has_hidden_members);
		self
	}
	/** *Optional*. *True*, if privacy settings of the other party in the private chat allows to use `tg://user?id=<user_id>` links only in chats with the user
	Default value: true*/
	pub fn has_private_forwards(mut self, has_private_forwards: bool) -> Self {
		self.has_private_forwards = Some(has_private_forwards);
		self
	}
	/** *Optional*. *True*, if messages from the chat can't be forwarded to other chats
	Default value: true*/
	pub fn has_protected_content(mut self, has_protected_content: bool) -> Self {
		self.has_protected_content = Some(has_protected_content);
		self
	}
	/** *Optional*. *True*, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
	Default value: true*/
	pub fn has_restricted_voice_and_video_messages(mut self, has_restricted_voice_and_video_messages: bool) -> Self {
		self.has_restricted_voice_and_video_messages = Some(has_restricted_voice_and_video_messages);
		self
	}
	/** *Optional*. *True*, if new chat members will have access to old messages; available only to chat administrators
	Default value: true*/
	pub fn has_visible_history(mut self, has_visible_history: bool) -> Self {
		self.has_visible_history = Some(has_visible_history);
		self
	}
	/** *Optional*. Primary invite link, for groups, supergroups and channel chats*/
	pub fn invite_link(mut self, invite_link: impl Into<String>) -> Self {
		self.invite_link = Some(invite_link.into());
		self
	}
	/** *Optional*. *True*, if the supergroup chat is a forum (has [topics](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
	Default value: true*/
	pub fn is_forum(mut self, is_forum: bool) -> Self {
		self.is_forum = Some(is_forum);
		self
	}
	/** *Optional*. *True*, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
	Default value: true*/
	pub fn join_by_request(mut self, join_by_request: bool) -> Self {
		self.join_by_request = Some(join_by_request);
		self
	}
	/** *Optional*. *True*, if users need to join the supergroup before they can send messages
	Default value: true*/
	pub fn join_to_send_messages(mut self, join_to_send_messages: bool) -> Self {
		self.join_to_send_messages = Some(join_to_send_messages);
		self
	}
	/** *Optional*. Last name of the other party in a private chat*/
	pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
		self.last_name = Some(last_name.into());
		self
	}
	/** *Optional*. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.*/
	pub fn linked_chat_id(mut self, linked_chat_id: impl Into<i64>) -> Self {
		self.linked_chat_id = Some(linked_chat_id.into());
		self
	}
	/** *Optional*. For supergroups, the location to which the supergroup is connected*/
	pub fn location(mut self, location: impl Into<ChatLocation>) -> Self {
		self.location = Some(location.into());
		self
	}
	/** *Optional*. The time after which all messages sent to the chat will be automatically deleted; in seconds*/
	pub fn message_auto_delete_time(mut self, message_auto_delete_time: impl Into<i64>) -> Self {
		self.message_auto_delete_time = Some(message_auto_delete_time.into());
		self
	}
	/** *Optional*. Default chat member permissions, for groups and supergroups*/
	pub fn permissions(mut self, permissions: impl Into<ChatPermissions>) -> Self {
		self.permissions = Some(permissions.into());
		self
	}
	/** *Optional*. For private chats, the personal channel of the user*/
	pub fn personal_chat(mut self, personal_chat: impl Into<Chat>) -> Self {
		self.personal_chat = Some(personal_chat.into());
		self
	}
	/** *Optional*. Chat photo*/
	pub fn photo(mut self, photo: impl Into<ChatPhoto>) -> Self {
		self.photo = Some(photo.into());
		self
	}
	/** *Optional*. The most recent pinned message (by sending date)*/
	pub fn pinned_message(mut self, pinned_message: impl Into<Message>) -> Self {
		self.pinned_message = Some(pinned_message.into());
		self
	}
	/** *Optional*. Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api/#profile-accent-colors) for more details.*/
	pub fn profile_accent_color_id(mut self, profile_accent_color_id: impl Into<i64>) -> Self {
		self.profile_accent_color_id = Some(profile_accent_color_id.into());
		self
	}
	/** *Optional*. Custom emoji identifier of the emoji chosen by the chat for its profile background*/
	pub fn profile_background_custom_emoji_id(mut self, profile_background_custom_emoji_id: impl Into<String>) -> Self {
		self.profile_background_custom_emoji_id = Some(profile_background_custom_emoji_id.into());
		self
	}
	/** *Optional*. For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds*/
	pub fn slow_mode_delay(mut self, slow_mode_delay: impl Into<i64>) -> Self {
		self.slow_mode_delay = Some(slow_mode_delay.into());
		self
	}
	/** *Optional*. For supergroups, name of the group sticker set*/
	pub fn sticker_set_name(mut self, sticker_set_name: impl Into<String>) -> Self {
		self.sticker_set_name = Some(sticker_set_name.into());
		self
	}
	/** *Optional*. Title, for supergroups, channels and group chats*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
	/** *Optional*. For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions*/
	pub fn unrestrict_boost_count(mut self, unrestrict_boost_count: impl Into<i64>) -> Self {
		self.unrestrict_boost_count = Some(unrestrict_boost_count.into());
		self
	}
	/** *Optional*. Username, for private chats, supergroups and channels if available*/
	pub fn username(mut self, username: impl Into<String>) -> Self {
		self.username = Some(username.into());
		self
	}
}
/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
#[derive(Clone, Debug, Serialize, From, Display)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum ChatId {
	Id(i64),
	Username(String),
}
/**Represents an invite link for a chat.

https://core.telegram.org/bots/api/#chatinvitelink*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatInviteLink {
	/**if users joining the chat via the link need to be approved by chat administrators*/
	pub creates_join_request: bool,
	/**Creator of the link*/
	pub creator: User,
	/**Point in time (Unix timestamp) when the link will expire or has been expired*/
	pub expire_date: Option<i64>,
	/**The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with “…”.*/
	pub invite_link: String,
	/**if the link is primary*/
	pub is_primary: bool,
	/**if the link is revoked*/
	pub is_revoked: bool,
	/**The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999*/
	pub member_limit: Option<i64>,
	/**Invite link name*/
	pub name: Option<String>,
	/**Number of pending join requests created using this link*/
	pub pending_join_request_count: Option<i64>,
	/**The number of seconds the subscription will be active for before the next payment*/
	pub subscription_period: Option<i64>,
	/**The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat using the link*/
	pub subscription_price: Option<i64>,
}
impl ChatInviteLink {
	pub fn new(creates_join_request: bool, creator: impl Into<User>, invite_link: impl Into<String>, is_primary: bool, is_revoked: bool) -> Self {
		Self {
			creates_join_request: creates_join_request,
			creator: creator.into(),
			expire_date: None,
			invite_link: invite_link.into(),
			is_primary: is_primary,
			is_revoked: is_revoked,
			member_limit: None,
			name: None,
			pending_join_request_count: None,
			subscription_period: None,
			subscription_price: None,
		}
	}
	/** *Optional*. Point in time (Unix timestamp) when the link will expire or has been expired*/
	pub fn expire_date(mut self, expire_date: impl Into<i64>) -> Self {
		self.expire_date = Some(expire_date.into());
		self
	}
	/** *Optional*. The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999*/
	pub fn member_limit(mut self, member_limit: impl Into<i64>) -> Self {
		self.member_limit = Some(member_limit.into());
		self
	}
	/** *Optional*. Invite link name*/
	pub fn name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}
	/** *Optional*. Number of pending join requests created using this link*/
	pub fn pending_join_request_count(mut self, pending_join_request_count: impl Into<i64>) -> Self {
		self.pending_join_request_count = Some(pending_join_request_count.into());
		self
	}
	/** *Optional*. The number of seconds the subscription will be active for before the next payment*/
	pub fn subscription_period(mut self, subscription_period: impl Into<i64>) -> Self {
		self.subscription_period = Some(subscription_period.into());
		self
	}
	/** *Optional*. The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat using the link*/
	pub fn subscription_price(mut self, subscription_price: impl Into<i64>) -> Self {
		self.subscription_price = Some(subscription_price.into());
		self
	}
}
/**Represents a join request sent to a chat.

https://core.telegram.org/bots/api/#chatjoinrequest*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatJoinRequest {
	/**Bio of the user.*/
	pub bio: Option<String>,
	/**Chat to which the request was sent*/
	pub chat: Chat,
	/**Date the request was sent in Unix time*/
	pub date: i64,
	/**User that sent the join request*/
	pub from: User,
	/**Chat invite link that was used by the user to send the join request*/
	pub invite_link: Option<ChatInviteLink>,
	/**Identifier of a private chat with the user who sent the join request. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot can use this identifier for 5 minutes to send messages until the join request is processed, assuming no other administrator contacted the user.*/
	pub user_chat_id: i64,
}
impl ChatJoinRequest {
	pub fn new(chat: impl Into<Chat>, date: impl Into<i64>, from: impl Into<User>, user_chat_id: impl Into<i64>) -> Self {
		Self {
			bio: None,
			chat: chat.into(),
			date: date.into(),
			from: from.into(),
			invite_link: None,
			user_chat_id: user_chat_id.into(),
		}
	}
	/** *Optional*. Bio of the user.*/
	pub fn bio(mut self, bio: impl Into<String>) -> Self {
		self.bio = Some(bio.into());
		self
	}
	/** *Optional*. Chat invite link that was used by the user to send the join request*/
	pub fn invite_link(mut self, invite_link: impl Into<ChatInviteLink>) -> Self {
		self.invite_link = Some(invite_link.into());
		self
	}
}
/**Represents a location to which a chat is connected.

https://core.telegram.org/bots/api/#chatlocation*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatLocation {
	/**Location address; 1-64 characters, as defined by the chat owner
	Min len: 1
	Max len: 64*/
	pub address: String,
	/**The location to which the supergroup is connected. Can't be a live location.*/
	pub location: Location,
}
impl ChatLocation {
	pub fn new(address: impl Into<String>, location: impl Into<Location>) -> Self {
		Self {
			address: address.into(),
			location: location.into(),
		}
	}
}
/**This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:

* [ChatMemberOwner](https://core.telegram.org/bots/api/#chatmemberowner)
* [ChatMemberAdministrator](https://core.telegram.org/bots/api/#chatmemberadministrator)
* [ChatMemberMember](https://core.telegram.org/bots/api/#chatmembermember)
* [ChatMemberRestricted](https://core.telegram.org/bots/api/#chatmemberrestricted)
* [ChatMemberLeft](https://core.telegram.org/bots/api/#chatmemberleft)
* [ChatMemberBanned](https://core.telegram.org/bots/api/#chatmemberbanned)

https://core.telegram.org/bots/api/#chatmember*/
#[derive(Clone, Debug, Deserialize, From)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ChatMember {
	Administrator(ChatMemberAdministrator),
	#[serde(rename = "kicked")]
	Banned(ChatMemberBanned),
	Left(ChatMemberLeft),
	Member(ChatMemberMember),
	#[serde(rename = "creator")]
	Owner(ChatMemberOwner),
	Restricted(ChatMemberRestricted),
}
/**Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that has some additional privileges.

https://core.telegram.org/bots/api/#chatmemberadministrator*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatMemberAdministrator {
	/**if the bot is allowed to edit administrator privileges of that user*/
	pub can_be_edited: bool,
	/**if the user is allowed to change the chat title, photo and other settings*/
	pub can_change_info: bool,
	/**if the administrator can delete messages of other users*/
	pub can_delete_messages: bool,
	/**if the administrator can delete stories posted by other users*/
	pub can_delete_stories: bool,
	/**if the administrator can edit messages of other users and can pin messages; for channels only*/
	pub can_edit_messages: Option<bool>,
	/**if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive*/
	pub can_edit_stories: bool,
	/**if the user is allowed to invite new users to the chat*/
	pub can_invite_users: bool,
	/**if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages and ignore slow mode. Implied by any other administrator privilege.*/
	pub can_manage_chat: bool,
	/**if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only*/
	pub can_manage_topics: Option<bool>,
	/**if the administrator can manage video chats*/
	pub can_manage_video_chats: bool,
	/**if the user is allowed to pin messages; for groups and supergroups only*/
	pub can_pin_messages: Option<bool>,
	/**if the administrator can post messages in the channel, or access channel statistics; for channels only*/
	pub can_post_messages: Option<bool>,
	/**if the administrator can post stories to the chat*/
	pub can_post_stories: bool,
	/**if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user)*/
	pub can_promote_members: bool,
	/**if the administrator can restrict, ban or unban chat members, or access supergroup statistics*/
	pub can_restrict_members: bool,
	/**Custom title for this user*/
	pub custom_title: Option<String>,
	/**if the user's presence in the chat is hidden*/
	pub is_anonymous: bool,
	/**Information about the user*/
	pub user: User,
}
impl ChatMemberAdministrator {
	pub fn new(can_be_edited: bool, can_change_info: bool, can_delete_messages: bool, can_delete_stories: bool, can_edit_stories: bool, can_invite_users: bool, can_manage_chat: bool, can_manage_video_chats: bool, can_post_stories: bool, can_promote_members: bool, can_restrict_members: bool, is_anonymous: bool, user: impl Into<User>) -> Self {
		Self {
			can_be_edited: can_be_edited,
			can_change_info: can_change_info,
			can_delete_messages: can_delete_messages,
			can_delete_stories: can_delete_stories,
			can_edit_messages: None,
			can_edit_stories: can_edit_stories,
			can_invite_users: can_invite_users,
			can_manage_chat: can_manage_chat,
			can_manage_topics: None,
			can_manage_video_chats: can_manage_video_chats,
			can_pin_messages: None,
			can_post_messages: None,
			can_post_stories: can_post_stories,
			can_promote_members: can_promote_members,
			can_restrict_members: can_restrict_members,
			custom_title: None,
			is_anonymous: is_anonymous,
			user: user.into(),
		}
	}
	/** *Optional*. *True*, if the administrator can edit messages of other users and can pin messages; for channels only*/
	pub fn can_edit_messages(mut self, can_edit_messages: bool) -> Self {
		self.can_edit_messages = Some(can_edit_messages);
		self
	}
	/** *Optional*. *True*, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only*/
	pub fn can_manage_topics(mut self, can_manage_topics: bool) -> Self {
		self.can_manage_topics = Some(can_manage_topics);
		self
	}
	/** *Optional*. *True*, if the user is allowed to pin messages; for groups and supergroups only*/
	pub fn can_pin_messages(mut self, can_pin_messages: bool) -> Self {
		self.can_pin_messages = Some(can_pin_messages);
		self
	}
	/** *Optional*. *True*, if the administrator can post messages in the channel, or access channel statistics; for channels only*/
	pub fn can_post_messages(mut self, can_post_messages: bool) -> Self {
		self.can_post_messages = Some(can_post_messages);
		self
	}
	/** *Optional*. Custom title for this user*/
	pub fn custom_title(mut self, custom_title: impl Into<String>) -> Self {
		self.custom_title = Some(custom_title.into());
		self
	}
}
/**Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that was banned in the chat and can't return to the chat or view chat messages.

https://core.telegram.org/bots/api/#chatmemberbanned*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatMemberBanned {
	/**Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever*/
	pub until_date: i64,
	/**Information about the user*/
	pub user: User,
}
impl ChatMemberBanned {
	pub fn new(until_date: impl Into<i64>, user: impl Into<User>) -> Self {
		Self {
			until_date: until_date.into(),
			user: user.into(),
		}
	}
}
/**Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that isn't currently a member of the chat, but may join it themselves.

https://core.telegram.org/bots/api/#chatmemberleft*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatMemberLeft {
	/**Information about the user*/
	pub user: User,
}
impl ChatMemberLeft {
	pub fn new(user: impl Into<User>) -> Self {
		Self {
			user: user.into(),
		}
	}
}
/**Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that has no additional privileges or restrictions.

https://core.telegram.org/bots/api/#chatmembermember*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatMemberMember {
	/**Date when the user's subscription will expire; Unix time*/
	pub until_date: Option<i64>,
	/**Information about the user*/
	pub user: User,
}
impl ChatMemberMember {
	pub fn new(user: impl Into<User>) -> Self {
		Self {
			until_date: None,
			user: user.into(),
		}
	}
	/** *Optional*. Date when the user's subscription will expire; Unix time*/
	pub fn until_date(mut self, until_date: impl Into<i64>) -> Self {
		self.until_date = Some(until_date.into());
		self
	}
}
/**Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that owns the chat and has all administrator privileges.

https://core.telegram.org/bots/api/#chatmemberowner*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatMemberOwner {
	/**Custom title for this user*/
	pub custom_title: Option<String>,
	/**if the user's presence in the chat is hidden*/
	pub is_anonymous: bool,
	/**Information about the user*/
	pub user: User,
}
impl ChatMemberOwner {
	pub fn new(is_anonymous: bool, user: impl Into<User>) -> Self {
		Self {
			custom_title: None,
			is_anonymous: is_anonymous,
			user: user.into(),
		}
	}
	/** *Optional*. Custom title for this user*/
	pub fn custom_title(mut self, custom_title: impl Into<String>) -> Self {
		self.custom_title = Some(custom_title.into());
		self
	}
}
/**Represents a [chat member](https://core.telegram.org/bots/api/#chatmember) that is under certain restrictions in the chat. Supergroups only.

https://core.telegram.org/bots/api/#chatmemberrestricted*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatMemberRestricted {
	/**if the user is allowed to add web page previews to their messages*/
	pub can_add_web_page_previews: bool,
	/**if the user is allowed to change the chat title, photo and other settings*/
	pub can_change_info: bool,
	/**if the user is allowed to invite new users to the chat*/
	pub can_invite_users: bool,
	/**if the user is allowed to create forum topics*/
	pub can_manage_topics: bool,
	/**if the user is allowed to pin messages*/
	pub can_pin_messages: bool,
	/**if the user is allowed to send audios*/
	pub can_send_audios: bool,
	/**if the user is allowed to send documents*/
	pub can_send_documents: bool,
	/**if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues*/
	pub can_send_messages: bool,
	/**if the user is allowed to send animations, games, stickers and use inline bots*/
	pub can_send_other_messages: bool,
	/**if the user is allowed to send photos*/
	pub can_send_photos: bool,
	/**if the user is allowed to send polls*/
	pub can_send_polls: bool,
	/**if the user is allowed to send video notes*/
	pub can_send_video_notes: bool,
	/**if the user is allowed to send videos*/
	pub can_send_videos: bool,
	/**if the user is allowed to send voice notes*/
	pub can_send_voice_notes: bool,
	/**if the user is a member of the chat at the moment of the request*/
	pub is_member: bool,
	/**Date when restrictions will be lifted for this user; Unix time. If 0, then the user is restricted forever*/
	pub until_date: i64,
	/**Information about the user*/
	pub user: User,
}
impl ChatMemberRestricted {
	pub fn new(can_add_web_page_previews: bool, can_change_info: bool, can_invite_users: bool, can_manage_topics: bool, can_pin_messages: bool, can_send_audios: bool, can_send_documents: bool, can_send_messages: bool, can_send_other_messages: bool, can_send_photos: bool, can_send_polls: bool, can_send_video_notes: bool, can_send_videos: bool, can_send_voice_notes: bool, is_member: bool, until_date: impl Into<i64>, user: impl Into<User>) -> Self {
		Self {
			can_add_web_page_previews: can_add_web_page_previews,
			can_change_info: can_change_info,
			can_invite_users: can_invite_users,
			can_manage_topics: can_manage_topics,
			can_pin_messages: can_pin_messages,
			can_send_audios: can_send_audios,
			can_send_documents: can_send_documents,
			can_send_messages: can_send_messages,
			can_send_other_messages: can_send_other_messages,
			can_send_photos: can_send_photos,
			can_send_polls: can_send_polls,
			can_send_video_notes: can_send_video_notes,
			can_send_videos: can_send_videos,
			can_send_voice_notes: can_send_voice_notes,
			is_member: is_member,
			until_date: until_date.into(),
			user: user.into(),
		}
	}
}
/**This object represents changes in the status of a chat member.

https://core.telegram.org/bots/api/#chatmemberupdated*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatMemberUpdated {
	/**Chat the user belongs to*/
	pub chat: Chat,
	/**Date the change was done in Unix time*/
	pub date: i64,
	/**Performer of the action, which resulted in the change*/
	pub from: User,
	/**Chat invite link, which was used by the user to join the chat; for joining by invite link events only.*/
	pub invite_link: Option<ChatInviteLink>,
	/**New information about the chat member*/
	pub new_chat_member: ChatMember,
	/**Previous information about the chat member*/
	pub old_chat_member: ChatMember,
	/**True, if the user joined the chat via a chat folder invite link*/
	pub via_chat_folder_invite_link: Option<bool>,
	/**True, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator*/
	pub via_join_request: Option<bool>,
}
impl ChatMemberUpdated {
	pub fn new(chat: impl Into<Chat>, date: impl Into<i64>, from: impl Into<User>, new_chat_member: impl Into<ChatMember>, old_chat_member: impl Into<ChatMember>) -> Self {
		Self {
			chat: chat.into(),
			date: date.into(),
			from: from.into(),
			invite_link: None,
			new_chat_member: new_chat_member.into(),
			old_chat_member: old_chat_member.into(),
			via_chat_folder_invite_link: None,
			via_join_request: None,
		}
	}
	/** *Optional*. Chat invite link, which was used by the user to join the chat; for joining by invite link events only.*/
	pub fn invite_link(mut self, invite_link: impl Into<ChatInviteLink>) -> Self {
		self.invite_link = Some(invite_link.into());
		self
	}
	/** *Optional*. True, if the user joined the chat via a chat folder invite link*/
	pub fn via_chat_folder_invite_link(mut self, via_chat_folder_invite_link: bool) -> Self {
		self.via_chat_folder_invite_link = Some(via_chat_folder_invite_link);
		self
	}
	/** *Optional*. True, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator*/
	pub fn via_join_request(mut self, via_join_request: bool) -> Self {
		self.via_join_request = Some(via_join_request);
		self
	}
}
/**Describes actions that a non-administrator user is allowed to take in a chat.

https://core.telegram.org/bots/api/#chatpermissions*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatPermissions {
	/**if the user is allowed to add web page previews to their messages*/
	pub can_add_web_page_previews: Option<bool>,
	/**if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups*/
	pub can_change_info: Option<bool>,
	/**if the user is allowed to invite new users to the chat*/
	pub can_invite_users: Option<bool>,
	/**if the user is allowed to create forum topics. If omitted defaults to the value of can\_pin\_messages*/
	pub can_manage_topics: Option<bool>,
	/**if the user is allowed to pin messages. Ignored in public supergroups*/
	pub can_pin_messages: Option<bool>,
	/**if the user is allowed to send audios*/
	pub can_send_audios: Option<bool>,
	/**if the user is allowed to send documents*/
	pub can_send_documents: Option<bool>,
	/**if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues*/
	pub can_send_messages: Option<bool>,
	/**if the user is allowed to send animations, games, stickers and use inline bots*/
	pub can_send_other_messages: Option<bool>,
	/**if the user is allowed to send photos*/
	pub can_send_photos: Option<bool>,
	/**if the user is allowed to send polls*/
	pub can_send_polls: Option<bool>,
	/**if the user is allowed to send video notes*/
	pub can_send_video_notes: Option<bool>,
	/**if the user is allowed to send videos*/
	pub can_send_videos: Option<bool>,
	/**if the user is allowed to send voice notes*/
	pub can_send_voice_notes: Option<bool>,
}
impl ChatPermissions {
	pub fn new() -> Self {
		Self {
			can_add_web_page_previews: None,
			can_change_info: None,
			can_invite_users: None,
			can_manage_topics: None,
			can_pin_messages: None,
			can_send_audios: None,
			can_send_documents: None,
			can_send_messages: None,
			can_send_other_messages: None,
			can_send_photos: None,
			can_send_polls: None,
			can_send_video_notes: None,
			can_send_videos: None,
			can_send_voice_notes: None,
		}
	}
	/** *Optional*. *True*, if the user is allowed to add web page previews to their messages*/
	pub fn can_add_web_page_previews(mut self, can_add_web_page_previews: bool) -> Self {
		self.can_add_web_page_previews = Some(can_add_web_page_previews);
		self
	}
	/** *Optional*. *True*, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups*/
	pub fn can_change_info(mut self, can_change_info: bool) -> Self {
		self.can_change_info = Some(can_change_info);
		self
	}
	/** *Optional*. *True*, if the user is allowed to invite new users to the chat*/
	pub fn can_invite_users(mut self, can_invite_users: bool) -> Self {
		self.can_invite_users = Some(can_invite_users);
		self
	}
	/** *Optional*. *True*, if the user is allowed to create forum topics. If omitted defaults to the value of can\_pin\_messages*/
	pub fn can_manage_topics(mut self, can_manage_topics: bool) -> Self {
		self.can_manage_topics = Some(can_manage_topics);
		self
	}
	/** *Optional*. *True*, if the user is allowed to pin messages. Ignored in public supergroups*/
	pub fn can_pin_messages(mut self, can_pin_messages: bool) -> Self {
		self.can_pin_messages = Some(can_pin_messages);
		self
	}
	/** *Optional*. *True*, if the user is allowed to send audios*/
	pub fn can_send_audios(mut self, can_send_audios: bool) -> Self {
		self.can_send_audios = Some(can_send_audios);
		self
	}
	/** *Optional*. *True*, if the user is allowed to send documents*/
	pub fn can_send_documents(mut self, can_send_documents: bool) -> Self {
		self.can_send_documents = Some(can_send_documents);
		self
	}
	/** *Optional*. *True*, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues*/
	pub fn can_send_messages(mut self, can_send_messages: bool) -> Self {
		self.can_send_messages = Some(can_send_messages);
		self
	}
	/** *Optional*. *True*, if the user is allowed to send animations, games, stickers and use inline bots*/
	pub fn can_send_other_messages(mut self, can_send_other_messages: bool) -> Self {
		self.can_send_other_messages = Some(can_send_other_messages);
		self
	}
	/** *Optional*. *True*, if the user is allowed to send photos*/
	pub fn can_send_photos(mut self, can_send_photos: bool) -> Self {
		self.can_send_photos = Some(can_send_photos);
		self
	}
	/** *Optional*. *True*, if the user is allowed to send polls*/
	pub fn can_send_polls(mut self, can_send_polls: bool) -> Self {
		self.can_send_polls = Some(can_send_polls);
		self
	}
	/** *Optional*. *True*, if the user is allowed to send video notes*/
	pub fn can_send_video_notes(mut self, can_send_video_notes: bool) -> Self {
		self.can_send_video_notes = Some(can_send_video_notes);
		self
	}
	/** *Optional*. *True*, if the user is allowed to send videos*/
	pub fn can_send_videos(mut self, can_send_videos: bool) -> Self {
		self.can_send_videos = Some(can_send_videos);
		self
	}
	/** *Optional*. *True*, if the user is allowed to send voice notes*/
	pub fn can_send_voice_notes(mut self, can_send_voice_notes: bool) -> Self {
		self.can_send_voice_notes = Some(can_send_voice_notes);
		self
	}
}
/**This object represents a chat photo.

https://core.telegram.org/bots/api/#chatphoto*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatPhoto {
	/**File identifier of big (640x640) chat photo. This file\_id can be used only for photo download and only for as long as the photo is not changed.*/
	pub big_file_id: String,
	/**Unique file identifier of big (640x640) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub big_file_unique_id: String,
	/**File identifier of small (160x160) chat photo. This file\_id can be used only for photo download and only for as long as the photo is not changed.*/
	pub small_file_id: String,
	/**Unique file identifier of small (160x160) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub small_file_unique_id: String,
}
impl ChatPhoto {
	pub fn new(big_file_id: impl Into<String>, big_file_unique_id: impl Into<String>, small_file_id: impl Into<String>, small_file_unique_id: impl Into<String>) -> Self {
		Self {
			big_file_id: big_file_id.into(),
			big_file_unique_id: big_file_unique_id.into(),
			small_file_id: small_file_id.into(),
			small_file_unique_id: small_file_unique_id.into(),
		}
	}
}
/**This object contains information about a chat that was shared with the bot using a [KeyboardButtonRequestChat](https://core.telegram.org/bots/api/#keyboardbuttonrequestchat) button.

https://core.telegram.org/bots/api/#chatshared*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChatShared {
	/**Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.*/
	pub chat_id: i64,
	/**Available sizes of the chat photo, if the photo was requested by the bot*/
	pub photo: Vec<PhotoSize>,
	/**Identifier of the request*/
	pub request_id: i64,
	/**Title of the chat, if the title was requested by the bot.*/
	pub title: Option<String>,
	/**Username of the chat, if the username was requested by the bot and available.*/
	pub username: Option<String>,
}
impl ChatShared {
	pub fn new(chat_id: impl Into<i64>, request_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			photo: Vec::new(),
			request_id: request_id.into(),
			title: None,
			username: None,
		}
	}
	pub fn add_photo(mut self, photo: impl Into<PhotoSize>) -> Self {
		self.photo.push(photo.into());
		self
	}
	/** *Optional*. Available sizes of the chat photo, if the photo was requested by the bot*/
	pub fn photo(mut self, photo: impl Into<Vec<PhotoSize>>) -> Self {
		self.photo = photo.into();
		self
	}
	/** *Optional*. Title of the chat, if the title was requested by the bot.*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
	/** *Optional*. Username of the chat, if the username was requested by the bot and available.*/
	pub fn username(mut self, username: impl Into<String>) -> Self {
		self.username = Some(username.into());
		self
	}
}
/**Represents a [result](https://core.telegram.org/bots/api/#inlinequeryresult) of an inline query that was chosen by the user and sent to their chat partner.

https://core.telegram.org/bots/api/#choseninlineresult*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ChosenInlineResult {
	/**The user that chose the result*/
	pub from: User,
	/**Identifier of the sent inline message. Available only if there is an [inline keyboard](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) attached to the message. Will be also received in [callback queries](https://core.telegram.org/bots/api/#callbackquery) and can be used to [edit](https://core.telegram.org/bots/api/#updating-messages) the message.*/
	pub inline_message_id: Option<String>,
	/**Sender location, only for bots that require user location*/
	pub location: Option<Location>,
	/**The query that was used to obtain the result*/
	pub query: String,
	/**The unique identifier for the result that was chosen*/
	pub result_id: String,
}
impl ChosenInlineResult {
	pub fn new(from: impl Into<User>, query: impl Into<String>, result_id: impl Into<String>) -> Self {
		Self {
			from: from.into(),
			inline_message_id: None,
			location: None,
			query: query.into(),
			result_id: result_id.into(),
		}
	}
	/** *Optional*. Identifier of the sent inline message. Available only if there is an [inline keyboard](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) attached to the message. Will be also received in [callback queries](https://core.telegram.org/bots/api/#callbackquery) and can be used to [edit](https://core.telegram.org/bots/api/#updating-messages) the message.*/
	pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
		self.inline_message_id = Some(inline_message_id.into());
		self
	}
	/** *Optional*. Sender location, only for bots that require user location*/
	pub fn location(mut self, location: impl Into<Location>) -> Self {
		self.location = Some(location.into());
		self
	}
}
/**This object represents a phone contact.

https://core.telegram.org/bots/api/#contact*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Contact {
	/**Contact's first name*/
	pub first_name: String,
	/**Contact's last name*/
	pub last_name: Option<String>,
	/**Contact's phone number*/
	pub phone_number: String,
	/**Contact's user identifier in Telegram. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.*/
	pub user_id: Option<i64>,
	/**Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard)*/
	pub vcard: Option<String>,
}
impl Contact {
	pub fn new(first_name: impl Into<String>, phone_number: impl Into<String>) -> Self {
		Self {
			first_name: first_name.into(),
			last_name: None,
			phone_number: phone_number.into(),
			user_id: None,
			vcard: None,
		}
	}
	/** *Optional*. Contact's last name*/
	pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
		self.last_name = Some(last_name.into());
		self
	}
	/** *Optional*. Contact's user identifier in Telegram. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.*/
	pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
		self.user_id = Some(user_id.into());
		self
	}
	/** *Optional*. Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard)*/
	pub fn vcard(mut self, vcard: impl Into<String>) -> Self {
		self.vcard = Some(vcard.into());
		self
	}
}
/**This object represents an inline keyboard button that copies specified text to the clipboard.

https://core.telegram.org/bots/api/#copytextbutton*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CopyTextButton {
	/**The text to be copied to the clipboard; 1-256 characters
	Min len: 1
	Max len: 256*/
	pub text: String,
}
impl CopyTextButton {
	pub fn new(text: impl Into<String>) -> Self {
		Self {
			text: text.into(),
		}
	}
}
/**This object represents an animated emoji that displays a random value.

https://core.telegram.org/bots/api/#dice*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Dice {
	/**Emoji on which the dice throw animation is based*/
	pub emoji: String,
	/**Value of the dice, 1-6 for “🎲”, “🎯” and “🎳” base emoji, 1-5 for “🏀” and “⚽” base emoji, 1-64 for “🎰” base emoji*/
	pub value: i64,
}
impl Dice {
	pub fn new(emoji: impl Into<String>, value: impl Into<i64>) -> Self {
		Self {
			emoji: emoji.into(),
			value: value.into(),
		}
	}
}
/**This object represents a general file (as opposed to [photos](https://core.telegram.org/bots/api/#photosize), [voice messages](https://core.telegram.org/bots/api/#voice) and [audio files](https://core.telegram.org/bots/api/#audio)).

https://core.telegram.org/bots/api/#document*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Document {
	/**Identifier for this file, which can be used to download or reuse the file*/
	pub file_id: String,
	/**Original filename as defined by the sender*/
	pub file_name: Option<String>,
	/**File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub file_size: Option<i64>,
	/**Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub file_unique_id: String,
	/**MIME type of the file as defined by the sender*/
	pub mime_type: Option<String>,
	/**Document thumbnail as defined by the sender*/
	pub thumbnail: Option<PhotoSize>,
}
impl Document {
	pub fn new(file_id: impl Into<String>, file_unique_id: impl Into<String>) -> Self {
		Self {
			file_id: file_id.into(),
			file_name: None,
			file_size: None,
			file_unique_id: file_unique_id.into(),
			mime_type: None,
			thumbnail: None,
		}
	}
	/** *Optional*. Original filename as defined by the sender*/
	pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
		self.file_name = Some(file_name.into());
		self
	}
	/** *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub fn file_size(mut self, file_size: impl Into<i64>) -> Self {
		self.file_size = Some(file_size.into());
		self
	}
	/** *Optional*. MIME type of the file as defined by the sender*/
	pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
		self.mime_type = Some(mime_type.into());
		self
	}
	/** *Optional*. Document thumbnail as defined by the sender*/
	pub fn thumbnail(mut self, thumbnail: impl Into<PhotoSize>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
}
/**Describes data required for decrypting and authenticating [EncryptedPassportElement](https://core.telegram.org/bots/api/#encryptedpassportelement). See the [Telegram Passport Documentation](https://core.telegram.org/passport#receiving-information) for a complete description of the data decryption and authentication processes.

https://core.telegram.org/bots/api/#encryptedcredentials*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct EncryptedCredentials {
	/**Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for [EncryptedPassportElement](https://core.telegram.org/bots/api/#encryptedpassportelement) decryption and authentication*/
	pub data: String,
	/**Base64-encoded data hash for data authentication*/
	pub hash: String,
	/**Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption*/
	pub secret: String,
}
impl EncryptedCredentials {
	pub fn new(data: impl Into<String>, hash: impl Into<String>, secret: impl Into<String>) -> Self {
		Self {
			data: data.into(),
			hash: hash.into(),
			secret: secret.into(),
		}
	}
}
/**Describes documents or other Telegram Passport elements shared with the bot by the user.

https://core.telegram.org/bots/api/#encryptedpassportelement*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct EncryptedPassportElement {
	/**Base64-encoded encrypted Telegram Passport element data provided by the user; available only for “personal\_details”, “passport”, “driver\_license”, “identity\_card”, “internal\_passport” and “address” types. Can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub data: Option<String>,
	/**User's verified email address; available only for “email” type*/
	pub email: Option<String>,
	/**Array of encrypted files with documents provided by the user; available only for “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration” and “temporary\_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub files: Vec<PassportFile>,
	/**Encrypted file with the front side of the document, provided by the user; available only for “passport”, “driver\_license”, “identity\_card” and “internal\_passport”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub front_side: Option<PassportFile>,
	/**Base64-encoded element hash for using in [PassportElementErrorUnspecified](https://core.telegram.org/bots/api/#passportelementerrorunspecified)*/
	pub hash: String,
	/**User's verified phone number; available only for “phone\_number” type*/
	pub phone_number: Option<String>,
	/**Element type. One of “personal\_details”, “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “address”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”, “phone\_number”, “email”.
	One of: personal_details, passport, driver_license, identity_card, internal_passport, address, utility_bill, bank_statement, rental_agreement, passport_registration, temporary_registration, phone_number, email*/
	pub r#type: String,
	/**Encrypted file with the reverse side of the document, provided by the user; available only for “driver\_license” and “identity\_card”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub reverse_side: Option<PassportFile>,
	/**Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for “passport”, “driver\_license”, “identity\_card” and “internal\_passport”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub selfie: Option<PassportFile>,
	/**Array of encrypted files with translated versions of documents provided by the user; available if requested for “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration” and “temporary\_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub translation: Vec<PassportFile>,
}
impl EncryptedPassportElement {
	pub fn new(hash: impl Into<String>, r#type: impl Into<String>) -> Self {
		Self {
			data: None,
			email: None,
			files: Vec::new(),
			front_side: None,
			hash: hash.into(),
			phone_number: None,
			r#type: r#type.into(),
			reverse_side: None,
			selfie: None,
			translation: Vec::new(),
		}
	}
	/** *Optional*. Base64-encoded encrypted Telegram Passport element data provided by the user; available only for “personal\_details”, “passport”, “driver\_license”, “identity\_card”, “internal\_passport” and “address” types. Can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub fn data(mut self, data: impl Into<String>) -> Self {
		self.data = Some(data.into());
		self
	}
	/** *Optional*. User's verified email address; available only for “email” type*/
	pub fn email(mut self, email: impl Into<String>) -> Self {
		self.email = Some(email.into());
		self
	}
	pub fn add_file(mut self, file: impl Into<PassportFile>) -> Self {
		self.files.push(file.into());
		self
	}
	/** *Optional*. Array of encrypted files with documents provided by the user; available only for “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration” and “temporary\_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub fn files(mut self, files: impl Into<Vec<PassportFile>>) -> Self {
		self.files = files.into();
		self
	}
	/** *Optional*. Encrypted file with the front side of the document, provided by the user; available only for “passport”, “driver\_license”, “identity\_card” and “internal\_passport”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub fn front_side(mut self, front_side: impl Into<PassportFile>) -> Self {
		self.front_side = Some(front_side.into());
		self
	}
	/** *Optional*. User's verified phone number; available only for “phone\_number” type*/
	pub fn phone_number(mut self, phone_number: impl Into<String>) -> Self {
		self.phone_number = Some(phone_number.into());
		self
	}
	/** *Optional*. Encrypted file with the reverse side of the document, provided by the user; available only for “driver\_license” and “identity\_card”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub fn reverse_side(mut self, reverse_side: impl Into<PassportFile>) -> Self {
		self.reverse_side = Some(reverse_side.into());
		self
	}
	/** *Optional*. Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for “passport”, “driver\_license”, “identity\_card” and “internal\_passport”. The file can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub fn selfie(mut self, selfie: impl Into<PassportFile>) -> Self {
		self.selfie = Some(selfie.into());
		self
	}
	pub fn add_translation(mut self, translation: impl Into<PassportFile>) -> Self {
		self.translation.push(translation.into());
		self
	}
	/** *Optional*. Array of encrypted files with translated versions of documents provided by the user; available if requested for “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration” and “temporary\_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).*/
	pub fn translation(mut self, translation: impl Into<Vec<PassportFile>>) -> Self {
		self.translation = translation.into();
		self
	}
}
/**This object contains information about a message that is being replied to, which may come from another chat or forum topic.

https://core.telegram.org/bots/api/#externalreplyinfo*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ExternalReplyInfo {
	/**Message is an animation, information about the animation*/
	pub animation: Option<Animation>,
	/**Message is an audio file, information about the file*/
	pub audio: Option<Audio>,
	/**Chat the original message belongs to. Available only if the chat is a supergroup or a channel.*/
	pub chat: Option<Chat>,
	/**Message is a shared contact, information about the contact*/
	pub contact: Option<Contact>,
	/**Message is a dice with random value*/
	pub dice: Option<Dice>,
	/**Message is a general file, information about the file*/
	pub document: Option<Document>,
	/**Message is a game, information about the game. [More about games »](https://core.telegram.org/bots/api/#games)*/
	pub game: Option<Game>,
	/**Message is a scheduled giveaway, information about the giveaway*/
	pub giveaway: Option<Giveaway>,
	/**A giveaway with public winners was completed*/
	pub giveaway_winners: Option<GiveawayWinners>,
	/**if the message media is covered by a spoiler animation
	Default value: true*/
	pub has_media_spoiler: Option<bool>,
	/**Message is an invoice for a [payment](https://core.telegram.org/bots/api/#payments), information about the invoice. [More about payments »](https://core.telegram.org/bots/api/#payments)*/
	pub invoice: Option<Invoice>,
	/**Options used for link preview generation for the original message, if it is a text message*/
	pub link_preview_options: Option<LinkPreviewOptions>,
	/**Message is a shared location, information about the location*/
	pub location: Option<Location>,
	/**Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.*/
	pub message_id: Option<i64>,
	/**Origin of the message replied to by the given message*/
	pub origin: MessageOrigin,
	/**Message contains paid media; information about the paid media*/
	pub paid_media: Option<PaidMediaInfo>,
	/**Message is a photo, available sizes of the photo*/
	pub photo: Vec<PhotoSize>,
	/**Message is a native poll, information about the poll*/
	pub poll: Option<Poll>,
	/**Message is a sticker, information about the sticker*/
	pub sticker: Option<Sticker>,
	/**Message is a forwarded story*/
	pub story: Option<Story>,
	/**Message is a venue, information about the venue*/
	pub venue: Option<Venue>,
	/**Message is a video, information about the video*/
	pub video: Option<Video>,
	/**Message is a [video note](https://telegram.org/blog/video-messages-and-telescope), information about the video message*/
	pub video_note: Option<VideoNote>,
	/**Message is a voice message, information about the file*/
	pub voice: Option<Voice>,
}
impl ExternalReplyInfo {
	pub fn new(origin: impl Into<MessageOrigin>) -> Self {
		Self {
			animation: None,
			audio: None,
			chat: None,
			contact: None,
			dice: None,
			document: None,
			game: None,
			giveaway: None,
			giveaway_winners: None,
			has_media_spoiler: None,
			invoice: None,
			link_preview_options: None,
			location: None,
			message_id: None,
			origin: origin.into(),
			paid_media: None,
			photo: Vec::new(),
			poll: None,
			sticker: None,
			story: None,
			venue: None,
			video: None,
			video_note: None,
			voice: None,
		}
	}
	/** *Optional*. Message is an animation, information about the animation*/
	pub fn animation(mut self, animation: impl Into<Animation>) -> Self {
		self.animation = Some(animation.into());
		self
	}
	/** *Optional*. Message is an audio file, information about the file*/
	pub fn audio(mut self, audio: impl Into<Audio>) -> Self {
		self.audio = Some(audio.into());
		self
	}
	/** *Optional*. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.*/
	pub fn chat(mut self, chat: impl Into<Chat>) -> Self {
		self.chat = Some(chat.into());
		self
	}
	/** *Optional*. Message is a shared contact, information about the contact*/
	pub fn contact(mut self, contact: impl Into<Contact>) -> Self {
		self.contact = Some(contact.into());
		self
	}
	/** *Optional*. Message is a dice with random value*/
	pub fn dice(mut self, dice: impl Into<Dice>) -> Self {
		self.dice = Some(dice.into());
		self
	}
	/** *Optional*. Message is a general file, information about the file*/
	pub fn document(mut self, document: impl Into<Document>) -> Self {
		self.document = Some(document.into());
		self
	}
	/** *Optional*. Message is a game, information about the game. [More about games »](https://core.telegram.org/bots/api/#games)*/
	pub fn game(mut self, game: impl Into<Game>) -> Self {
		self.game = Some(game.into());
		self
	}
	/** *Optional*. Message is a scheduled giveaway, information about the giveaway*/
	pub fn giveaway(mut self, giveaway: impl Into<Giveaway>) -> Self {
		self.giveaway = Some(giveaway.into());
		self
	}
	/** *Optional*. A giveaway with public winners was completed*/
	pub fn giveaway_winners(mut self, giveaway_winners: impl Into<GiveawayWinners>) -> Self {
		self.giveaway_winners = Some(giveaway_winners.into());
		self
	}
	/** *Optional*. *True*, if the message media is covered by a spoiler animation
	Default value: true*/
	pub fn has_media_spoiler(mut self, has_media_spoiler: bool) -> Self {
		self.has_media_spoiler = Some(has_media_spoiler);
		self
	}
	/** *Optional*. Message is an invoice for a [payment](https://core.telegram.org/bots/api/#payments), information about the invoice. [More about payments »](https://core.telegram.org/bots/api/#payments)*/
	pub fn invoice(mut self, invoice: impl Into<Invoice>) -> Self {
		self.invoice = Some(invoice.into());
		self
	}
	/** *Optional*. Options used for link preview generation for the original message, if it is a text message*/
	pub fn link_preview_options(mut self, link_preview_options: impl Into<LinkPreviewOptions>) -> Self {
		self.link_preview_options = Some(link_preview_options.into());
		self
	}
	/** *Optional*. Message is a shared location, information about the location*/
	pub fn location(mut self, location: impl Into<Location>) -> Self {
		self.location = Some(location.into());
		self
	}
	/** *Optional*. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.*/
	pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_id = Some(message_id.into());
		self
	}
	/** *Optional*. Message contains paid media; information about the paid media*/
	pub fn paid_media(mut self, paid_media: impl Into<PaidMediaInfo>) -> Self {
		self.paid_media = Some(paid_media.into());
		self
	}
	pub fn add_photo(mut self, photo: impl Into<PhotoSize>) -> Self {
		self.photo.push(photo.into());
		self
	}
	/** *Optional*. Message is a photo, available sizes of the photo*/
	pub fn photo(mut self, photo: impl Into<Vec<PhotoSize>>) -> Self {
		self.photo = photo.into();
		self
	}
	/** *Optional*. Message is a native poll, information about the poll*/
	pub fn poll(mut self, poll: impl Into<Poll>) -> Self {
		self.poll = Some(poll.into());
		self
	}
	/** *Optional*. Message is a sticker, information about the sticker*/
	pub fn sticker(mut self, sticker: impl Into<Sticker>) -> Self {
		self.sticker = Some(sticker.into());
		self
	}
	/** *Optional*. Message is a forwarded story*/
	pub fn story(mut self, story: impl Into<Story>) -> Self {
		self.story = Some(story.into());
		self
	}
	/** *Optional*. Message is a venue, information about the venue*/
	pub fn venue(mut self, venue: impl Into<Venue>) -> Self {
		self.venue = Some(venue.into());
		self
	}
	/** *Optional*. Message is a video, information about the video*/
	pub fn video(mut self, video: impl Into<Video>) -> Self {
		self.video = Some(video.into());
		self
	}
	/** *Optional*. Message is a [video note](https://telegram.org/blog/video-messages-and-telescope), information about the video message*/
	pub fn video_note(mut self, video_note: impl Into<VideoNote>) -> Self {
		self.video_note = Some(video_note.into());
		self
	}
	/** *Optional*. Message is a voice message, information about the file*/
	pub fn voice(mut self, voice: impl Into<Voice>) -> Self {
		self.voice = Some(voice.into());
		self
	}
}
/**This object represents a file ready to be downloaded. The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api/#getfile).

The maximum file size to download is 20 MB

https://core.telegram.org/bots/api/#file*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct File {
	/**Identifier for this file, which can be used to download or reuse the file*/
	pub file_id: String,
	/**File path. Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file.*/
	pub file_path: Option<String>,
	/**File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub file_size: Option<i64>,
	/**Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub file_unique_id: String,
}
impl File {
	pub fn new(file_id: impl Into<String>, file_unique_id: impl Into<String>) -> Self {
		Self {
			file_id: file_id.into(),
			file_path: None,
			file_size: None,
			file_unique_id: file_unique_id.into(),
		}
	}
	/** *Optional*. File path. Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file.*/
	pub fn file_path(mut self, file_path: impl Into<String>) -> Self {
		self.file_path = Some(file_path.into());
		self
	}
	/** *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub fn file_size(mut self, file_size: impl Into<i64>) -> Self {
		self.file_size = Some(file_size.into());
		self
	}
}
/**Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice [privacy mode](https://core.telegram.org/bots/features#privacy-mode). Not supported in channels and for messages sent on behalf of a Telegram Business account.

https://core.telegram.org/bots/api/#forcereply*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct ForceReply {
	/**Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
	Default value: true*/
	pub force_reply: bool,
	/**The placeholder to be shown in the input field when the reply is active; 1-64 characters
	Min len: 1
	Max len: 64*/
	pub input_field_placeholder: Option<String>,
	/**Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the *text* of the [Message](https://core.telegram.org/bots/api/#message) object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.*/
	pub selective: Option<bool>,
}
impl ForceReply {
	pub fn new(force_reply: bool) -> Self {
		Self {
			force_reply: force_reply,
			input_field_placeholder: None,
			selective: None,
		}
	}
	/** *Optional*. The placeholder to be shown in the input field when the reply is active; 1-64 characters
	Min len: 1
	Max len: 64*/
	pub fn input_field_placeholder(mut self, input_field_placeholder: impl Into<String>) -> Self {
		self.input_field_placeholder = Some(input_field_placeholder.into());
		self
	}
	/** *Optional*. Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the *text* of the [Message](https://core.telegram.org/bots/api/#message) object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.*/
	pub fn selective(mut self, selective: bool) -> Self {
		self.selective = Some(selective);
		self
	}
}
/**This object represents a forum topic.

https://core.telegram.org/bots/api/#forumtopic*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ForumTopic {
	/**Color of the topic icon in RGB format*/
	pub icon_color: i64,
	/**Unique identifier of the custom emoji shown as the topic icon*/
	pub icon_custom_emoji_id: Option<String>,
	/**Unique identifier of the forum topic*/
	pub message_thread_id: i64,
	/**Name of the topic*/
	pub name: String,
}
impl ForumTopic {
	pub fn new(icon_color: impl Into<i64>, message_thread_id: impl Into<i64>, name: impl Into<String>) -> Self {
		Self {
			icon_color: icon_color.into(),
			icon_custom_emoji_id: None,
			message_thread_id: message_thread_id.into(),
			name: name.into(),
		}
	}
	/** *Optional*. Unique identifier of the custom emoji shown as the topic icon*/
	pub fn icon_custom_emoji_id(mut self, icon_custom_emoji_id: impl Into<String>) -> Self {
		self.icon_custom_emoji_id = Some(icon_custom_emoji_id.into());
		self
	}
}
/**This object represents a service message about a forum topic closed in the chat. Currently holds no information.

https://core.telegram.org/bots/api/#forumtopicclosed*/
pub type ForumTopicClosed = ();
/**This object represents a service message about a new forum topic created in the chat.

https://core.telegram.org/bots/api/#forumtopiccreated*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ForumTopicCreated {
	/**Color of the topic icon in RGB format*/
	pub icon_color: i64,
	/**Unique identifier of the custom emoji shown as the topic icon*/
	pub icon_custom_emoji_id: Option<String>,
	/**Name of the topic*/
	pub name: String,
}
impl ForumTopicCreated {
	pub fn new(icon_color: impl Into<i64>, name: impl Into<String>) -> Self {
		Self {
			icon_color: icon_color.into(),
			icon_custom_emoji_id: None,
			name: name.into(),
		}
	}
	/** *Optional*. Unique identifier of the custom emoji shown as the topic icon*/
	pub fn icon_custom_emoji_id(mut self, icon_custom_emoji_id: impl Into<String>) -> Self {
		self.icon_custom_emoji_id = Some(icon_custom_emoji_id.into());
		self
	}
}
/**This object represents a service message about an edited forum topic.

https://core.telegram.org/bots/api/#forumtopicedited*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ForumTopicEdited {
	/**New identifier of the custom emoji shown as the topic icon, if it was edited; an empty string if the icon was removed*/
	pub icon_custom_emoji_id: Option<String>,
	/**New name of the topic, if it was edited*/
	pub name: Option<String>,
}
impl ForumTopicEdited {
	pub fn new() -> Self {
		Self {
			icon_custom_emoji_id: None,
			name: None,
		}
	}
	/** *Optional*. New identifier of the custom emoji shown as the topic icon, if it was edited; an empty string if the icon was removed*/
	pub fn icon_custom_emoji_id(mut self, icon_custom_emoji_id: impl Into<String>) -> Self {
		self.icon_custom_emoji_id = Some(icon_custom_emoji_id.into());
		self
	}
	/** *Optional*. New name of the topic, if it was edited*/
	pub fn name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}
}
/**This object represents a service message about a forum topic reopened in the chat. Currently holds no information.

https://core.telegram.org/bots/api/#forumtopicreopened*/
pub type ForumTopicReopened = ();
/**Unique identifier for the chat where the original messages were sent (or channel username in the format `@channelusername`)*/
#[derive(Clone, Debug, Serialize, From, Display)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum FromChatId {
	Id(i64),
	Username(String),
}
/**This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.

https://core.telegram.org/bots/api/#game*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Game {
	/**Animation that will be displayed in the game message in chats. Upload via [BotFather](https://t.me/botfather)*/
	pub animation: Option<Animation>,
	/**Description of the game*/
	pub description: String,
	/**Photo that will be displayed in the game message in chats.*/
	pub photo: Vec<PhotoSize>,
	/**Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls [setGameScore](https://core.telegram.org/bots/api/#setgamescore), or manually edited using [editMessageText](https://core.telegram.org/bots/api/#editmessagetext). 0-4096 characters.
	Min len: 0
	Max len: 4096*/
	pub text: Option<String>,
	/**Special entities that appear in *text*, such as usernames, URLs, bot commands, etc.*/
	pub text_entities: Vec<MessageEntity>,
	/**Title of the game*/
	pub title: String,
}
impl Game {
	pub fn new(description: impl Into<String>, photo: impl Into<Vec<PhotoSize>>, title: impl Into<String>) -> Self {
		Self {
			animation: None,
			description: description.into(),
			photo: photo.into(),
			text: None,
			text_entities: Vec::new(),
			title: title.into(),
		}
	}
	/** *Optional*. Animation that will be displayed in the game message in chats. Upload via [BotFather](https://t.me/botfather)*/
	pub fn animation(mut self, animation: impl Into<Animation>) -> Self {
		self.animation = Some(animation.into());
		self
	}
	pub fn add_photo(mut self, photo: impl Into<PhotoSize>) -> Self {
		self.photo.push(photo.into());
		self
	}
	/** *Optional*. Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls [setGameScore](https://core.telegram.org/bots/api/#setgamescore), or manually edited using [editMessageText](https://core.telegram.org/bots/api/#editmessagetext). 0-4096 characters.
	Min len: 0
	Max len: 4096*/
	pub fn text(mut self, text: impl Into<String>) -> Self {
		self.text = Some(text.into());
		self
	}
	pub fn add_text_entity(mut self, text_entity: impl Into<MessageEntity>) -> Self {
		self.text_entities.push(text_entity.into());
		self
	}
	/** *Optional*. Special entities that appear in *text*, such as usernames, URLs, bot commands, etc.*/
	pub fn text_entities(mut self, text_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.text_entities = text_entities.into();
		self
	}
}
/**This object represents one row of the high scores table for a game.

https://core.telegram.org/bots/api/#gamehighscore*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct GameHighScore {
	/**Position in high score table for the game*/
	pub position: i64,
	/**Score*/
	pub score: i64,
	/**User*/
	pub user: User,
}
impl GameHighScore {
	pub fn new(position: impl Into<i64>, score: impl Into<i64>, user: impl Into<User>) -> Self {
		Self {
			position: position.into(),
			score: score.into(),
			user: user.into(),
		}
	}
}
/**This object represents a service message about General forum topic hidden in the chat. Currently holds no information.

https://core.telegram.org/bots/api/#generalforumtopichidden*/
pub type GeneralForumTopicHidden = ();
/**This object represents a service message about General forum topic unhidden in the chat. Currently holds no information.

https://core.telegram.org/bots/api/#generalforumtopicunhidden*/
pub type GeneralForumTopicUnhidden = ();
/**This object represents a gift that can be sent by the bot.

https://core.telegram.org/bots/api/#gift*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Gift {
	/**Unique identifier of the gift*/
	pub id: String,
	/**The number of remaining gifts of this type that can be sent; for limited gifts only*/
	pub remaining_count: Option<i64>,
	/**The number of Telegram Stars that must be paid to send the sticker*/
	pub star_count: i64,
	/**The sticker that represents the gift*/
	pub sticker: Sticker,
	/**The total number of the gifts of this type that can be sent; for limited gifts only*/
	pub total_count: Option<i64>,
	/**The number of Telegram Stars that must be paid to upgrade the gift to a unique one*/
	pub upgrade_star_count: Option<i64>,
}
impl Gift {
	pub fn new(id: impl Into<String>, star_count: impl Into<i64>, sticker: impl Into<Sticker>) -> Self {
		Self {
			id: id.into(),
			remaining_count: None,
			star_count: star_count.into(),
			sticker: sticker.into(),
			total_count: None,
			upgrade_star_count: None,
		}
	}
	/** *Optional*. The number of remaining gifts of this type that can be sent; for limited gifts only*/
	pub fn remaining_count(mut self, remaining_count: impl Into<i64>) -> Self {
		self.remaining_count = Some(remaining_count.into());
		self
	}
	/** *Optional*. The total number of the gifts of this type that can be sent; for limited gifts only*/
	pub fn total_count(mut self, total_count: impl Into<i64>) -> Self {
		self.total_count = Some(total_count.into());
		self
	}
	/** *Optional*. The number of Telegram Stars that must be paid to upgrade the gift to a unique one*/
	pub fn upgrade_star_count(mut self, upgrade_star_count: impl Into<i64>) -> Self {
		self.upgrade_star_count = Some(upgrade_star_count.into());
		self
	}
}
/**This object represent a list of gifts.

https://core.telegram.org/bots/api/#gifts*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Gifts {
	/**The list of gifts*/
	pub gifts: Vec<Gift>,
}
impl Gifts {
	pub fn new(gifts: impl Into<Vec<Gift>>) -> Self {
		Self {
			gifts: gifts.into(),
		}
	}
	pub fn add_gift(mut self, gift: impl Into<Gift>) -> Self {
		self.gifts.push(gift.into());
		self
	}
}
/**This object represents a message about a scheduled giveaway.

https://core.telegram.org/bots/api/#giveaway*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Giveaway {
	/**The list of chats which the user must join to participate in the giveaway*/
	pub chats: Vec<Chat>,
	/**A list of two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.*/
	pub country_codes: Vec<String>,
	/**if the list of giveaway winners will be visible to everyone
	Default value: true*/
	pub has_public_winners: Option<bool>,
	/**if only users who join the chats after the giveaway started should be eligible to win
	Default value: true*/
	pub only_new_members: Option<bool>,
	/**The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only*/
	pub premium_subscription_month_count: Option<i64>,
	/**Description of additional giveaway prize*/
	pub prize_description: Option<String>,
	/**The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only*/
	pub prize_star_count: Option<i64>,
	/**The number of users which are supposed to be selected as winners of the giveaway*/
	pub winner_count: i64,
	/**Point in time (Unix timestamp) when winners of the giveaway will be selected*/
	pub winners_selection_date: i64,
}
impl Giveaway {
	pub fn new(chats: impl Into<Vec<Chat>>, winner_count: impl Into<i64>, winners_selection_date: impl Into<i64>) -> Self {
		Self {
			chats: chats.into(),
			country_codes: Vec::new(),
			has_public_winners: None,
			only_new_members: None,
			premium_subscription_month_count: None,
			prize_description: None,
			prize_star_count: None,
			winner_count: winner_count.into(),
			winners_selection_date: winners_selection_date.into(),
		}
	}
	pub fn add_chat(mut self, chat: impl Into<Chat>) -> Self {
		self.chats.push(chat.into());
		self
	}
	pub fn add_country_code(mut self, country_code: impl Into<String>) -> Self {
		self.country_codes.push(country_code.into());
		self
	}
	/** *Optional*. A list of two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.*/
	pub fn country_codes(mut self, country_codes: impl Into<Vec<String>>) -> Self {
		self.country_codes = country_codes.into();
		self
	}
	/** *Optional*. *True*, if the list of giveaway winners will be visible to everyone
	Default value: true*/
	pub fn has_public_winners(mut self, has_public_winners: bool) -> Self {
		self.has_public_winners = Some(has_public_winners);
		self
	}
	/** *Optional*. *True*, if only users who join the chats after the giveaway started should be eligible to win
	Default value: true*/
	pub fn only_new_members(mut self, only_new_members: bool) -> Self {
		self.only_new_members = Some(only_new_members);
		self
	}
	/** *Optional*. The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only*/
	pub fn premium_subscription_month_count(mut self, premium_subscription_month_count: impl Into<i64>) -> Self {
		self.premium_subscription_month_count = Some(premium_subscription_month_count.into());
		self
	}
	/** *Optional*. Description of additional giveaway prize*/
	pub fn prize_description(mut self, prize_description: impl Into<String>) -> Self {
		self.prize_description = Some(prize_description.into());
		self
	}
	/** *Optional*. The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only*/
	pub fn prize_star_count(mut self, prize_star_count: impl Into<i64>) -> Self {
		self.prize_star_count = Some(prize_star_count.into());
		self
	}
}
/**This object represents a service message about the completion of a giveaway without public winners.

https://core.telegram.org/bots/api/#giveawaycompleted*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct GiveawayCompleted {
	/**Message with the giveaway that was completed, if it wasn't deleted*/
	pub giveaway_message: Option<Box<Message>>,
	/**if the giveaway is a Telegram Star giveaway. Otherwise, currently, the giveaway is a Telegram Premium giveaway.
	Default value: true*/
	pub is_star_giveaway: Option<bool>,
	/**Number of undistributed prizes*/
	pub unclaimed_prize_count: Option<i64>,
	/**Number of winners in the giveaway*/
	pub winner_count: i64,
}
impl GiveawayCompleted {
	pub fn new(winner_count: impl Into<i64>) -> Self {
		Self {
			giveaway_message: None,
			is_star_giveaway: None,
			unclaimed_prize_count: None,
			winner_count: winner_count.into(),
		}
	}
	/** *Optional*. Message with the giveaway that was completed, if it wasn't deleted*/
	pub fn giveaway_message(mut self, giveaway_message: impl Into<Box<Message>>) -> Self {
		self.giveaway_message = Some(giveaway_message.into());
		self
	}
	/** *Optional*. *True*, if the giveaway is a Telegram Star giveaway. Otherwise, currently, the giveaway is a Telegram Premium giveaway.
	Default value: true*/
	pub fn is_star_giveaway(mut self, is_star_giveaway: bool) -> Self {
		self.is_star_giveaway = Some(is_star_giveaway);
		self
	}
	/** *Optional*. Number of undistributed prizes*/
	pub fn unclaimed_prize_count(mut self, unclaimed_prize_count: impl Into<i64>) -> Self {
		self.unclaimed_prize_count = Some(unclaimed_prize_count.into());
		self
	}
}
/**This object represents a service message about the creation of a scheduled giveaway.

https://core.telegram.org/bots/api/#giveawaycreated*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct GiveawayCreated {
	/**The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only*/
	pub prize_star_count: Option<i64>,
}
impl GiveawayCreated {
	pub fn new() -> Self {
		Self {
			prize_star_count: None,
		}
	}
	/** *Optional*. The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only*/
	pub fn prize_star_count(mut self, prize_star_count: impl Into<i64>) -> Self {
		self.prize_star_count = Some(prize_star_count.into());
		self
	}
}
/**This object represents a message about the completion of a giveaway with public winners.

https://core.telegram.org/bots/api/#giveawaywinners*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct GiveawayWinners {
	/**The number of other chats the user had to join in order to be eligible for the giveaway*/
	pub additional_chat_count: Option<i64>,
	/**The chat that created the giveaway*/
	pub chat: Chat,
	/**Identifier of the message with the giveaway in the chat*/
	pub giveaway_message_id: i64,
	/**if only users who had joined the chats after the giveaway started were eligible to win
	Default value: true*/
	pub only_new_members: Option<bool>,
	/**The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only*/
	pub premium_subscription_month_count: Option<i64>,
	/**Description of additional giveaway prize*/
	pub prize_description: Option<String>,
	/**The number of Telegram Stars that were split between giveaway winners; for Telegram Star giveaways only*/
	pub prize_star_count: Option<i64>,
	/**Number of undistributed prizes*/
	pub unclaimed_prize_count: Option<i64>,
	/**if the giveaway was canceled because the payment for it was refunded
	Default value: true*/
	pub was_refunded: Option<bool>,
	/**Total number of winners in the giveaway*/
	pub winner_count: i64,
	/**List of up to 100 winners of the giveaway*/
	pub winners: Vec<User>,
	/**Point in time (Unix timestamp) when winners of the giveaway were selected*/
	pub winners_selection_date: i64,
}
impl GiveawayWinners {
	pub fn new(chat: impl Into<Chat>, giveaway_message_id: impl Into<i64>, winner_count: impl Into<i64>, winners: impl Into<Vec<User>>, winners_selection_date: impl Into<i64>) -> Self {
		Self {
			additional_chat_count: None,
			chat: chat.into(),
			giveaway_message_id: giveaway_message_id.into(),
			only_new_members: None,
			premium_subscription_month_count: None,
			prize_description: None,
			prize_star_count: None,
			unclaimed_prize_count: None,
			was_refunded: None,
			winner_count: winner_count.into(),
			winners: winners.into(),
			winners_selection_date: winners_selection_date.into(),
		}
	}
	/** *Optional*. The number of other chats the user had to join in order to be eligible for the giveaway*/
	pub fn additional_chat_count(mut self, additional_chat_count: impl Into<i64>) -> Self {
		self.additional_chat_count = Some(additional_chat_count.into());
		self
	}
	/** *Optional*. *True*, if only users who had joined the chats after the giveaway started were eligible to win
	Default value: true*/
	pub fn only_new_members(mut self, only_new_members: bool) -> Self {
		self.only_new_members = Some(only_new_members);
		self
	}
	/** *Optional*. The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only*/
	pub fn premium_subscription_month_count(mut self, premium_subscription_month_count: impl Into<i64>) -> Self {
		self.premium_subscription_month_count = Some(premium_subscription_month_count.into());
		self
	}
	/** *Optional*. Description of additional giveaway prize*/
	pub fn prize_description(mut self, prize_description: impl Into<String>) -> Self {
		self.prize_description = Some(prize_description.into());
		self
	}
	/** *Optional*. The number of Telegram Stars that were split between giveaway winners; for Telegram Star giveaways only*/
	pub fn prize_star_count(mut self, prize_star_count: impl Into<i64>) -> Self {
		self.prize_star_count = Some(prize_star_count.into());
		self
	}
	/** *Optional*. Number of undistributed prizes*/
	pub fn unclaimed_prize_count(mut self, unclaimed_prize_count: impl Into<i64>) -> Self {
		self.unclaimed_prize_count = Some(unclaimed_prize_count.into());
		self
	}
	/** *Optional*. *True*, if the giveaway was canceled because the payment for it was refunded
	Default value: true*/
	pub fn was_refunded(mut self, was_refunded: bool) -> Self {
		self.was_refunded = Some(was_refunded);
		self
	}
	pub fn add_winner(mut self, winner: impl Into<User>) -> Self {
		self.winners.push(winner.into());
		self
	}
}
/**This object describes a message that was deleted or is otherwise inaccessible to the bot.

https://core.telegram.org/bots/api/#inaccessiblemessage*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct InaccessibleMessage {
	/**Chat the message belonged to*/
	pub chat: Chat,
	/**Always 0. The field can be used to differentiate regular and inaccessible messages.*/
	pub date: i64,
	/**Unique message identifier inside the chat*/
	pub message_id: i64,
}
impl InaccessibleMessage {
	pub fn new(chat: impl Into<Chat>, date: impl Into<i64>, message_id: impl Into<i64>) -> Self {
		Self {
			chat: chat.into(),
			date: date.into(),
			message_id: message_id.into(),
		}
	}
}
/**This object represents one button of an inline keyboard. Exactly one of the optional fields must be used to specify type of the button.

https://core.telegram.org/bots/api/#inlinekeyboardbutton*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
	/**Data to be sent in a [callback query](https://core.telegram.org/bots/api/#callbackquery) to the bot when the button is pressed, 1-64 bytes*/
	pub callback_data: Option<String>,
	/**Description of the game that will be launched when the user presses the button.  

	**NOTE:** This type of button **must** always be the first button in the first row.*/
	pub callback_game: Option<CallbackGame>,
	/**Description of the button that copies the specified text to the clipboard.*/
	pub copy_text: Option<CopyTextButton>,
	/**An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the [Telegram Login Widget](https://core.telegram.org/widgets/login).*/
	pub login_url: Option<LoginUrl>,
	/**Specify *True*, to send a [Pay button](https://core.telegram.org/bots/api/#payments). Substrings “⭐” and “XTR” in the buttons's text will be replaced with a Telegram Star icon.  

	**NOTE:** This type of button **must** always be the first button in the first row and can only be used in invoice messages.*/
	pub pay: Option<bool>,
	/**If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent on behalf of a Telegram Business account.*/
	pub switch_inline_query: Option<String>,
	/**If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent on behalf of a Telegram Business account.*/
	pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
	/**If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted.  

	This offers a quick way for the user to open your bot in inline mode in the same chat - good for selecting something from multiple options. Not supported in channels and for messages sent on behalf of a Telegram Business account.*/
	pub switch_inline_query_current_chat: Option<String>,
	/**Label text on the button*/
	pub text: String,
	/**HTTP or tg:// URL to be opened when the button is pressed. Links `tg://user?id=<user_id>` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.*/
	pub url: Option<String>,
	/**Description of the [Web App](https://core.telegram.org/bots/webapps) that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [answerWebAppQuery](https://core.telegram.org/bots/api/#answerwebappquery). Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a Telegram Business account.*/
	pub web_app: Option<WebAppInfo>,
}
impl InlineKeyboardButton {
	pub fn new(text: impl Into<String>) -> Self {
		Self {
			callback_data: None,
			callback_game: None,
			copy_text: None,
			login_url: None,
			pay: None,
			switch_inline_query: None,
			switch_inline_query_chosen_chat: None,
			switch_inline_query_current_chat: None,
			text: text.into(),
			url: None,
			web_app: None,
		}
	}
	/** *Optional*. Data to be sent in a [callback query](https://core.telegram.org/bots/api/#callbackquery) to the bot when the button is pressed, 1-64 bytes*/
	pub fn callback_data(mut self, callback_data: impl Into<String>) -> Self {
		self.callback_data = Some(callback_data.into());
		self
	}
	/** *Optional*. Description of the game that will be launched when the user presses the button.  

	**NOTE:** This type of button **must** always be the first button in the first row.*/
	pub fn callback_game(mut self, callback_game: impl Into<CallbackGame>) -> Self {
		self.callback_game = Some(callback_game.into());
		self
	}
	/** *Optional*. Description of the button that copies the specified text to the clipboard.*/
	pub fn copy_text(mut self, copy_text: impl Into<CopyTextButton>) -> Self {
		self.copy_text = Some(copy_text.into());
		self
	}
	/** *Optional*. An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the [Telegram Login Widget](https://core.telegram.org/widgets/login).*/
	pub fn login_url(mut self, login_url: impl Into<LoginUrl>) -> Self {
		self.login_url = Some(login_url.into());
		self
	}
	/** *Optional*. Specify *True*, to send a [Pay button](https://core.telegram.org/bots/api/#payments). Substrings “⭐” and “XTR” in the buttons's text will be replaced with a Telegram Star icon.  

	**NOTE:** This type of button **must** always be the first button in the first row and can only be used in invoice messages.*/
	pub fn pay(mut self, pay: bool) -> Self {
		self.pay = Some(pay);
		self
	}
	/** *Optional*. If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted. Not supported for messages sent on behalf of a Telegram Business account.*/
	pub fn switch_inline_query(mut self, switch_inline_query: impl Into<String>) -> Self {
		self.switch_inline_query = Some(switch_inline_query.into());
		self
	}
	/** *Optional*. If set, pressing the button will prompt the user to select one of their chats of the specified type, open that chat and insert the bot's username and the specified inline query in the input field. Not supported for messages sent on behalf of a Telegram Business account.*/
	pub fn switch_inline_query_chosen_chat(mut self, switch_inline_query_chosen_chat: impl Into<SwitchInlineQueryChosenChat>) -> Self {
		self.switch_inline_query_chosen_chat = Some(switch_inline_query_chosen_chat.into());
		self
	}
	/** *Optional*. If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted.  

	This offers a quick way for the user to open your bot in inline mode in the same chat - good for selecting something from multiple options. Not supported in channels and for messages sent on behalf of a Telegram Business account.*/
	pub fn switch_inline_query_current_chat(mut self, switch_inline_query_current_chat: impl Into<String>) -> Self {
		self.switch_inline_query_current_chat = Some(switch_inline_query_current_chat.into());
		self
	}
	/** *Optional*. HTTP or tg:// URL to be opened when the button is pressed. Links `tg://user?id=<user_id>` can be used to mention a user by their identifier without using a username, if this is allowed by their privacy settings.*/
	pub fn url(mut self, url: impl Into<String>) -> Self {
		self.url = Some(url.into());
		self
	}
	/** *Optional*. Description of the [Web App](https://core.telegram.org/bots/webapps) that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [answerWebAppQuery](https://core.telegram.org/bots/api/#answerwebappquery). Available only in private chats between a user and the bot. Not supported for messages sent on behalf of a Telegram Business account.*/
	pub fn web_app(mut self, web_app: impl Into<WebAppInfo>) -> Self {
		self.web_app = Some(web_app.into());
		self
	}
}
/**This object represents an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) that appears right next to the message it belongs to.

https://core.telegram.org/bots/api/#inlinekeyboardmarkup*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
	/**Array of button rows, each represented by an Array of [InlineKeyboardButton](https://core.telegram.org/bots/api/#inlinekeyboardbutton) objects*/
	pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
impl InlineKeyboardMarkup {
	pub fn new(inline_keyboard: impl Into<Vec<Vec<InlineKeyboardButton>>>) -> Self {
		Self {
			inline_keyboard: inline_keyboard.into(),
		}
	}
	pub fn add_inline_keyboard(mut self, inline_keyboard: impl Into<Vec<InlineKeyboardButton>>) -> Self {
		self.inline_keyboard.push(inline_keyboard.into());
		self
	}
}
/**This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.

https://core.telegram.org/bots/api/#inlinequery*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct InlineQuery {
	/**Type of the chat from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
	One of: sender, private, group, supergroup, channel*/
	pub chat_type: Option<String>,
	/**Sender*/
	pub from: User,
	/**Unique identifier for this query*/
	pub id: String,
	/**Sender location, only for bots that request user location*/
	pub location: Option<Location>,
	/**Offset of the results to be returned, can be controlled by the bot*/
	pub offset: String,
	/**Text of the query (up to 256 characters)*/
	pub query: String,
}
impl InlineQuery {
	pub fn new(from: impl Into<User>, id: impl Into<String>, offset: impl Into<String>, query: impl Into<String>) -> Self {
		Self {
			chat_type: None,
			from: from.into(),
			id: id.into(),
			location: None,
			offset: offset.into(),
			query: query.into(),
		}
	}
	/** *Optional*. Type of the chat from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
	One of: sender, private, group, supergroup, channel*/
	pub fn chat_type(mut self, chat_type: impl Into<String>) -> Self {
		self.chat_type = Some(chat_type.into());
		self
	}
	/** *Optional*. Sender location, only for bots that request user location*/
	pub fn location(mut self, location: impl Into<Location>) -> Self {
		self.location = Some(location.into());
		self
	}
}
/**This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:

* [InlineQueryResultCachedAudio](https://core.telegram.org/bots/api/#inlinequeryresultcachedaudio)
* [InlineQueryResultCachedDocument](https://core.telegram.org/bots/api/#inlinequeryresultcacheddocument)
* [InlineQueryResultCachedGif](https://core.telegram.org/bots/api/#inlinequeryresultcachedgif)
* [InlineQueryResultCachedMpeg4Gif](https://core.telegram.org/bots/api/#inlinequeryresultcachedmpeg4gif)
* [InlineQueryResultCachedPhoto](https://core.telegram.org/bots/api/#inlinequeryresultcachedphoto)
* [InlineQueryResultCachedSticker](https://core.telegram.org/bots/api/#inlinequeryresultcachedsticker)
* [InlineQueryResultCachedVideo](https://core.telegram.org/bots/api/#inlinequeryresultcachedvideo)
* [InlineQueryResultCachedVoice](https://core.telegram.org/bots/api/#inlinequeryresultcachedvoice)
* [InlineQueryResultArticle](https://core.telegram.org/bots/api/#inlinequeryresultarticle)
* [InlineQueryResultAudio](https://core.telegram.org/bots/api/#inlinequeryresultaudio)
* [InlineQueryResultContact](https://core.telegram.org/bots/api/#inlinequeryresultcontact)
* [InlineQueryResultGame](https://core.telegram.org/bots/api/#inlinequeryresultgame)
* [InlineQueryResultDocument](https://core.telegram.org/bots/api/#inlinequeryresultdocument)
* [InlineQueryResultGif](https://core.telegram.org/bots/api/#inlinequeryresultgif)
* [InlineQueryResultLocation](https://core.telegram.org/bots/api/#inlinequeryresultlocation)
* [InlineQueryResultMpeg4Gif](https://core.telegram.org/bots/api/#inlinequeryresultmpeg4gif)
* [InlineQueryResultPhoto](https://core.telegram.org/bots/api/#inlinequeryresultphoto)
* [InlineQueryResultVenue](https://core.telegram.org/bots/api/#inlinequeryresultvenue)
* [InlineQueryResultVideo](https://core.telegram.org/bots/api/#inlinequeryresultvideo)
* [InlineQueryResultVoice](https://core.telegram.org/bots/api/#inlinequeryresultvoice)

https://core.telegram.org/bots/api/#inlinequeryresult*/
#[derive(Clone, Debug, Serialize, From)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum InlineQueryResult {
	Article(InlineQueryResultArticle),
	Audio(InlineQueryResultAudio),
	CachedAudio(InlineQueryResultCachedAudio),
	CachedDocument(InlineQueryResultCachedDocument),
	CachedGif(InlineQueryResultCachedGif),
	CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
	CachedPhoto(InlineQueryResultCachedPhoto),
	CachedSticker(InlineQueryResultCachedSticker),
	CachedVideo(InlineQueryResultCachedVideo),
	CachedVoice(InlineQueryResultCachedVoice),
	Contact(InlineQueryResultContact),
	Document(InlineQueryResultDocument),
	Game(InlineQueryResultGame),
	Gif(InlineQueryResultGif),
	Location(InlineQueryResultLocation),
	Mpeg4Gif(InlineQueryResultMpeg4Gif),
	Photo(InlineQueryResultPhoto),
	Venue(InlineQueryResultVenue),
	Video(InlineQueryResultVideo),
	Voice(InlineQueryResultVoice),
}
/**Represents a link to an article or web page.

https://core.telegram.org/bots/api/#inlinequeryresultarticle*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultArticle {
	/**Short description of the result*/
	pub description: Option<String>,
	/**Unique identifier for this result, 1-64 Bytes*/
	pub id: String,
	/**Content of the message to be sent*/
	pub input_message_content: InputMessageContent,
	/**Type of the result, must be *article*
	Default: article*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Thumbnail height*/
	pub thumbnail_height: Option<i64>,
	/**Url of the thumbnail for the result*/
	pub thumbnail_url: Option<String>,
	/**Thumbnail width*/
	pub thumbnail_width: Option<i64>,
	/**Title of the result*/
	pub title: String,
	/**URL of the result*/
	pub url: Option<String>,
}
impl InlineQueryResultArticle {
	pub fn new(id: impl Into<String>, input_message_content: impl Into<InputMessageContent>, title: impl Into<String>) -> Self {
		Self {
			description: None,
			id: id.into(),
			input_message_content: input_message_content.into(),
			r#type: "article",
			reply_markup: None,
			thumbnail_height: None,
			thumbnail_url: None,
			thumbnail_width: None,
			title: title.into(),
			url: None,
		}
	}
	/** *Optional*. Short description of the result*/
	pub fn description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Thumbnail height*/
	pub fn thumbnail_height(mut self, thumbnail_height: impl Into<i64>) -> Self {
		self.thumbnail_height = Some(thumbnail_height.into());
		self
	}
	/** *Optional*. Url of the thumbnail for the result*/
	pub fn thumbnail_url(mut self, thumbnail_url: impl Into<String>) -> Self {
		self.thumbnail_url = Some(thumbnail_url.into());
		self
	}
	/** *Optional*. Thumbnail width*/
	pub fn thumbnail_width(mut self, thumbnail_width: impl Into<i64>) -> Self {
		self.thumbnail_width = Some(thumbnail_width.into());
		self
	}
	/** *Optional*. URL of the result*/
	pub fn url(mut self, url: impl Into<String>) -> Self {
		self.url = Some(url.into());
		self
	}
}
/**Represents a link to an MP3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the audio.

https://core.telegram.org/bots/api/#inlinequeryresultaudio*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultAudio {
	/**Audio duration in seconds*/
	pub audio_duration: Option<i64>,
	/**A valid URL for the audio file*/
	pub audio_url: String,
	/**Caption, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the audio*/
	pub input_message_content: Option<InputMessageContent>,
	/**Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Performer*/
	pub performer: Option<String>,
	/**Type of the result, must be *audio*
	Default: audio*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Title*/
	pub title: String,
}
impl InlineQueryResultAudio {
	pub fn new(audio_url: impl Into<String>, id: impl Into<String>, title: impl Into<String>) -> Self {
		Self {
			audio_duration: None,
			audio_url: audio_url.into(),
			caption: None,
			caption_entities: Vec::new(),
			id: id.into(),
			input_message_content: None,
			parse_mode: None,
			performer: None,
			r#type: "audio",
			reply_markup: None,
			title: title.into(),
		}
	}
	/** *Optional*. Audio duration in seconds*/
	pub fn audio_duration(mut self, audio_duration: impl Into<i64>) -> Self {
		self.audio_duration = Some(audio_duration.into());
		self
	}
	/** *Optional*. Caption, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Content of the message to be sent instead of the audio*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. Performer*/
	pub fn performer(mut self, performer: impl Into<String>) -> Self {
		self.performer = Some(performer.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
/**Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the audio.

https://core.telegram.org/bots/api/#inlinequeryresultcachedaudio*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultCachedAudio {
	/**A valid file identifier for the audio file*/
	pub audio_file_id: String,
	/**Caption, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the audio*/
	pub input_message_content: Option<InputMessageContent>,
	/**Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *audio*
	Default: audio*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl InlineQueryResultCachedAudio {
	pub fn new(audio_file_id: impl Into<String>, id: impl Into<String>) -> Self {
		Self {
			audio_file_id: audio_file_id.into(),
			caption: None,
			caption_entities: Vec::new(),
			id: id.into(),
			input_message_content: None,
			parse_mode: None,
			r#type: "audio",
			reply_markup: None,
		}
	}
	/** *Optional*. Caption, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Content of the message to be sent instead of the audio*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
/**Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the file.

https://core.telegram.org/bots/api/#inlinequeryresultcacheddocument*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultCachedDocument {
	/**Caption of the document to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Short description of the result*/
	pub description: Option<String>,
	/**A valid file identifier for the file*/
	pub document_file_id: String,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the file*/
	pub input_message_content: Option<InputMessageContent>,
	/**Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *document*
	Default: document*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Title for the result*/
	pub title: String,
}
impl InlineQueryResultCachedDocument {
	pub fn new(document_file_id: impl Into<String>, id: impl Into<String>, title: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			description: None,
			document_file_id: document_file_id.into(),
			id: id.into(),
			input_message_content: None,
			parse_mode: None,
			r#type: "document",
			reply_markup: None,
			title: title.into(),
		}
	}
	/** *Optional*. Caption of the document to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Short description of the result*/
	pub fn description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}
	/** *Optional*. Content of the message to be sent instead of the file*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
/**Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with specified content instead of the animation.

https://core.telegram.org/bots/api/#inlinequeryresultcachedgif*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultCachedGif {
	/**Caption of the GIF file to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**A valid file identifier for the GIF file*/
	pub gif_file_id: String,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the GIF animation*/
	pub input_message_content: Option<InputMessageContent>,
	/**Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *gif*
	Default: gif*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**Title for the result*/
	pub title: Option<String>,
}
impl InlineQueryResultCachedGif {
	pub fn new(gif_file_id: impl Into<String>, id: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			gif_file_id: gif_file_id.into(),
			id: id.into(),
			input_message_content: None,
			parse_mode: None,
			r#type: "gif",
			reply_markup: None,
			show_caption_above_media: None,
			title: None,
		}
	}
	/** *Optional*. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Content of the message to be sent instead of the GIF animation*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** *Optional*. Title for the result*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
}
/**Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the animation.

https://core.telegram.org/bots/api/#inlinequeryresultcachedmpeg4gif*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultCachedMpeg4Gif {
	/**Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the video animation*/
	pub input_message_content: Option<InputMessageContent>,
	/**A valid file identifier for the MPEG4 file*/
	pub mpeg4_file_id: String,
	/**Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *mpeg4\_gif*
	Default: mpeg4_gif*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**Title for the result*/
	pub title: Option<String>,
}
impl InlineQueryResultCachedMpeg4Gif {
	pub fn new(id: impl Into<String>, mpeg4_file_id: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			id: id.into(),
			input_message_content: None,
			mpeg4_file_id: mpeg4_file_id.into(),
			parse_mode: None,
			r#type: "mpeg4_gif",
			reply_markup: None,
			show_caption_above_media: None,
			title: None,
		}
	}
	/** *Optional*. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Content of the message to be sent instead of the video animation*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** *Optional*. Title for the result*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
}
/**Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the photo.

https://core.telegram.org/bots/api/#inlinequeryresultcachedphoto*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultCachedPhoto {
	/**Caption of the photo to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Short description of the result*/
	pub description: Option<String>,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the photo*/
	pub input_message_content: Option<InputMessageContent>,
	/**Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**A valid file identifier of the photo*/
	pub photo_file_id: String,
	/**Type of the result, must be *photo*
	Default: photo*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**Title for the result*/
	pub title: Option<String>,
}
impl InlineQueryResultCachedPhoto {
	pub fn new(id: impl Into<String>, photo_file_id: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			description: None,
			id: id.into(),
			input_message_content: None,
			parse_mode: None,
			photo_file_id: photo_file_id.into(),
			r#type: "photo",
			reply_markup: None,
			show_caption_above_media: None,
			title: None,
		}
	}
	/** *Optional*. Caption of the photo to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Short description of the result*/
	pub fn description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}
	/** *Optional*. Content of the message to be sent instead of the photo*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** *Optional*. Title for the result*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
}
/**Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the sticker.

https://core.telegram.org/bots/api/#inlinequeryresultcachedsticker*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultCachedSticker {
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the sticker*/
	pub input_message_content: Option<InputMessageContent>,
	/**Type of the result, must be *sticker*
	Default: sticker*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**A valid file identifier of the sticker*/
	pub sticker_file_id: String,
}
impl InlineQueryResultCachedSticker {
	pub fn new(id: impl Into<String>, sticker_file_id: impl Into<String>) -> Self {
		Self {
			id: id.into(),
			input_message_content: None,
			r#type: "sticker",
			reply_markup: None,
			sticker_file_id: sticker_file_id.into(),
		}
	}
	/** *Optional*. Content of the message to be sent instead of the sticker*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
/**Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the video.

https://core.telegram.org/bots/api/#inlinequeryresultcachedvideo*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultCachedVideo {
	/**Caption of the video to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Short description of the result*/
	pub description: Option<String>,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the video*/
	pub input_message_content: Option<InputMessageContent>,
	/**Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *video*
	Default: video*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**Title for the result*/
	pub title: String,
	/**A valid file identifier for the video file*/
	pub video_file_id: String,
}
impl InlineQueryResultCachedVideo {
	pub fn new(id: impl Into<String>, title: impl Into<String>, video_file_id: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			description: None,
			id: id.into(),
			input_message_content: None,
			parse_mode: None,
			r#type: "video",
			reply_markup: None,
			show_caption_above_media: None,
			title: title.into(),
			video_file_id: video_file_id.into(),
		}
	}
	/** *Optional*. Caption of the video to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Short description of the result*/
	pub fn description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}
	/** *Optional*. Content of the message to be sent instead of the video*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
}
/**Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the voice message.

https://core.telegram.org/bots/api/#inlinequeryresultcachedvoice*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultCachedVoice {
	/**Caption, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the voice message*/
	pub input_message_content: Option<InputMessageContent>,
	/**Mode for parsing entities in the voice message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *voice*
	Default: voice*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Voice message title*/
	pub title: String,
	/**A valid file identifier for the voice message*/
	pub voice_file_id: String,
}
impl InlineQueryResultCachedVoice {
	pub fn new(id: impl Into<String>, title: impl Into<String>, voice_file_id: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			id: id.into(),
			input_message_content: None,
			parse_mode: None,
			r#type: "voice",
			reply_markup: None,
			title: title.into(),
			voice_file_id: voice_file_id.into(),
		}
	}
	/** *Optional*. Caption, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Content of the message to be sent instead of the voice message*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the voice message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
/**Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the contact.

https://core.telegram.org/bots/api/#inlinequeryresultcontact*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultContact {
	/**Contact's first name*/
	pub first_name: String,
	/**Unique identifier for this result, 1-64 Bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the contact*/
	pub input_message_content: Option<InputMessageContent>,
	/**Contact's last name*/
	pub last_name: Option<String>,
	/**Contact's phone number*/
	pub phone_number: String,
	/**Type of the result, must be *contact*
	Default: contact*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Thumbnail height*/
	pub thumbnail_height: Option<i64>,
	/**Url of the thumbnail for the result*/
	pub thumbnail_url: Option<String>,
	/**Thumbnail width*/
	pub thumbnail_width: Option<i64>,
	/**Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes*/
	pub vcard: Option<String>,
}
impl InlineQueryResultContact {
	pub fn new(first_name: impl Into<String>, id: impl Into<String>, phone_number: impl Into<String>) -> Self {
		Self {
			first_name: first_name.into(),
			id: id.into(),
			input_message_content: None,
			last_name: None,
			phone_number: phone_number.into(),
			r#type: "contact",
			reply_markup: None,
			thumbnail_height: None,
			thumbnail_url: None,
			thumbnail_width: None,
			vcard: None,
		}
	}
	/** *Optional*. Content of the message to be sent instead of the contact*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Contact's last name*/
	pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
		self.last_name = Some(last_name.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Thumbnail height*/
	pub fn thumbnail_height(mut self, thumbnail_height: impl Into<i64>) -> Self {
		self.thumbnail_height = Some(thumbnail_height.into());
		self
	}
	/** *Optional*. Url of the thumbnail for the result*/
	pub fn thumbnail_url(mut self, thumbnail_url: impl Into<String>) -> Self {
		self.thumbnail_url = Some(thumbnail_url.into());
		self
	}
	/** *Optional*. Thumbnail width*/
	pub fn thumbnail_width(mut self, thumbnail_width: impl Into<i64>) -> Self {
		self.thumbnail_width = Some(thumbnail_width.into());
		self
	}
	/** *Optional*. Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes*/
	pub fn vcard(mut self, vcard: impl Into<String>) -> Self {
		self.vcard = Some(vcard.into());
		self
	}
}
/**Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the file. Currently, only **.PDF** and **.ZIP** files can be sent using this method.

https://core.telegram.org/bots/api/#inlinequeryresultdocument*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultDocument {
	/**Caption of the document to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Short description of the result*/
	pub description: Option<String>,
	/**A valid URL for the file*/
	pub document_url: String,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the file*/
	pub input_message_content: Option<InputMessageContent>,
	/**MIME type of the content of the file, either “application/pdf” or “application/zip”
	One of: application/pdf, application/zip*/
	pub mime_type: String,
	/**Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *document*
	Default: document*/
	pub r#type: &'static str,
	/**Inline keyboard attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Thumbnail height*/
	pub thumbnail_height: Option<i64>,
	/**URL of the thumbnail (JPEG only) for the file*/
	pub thumbnail_url: Option<String>,
	/**Thumbnail width*/
	pub thumbnail_width: Option<i64>,
	/**Title for the result*/
	pub title: String,
}
impl InlineQueryResultDocument {
	pub fn new(document_url: impl Into<String>, id: impl Into<String>, mime_type: impl Into<String>, title: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			description: None,
			document_url: document_url.into(),
			id: id.into(),
			input_message_content: None,
			mime_type: mime_type.into(),
			parse_mode: None,
			r#type: "document",
			reply_markup: None,
			thumbnail_height: None,
			thumbnail_url: None,
			thumbnail_width: None,
			title: title.into(),
		}
	}
	/** *Optional*. Caption of the document to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Short description of the result*/
	pub fn description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}
	/** *Optional*. Content of the message to be sent instead of the file*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. Inline keyboard attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Thumbnail height*/
	pub fn thumbnail_height(mut self, thumbnail_height: impl Into<i64>) -> Self {
		self.thumbnail_height = Some(thumbnail_height.into());
		self
	}
	/** *Optional*. URL of the thumbnail (JPEG only) for the file*/
	pub fn thumbnail_url(mut self, thumbnail_url: impl Into<String>) -> Self {
		self.thumbnail_url = Some(thumbnail_url.into());
		self
	}
	/** *Optional*. Thumbnail width*/
	pub fn thumbnail_width(mut self, thumbnail_width: impl Into<i64>) -> Self {
		self.thumbnail_width = Some(thumbnail_width.into());
		self
	}
}
/**Represents a [Game](https://core.telegram.org/bots/api/#games).

https://core.telegram.org/bots/api/#inlinequeryresultgame*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultGame {
	/**Short name of the game*/
	pub game_short_name: String,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Type of the result, must be *game*
	Default: game*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl InlineQueryResultGame {
	pub fn new(game_short_name: impl Into<String>, id: impl Into<String>) -> Self {
		Self {
			game_short_name: game_short_name.into(),
			id: id.into(),
			r#type: "game",
			reply_markup: None,
		}
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
/**Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the animation.

https://core.telegram.org/bots/api/#inlinequeryresultgif*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultGif {
	/**Caption of the GIF file to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Duration of the GIF in seconds*/
	pub gif_duration: Option<i64>,
	/**Height of the GIF*/
	pub gif_height: Option<i64>,
	/**A valid URL for the GIF file*/
	pub gif_url: String,
	/**Width of the GIF*/
	pub gif_width: Option<i64>,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the GIF animation*/
	pub input_message_content: Option<InputMessageContent>,
	/**Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *gif*
	Default: gif*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
	One of: image/jpeg, image/gif, video/mp4
	Default: image/jpeg*/
	pub thumbnail_mime_type: Option<String>,
	/**URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result*/
	pub thumbnail_url: String,
	/**Title for the result*/
	pub title: Option<String>,
}
impl InlineQueryResultGif {
	pub fn new(gif_url: impl Into<String>, id: impl Into<String>, thumbnail_url: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			gif_duration: None,
			gif_height: None,
			gif_url: gif_url.into(),
			gif_width: None,
			id: id.into(),
			input_message_content: None,
			parse_mode: None,
			r#type: "gif",
			reply_markup: None,
			show_caption_above_media: None,
			thumbnail_mime_type: None,
			thumbnail_url: thumbnail_url.into(),
			title: None,
		}
	}
	/** *Optional*. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Duration of the GIF in seconds*/
	pub fn gif_duration(mut self, gif_duration: impl Into<i64>) -> Self {
		self.gif_duration = Some(gif_duration.into());
		self
	}
	/** *Optional*. Height of the GIF*/
	pub fn gif_height(mut self, gif_height: impl Into<i64>) -> Self {
		self.gif_height = Some(gif_height.into());
		self
	}
	/** *Optional*. Width of the GIF*/
	pub fn gif_width(mut self, gif_width: impl Into<i64>) -> Self {
		self.gif_width = Some(gif_width.into());
		self
	}
	/** *Optional*. Content of the message to be sent instead of the GIF animation*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** *Optional*. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
	One of: image/jpeg, image/gif, video/mp4
	Default: image/jpeg*/
	pub fn thumbnail_mime_type(mut self, thumbnail_mime_type: impl Into<String>) -> Self {
		self.thumbnail_mime_type = Some(thumbnail_mime_type.into());
		self
	}
	/** *Optional*. Title for the result*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
}
/**Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the location.

https://core.telegram.org/bots/api/#inlinequeryresultlocation*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultLocation {
	/**For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.*/
	pub heading: Option<i64>,
	/**The radius of uncertainty for the location, measured in meters; 0-1500*/
	pub horizontal_accuracy: Option<f32>,
	/**Unique identifier for this result, 1-64 Bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the location*/
	pub input_message_content: Option<InputMessageContent>,
	/**Location latitude in degrees*/
	pub latitude: f32,
	/**Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.*/
	pub live_period: Option<i64>,
	/**Location longitude in degrees*/
	pub longitude: f32,
	/**For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.*/
	pub proximity_alert_radius: Option<i64>,
	/**Type of the result, must be *location*
	Default: location*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Thumbnail height*/
	pub thumbnail_height: Option<i64>,
	/**Url of the thumbnail for the result*/
	pub thumbnail_url: Option<String>,
	/**Thumbnail width*/
	pub thumbnail_width: Option<i64>,
	/**Location title*/
	pub title: String,
}
impl InlineQueryResultLocation {
	pub fn new(id: impl Into<String>, latitude: impl Into<f32>, longitude: impl Into<f32>, title: impl Into<String>) -> Self {
		Self {
			heading: None,
			horizontal_accuracy: None,
			id: id.into(),
			input_message_content: None,
			latitude: latitude.into(),
			live_period: None,
			longitude: longitude.into(),
			proximity_alert_radius: None,
			r#type: "location",
			reply_markup: None,
			thumbnail_height: None,
			thumbnail_url: None,
			thumbnail_width: None,
			title: title.into(),
		}
	}
	/** *Optional*. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.*/
	pub fn heading(mut self, heading: impl Into<i64>) -> Self {
		self.heading = Some(heading.into());
		self
	}
	/** *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500*/
	pub fn horizontal_accuracy(mut self, horizontal_accuracy: impl Into<f32>) -> Self {
		self.horizontal_accuracy = Some(horizontal_accuracy.into());
		self
	}
	/** *Optional*. Content of the message to be sent instead of the location*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.*/
	pub fn live_period(mut self, live_period: impl Into<i64>) -> Self {
		self.live_period = Some(live_period.into());
		self
	}
	/** *Optional*. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.*/
	pub fn proximity_alert_radius(mut self, proximity_alert_radius: impl Into<i64>) -> Self {
		self.proximity_alert_radius = Some(proximity_alert_radius.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Thumbnail height*/
	pub fn thumbnail_height(mut self, thumbnail_height: impl Into<i64>) -> Self {
		self.thumbnail_height = Some(thumbnail_height.into());
		self
	}
	/** *Optional*. Url of the thumbnail for the result*/
	pub fn thumbnail_url(mut self, thumbnail_url: impl Into<String>) -> Self {
		self.thumbnail_url = Some(thumbnail_url.into());
		self
	}
	/** *Optional*. Thumbnail width*/
	pub fn thumbnail_width(mut self, thumbnail_width: impl Into<i64>) -> Self {
		self.thumbnail_width = Some(thumbnail_width.into());
		self
	}
}
/**Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the animation.

https://core.telegram.org/bots/api/#inlinequeryresultmpeg4gif*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultMpeg4Gif {
	/**Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the video animation*/
	pub input_message_content: Option<InputMessageContent>,
	/**Video duration in seconds*/
	pub mpeg4_duration: Option<i64>,
	/**Video height*/
	pub mpeg4_height: Option<i64>,
	/**A valid URL for the MPEG4 file*/
	pub mpeg4_url: String,
	/**Video width*/
	pub mpeg4_width: Option<i64>,
	/**Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *mpeg4\_gif*
	Default: mpeg4_gif*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
	One of: image/jpeg, image/gif, video/mp4
	Default: image/jpeg*/
	pub thumbnail_mime_type: Option<String>,
	/**URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result*/
	pub thumbnail_url: String,
	/**Title for the result*/
	pub title: Option<String>,
}
impl InlineQueryResultMpeg4Gif {
	pub fn new(id: impl Into<String>, mpeg4_url: impl Into<String>, thumbnail_url: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			id: id.into(),
			input_message_content: None,
			mpeg4_duration: None,
			mpeg4_height: None,
			mpeg4_url: mpeg4_url.into(),
			mpeg4_width: None,
			parse_mode: None,
			r#type: "mpeg4_gif",
			reply_markup: None,
			show_caption_above_media: None,
			thumbnail_mime_type: None,
			thumbnail_url: thumbnail_url.into(),
			title: None,
		}
	}
	/** *Optional*. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Content of the message to be sent instead of the video animation*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Video duration in seconds*/
	pub fn mpeg4_duration(mut self, mpeg4_duration: impl Into<i64>) -> Self {
		self.mpeg4_duration = Some(mpeg4_duration.into());
		self
	}
	/** *Optional*. Video height*/
	pub fn mpeg4_height(mut self, mpeg4_height: impl Into<i64>) -> Self {
		self.mpeg4_height = Some(mpeg4_height.into());
		self
	}
	/** *Optional*. Video width*/
	pub fn mpeg4_width(mut self, mpeg4_width: impl Into<i64>) -> Self {
		self.mpeg4_width = Some(mpeg4_width.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** *Optional*. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
	One of: image/jpeg, image/gif, video/mp4
	Default: image/jpeg*/
	pub fn thumbnail_mime_type(mut self, thumbnail_mime_type: impl Into<String>) -> Self {
		self.thumbnail_mime_type = Some(thumbnail_mime_type.into());
		self
	}
	/** *Optional*. Title for the result*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
}
/**Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the photo.

https://core.telegram.org/bots/api/#inlinequeryresultphoto*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultPhoto {
	/**Caption of the photo to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Short description of the result*/
	pub description: Option<String>,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the photo*/
	pub input_message_content: Option<InputMessageContent>,
	/**Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Height of the photo*/
	pub photo_height: Option<i64>,
	/**A valid URL of the photo. Photo must be in **JPEG** format. Photo size must not exceed 5MB*/
	pub photo_url: String,
	/**Width of the photo*/
	pub photo_width: Option<i64>,
	/**Type of the result, must be *photo*
	Default: photo*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**URL of the thumbnail for the photo*/
	pub thumbnail_url: String,
	/**Title for the result*/
	pub title: Option<String>,
}
impl InlineQueryResultPhoto {
	pub fn new(id: impl Into<String>, photo_url: impl Into<String>, thumbnail_url: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			description: None,
			id: id.into(),
			input_message_content: None,
			parse_mode: None,
			photo_height: None,
			photo_url: photo_url.into(),
			photo_width: None,
			r#type: "photo",
			reply_markup: None,
			show_caption_above_media: None,
			thumbnail_url: thumbnail_url.into(),
			title: None,
		}
	}
	/** *Optional*. Caption of the photo to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Short description of the result*/
	pub fn description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}
	/** *Optional*. Content of the message to be sent instead of the photo*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. Height of the photo*/
	pub fn photo_height(mut self, photo_height: impl Into<i64>) -> Self {
		self.photo_height = Some(photo_height.into());
		self
	}
	/** *Optional*. Width of the photo*/
	pub fn photo_width(mut self, photo_width: impl Into<i64>) -> Self {
		self.photo_width = Some(photo_width.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** *Optional*. Title for the result*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
}
/**Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the venue.

https://core.telegram.org/bots/api/#inlinequeryresultvenue*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultVenue {
	/**Address of the venue*/
	pub address: String,
	/**Foursquare identifier of the venue if known*/
	pub foursquare_id: Option<String>,
	/**Foursquare type of the venue, if known. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)*/
	pub foursquare_type: Option<String>,
	/**Google Places identifier of the venue*/
	pub google_place_id: Option<String>,
	/**Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)*/
	pub google_place_type: Option<String>,
	/**Unique identifier for this result, 1-64 Bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the venue*/
	pub input_message_content: Option<InputMessageContent>,
	/**Latitude of the venue location in degrees*/
	pub latitude: f32,
	/**Longitude of the venue location in degrees*/
	pub longitude: f32,
	/**Type of the result, must be *venue*
	Default: venue*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Thumbnail height*/
	pub thumbnail_height: Option<i64>,
	/**Url of the thumbnail for the result*/
	pub thumbnail_url: Option<String>,
	/**Thumbnail width*/
	pub thumbnail_width: Option<i64>,
	/**Title of the venue*/
	pub title: String,
}
impl InlineQueryResultVenue {
	pub fn new(address: impl Into<String>, id: impl Into<String>, latitude: impl Into<f32>, longitude: impl Into<f32>, title: impl Into<String>) -> Self {
		Self {
			address: address.into(),
			foursquare_id: None,
			foursquare_type: None,
			google_place_id: None,
			google_place_type: None,
			id: id.into(),
			input_message_content: None,
			latitude: latitude.into(),
			longitude: longitude.into(),
			r#type: "venue",
			reply_markup: None,
			thumbnail_height: None,
			thumbnail_url: None,
			thumbnail_width: None,
			title: title.into(),
		}
	}
	/** *Optional*. Foursquare identifier of the venue if known*/
	pub fn foursquare_id(mut self, foursquare_id: impl Into<String>) -> Self {
		self.foursquare_id = Some(foursquare_id.into());
		self
	}
	/** *Optional*. Foursquare type of the venue, if known. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)*/
	pub fn foursquare_type(mut self, foursquare_type: impl Into<String>) -> Self {
		self.foursquare_type = Some(foursquare_type.into());
		self
	}
	/** *Optional*. Google Places identifier of the venue*/
	pub fn google_place_id(mut self, google_place_id: impl Into<String>) -> Self {
		self.google_place_id = Some(google_place_id.into());
		self
	}
	/** *Optional*. Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)*/
	pub fn google_place_type(mut self, google_place_type: impl Into<String>) -> Self {
		self.google_place_type = Some(google_place_type.into());
		self
	}
	/** *Optional*. Content of the message to be sent instead of the venue*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Thumbnail height*/
	pub fn thumbnail_height(mut self, thumbnail_height: impl Into<i64>) -> Self {
		self.thumbnail_height = Some(thumbnail_height.into());
		self
	}
	/** *Optional*. Url of the thumbnail for the result*/
	pub fn thumbnail_url(mut self, thumbnail_url: impl Into<String>) -> Self {
		self.thumbnail_url = Some(thumbnail_url.into());
		self
	}
	/** *Optional*. Thumbnail width*/
	pub fn thumbnail_width(mut self, thumbnail_width: impl Into<i64>) -> Self {
		self.thumbnail_width = Some(thumbnail_width.into());
		self
	}
}
/**Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the video.

If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you **must** replace its content using *input\_message\_content*.

https://core.telegram.org/bots/api/#inlinequeryresultvideo*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultVideo {
	/**Caption of the video to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Short description of the result*/
	pub description: Option<String>,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the video. This field is **required** if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).*/
	pub input_message_content: Option<InputMessageContent>,
	/**MIME type of the content of the video URL, “text/html” or “video/mp4”
	One of: text/html, video/mp4*/
	pub mime_type: String,
	/**Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *video*
	Default: video*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**URL of the thumbnail (JPEG only) for the video*/
	pub thumbnail_url: String,
	/**Title for the result*/
	pub title: String,
	/**Video duration in seconds*/
	pub video_duration: Option<i64>,
	/**Video height*/
	pub video_height: Option<i64>,
	/**A valid URL for the embedded video player or video file*/
	pub video_url: String,
	/**Video width*/
	pub video_width: Option<i64>,
}
impl InlineQueryResultVideo {
	pub fn new(id: impl Into<String>, mime_type: impl Into<String>, thumbnail_url: impl Into<String>, title: impl Into<String>, video_url: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			description: None,
			id: id.into(),
			input_message_content: None,
			mime_type: mime_type.into(),
			parse_mode: None,
			r#type: "video",
			reply_markup: None,
			show_caption_above_media: None,
			thumbnail_url: thumbnail_url.into(),
			title: title.into(),
			video_duration: None,
			video_height: None,
			video_url: video_url.into(),
			video_width: None,
		}
	}
	/** *Optional*. Caption of the video to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Short description of the result*/
	pub fn description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}
	/** *Optional*. Content of the message to be sent instead of the video. This field is **required** if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** *Optional*. Video duration in seconds*/
	pub fn video_duration(mut self, video_duration: impl Into<i64>) -> Self {
		self.video_duration = Some(video_duration.into());
		self
	}
	/** *Optional*. Video height*/
	pub fn video_height(mut self, video_height: impl Into<i64>) -> Self {
		self.video_height = Some(video_height.into());
		self
	}
	/** *Optional*. Video width*/
	pub fn video_width(mut self, video_width: impl Into<i64>) -> Self {
		self.video_width = Some(video_width.into());
		self
	}
}
/**Represents a link to a voice recording in an .OGG container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use *input\_message\_content* to send a message with the specified content instead of the the voice message.

https://core.telegram.org/bots/api/#inlinequeryresultvoice*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultVoice {
	/**Caption, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for this result, 1-64 bytes*/
	pub id: String,
	/**Content of the message to be sent instead of the voice recording*/
	pub input_message_content: Option<InputMessageContent>,
	/**Mode for parsing entities in the voice message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *voice*
	Default: voice*/
	pub r#type: &'static str,
	/**[Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Recording title*/
	pub title: String,
	/**Recording duration in seconds*/
	pub voice_duration: Option<i64>,
	/**A valid URL for the voice recording*/
	pub voice_url: String,
}
impl InlineQueryResultVoice {
	pub fn new(id: impl Into<String>, title: impl Into<String>, voice_url: impl Into<String>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			id: id.into(),
			input_message_content: None,
			parse_mode: None,
			r#type: "voice",
			reply_markup: None,
			title: title.into(),
			voice_duration: None,
			voice_url: voice_url.into(),
		}
	}
	/** *Optional*. Caption, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Content of the message to be sent instead of the voice recording*/
	pub fn input_message_content(mut self, input_message_content: impl Into<InputMessageContent>) -> Self {
		self.input_message_content = Some(input_message_content.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the voice message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. Recording duration in seconds*/
	pub fn voice_duration(mut self, voice_duration: impl Into<i64>) -> Self {
		self.voice_duration = Some(voice_duration.into());
		self
	}
}
/**This object represents a button to be shown above inline query results. You **must** use exactly one of the optional fields.

https://core.telegram.org/bots/api/#inlinequeryresultsbutton*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InlineQueryResultsButton {
	/**[Deep-linking](https://core.telegram.org/bots/features#deep-linking) parameter for the /start message sent to the bot when a user presses the button. 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.  

	*Example:* An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an OAuth link. Once done, the bot can offer a [*switch\_inline*](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
	Min len: 1
	Max len: 64*/
	pub start_parameter: Option<String>,
	/**Label text on the button*/
	pub text: String,
	/**Description of the [Web App](https://core.telegram.org/bots/webapps) that will be launched when the user presses the button. The Web App will be able to switch back to the inline mode using the method [switchInlineQuery](https://core.telegram.org/bots/webapps#initializing-mini-apps) inside the Web App.*/
	pub web_app: Option<WebAppInfo>,
}
impl InlineQueryResultsButton {
	pub fn new(text: impl Into<String>) -> Self {
		Self {
			start_parameter: None,
			text: text.into(),
			web_app: None,
		}
	}
	/** *Optional*. [Deep-linking](https://core.telegram.org/bots/features#deep-linking) parameter for the /start message sent to the bot when a user presses the button. 1-64 characters, only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.  

	*Example:* An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an OAuth link. Once done, the bot can offer a [*switch\_inline*](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
	Min len: 1
	Max len: 64*/
	pub fn start_parameter(mut self, start_parameter: impl Into<String>) -> Self {
		self.start_parameter = Some(start_parameter.into());
		self
	}
	/** *Optional*. Description of the [Web App](https://core.telegram.org/bots/webapps) that will be launched when the user presses the button. The Web App will be able to switch back to the inline mode using the method [switchInlineQuery](https://core.telegram.org/bots/webapps#initializing-mini-apps) inside the Web App.*/
	pub fn web_app(mut self, web_app: impl Into<WebAppInfo>) -> Self {
		self.web_app = Some(web_app.into());
		self
	}
}
/**Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a contact message to be sent as the result of an inline query.

https://core.telegram.org/bots/api/#inputcontactmessagecontent*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputContactMessageContent {
	/**Contact's first name*/
	pub first_name: String,
	/**Contact's last name*/
	pub last_name: Option<String>,
	/**Contact's phone number*/
	pub phone_number: String,
	/**Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes*/
	pub vcard: Option<String>,
}
impl InputContactMessageContent {
	pub fn new(first_name: impl Into<String>, phone_number: impl Into<String>) -> Self {
		Self {
			first_name: first_name.into(),
			last_name: None,
			phone_number: phone_number.into(),
			vcard: None,
		}
	}
	/** *Optional*. Contact's last name*/
	pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
		self.last_name = Some(last_name.into());
		self
	}
	/** *Optional*. Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes*/
	pub fn vcard(mut self, vcard: impl Into<String>) -> Self {
		self.vcard = Some(vcard.into());
		self
	}
}
/**Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of an invoice message to be sent as the result of an inline query.

https://core.telegram.org/bots/api/#inputinvoicemessagecontent*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputInvoiceMessageContent {
	/**Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub currency: String,
	/**Product description, 1-255 characters
	Min len: 1
	Max len: 255*/
	pub description: String,
	/**Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub is_flexible: Option<bool>,
	/**The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).
	Default value: 0*/
	pub max_tip_amount: Option<i64>,
	/**Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_email: Option<bool>,
	/**Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_name: Option<bool>,
	/**Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_phone_number: Option<bool>,
	/**Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_shipping_address: Option<bool>,
	/**Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.*/
	pub payload: String,
	/**Photo height*/
	pub photo_height: Option<i64>,
	/**Photo size in bytes*/
	pub photo_size: Option<i64>,
	/**URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.*/
	pub photo_url: Option<String>,
	/**Photo width*/
	pub photo_width: Option<i64>,
	/**Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub prices: Vec<LabeledPrice>,
	/**A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.*/
	pub provider_data: Option<String>,
	/**Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub provider_token: Option<String>,
	/**Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub send_email_to_provider: Option<bool>,
	/**Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub send_phone_number_to_provider: Option<bool>,
	/**A JSON-serialized array of suggested amounts of tip in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\_tip\_amount*.*/
	pub suggested_tip_amounts: Vec<i64>,
	/**Product name, 1-32 characters
	Min len: 1
	Max len: 32*/
	pub title: String,
}
impl InputInvoiceMessageContent {
	pub fn new(currency: impl Into<String>, description: impl Into<String>, payload: impl Into<String>, prices: impl Into<Vec<LabeledPrice>>, title: impl Into<String>) -> Self {
		Self {
			currency: currency.into(),
			description: description.into(),
			is_flexible: None,
			max_tip_amount: None,
			need_email: None,
			need_name: None,
			need_phone_number: None,
			need_shipping_address: None,
			payload: payload.into(),
			photo_height: None,
			photo_size: None,
			photo_url: None,
			photo_width: None,
			prices: prices.into(),
			provider_data: None,
			provider_token: None,
			send_email_to_provider: None,
			send_phone_number_to_provider: None,
			suggested_tip_amounts: Vec::new(),
			title: title.into(),
		}
	}
	/** *Optional*. Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn is_flexible(mut self, is_flexible: bool) -> Self {
		self.is_flexible = Some(is_flexible);
		self
	}
	/** *Optional*. The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).
	Default value: 0*/
	pub fn max_tip_amount(mut self, max_tip_amount: impl Into<i64>) -> Self {
		self.max_tip_amount = Some(max_tip_amount.into());
		self
	}
	/** *Optional*. Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_email(mut self, need_email: bool) -> Self {
		self.need_email = Some(need_email);
		self
	}
	/** *Optional*. Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_name(mut self, need_name: bool) -> Self {
		self.need_name = Some(need_name);
		self
	}
	/** *Optional*. Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_phone_number(mut self, need_phone_number: bool) -> Self {
		self.need_phone_number = Some(need_phone_number);
		self
	}
	/** *Optional*. Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_shipping_address(mut self, need_shipping_address: bool) -> Self {
		self.need_shipping_address = Some(need_shipping_address);
		self
	}
	/** *Optional*. Photo height*/
	pub fn photo_height(mut self, photo_height: impl Into<i64>) -> Self {
		self.photo_height = Some(photo_height.into());
		self
	}
	/** *Optional*. Photo size in bytes*/
	pub fn photo_size(mut self, photo_size: impl Into<i64>) -> Self {
		self.photo_size = Some(photo_size.into());
		self
	}
	/** *Optional*. URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.*/
	pub fn photo_url(mut self, photo_url: impl Into<String>) -> Self {
		self.photo_url = Some(photo_url.into());
		self
	}
	/** *Optional*. Photo width*/
	pub fn photo_width(mut self, photo_width: impl Into<i64>) -> Self {
		self.photo_width = Some(photo_width.into());
		self
	}
	pub fn add_price(mut self, price: impl Into<LabeledPrice>) -> Self {
		self.prices.push(price.into());
		self
	}
	/** *Optional*. A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.*/
	pub fn provider_data(mut self, provider_data: impl Into<String>) -> Self {
		self.provider_data = Some(provider_data.into());
		self
	}
	/** *Optional*. Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn provider_token(mut self, provider_token: impl Into<String>) -> Self {
		self.provider_token = Some(provider_token.into());
		self
	}
	/** *Optional*. Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn send_email_to_provider(mut self, send_email_to_provider: bool) -> Self {
		self.send_email_to_provider = Some(send_email_to_provider);
		self
	}
	/** *Optional*. Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn send_phone_number_to_provider(mut self, send_phone_number_to_provider: bool) -> Self {
		self.send_phone_number_to_provider = Some(send_phone_number_to_provider);
		self
	}
	pub fn add_suggested_tip_amount(mut self, suggested_tip_amount: impl Into<i64>) -> Self {
		self.suggested_tip_amounts.push(suggested_tip_amount.into());
		self
	}
	/** *Optional*. A JSON-serialized array of suggested amounts of tip in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\_tip\_amount*.*/
	pub fn suggested_tip_amounts(mut self, suggested_tip_amounts: impl Into<Vec<i64>>) -> Self {
		self.suggested_tip_amounts = suggested_tip_amounts.into();
		self
	}
}
/**Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a location message to be sent as the result of an inline query.

https://core.telegram.org/bots/api/#inputlocationmessagecontent*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputLocationMessageContent {
	/**For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.*/
	pub heading: Option<i64>,
	/**The radius of uncertainty for the location, measured in meters; 0-1500*/
	pub horizontal_accuracy: Option<f32>,
	/**Latitude of the location in degrees*/
	pub latitude: f32,
	/**Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.*/
	pub live_period: Option<i64>,
	/**Longitude of the location in degrees*/
	pub longitude: f32,
	/**For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.*/
	pub proximity_alert_radius: Option<i64>,
}
impl InputLocationMessageContent {
	pub fn new(latitude: impl Into<f32>, longitude: impl Into<f32>) -> Self {
		Self {
			heading: None,
			horizontal_accuracy: None,
			latitude: latitude.into(),
			live_period: None,
			longitude: longitude.into(),
			proximity_alert_radius: None,
		}
	}
	/** *Optional*. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.*/
	pub fn heading(mut self, heading: impl Into<i64>) -> Self {
		self.heading = Some(heading.into());
		self
	}
	/** *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500*/
	pub fn horizontal_accuracy(mut self, horizontal_accuracy: impl Into<f32>) -> Self {
		self.horizontal_accuracy = Some(horizontal_accuracy.into());
		self
	}
	/** *Optional*. Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.*/
	pub fn live_period(mut self, live_period: impl Into<i64>) -> Self {
		self.live_period = Some(live_period.into());
		self
	}
	/** *Optional*. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.*/
	pub fn proximity_alert_radius(mut self, proximity_alert_radius: impl Into<i64>) -> Self {
		self.proximity_alert_radius = Some(proximity_alert_radius.into());
		self
	}
}
/**This object represents the content of a media message to be sent. It should be one of

* [InputMediaAnimation](https://core.telegram.org/bots/api/#inputmediaanimation)
* [InputMediaDocument](https://core.telegram.org/bots/api/#inputmediadocument)
* [InputMediaAudio](https://core.telegram.org/bots/api/#inputmediaaudio)
* [InputMediaPhoto](https://core.telegram.org/bots/api/#inputmediaphoto)
* [InputMediaVideo](https://core.telegram.org/bots/api/#inputmediavideo)

https://core.telegram.org/bots/api/#inputmedia*/
#[derive(Clone, Debug, Serialize, From)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum InputMedia {
	Animation(InputMediaAnimation),
	Audio(InputMediaAudio),
	Document(InputMediaDocument),
	Photo(InputMediaPhoto),
	Video(InputMediaVideo),
}
/**Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.

https://core.telegram.org/bots/api/#inputmediaanimation*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputMediaAnimation {
	/**Caption of the animation to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Animation duration in seconds*/
	pub duration: Option<i64>,
	/**Pass *True* if the animation needs to be covered with a spoiler animation*/
	pub has_spoiler: Option<bool>,
	/**Animation height*/
	pub height: Option<i64>,
	/**File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub media: Asset,
	/**Mode for parsing entities in the animation caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *animation*
	Default: animation*/
	pub r#type: &'static str,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub thumbnail: Option<Asset>,
	/**Animation width*/
	pub width: Option<i64>,
}
impl InputMediaAnimation {
	pub fn new(media: impl Into<Asset>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			duration: None,
			has_spoiler: None,
			height: None,
			media: media.into(),
			parse_mode: None,
			r#type: "animation",
			show_caption_above_media: None,
			thumbnail: None,
			width: None,
		}
	}
	/** *Optional*. Caption of the animation to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Animation duration in seconds*/
	pub fn duration(mut self, duration: impl Into<i64>) -> Self {
		self.duration = Some(duration.into());
		self
	}
	/** *Optional*. Pass *True* if the animation needs to be covered with a spoiler animation*/
	pub fn has_spoiler(mut self, has_spoiler: bool) -> Self {
		self.has_spoiler = Some(has_spoiler);
		self
	}
	/** *Optional*. Animation height*/
	pub fn height(mut self, height: impl Into<i64>) -> Self {
		self.height = Some(height.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the animation caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** *Optional*. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn thumbnail(mut self, thumbnail: impl Into<Asset>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
	/** *Optional*. Animation width*/
	pub fn width(mut self, width: impl Into<i64>) -> Self {
		self.width = Some(width.into());
		self
	}
}
/**Represents an audio file to be treated as music to be sent.

https://core.telegram.org/bots/api/#inputmediaaudio*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputMediaAudio {
	/**Caption of the audio to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Duration of the audio in seconds*/
	pub duration: Option<i64>,
	/**File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub media: Asset,
	/**Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Performer of the audio*/
	pub performer: Option<String>,
	/**Type of the result, must be *audio*
	Default: audio*/
	pub r#type: &'static str,
	/**Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub thumbnail: Option<Asset>,
	/**Title of the audio*/
	pub title: Option<String>,
}
impl InputMediaAudio {
	pub fn new(media: impl Into<Asset>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			duration: None,
			media: media.into(),
			parse_mode: None,
			performer: None,
			r#type: "audio",
			thumbnail: None,
			title: None,
		}
	}
	/** *Optional*. Caption of the audio to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Duration of the audio in seconds*/
	pub fn duration(mut self, duration: impl Into<i64>) -> Self {
		self.duration = Some(duration.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. Performer of the audio*/
	pub fn performer(mut self, performer: impl Into<String>) -> Self {
		self.performer = Some(performer.into());
		self
	}
	/** *Optional*. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn thumbnail(mut self, thumbnail: impl Into<Asset>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
	/** *Optional*. Title of the audio*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
}
/**Represents a general file to be sent.

https://core.telegram.org/bots/api/#inputmediadocument*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputMediaDocument {
	/**Caption of the document to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always if the document is sent as part of an album.*/
	pub disable_content_type_detection: Option<bool>,
	/**File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub media: Asset,
	/**Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *document*
	Default: document*/
	pub r#type: &'static str,
	/**Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub thumbnail: Option<Asset>,
}
impl InputMediaDocument {
	pub fn new(media: impl Into<Asset>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			disable_content_type_detection: None,
			media: media.into(),
			parse_mode: None,
			r#type: "document",
			thumbnail: None,
		}
	}
	/** *Optional*. Caption of the document to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always *True*, if the document is sent as part of an album.*/
	pub fn disable_content_type_detection(mut self, disable_content_type_detection: bool) -> Self {
		self.disable_content_type_detection = Some(disable_content_type_detection);
		self
	}
	/** *Optional*. Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn thumbnail(mut self, thumbnail: impl Into<Asset>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
}
/**Represents a photo to be sent.

https://core.telegram.org/bots/api/#inputmediaphoto*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputMediaPhoto {
	/**Caption of the photo to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Pass *True* if the photo needs to be covered with a spoiler animation*/
	pub has_spoiler: Option<bool>,
	/**File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub media: Asset,
	/**Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *photo*
	Default: photo*/
	pub r#type: &'static str,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
}
impl InputMediaPhoto {
	pub fn new(media: impl Into<Asset>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			has_spoiler: None,
			media: media.into(),
			parse_mode: None,
			r#type: "photo",
			show_caption_above_media: None,
		}
	}
	/** *Optional*. Caption of the photo to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Pass *True* if the photo needs to be covered with a spoiler animation*/
	pub fn has_spoiler(mut self, has_spoiler: bool) -> Self {
		self.has_spoiler = Some(has_spoiler);
		self
	}
	/** *Optional*. Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
}
/**Represents a video to be sent.

https://core.telegram.org/bots/api/#inputmediavideo*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputMediaVideo {
	/**Caption of the video to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub caption: Option<String>,
	/**List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Cover for the video in the message. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub cover: Option<Asset>,
	/**Video duration in seconds*/
	pub duration: Option<i64>,
	/**Pass *True* if the video needs to be covered with a spoiler animation*/
	pub has_spoiler: Option<bool>,
	/**Video height*/
	pub height: Option<i64>,
	/**File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub media: Asset,
	/**Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Type of the result, must be *video*
	Default: video*/
	pub r#type: &'static str,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**Start timestamp for the video in the message*/
	pub start_timestamp: Option<i64>,
	/**Pass *True* if the uploaded video is suitable for streaming*/
	pub supports_streaming: Option<bool>,
	/**Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub thumbnail: Option<Asset>,
	/**Video width*/
	pub width: Option<i64>,
}
impl InputMediaVideo {
	pub fn new(media: impl Into<Asset>) -> Self {
		Self {
			caption: None,
			caption_entities: Vec::new(),
			cover: None,
			duration: None,
			has_spoiler: None,
			height: None,
			media: media.into(),
			parse_mode: None,
			r#type: "video",
			show_caption_above_media: None,
			start_timestamp: None,
			supports_streaming: None,
			thumbnail: None,
			width: None,
		}
	}
	/** *Optional*. Caption of the video to be sent, 0-1024 characters after entities parsing
	Min len: 0
	Max len: 1024*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Cover for the video in the message. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn cover(mut self, cover: impl Into<Asset>) -> Self {
		self.cover = Some(cover.into());
		self
	}
	/** *Optional*. Video duration in seconds*/
	pub fn duration(mut self, duration: impl Into<i64>) -> Self {
		self.duration = Some(duration.into());
		self
	}
	/** *Optional*. Pass *True* if the video needs to be covered with a spoiler animation*/
	pub fn has_spoiler(mut self, has_spoiler: bool) -> Self {
		self.has_spoiler = Some(has_spoiler);
		self
	}
	/** *Optional*. Video height*/
	pub fn height(mut self, height: impl Into<i64>) -> Self {
		self.height = Some(height.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** *Optional*. Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** *Optional*. Start timestamp for the video in the message*/
	pub fn start_timestamp(mut self, start_timestamp: impl Into<i64>) -> Self {
		self.start_timestamp = Some(start_timestamp.into());
		self
	}
	/** *Optional*. Pass *True* if the uploaded video is suitable for streaming*/
	pub fn supports_streaming(mut self, supports_streaming: bool) -> Self {
		self.supports_streaming = Some(supports_streaming);
		self
	}
	/** *Optional*. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn thumbnail(mut self, thumbnail: impl Into<Asset>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
	/** *Optional*. Video width*/
	pub fn width(mut self, width: impl Into<i64>) -> Self {
		self.width = Some(width.into());
		self
	}
}
/**This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:

* [InputTextMessageContent](https://core.telegram.org/bots/api/#inputtextmessagecontent)
* [InputLocationMessageContent](https://core.telegram.org/bots/api/#inputlocationmessagecontent)
* [InputVenueMessageContent](https://core.telegram.org/bots/api/#inputvenuemessagecontent)
* [InputContactMessageContent](https://core.telegram.org/bots/api/#inputcontactmessagecontent)
* [InputInvoiceMessageContent](https://core.telegram.org/bots/api/#inputinvoicemessagecontent)

https://core.telegram.org/bots/api/#inputmessagecontent*/
#[derive(Clone, Debug, Serialize, From)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum InputMessageContent {
	InputContactMessageContent(InputContactMessageContent),
	InputInvoiceMessageContent(InputInvoiceMessageContent),
	InputLocationMessageContent(InputLocationMessageContent),
	InputTextMessageContent(InputTextMessageContent),
	InputVenueMessageContent(InputVenueMessageContent),
}
/**This object describes the paid media to be sent. Currently, it can be one of

* [InputPaidMediaPhoto](https://core.telegram.org/bots/api/#inputpaidmediaphoto)
* [InputPaidMediaVideo](https://core.telegram.org/bots/api/#inputpaidmediavideo)

https://core.telegram.org/bots/api/#inputpaidmedia*/
#[derive(Clone, Debug, Serialize, From)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum InputPaidMedia {
	Photo(InputPaidMediaPhoto),
	Video(InputPaidMediaVideo),
}
/**The paid media to send is a photo.

https://core.telegram.org/bots/api/#inputpaidmediaphoto*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputPaidMediaPhoto {
	/**File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub media: Asset,
	/**Type of the media, must be *photo*
	Default: photo*/
	pub r#type: String,
}
impl InputPaidMediaPhoto {
	pub fn new(media: impl Into<Asset>, r#type: impl Into<String>) -> Self {
		Self {
			media: media.into(),
			r#type: r#type.into(),
		}
	}
}
/**The paid media to send is a video.

https://core.telegram.org/bots/api/#inputpaidmediavideo*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputPaidMediaVideo {
	/**Cover for the video in the message. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub cover: Option<Asset>,
	/**Video duration in seconds*/
	pub duration: Option<i64>,
	/**Video height*/
	pub height: Option<i64>,
	/**File to send. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub media: Asset,
	/**Type of the media, must be *video*
	Default: video*/
	pub r#type: String,
	/**Start timestamp for the video in the message*/
	pub start_timestamp: Option<i64>,
	/**Pass *True* if the uploaded video is suitable for streaming*/
	pub supports_streaming: Option<bool>,
	/**Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub thumbnail: Option<Asset>,
	/**Video width*/
	pub width: Option<i64>,
}
impl InputPaidMediaVideo {
	pub fn new(media: impl Into<Asset>, r#type: impl Into<String>) -> Self {
		Self {
			cover: None,
			duration: None,
			height: None,
			media: media.into(),
			r#type: r#type.into(),
			start_timestamp: None,
			supports_streaming: None,
			thumbnail: None,
			width: None,
		}
	}
	/** *Optional*. Cover for the video in the message. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn cover(mut self, cover: impl Into<Asset>) -> Self {
		self.cover = Some(cover.into());
		self
	}
	/** *Optional*. Video duration in seconds*/
	pub fn duration(mut self, duration: impl Into<i64>) -> Self {
		self.duration = Some(duration.into());
		self
	}
	/** *Optional*. Video height*/
	pub fn height(mut self, height: impl Into<i64>) -> Self {
		self.height = Some(height.into());
		self
	}
	/** *Optional*. Start timestamp for the video in the message*/
	pub fn start_timestamp(mut self, start_timestamp: impl Into<i64>) -> Self {
		self.start_timestamp = Some(start_timestamp.into());
		self
	}
	/** *Optional*. Pass *True* if the uploaded video is suitable for streaming*/
	pub fn supports_streaming(mut self, supports_streaming: bool) -> Self {
		self.supports_streaming = Some(supports_streaming);
		self
	}
	/** *Optional*. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn thumbnail(mut self, thumbnail: impl Into<Asset>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
	/** *Optional*. Video width*/
	pub fn width(mut self, width: impl Into<i64>) -> Self {
		self.width = Some(width.into());
		self
	}
}
/**This object contains information about one answer option in a poll to be sent.

https://core.telegram.org/bots/api/#inputpolloption*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputPollOption {
	/**Option text, 1-100 characters
	Min len: 1
	Max len: 100*/
	pub text: String,
	/**A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of *text\_parse\_mode**/
	pub text_entities: Vec<MessageEntity>,
	/**Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Currently, only custom emoji entities are allowed*/
	pub text_parse_mode: Option<String>,
}
impl InputPollOption {
	pub fn new(text: impl Into<String>) -> Self {
		Self {
			text: text.into(),
			text_entities: Vec::new(),
			text_parse_mode: None,
		}
	}
	pub fn add_text_entity(mut self, text_entity: impl Into<MessageEntity>) -> Self {
		self.text_entities.push(text_entity.into());
		self
	}
	/** *Optional*. A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of *text\_parse\_mode**/
	pub fn text_entities(mut self, text_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.text_entities = text_entities.into();
		self
	}
	/** *Optional*. Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Currently, only custom emoji entities are allowed*/
	pub fn text_parse_mode(mut self, text_parse_mode: impl Into<String>) -> Self {
		self.text_parse_mode = Some(text_parse_mode.into());
		self
	}
}
/**This object describes a sticker to be added to a sticker set.

https://core.telegram.org/bots/api/#inputsticker*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputSticker {
	/**List of 1-20 emoji associated with the sticker*/
	pub emoji_list: Vec<String>,
	/**Format of the added sticker, must be one of “static” for a **.WEBP** or **.PNG** image, “animated” for a **.TGS** animation, “video” for a **.WEBM** video
	One of: static, animated, video*/
	pub format: String,
	/**List of 0-20 search keywords for the sticker with total length of up to 64 characters. For “regular” and “custom\_emoji” stickers only.*/
	pub keywords: Vec<String>,
	/**Position where the mask should be placed on faces. For “mask” stickers only.*/
	pub mask_position: Option<MaskPosition>,
	/**The added sticker. Pass a *file\_id* as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, upload a new one using multipart/form-data, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. Animated and video stickers can't be uploaded via HTTP URL. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub sticker: Asset,
}
impl InputSticker {
	pub fn new(emoji_list: impl Into<Vec<String>>, format: impl Into<String>, sticker: impl Into<Asset>) -> Self {
		Self {
			emoji_list: emoji_list.into(),
			format: format.into(),
			keywords: Vec::new(),
			mask_position: None,
			sticker: sticker.into(),
		}
	}
	pub fn add_emoji_list(mut self, emoji_list: impl Into<String>) -> Self {
		self.emoji_list.push(emoji_list.into());
		self
	}
	pub fn add_keyword(mut self, keyword: impl Into<String>) -> Self {
		self.keywords.push(keyword.into());
		self
	}
	/** *Optional*. List of 0-20 search keywords for the sticker with total length of up to 64 characters. For “regular” and “custom\_emoji” stickers only.*/
	pub fn keywords(mut self, keywords: impl Into<Vec<String>>) -> Self {
		self.keywords = keywords.into();
		self
	}
	/** *Optional*. Position where the mask should be placed on faces. For “mask” stickers only.*/
	pub fn mask_position(mut self, mask_position: impl Into<MaskPosition>) -> Self {
		self.mask_position = Some(mask_position.into());
		self
	}
}
/**Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a text message to be sent as the result of an inline query.

https://core.telegram.org/bots/api/#inputtextmessagecontent*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputTextMessageContent {
	/**List of special entities that appear in message text, which can be specified instead of *parse\_mode**/
	pub entities: Vec<MessageEntity>,
	/**Link preview generation options for the message*/
	pub link_preview_options: Option<LinkPreviewOptions>,
	/**Text of the message to be sent, 1-4096 characters
	Min len: 1
	Max len: 4096*/
	pub message_text: String,
	/**Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
}
impl InputTextMessageContent {
	pub fn new(message_text: impl Into<String>) -> Self {
		Self {
			entities: Vec::new(),
			link_preview_options: None,
			message_text: message_text.into(),
			parse_mode: None,
		}
	}
	pub fn add_entity(mut self, entity: impl Into<MessageEntity>) -> Self {
		self.entities.push(entity.into());
		self
	}
	/** *Optional*. List of special entities that appear in message text, which can be specified instead of *parse\_mode**/
	pub fn entities(mut self, entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.entities = entities.into();
		self
	}
	/** *Optional*. Link preview generation options for the message*/
	pub fn link_preview_options(mut self, link_preview_options: impl Into<LinkPreviewOptions>) -> Self {
		self.link_preview_options = Some(link_preview_options.into());
		self
	}
	/** *Optional*. Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
}
/**Represents the [content](https://core.telegram.org/bots/api/#inputmessagecontent) of a venue message to be sent as the result of an inline query.

https://core.telegram.org/bots/api/#inputvenuemessagecontent*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct InputVenueMessageContent {
	/**Address of the venue*/
	pub address: String,
	/**Foursquare identifier of the venue, if known*/
	pub foursquare_id: Option<String>,
	/**Foursquare type of the venue, if known. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)*/
	pub foursquare_type: Option<String>,
	/**Google Places identifier of the venue*/
	pub google_place_id: Option<String>,
	/**Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)*/
	pub google_place_type: Option<String>,
	/**Latitude of the venue in degrees*/
	pub latitude: f32,
	/**Longitude of the venue in degrees*/
	pub longitude: f32,
	/**Name of the venue*/
	pub title: String,
}
impl InputVenueMessageContent {
	pub fn new(address: impl Into<String>, latitude: impl Into<f32>, longitude: impl Into<f32>, title: impl Into<String>) -> Self {
		Self {
			address: address.into(),
			foursquare_id: None,
			foursquare_type: None,
			google_place_id: None,
			google_place_type: None,
			latitude: latitude.into(),
			longitude: longitude.into(),
			title: title.into(),
		}
	}
	/** *Optional*. Foursquare identifier of the venue, if known*/
	pub fn foursquare_id(mut self, foursquare_id: impl Into<String>) -> Self {
		self.foursquare_id = Some(foursquare_id.into());
		self
	}
	/** *Optional*. Foursquare type of the venue, if known. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)*/
	pub fn foursquare_type(mut self, foursquare_type: impl Into<String>) -> Self {
		self.foursquare_type = Some(foursquare_type.into());
		self
	}
	/** *Optional*. Google Places identifier of the venue*/
	pub fn google_place_id(mut self, google_place_id: impl Into<String>) -> Self {
		self.google_place_id = Some(google_place_id.into());
		self
	}
	/** *Optional*. Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)*/
	pub fn google_place_type(mut self, google_place_type: impl Into<String>) -> Self {
		self.google_place_type = Some(google_place_type.into());
		self
	}
}
/**This object contains basic information about an invoice.

https://core.telegram.org/bots/api/#invoice*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Invoice {
	/**Three-letter ISO 4217 [currency](https://core.telegram.org/bots/payments#supported-currencies) code, or “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90)*/
	pub currency: String,
	/**Product description*/
	pub description: String,
	/**Unique bot deep-linking parameter that can be used to generate this invoice*/
	pub start_parameter: String,
	/**Product name*/
	pub title: String,
	/**Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).*/
	pub total_amount: i64,
}
impl Invoice {
	pub fn new(currency: impl Into<String>, description: impl Into<String>, start_parameter: impl Into<String>, title: impl Into<String>, total_amount: impl Into<i64>) -> Self {
		Self {
			currency: currency.into(),
			description: description.into(),
			start_parameter: start_parameter.into(),
			title: title.into(),
			total_amount: total_amount.into(),
		}
	}
}
/**This object represents one button of the reply keyboard. At most one of the optional fields must be used to specify type of the button. For simple text buttons, *String* can be used instead of this object to specify the button text.

https://core.telegram.org/bots/api/#keyboardbutton*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct KeyboardButton {
	/***Optional.* If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a “chat\_shared” service message. Available in private chats only.*/
	pub request_chat: Option<KeyboardButtonRequestChat>,
	/**If *True*, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.*/
	pub request_contact: Option<bool>,
	/**If *True*, the user's current location will be sent when the button is pressed. Available in private chats only.*/
	pub request_location: Option<bool>,
	/**If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.*/
	pub request_poll: Option<KeyboardButtonPollType>,
	/***Optional.* If specified, pressing the button will open a list of suitable users. Identifiers of selected users will be sent to the bot in a “users\_shared” service message. Available in private chats only.*/
	pub request_users: Option<KeyboardButtonRequestUsers>,
	/**Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed*/
	pub text: String,
	/**If specified, the described [Web App](https://core.telegram.org/bots/webapps) will be launched when the button is pressed. The Web App will be able to send a “web\_app\_data” service message. Available in private chats only.*/
	pub web_app: Option<WebAppInfo>,
}
impl KeyboardButton {
	pub fn new(text: impl Into<String>) -> Self {
		Self {
			request_chat: None,
			request_contact: None,
			request_location: None,
			request_poll: None,
			request_users: None,
			text: text.into(),
			web_app: None,
		}
	}
	/** *Optional.* If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a “chat\_shared” service message. Available in private chats only.*/
	pub fn request_chat(mut self, request_chat: impl Into<KeyboardButtonRequestChat>) -> Self {
		self.request_chat = Some(request_chat.into());
		self
	}
	/** *Optional*. If *True*, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.*/
	pub fn request_contact(mut self, request_contact: bool) -> Self {
		self.request_contact = Some(request_contact);
		self
	}
	/** *Optional*. If *True*, the user's current location will be sent when the button is pressed. Available in private chats only.*/
	pub fn request_location(mut self, request_location: bool) -> Self {
		self.request_location = Some(request_location);
		self
	}
	/** *Optional*. If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.*/
	pub fn request_poll(mut self, request_poll: impl Into<KeyboardButtonPollType>) -> Self {
		self.request_poll = Some(request_poll.into());
		self
	}
	/** *Optional.* If specified, pressing the button will open a list of suitable users. Identifiers of selected users will be sent to the bot in a “users\_shared” service message. Available in private chats only.*/
	pub fn request_users(mut self, request_users: impl Into<KeyboardButtonRequestUsers>) -> Self {
		self.request_users = Some(request_users.into());
		self
	}
	/** *Optional*. If specified, the described [Web App](https://core.telegram.org/bots/webapps) will be launched when the button is pressed. The Web App will be able to send a “web\_app\_data” service message. Available in private chats only.*/
	pub fn web_app(mut self, web_app: impl Into<WebAppInfo>) -> Self {
		self.web_app = Some(web_app.into());
		self
	}
}
/**This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.

https://core.telegram.org/bots/api/#keyboardbuttonpolltype*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct KeyboardButtonPollType {
	/**If *quiz* is passed, the user will be allowed to create only polls in the quiz mode. If *regular* is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.*/
	pub r#type: Option<String>,
}
impl KeyboardButtonPollType {
	pub fn new() -> Self {
		Self {
			r#type: None,
		}
	}
	/** *Optional*. If *quiz* is passed, the user will be allowed to create only polls in the quiz mode. If *regular* is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.*/
	pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
		self.r#type = Some(r#type.into());
		self
	}
}
/**This object defines the criteria used to request a suitable chat. Information about the selected chat will be shared with the bot when the corresponding button is pressed. The bot will be granted requested rights in the chat if appropriate. [More about requesting chats »](https://core.telegram.org/bots/features#chat-and-user-selection).

https://core.telegram.org/bots/api/#keyboardbuttonrequestchat*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct KeyboardButtonRequestChat {
	/**A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of *user\_administrator\_rights*. If not specified, no additional restrictions are applied.*/
	pub bot_administrator_rights: Option<ChatAdministratorRights>,
	/**Pass *True* to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.*/
	pub bot_is_member: Option<bool>,
	/**Pass *True* to request a supergroup or a channel with a username, pass *False* to request a chat without a username. If not specified, no additional restrictions are applied.*/
	pub chat_has_username: Option<bool>,
	/**Pass *True* to request a channel chat, pass *False* to request a group or a supergroup chat.*/
	pub chat_is_channel: bool,
	/**Pass *True* to request a chat owned by the user. Otherwise, no additional restrictions are applied.*/
	pub chat_is_created: Option<bool>,
	/**Pass *True* to request a forum supergroup, pass *False* to request a non-forum chat. If not specified, no additional restrictions are applied.*/
	pub chat_is_forum: Option<bool>,
	/**Signed 32-bit identifier of the request, which will be received back in the [ChatShared](https://core.telegram.org/bots/api/#chatshared) object. Must be unique within the message*/
	pub request_id: i64,
	/**Pass *True* to request the chat's photo*/
	pub request_photo: Option<bool>,
	/**Pass *True* to request the chat's title*/
	pub request_title: Option<bool>,
	/**Pass *True* to request the chat's username*/
	pub request_username: Option<bool>,
	/**A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of *bot\_administrator\_rights*. If not specified, no additional restrictions are applied.*/
	pub user_administrator_rights: Option<ChatAdministratorRights>,
}
impl KeyboardButtonRequestChat {
	pub fn new(chat_is_channel: bool, request_id: impl Into<i64>) -> Self {
		Self {
			bot_administrator_rights: None,
			bot_is_member: None,
			chat_has_username: None,
			chat_is_channel: chat_is_channel,
			chat_is_created: None,
			chat_is_forum: None,
			request_id: request_id.into(),
			request_photo: None,
			request_title: None,
			request_username: None,
			user_administrator_rights: None,
		}
	}
	/** *Optional*. A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of *user\_administrator\_rights*. If not specified, no additional restrictions are applied.*/
	pub fn bot_administrator_rights(mut self, bot_administrator_rights: impl Into<ChatAdministratorRights>) -> Self {
		self.bot_administrator_rights = Some(bot_administrator_rights.into());
		self
	}
	/** *Optional*. Pass *True* to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.*/
	pub fn bot_is_member(mut self, bot_is_member: bool) -> Self {
		self.bot_is_member = Some(bot_is_member);
		self
	}
	/** *Optional*. Pass *True* to request a supergroup or a channel with a username, pass *False* to request a chat without a username. If not specified, no additional restrictions are applied.*/
	pub fn chat_has_username(mut self, chat_has_username: bool) -> Self {
		self.chat_has_username = Some(chat_has_username);
		self
	}
	/** *Optional*. Pass *True* to request a chat owned by the user. Otherwise, no additional restrictions are applied.*/
	pub fn chat_is_created(mut self, chat_is_created: bool) -> Self {
		self.chat_is_created = Some(chat_is_created);
		self
	}
	/** *Optional*. Pass *True* to request a forum supergroup, pass *False* to request a non-forum chat. If not specified, no additional restrictions are applied.*/
	pub fn chat_is_forum(mut self, chat_is_forum: bool) -> Self {
		self.chat_is_forum = Some(chat_is_forum);
		self
	}
	/** *Optional*. Pass *True* to request the chat's photo*/
	pub fn request_photo(mut self, request_photo: bool) -> Self {
		self.request_photo = Some(request_photo);
		self
	}
	/** *Optional*. Pass *True* to request the chat's title*/
	pub fn request_title(mut self, request_title: bool) -> Self {
		self.request_title = Some(request_title);
		self
	}
	/** *Optional*. Pass *True* to request the chat's username*/
	pub fn request_username(mut self, request_username: bool) -> Self {
		self.request_username = Some(request_username);
		self
	}
	/** *Optional*. A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of *bot\_administrator\_rights*. If not specified, no additional restrictions are applied.*/
	pub fn user_administrator_rights(mut self, user_administrator_rights: impl Into<ChatAdministratorRights>) -> Self {
		self.user_administrator_rights = Some(user_administrator_rights.into());
		self
	}
}
/**This object defines the criteria used to request suitable users. Information about the selected users will be shared with the bot when the corresponding button is pressed. [More about requesting users »](https://core.telegram.org/bots/features#chat-and-user-selection)

https://core.telegram.org/bots/api/#keyboardbuttonrequestusers*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct KeyboardButtonRequestUsers {
	/**The maximum number of users to be selected; 1-10. Defaults to 1.
	Default value: 1*/
	pub max_quantity: Option<i64>,
	/**Signed 32-bit identifier of the request that will be received back in the [UsersShared](https://core.telegram.org/bots/api/#usersshared) object. Must be unique within the message*/
	pub request_id: i64,
	/**Pass *True* to request the users' first and last names*/
	pub request_name: Option<bool>,
	/**Pass *True* to request the users' photos*/
	pub request_photo: Option<bool>,
	/**Pass *True* to request the users' usernames*/
	pub request_username: Option<bool>,
	/**Pass *True* to request bots, pass *False* to request regular users. If not specified, no additional restrictions are applied.*/
	pub user_is_bot: Option<bool>,
	/**Pass *True* to request premium users, pass *False* to request non-premium users. If not specified, no additional restrictions are applied.*/
	pub user_is_premium: Option<bool>,
}
impl KeyboardButtonRequestUsers {
	pub fn new(request_id: impl Into<i64>) -> Self {
		Self {
			max_quantity: None,
			request_id: request_id.into(),
			request_name: None,
			request_photo: None,
			request_username: None,
			user_is_bot: None,
			user_is_premium: None,
		}
	}
	/** *Optional*. The maximum number of users to be selected; 1-10. Defaults to 1.
	Default value: 1*/
	pub fn max_quantity(mut self, max_quantity: impl Into<i64>) -> Self {
		self.max_quantity = Some(max_quantity.into());
		self
	}
	/** *Optional*. Pass *True* to request the users' first and last names*/
	pub fn request_name(mut self, request_name: bool) -> Self {
		self.request_name = Some(request_name);
		self
	}
	/** *Optional*. Pass *True* to request the users' photos*/
	pub fn request_photo(mut self, request_photo: bool) -> Self {
		self.request_photo = Some(request_photo);
		self
	}
	/** *Optional*. Pass *True* to request the users' usernames*/
	pub fn request_username(mut self, request_username: bool) -> Self {
		self.request_username = Some(request_username);
		self
	}
	/** *Optional*. Pass *True* to request bots, pass *False* to request regular users. If not specified, no additional restrictions are applied.*/
	pub fn user_is_bot(mut self, user_is_bot: bool) -> Self {
		self.user_is_bot = Some(user_is_bot);
		self
	}
	/** *Optional*. Pass *True* to request premium users, pass *False* to request non-premium users. If not specified, no additional restrictions are applied.*/
	pub fn user_is_premium(mut self, user_is_premium: bool) -> Self {
		self.user_is_premium = Some(user_is_premium);
		self
	}
}
/**This object represents a portion of the price for goods or services.

https://core.telegram.org/bots/api/#labeledprice*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct LabeledPrice {
	/**Price of the product in the *smallest units* of the [currency](https://core.telegram.org/bots/payments#supported-currencies) (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).*/
	pub amount: i64,
	/**Portion label*/
	pub label: String,
}
impl LabeledPrice {
	pub fn new(amount: impl Into<i64>, label: impl Into<String>) -> Self {
		Self {
			amount: amount.into(),
			label: label.into(),
		}
	}
}
/**Describes the options used for link preview generation.

https://core.telegram.org/bots/api/#linkpreviewoptions*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
	/**if the link preview is disabled*/
	pub is_disabled: Option<bool>,
	/**if the media in the link preview is supposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview*/
	pub prefer_large_media: Option<bool>,
	/**if the media in the link preview is supposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview*/
	pub prefer_small_media: Option<bool>,
	/**if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text*/
	pub show_above_text: Option<bool>,
	/**URL to use for the link preview. If empty, then the first URL found in the message text will be used*/
	pub url: Option<String>,
}
impl LinkPreviewOptions {
	pub fn new() -> Self {
		Self {
			is_disabled: None,
			prefer_large_media: None,
			prefer_small_media: None,
			show_above_text: None,
			url: None,
		}
	}
	/** *Optional*. *True*, if the link preview is disabled*/
	pub fn is_disabled(mut self, is_disabled: bool) -> Self {
		self.is_disabled = Some(is_disabled);
		self
	}
	/** *Optional*. *True*, if the media in the link preview is supposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview*/
	pub fn prefer_large_media(mut self, prefer_large_media: bool) -> Self {
		self.prefer_large_media = Some(prefer_large_media);
		self
	}
	/** *Optional*. *True*, if the media in the link preview is supposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview*/
	pub fn prefer_small_media(mut self, prefer_small_media: bool) -> Self {
		self.prefer_small_media = Some(prefer_small_media);
		self
	}
	/** *Optional*. *True*, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text*/
	pub fn show_above_text(mut self, show_above_text: bool) -> Self {
		self.show_above_text = Some(show_above_text);
		self
	}
	/** *Optional*. URL to use for the link preview. If empty, then the first URL found in the message text will be used*/
	pub fn url(mut self, url: impl Into<String>) -> Self {
		self.url = Some(url.into());
		self
	}
}
/**This object represents a point on the map.

https://core.telegram.org/bots/api/#location*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Location {
	/**The direction in which user is moving, in degrees; 1-360. For active live locations only.*/
	pub heading: Option<i64>,
	/**The radius of uncertainty for the location, measured in meters; 0-1500*/
	pub horizontal_accuracy: Option<f32>,
	/**Latitude as defined by the sender*/
	pub latitude: f32,
	/**Time relative to the message sending date, during which the location can be updated; in seconds. For active live locations only.*/
	pub live_period: Option<i64>,
	/**Longitude as defined by the sender*/
	pub longitude: f32,
	/**The maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only.*/
	pub proximity_alert_radius: Option<i64>,
}
impl Location {
	pub fn new(latitude: impl Into<f32>, longitude: impl Into<f32>) -> Self {
		Self {
			heading: None,
			horizontal_accuracy: None,
			latitude: latitude.into(),
			live_period: None,
			longitude: longitude.into(),
			proximity_alert_radius: None,
		}
	}
	/** *Optional*. The direction in which user is moving, in degrees; 1-360. For active live locations only.*/
	pub fn heading(mut self, heading: impl Into<i64>) -> Self {
		self.heading = Some(heading.into());
		self
	}
	/** *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500*/
	pub fn horizontal_accuracy(mut self, horizontal_accuracy: impl Into<f32>) -> Self {
		self.horizontal_accuracy = Some(horizontal_accuracy.into());
		self
	}
	/** *Optional*. Time relative to the message sending date, during which the location can be updated; in seconds. For active live locations only.*/
	pub fn live_period(mut self, live_period: impl Into<i64>) -> Self {
		self.live_period = Some(live_period.into());
		self
	}
	/** *Optional*. The maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only.*/
	pub fn proximity_alert_radius(mut self, proximity_alert_radius: impl Into<i64>) -> Self {
		self.proximity_alert_radius = Some(proximity_alert_radius.into());
		self
	}
}
/**This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the [Telegram Login Widget](https://core.telegram.org/widgets/login) when the user is coming from Telegram. All the user needs to do is tap/click a button and confirm that they want to log in:

Telegram apps support these buttons as of [version 5.7](https://telegram.org/blog/privacy-discussions-web-bots#meet-seamless-web-bots).

Sample bot: [@discussbot](https://t.me/discussbot)

https://core.telegram.org/bots/api/#loginurl*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginUrl {
	/**Username of a bot, which will be used for user authorization. See [Setting up a bot](https://core.telegram.org/widgets/login#setting-up-a-bot) for more details. If not specified, the current bot's username will be assumed. The *url*'s domain must be the same as the domain linked with the bot. See [Linking your domain to the bot](https://core.telegram.org/widgets/login#linking-your-domain-to-the-bot) for more details.*/
	pub bot_username: Option<String>,
	/**New text of the button in forwarded messages.*/
	pub forward_text: Option<String>,
	/**Pass *True* to request the permission for your bot to send messages to the user.*/
	pub request_write_access: Option<bool>,
	/**An HTTPS URL to be opened with user authorization data added to the query string when the button is pressed. If the user refuses to provide authorization data, the original URL without information about the user will be opened. The data added is the same as described in [Receiving authorization data](https://core.telegram.org/widgets/login#receiving-authorization-data).  

	**NOTE:** You **must** always check the hash of the received data to verify the authentication and the integrity of the data as described in [Checking authorization](https://core.telegram.org/widgets/login#checking-authorization).*/
	pub url: String,
}
impl LoginUrl {
	pub fn new(url: impl Into<String>) -> Self {
		Self {
			bot_username: None,
			forward_text: None,
			request_write_access: None,
			url: url.into(),
		}
	}
	/** *Optional*. Username of a bot, which will be used for user authorization. See [Setting up a bot](https://core.telegram.org/widgets/login#setting-up-a-bot) for more details. If not specified, the current bot's username will be assumed. The *url*'s domain must be the same as the domain linked with the bot. See [Linking your domain to the bot](https://core.telegram.org/widgets/login#linking-your-domain-to-the-bot) for more details.*/
	pub fn bot_username(mut self, bot_username: impl Into<String>) -> Self {
		self.bot_username = Some(bot_username.into());
		self
	}
	/** *Optional*. New text of the button in forwarded messages.*/
	pub fn forward_text(mut self, forward_text: impl Into<String>) -> Self {
		self.forward_text = Some(forward_text.into());
		self
	}
	/** *Optional*. Pass *True* to request the permission for your bot to send messages to the user.*/
	pub fn request_write_access(mut self, request_write_access: bool) -> Self {
		self.request_write_access = Some(request_write_access);
		self
	}
}
/**This object describes the position on faces where a mask should be placed by default.

https://core.telegram.org/bots/api/#maskposition*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaskPosition {
	/**The part of the face relative to which the mask should be placed. One of “forehead”, “eyes”, “mouth”, or “chin”.
	One of: forehead, eyes, mouth, chin*/
	pub point: String,
	/**Mask scaling coefficient. For example, 2.0 means double size.*/
	pub scale: f32,
	/**Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.*/
	pub x_shift: f32,
	/**Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.*/
	pub y_shift: f32,
}
impl MaskPosition {
	pub fn new(point: impl Into<String>, scale: impl Into<f32>, x_shift: impl Into<f32>, y_shift: impl Into<f32>) -> Self {
		Self {
			point: point.into(),
			scale: scale.into(),
			x_shift: x_shift.into(),
			y_shift: y_shift.into(),
		}
	}
}
/**This object describes a message that can be inaccessible to the bot. It can be one of

* [Message](https://core.telegram.org/bots/api/#message)
* [InaccessibleMessage](https://core.telegram.org/bots/api/#inaccessiblemessage)

https://core.telegram.org/bots/api/#maybeinaccessiblemessage*/
#[derive(Clone, Debug, Deserialize, From)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum MaybeInaccessibleMessage {
	InaccessibleMessage(InaccessibleMessage),
	Message(Box<Message>),
}
/**A JSON-serialized array describing messages to be sent, must include 2-10 items*/
#[derive(Clone, Debug, Serialize, From)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum Media {
	InputMediaAudio(InputMediaAudio),
	InputMediaDocument(InputMediaDocument),
	InputMediaPhoto(InputMediaPhoto),
	InputMediaVideo(InputMediaVideo),
}
/**This object describes the bot's menu button in a private chat. It should be one of

* [MenuButtonCommands](https://core.telegram.org/bots/api/#menubuttoncommands)
* [MenuButtonWebApp](https://core.telegram.org/bots/api/#menubuttonwebapp)
* [MenuButtonDefault](https://core.telegram.org/bots/api/#menubuttondefault)

https://core.telegram.org/bots/api/#menubutton*/
#[derive(Clone, Debug, Serialize, Deserialize, From)]
#[serde(untagged, rename_all = "snake_case")]
pub enum MenuButton {
	Commands(MenuButtonCommands),
	Default(MenuButtonDefault),
	WebApp(MenuButtonWebApp),
}
/**Represents a menu button, which opens the bot's list of commands.

https://core.telegram.org/bots/api/#menubuttoncommands*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuButtonCommands {
	/**Type of the button, must be *commands*
	Default: commands*/
	pub r#type: String,
}
impl MenuButtonCommands {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**Describes that no specific value for the menu button was set.

https://core.telegram.org/bots/api/#menubuttondefault*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuButtonDefault {
	/**Type of the button, must be *default*
	Default: default*/
	pub r#type: String,
}
impl MenuButtonDefault {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**Represents a menu button, which launches a [Web App](https://core.telegram.org/bots/webapps).

https://core.telegram.org/bots/api/#menubuttonwebapp*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuButtonWebApp {
	/**Type of the button, must be *web\_app*
	Default: web_app*/
	pub r#type: String,
	/**Text on the button*/
	pub text: String,
	/**Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method [answerWebAppQuery](https://core.telegram.org/bots/api/#answerwebappquery). Alternatively, a `t.me` link to a Web App of the bot can be specified in the object instead of the Web App's URL, in which case the Web App will be opened as if the user pressed the link.*/
	pub web_app: WebAppInfo,
}
impl MenuButtonWebApp {
	pub fn new(r#type: impl Into<String>, text: impl Into<String>, web_app: impl Into<WebAppInfo>) -> Self {
		Self {
			r#type: r#type.into(),
			text: text.into(),
			web_app: web_app.into(),
		}
	}
}
/**This object represents a service message about a change in auto-delete timer settings.

https://core.telegram.org/bots/api/#messageautodeletetimerchanged*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct MessageAutoDeleteTimerChanged {
	/**New auto-delete time for messages in the chat; in seconds*/
	pub message_auto_delete_time: i64,
}
impl MessageAutoDeleteTimerChanged {
	pub fn new(message_auto_delete_time: impl Into<i64>) -> Self {
		Self {
			message_auto_delete_time: message_auto_delete_time.into(),
		}
	}
}
/**This object represents a unique message identifier.

https://core.telegram.org/bots/api/#messageid*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct MessageId {
	/**Unique message identifier. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent*/
	pub message_id: i64,
}
impl MessageId {
	pub fn new(message_id: impl Into<i64>) -> Self {
		Self {
			message_id: message_id.into(),
		}
	}
}
/**This object describes the origin of a message. It can be one of

* [MessageOriginUser](https://core.telegram.org/bots/api/#messageoriginuser)
* [MessageOriginHiddenUser](https://core.telegram.org/bots/api/#messageoriginhiddenuser)
* [MessageOriginChat](https://core.telegram.org/bots/api/#messageoriginchat)
* [MessageOriginChannel](https://core.telegram.org/bots/api/#messageoriginchannel)

https://core.telegram.org/bots/api/#messageorigin*/
#[derive(Clone, Debug, Deserialize, From)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum MessageOrigin {
	Channel(MessageOriginChannel),
	Chat(MessageOriginChat),
	HiddenUser(MessageOriginHiddenUser),
	User(MessageOriginUser),
}
/**The message was originally sent to a channel chat.

https://core.telegram.org/bots/api/#messageoriginchannel*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct MessageOriginChannel {
	/**Signature of the original post author*/
	pub author_signature: Option<String>,
	/**Channel chat to which the message was originally sent*/
	pub chat: Chat,
	/**Date the message was sent originally in Unix time*/
	pub date: i64,
	/**Unique message identifier inside the chat*/
	pub message_id: i64,
	/**Type of the message origin, always “channel”
	Default: channel*/
	pub r#type: String,
}
impl MessageOriginChannel {
	pub fn new(chat: impl Into<Chat>, date: impl Into<i64>, message_id: impl Into<i64>, r#type: impl Into<String>) -> Self {
		Self {
			author_signature: None,
			chat: chat.into(),
			date: date.into(),
			message_id: message_id.into(),
			r#type: r#type.into(),
		}
	}
	/** *Optional*. Signature of the original post author*/
	pub fn author_signature(mut self, author_signature: impl Into<String>) -> Self {
		self.author_signature = Some(author_signature.into());
		self
	}
}
/**The message was originally sent on behalf of a chat to a group chat.

https://core.telegram.org/bots/api/#messageoriginchat*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct MessageOriginChat {
	/**For messages originally sent by an anonymous chat administrator, original message author signature*/
	pub author_signature: Option<String>,
	/**Date the message was sent originally in Unix time*/
	pub date: i64,
	/**Type of the message origin, always “chat”
	Default: chat*/
	pub r#type: String,
	/**Chat that sent the message originally*/
	pub sender_chat: Chat,
}
impl MessageOriginChat {
	pub fn new(date: impl Into<i64>, r#type: impl Into<String>, sender_chat: impl Into<Chat>) -> Self {
		Self {
			author_signature: None,
			date: date.into(),
			r#type: r#type.into(),
			sender_chat: sender_chat.into(),
		}
	}
	/** *Optional*. For messages originally sent by an anonymous chat administrator, original message author signature*/
	pub fn author_signature(mut self, author_signature: impl Into<String>) -> Self {
		self.author_signature = Some(author_signature.into());
		self
	}
}
/**The message was originally sent by an unknown user.

https://core.telegram.org/bots/api/#messageoriginhiddenuser*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct MessageOriginHiddenUser {
	/**Date the message was sent originally in Unix time*/
	pub date: i64,
	/**Type of the message origin, always “hidden\_user”
	Default: hidden_user*/
	pub r#type: String,
	/**Name of the user that sent the message originally*/
	pub sender_user_name: String,
}
impl MessageOriginHiddenUser {
	pub fn new(date: impl Into<i64>, r#type: impl Into<String>, sender_user_name: impl Into<String>) -> Self {
		Self {
			date: date.into(),
			r#type: r#type.into(),
			sender_user_name: sender_user_name.into(),
		}
	}
}
/**The message was originally sent by a known user.

https://core.telegram.org/bots/api/#messageoriginuser*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct MessageOriginUser {
	/**Date the message was sent originally in Unix time*/
	pub date: i64,
	/**Type of the message origin, always “user”
	Default: user*/
	pub r#type: String,
	/**User that sent the message originally*/
	pub sender_user: User,
}
impl MessageOriginUser {
	pub fn new(date: impl Into<i64>, r#type: impl Into<String>, sender_user: impl Into<User>) -> Self {
		Self {
			date: date.into(),
			r#type: r#type.into(),
			sender_user: sender_user.into(),
		}
	}
}
/**This object represents reaction changes on a message with anonymous reactions.

https://core.telegram.org/bots/api/#messagereactioncountupdated*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct MessageReactionCountUpdated {
	/**The chat containing the message*/
	pub chat: Chat,
	/**Date of the change in Unix time*/
	pub date: i64,
	/**Unique message identifier inside the chat*/
	pub message_id: i64,
	/**List of reactions that are present on the message*/
	pub reactions: Vec<ReactionCount>,
}
impl MessageReactionCountUpdated {
	pub fn new(chat: impl Into<Chat>, date: impl Into<i64>, message_id: impl Into<i64>, reactions: impl Into<Vec<ReactionCount>>) -> Self {
		Self {
			chat: chat.into(),
			date: date.into(),
			message_id: message_id.into(),
			reactions: reactions.into(),
		}
	}
	pub fn add_reaction(mut self, reaction: impl Into<ReactionCount>) -> Self {
		self.reactions.push(reaction.into());
		self
	}
}
/**This object represents a change of a reaction on a message performed by a user.

https://core.telegram.org/bots/api/#messagereactionupdated*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct MessageReactionUpdated {
	/**The chat on behalf of which the reaction was changed, if the user is anonymous*/
	pub actor_chat: Option<Chat>,
	/**The chat containing the message the user reacted to*/
	pub chat: Chat,
	/**Date of the change in Unix time*/
	pub date: i64,
	/**Unique identifier of the message inside the chat*/
	pub message_id: i64,
	/**New list of reaction types that have been set by the user*/
	pub new_reaction: Vec<ReactionType>,
	/**Previous list of reaction types that were set by the user*/
	pub old_reaction: Vec<ReactionType>,
	/**The user that changed the reaction, if the user isn't anonymous*/
	pub user: Option<User>,
}
impl MessageReactionUpdated {
	pub fn new(chat: impl Into<Chat>, date: impl Into<i64>, message_id: impl Into<i64>, new_reaction: impl Into<Vec<ReactionType>>, old_reaction: impl Into<Vec<ReactionType>>) -> Self {
		Self {
			actor_chat: None,
			chat: chat.into(),
			date: date.into(),
			message_id: message_id.into(),
			new_reaction: new_reaction.into(),
			old_reaction: old_reaction.into(),
			user: None,
		}
	}
	/** *Optional*. The chat on behalf of which the reaction was changed, if the user is anonymous*/
	pub fn actor_chat(mut self, actor_chat: impl Into<Chat>) -> Self {
		self.actor_chat = Some(actor_chat.into());
		self
	}
	pub fn add_new_reaction(mut self, new_reaction: impl Into<ReactionType>) -> Self {
		self.new_reaction.push(new_reaction.into());
		self
	}
	pub fn add_old_reaction(mut self, old_reaction: impl Into<ReactionType>) -> Self {
		self.old_reaction.push(old_reaction.into());
		self
	}
	/** *Optional*. The user that changed the reaction, if the user isn't anonymous*/
	pub fn user(mut self, user: impl Into<User>) -> Self {
		self.user = Some(user.into());
		self
	}
}
/**This object represents information about an order.

https://core.telegram.org/bots/api/#orderinfo*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct OrderInfo {
	/**User email*/
	pub email: Option<String>,
	/**User name*/
	pub name: Option<String>,
	/**User's phone number*/
	pub phone_number: Option<String>,
	/**User shipping address*/
	pub shipping_address: Option<ShippingAddress>,
}
impl OrderInfo {
	pub fn new() -> Self {
		Self {
			email: None,
			name: None,
			phone_number: None,
			shipping_address: None,
		}
	}
	/** *Optional*. User email*/
	pub fn email(mut self, email: impl Into<String>) -> Self {
		self.email = Some(email.into());
		self
	}
	/** *Optional*. User name*/
	pub fn name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}
	/** *Optional*. User's phone number*/
	pub fn phone_number(mut self, phone_number: impl Into<String>) -> Self {
		self.phone_number = Some(phone_number.into());
		self
	}
	/** *Optional*. User shipping address*/
	pub fn shipping_address(mut self, shipping_address: impl Into<ShippingAddress>) -> Self {
		self.shipping_address = Some(shipping_address.into());
		self
	}
}
/**This object describes paid media. Currently, it can be one of

* [PaidMediaPreview](https://core.telegram.org/bots/api/#paidmediapreview)
* [PaidMediaPhoto](https://core.telegram.org/bots/api/#paidmediaphoto)
* [PaidMediaVideo](https://core.telegram.org/bots/api/#paidmediavideo)

https://core.telegram.org/bots/api/#paidmedia*/
#[derive(Clone, Debug, Deserialize, From)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum PaidMedia {
	Photo(PaidMediaPhoto),
	Preview(PaidMediaPreview),
	Video(PaidMediaVideo),
}
/**Describes the paid media added to a message.

https://core.telegram.org/bots/api/#paidmediainfo*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PaidMediaInfo {
	/**Information about the paid media*/
	pub paid_media: Vec<PaidMedia>,
	/**The number of Telegram Stars that must be paid to buy access to the media*/
	pub star_count: i64,
}
impl PaidMediaInfo {
	pub fn new(paid_media: impl Into<Vec<PaidMedia>>, star_count: impl Into<i64>) -> Self {
		Self {
			paid_media: paid_media.into(),
			star_count: star_count.into(),
		}
	}
	pub fn add_paid_media(mut self, paid_media: impl Into<PaidMedia>) -> Self {
		self.paid_media.push(paid_media.into());
		self
	}
}
/**The paid media is a photo.

https://core.telegram.org/bots/api/#paidmediaphoto*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PaidMediaPhoto {
	/**The photo*/
	pub photo: Vec<PhotoSize>,
	/**Type of the paid media, always “photo”
	Default: photo*/
	pub r#type: String,
}
impl PaidMediaPhoto {
	pub fn new(photo: impl Into<Vec<PhotoSize>>, r#type: impl Into<String>) -> Self {
		Self {
			photo: photo.into(),
			r#type: r#type.into(),
		}
	}
	pub fn add_photo(mut self, photo: impl Into<PhotoSize>) -> Self {
		self.photo.push(photo.into());
		self
	}
}
/**The paid media isn't available before the payment.

https://core.telegram.org/bots/api/#paidmediapreview*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PaidMediaPreview {
	/**Duration of the media in seconds as defined by the sender*/
	pub duration: Option<i64>,
	/**Media height as defined by the sender*/
	pub height: Option<i64>,
	/**Type of the paid media, always “preview”
	Default: preview*/
	pub r#type: String,
	/**Media width as defined by the sender*/
	pub width: Option<i64>,
}
impl PaidMediaPreview {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			duration: None,
			height: None,
			r#type: r#type.into(),
			width: None,
		}
	}
	/** *Optional*. Duration of the media in seconds as defined by the sender*/
	pub fn duration(mut self, duration: impl Into<i64>) -> Self {
		self.duration = Some(duration.into());
		self
	}
	/** *Optional*. Media height as defined by the sender*/
	pub fn height(mut self, height: impl Into<i64>) -> Self {
		self.height = Some(height.into());
		self
	}
	/** *Optional*. Media width as defined by the sender*/
	pub fn width(mut self, width: impl Into<i64>) -> Self {
		self.width = Some(width.into());
		self
	}
}
/**This object contains information about a paid media purchase.

https://core.telegram.org/bots/api/#paidmediapurchased*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PaidMediaPurchased {
	/**User who purchased the media*/
	pub from: User,
	/**Bot-specified paid media payload*/
	pub paid_media_payload: String,
}
impl PaidMediaPurchased {
	pub fn new(from: impl Into<User>, paid_media_payload: impl Into<String>) -> Self {
		Self {
			from: from.into(),
			paid_media_payload: paid_media_payload.into(),
		}
	}
}
/**The paid media is a video.

https://core.telegram.org/bots/api/#paidmediavideo*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PaidMediaVideo {
	/**Type of the paid media, always “video”
	Default: video*/
	pub r#type: String,
	/**The video*/
	pub video: Video,
}
impl PaidMediaVideo {
	pub fn new(r#type: impl Into<String>, video: impl Into<Video>) -> Self {
		Self {
			r#type: r#type.into(),
			video: video.into(),
		}
	}
}
/**Describes Telegram Passport data shared with the bot by the user.

https://core.telegram.org/bots/api/#passportdata*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PassportData {
	/**Encrypted credentials required to decrypt the data*/
	pub credentials: EncryptedCredentials,
	/**Array with information about documents and other Telegram Passport elements that was shared with the bot*/
	pub data: Vec<EncryptedPassportElement>,
}
impl PassportData {
	pub fn new(credentials: impl Into<EncryptedCredentials>, data: impl Into<Vec<EncryptedPassportElement>>) -> Self {
		Self {
			credentials: credentials.into(),
			data: data.into(),
		}
	}
	pub fn add_datum(mut self, datum: impl Into<EncryptedPassportElement>) -> Self {
		self.data.push(datum.into());
		self
	}
}
/**This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:

* [PassportElementErrorDataField](https://core.telegram.org/bots/api/#passportelementerrordatafield)
* [PassportElementErrorFrontSide](https://core.telegram.org/bots/api/#passportelementerrorfrontside)
* [PassportElementErrorReverseSide](https://core.telegram.org/bots/api/#passportelementerrorreverseside)
* [PassportElementErrorSelfie](https://core.telegram.org/bots/api/#passportelementerrorselfie)
* [PassportElementErrorFile](https://core.telegram.org/bots/api/#passportelementerrorfile)
* [PassportElementErrorFiles](https://core.telegram.org/bots/api/#passportelementerrorfiles)
* [PassportElementErrorTranslationFile](https://core.telegram.org/bots/api/#passportelementerrortranslationfile)
* [PassportElementErrorTranslationFiles](https://core.telegram.org/bots/api/#passportelementerrortranslationfiles)
* [PassportElementErrorUnspecified](https://core.telegram.org/bots/api/#passportelementerrorunspecified)

https://core.telegram.org/bots/api/#passportelementerror*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum PassportElementError {
	DataField(PassportElementErrorDataField),
	File(PassportElementErrorFile),
	Files(PassportElementErrorFiles),
	FrontSide(PassportElementErrorFrontSide),
	ReverseSide(PassportElementErrorReverseSide),
	Selfie(PassportElementErrorSelfie),
	TranslationFile(PassportElementErrorTranslationFile),
	TranslationFiles(PassportElementErrorTranslationFiles),
	Unspecified(PassportElementErrorUnspecified),
}
/**Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.

https://core.telegram.org/bots/api/#passportelementerrordatafield*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct PassportElementErrorDataField {
	/**Base64-encoded data hash*/
	pub data_hash: String,
	/**Name of the data field which has the error*/
	pub field_name: String,
	/**Error message*/
	pub message: String,
	/**The section of the user's Telegram Passport which has the error, one of “personal\_details”, “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “address”
	One of: personal_details, passport, driver_license, identity_card, internal_passport, address*/
	pub r#type: String,
	/**Error source, must be *data*
	Default: data*/
	pub source: String,
}
impl PassportElementErrorDataField {
	pub fn new(data_hash: impl Into<String>, field_name: impl Into<String>, message: impl Into<String>, r#type: impl Into<String>, source: impl Into<String>) -> Self {
		Self {
			data_hash: data_hash.into(),
			field_name: field_name.into(),
			message: message.into(),
			r#type: r#type.into(),
			source: source.into(),
		}
	}
}
/**Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.

https://core.telegram.org/bots/api/#passportelementerrorfile*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct PassportElementErrorFile {
	/**Base64-encoded file hash*/
	pub file_hash: String,
	/**Error message*/
	pub message: String,
	/**The section of the user's Telegram Passport which has the issue, one of “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
	One of: utility_bill, bank_statement, rental_agreement, passport_registration, temporary_registration*/
	pub r#type: String,
	/**Error source, must be *file*
	Default: file*/
	pub source: String,
}
impl PassportElementErrorFile {
	pub fn new(file_hash: impl Into<String>, message: impl Into<String>, r#type: impl Into<String>, source: impl Into<String>) -> Self {
		Self {
			file_hash: file_hash.into(),
			message: message.into(),
			r#type: r#type.into(),
			source: source.into(),
		}
	}
}
/**Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.

https://core.telegram.org/bots/api/#passportelementerrorfiles*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct PassportElementErrorFiles {
	/**List of base64-encoded file hashes*/
	pub file_hashes: Vec<String>,
	/**Error message*/
	pub message: String,
	/**The section of the user's Telegram Passport which has the issue, one of “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
	One of: utility_bill, bank_statement, rental_agreement, passport_registration, temporary_registration*/
	pub r#type: String,
	/**Error source, must be *files*
	Default: files*/
	pub source: String,
}
impl PassportElementErrorFiles {
	pub fn new(file_hashes: impl Into<Vec<String>>, message: impl Into<String>, r#type: impl Into<String>, source: impl Into<String>) -> Self {
		Self {
			file_hashes: file_hashes.into(),
			message: message.into(),
			r#type: r#type.into(),
			source: source.into(),
		}
	}
	pub fn add_file_hash(mut self, file_hash: impl Into<String>) -> Self {
		self.file_hashes.push(file_hash.into());
		self
	}
}
/**Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.

https://core.telegram.org/bots/api/#passportelementerrorfrontside*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct PassportElementErrorFrontSide {
	/**Base64-encoded hash of the file with the front side of the document*/
	pub file_hash: String,
	/**Error message*/
	pub message: String,
	/**The section of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”
	One of: passport, driver_license, identity_card, internal_passport*/
	pub r#type: String,
	/**Error source, must be *front\_side*
	Default: front_side*/
	pub source: String,
}
impl PassportElementErrorFrontSide {
	pub fn new(file_hash: impl Into<String>, message: impl Into<String>, r#type: impl Into<String>, source: impl Into<String>) -> Self {
		Self {
			file_hash: file_hash.into(),
			message: message.into(),
			r#type: r#type.into(),
			source: source.into(),
		}
	}
}
/**Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.

https://core.telegram.org/bots/api/#passportelementerrorreverseside*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct PassportElementErrorReverseSide {
	/**Base64-encoded hash of the file with the reverse side of the document*/
	pub file_hash: String,
	/**Error message*/
	pub message: String,
	/**The section of the user's Telegram Passport which has the issue, one of “driver\_license”, “identity\_card”
	One of: driver_license, identity_card*/
	pub r#type: String,
	/**Error source, must be *reverse\_side*
	Default: reverse_side*/
	pub source: String,
}
impl PassportElementErrorReverseSide {
	pub fn new(file_hash: impl Into<String>, message: impl Into<String>, r#type: impl Into<String>, source: impl Into<String>) -> Self {
		Self {
			file_hash: file_hash.into(),
			message: message.into(),
			r#type: r#type.into(),
			source: source.into(),
		}
	}
}
/**Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.

https://core.telegram.org/bots/api/#passportelementerrorselfie*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct PassportElementErrorSelfie {
	/**Base64-encoded hash of the file with the selfie*/
	pub file_hash: String,
	/**Error message*/
	pub message: String,
	/**The section of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”
	One of: passport, driver_license, identity_card, internal_passport*/
	pub r#type: String,
	/**Error source, must be *selfie*
	Default: selfie*/
	pub source: String,
}
impl PassportElementErrorSelfie {
	pub fn new(file_hash: impl Into<String>, message: impl Into<String>, r#type: impl Into<String>, source: impl Into<String>) -> Self {
		Self {
			file_hash: file_hash.into(),
			message: message.into(),
			r#type: r#type.into(),
			source: source.into(),
		}
	}
}
/**Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.

https://core.telegram.org/bots/api/#passportelementerrortranslationfile*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct PassportElementErrorTranslationFile {
	/**Base64-encoded file hash*/
	pub file_hash: String,
	/**Error message*/
	pub message: String,
	/**Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
	One of: passport, driver_license, identity_card, internal_passport, utility_bill, bank_statement, rental_agreement, passport_registration, temporary_registration*/
	pub r#type: String,
	/**Error source, must be *translation\_file*
	Default: translation_file*/
	pub source: String,
}
impl PassportElementErrorTranslationFile {
	pub fn new(file_hash: impl Into<String>, message: impl Into<String>, r#type: impl Into<String>, source: impl Into<String>) -> Self {
		Self {
			file_hash: file_hash.into(),
			message: message.into(),
			r#type: r#type.into(),
			source: source.into(),
		}
	}
}
/**Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.

https://core.telegram.org/bots/api/#passportelementerrortranslationfiles*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct PassportElementErrorTranslationFiles {
	/**List of base64-encoded file hashes*/
	pub file_hashes: Vec<String>,
	/**Error message*/
	pub message: String,
	/**Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver\_license”, “identity\_card”, “internal\_passport”, “utility\_bill”, “bank\_statement”, “rental\_agreement”, “passport\_registration”, “temporary\_registration”
	One of: passport, driver_license, identity_card, internal_passport, utility_bill, bank_statement, rental_agreement, passport_registration, temporary_registration*/
	pub r#type: String,
	/**Error source, must be *translation\_files*
	Default: translation_files*/
	pub source: String,
}
impl PassportElementErrorTranslationFiles {
	pub fn new(file_hashes: impl Into<Vec<String>>, message: impl Into<String>, r#type: impl Into<String>, source: impl Into<String>) -> Self {
		Self {
			file_hashes: file_hashes.into(),
			message: message.into(),
			r#type: r#type.into(),
			source: source.into(),
		}
	}
	pub fn add_file_hash(mut self, file_hash: impl Into<String>) -> Self {
		self.file_hashes.push(file_hash.into());
		self
	}
}
/**Represents an issue in an unspecified place. The error is considered resolved when new data is added.

https://core.telegram.org/bots/api/#passportelementerrorunspecified*/
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct PassportElementErrorUnspecified {
	/**Base64-encoded element hash*/
	pub element_hash: String,
	/**Error message*/
	pub message: String,
	/**Type of element of the user's Telegram Passport which has the issue*/
	pub r#type: String,
	/**Error source, must be *unspecified*
	Default: unspecified*/
	pub source: String,
}
impl PassportElementErrorUnspecified {
	pub fn new(element_hash: impl Into<String>, message: impl Into<String>, r#type: impl Into<String>, source: impl Into<String>) -> Self {
		Self {
			element_hash: element_hash.into(),
			message: message.into(),
			r#type: r#type.into(),
			source: source.into(),
		}
	}
}
/**This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.

https://core.telegram.org/bots/api/#passportfile*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PassportFile {
	/**Unix time when the file was uploaded*/
	pub file_date: i64,
	/**Identifier for this file, which can be used to download or reuse the file*/
	pub file_id: String,
	/**File size in bytes*/
	pub file_size: i64,
	/**Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub file_unique_id: String,
}
impl PassportFile {
	pub fn new(file_date: impl Into<i64>, file_id: impl Into<String>, file_size: impl Into<i64>, file_unique_id: impl Into<String>) -> Self {
		Self {
			file_date: file_date.into(),
			file_id: file_id.into(),
			file_size: file_size.into(),
			file_unique_id: file_unique_id.into(),
		}
	}
}
/**This object represents one size of a photo or a [file](https://core.telegram.org/bots/api/#document) / [sticker](https://core.telegram.org/bots/api/#sticker) thumbnail.

https://core.telegram.org/bots/api/#photosize*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PhotoSize {
	/**Identifier for this file, which can be used to download or reuse the file*/
	pub file_id: String,
	/**File size in bytes*/
	pub file_size: Option<i64>,
	/**Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub file_unique_id: String,
	/**Photo height*/
	pub height: i64,
	/**Photo width*/
	pub width: i64,
}
impl PhotoSize {
	pub fn new(file_id: impl Into<String>, file_unique_id: impl Into<String>, height: impl Into<i64>, width: impl Into<i64>) -> Self {
		Self {
			file_id: file_id.into(),
			file_size: None,
			file_unique_id: file_unique_id.into(),
			height: height.into(),
			width: width.into(),
		}
	}
	/** *Optional*. File size in bytes*/
	pub fn file_size(mut self, file_size: impl Into<i64>) -> Self {
		self.file_size = Some(file_size.into());
		self
	}
}
/**This object contains information about a poll.

https://core.telegram.org/bots/api/#poll*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Poll {
	/**if the poll allows multiple answers*/
	pub allows_multiple_answers: bool,
	/**Point in time (Unix timestamp) when the poll will be automatically closed*/
	pub close_date: Option<i64>,
	/**0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.*/
	pub correct_option_id: Option<i64>,
	/**Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
	Min len: 0
	Max len: 200*/
	pub explanation: Option<String>,
	/**Special entities like usernames, URLs, bot commands, etc. that appear in the *explanation**/
	pub explanation_entities: Vec<MessageEntity>,
	/**Unique poll identifier*/
	pub id: String,
	/**if the poll is anonymous*/
	pub is_anonymous: bool,
	/**if the poll is closed*/
	pub is_closed: bool,
	/**Amount of time in seconds the poll will be active after creation*/
	pub open_period: Option<i64>,
	/**List of poll options*/
	pub options: Vec<PollOption>,
	/**Poll question, 1-300 characters
	Min len: 1
	Max len: 300*/
	pub question: String,
	/**Special entities that appear in the *question*. Currently, only custom emoji entities are allowed in poll questions*/
	pub question_entities: Vec<MessageEntity>,
	/**Poll type, currently can be “regular” or “quiz”
	One of: regular, quiz*/
	pub r#type: String,
	/**Total number of users that voted in the poll*/
	pub total_voter_count: i64,
}
impl Poll {
	pub fn new(allows_multiple_answers: bool, id: impl Into<String>, is_anonymous: bool, is_closed: bool, options: impl Into<Vec<PollOption>>, question: impl Into<String>, r#type: impl Into<String>, total_voter_count: impl Into<i64>) -> Self {
		Self {
			allows_multiple_answers: allows_multiple_answers,
			close_date: None,
			correct_option_id: None,
			explanation: None,
			explanation_entities: Vec::new(),
			id: id.into(),
			is_anonymous: is_anonymous,
			is_closed: is_closed,
			open_period: None,
			options: options.into(),
			question: question.into(),
			question_entities: Vec::new(),
			r#type: r#type.into(),
			total_voter_count: total_voter_count.into(),
		}
	}
	/** *Optional*. Point in time (Unix timestamp) when the poll will be automatically closed*/
	pub fn close_date(mut self, close_date: impl Into<i64>) -> Self {
		self.close_date = Some(close_date.into());
		self
	}
	/** *Optional*. 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.*/
	pub fn correct_option_id(mut self, correct_option_id: impl Into<i64>) -> Self {
		self.correct_option_id = Some(correct_option_id.into());
		self
	}
	/** *Optional*. Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
	Min len: 0
	Max len: 200*/
	pub fn explanation(mut self, explanation: impl Into<String>) -> Self {
		self.explanation = Some(explanation.into());
		self
	}
	pub fn add_explanation_entity(mut self, explanation_entity: impl Into<MessageEntity>) -> Self {
		self.explanation_entities.push(explanation_entity.into());
		self
	}
	/** *Optional*. Special entities like usernames, URLs, bot commands, etc. that appear in the *explanation**/
	pub fn explanation_entities(mut self, explanation_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.explanation_entities = explanation_entities.into();
		self
	}
	/** *Optional*. Amount of time in seconds the poll will be active after creation*/
	pub fn open_period(mut self, open_period: impl Into<i64>) -> Self {
		self.open_period = Some(open_period.into());
		self
	}
	pub fn add_option(mut self, option: impl Into<PollOption>) -> Self {
		self.options.push(option.into());
		self
	}
	pub fn add_question_entity(mut self, question_entity: impl Into<MessageEntity>) -> Self {
		self.question_entities.push(question_entity.into());
		self
	}
	/** *Optional*. Special entities that appear in the *question*. Currently, only custom emoji entities are allowed in poll questions*/
	pub fn question_entities(mut self, question_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.question_entities = question_entities.into();
		self
	}
}
/**This object represents an answer of a user in a non-anonymous poll.

https://core.telegram.org/bots/api/#pollanswer*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PollAnswer {
	/**0-based identifiers of chosen answer options. May be empty if the vote was retracted.*/
	pub option_ids: Vec<i64>,
	/**Unique poll identifier*/
	pub poll_id: String,
	/**The user that changed the answer to the poll, if the voter isn't anonymous*/
	pub user: Option<User>,
	/**The chat that changed the answer to the poll, if the voter is anonymous*/
	pub voter_chat: Option<Chat>,
}
impl PollAnswer {
	pub fn new(option_ids: impl Into<Vec<i64>>, poll_id: impl Into<String>) -> Self {
		Self {
			option_ids: option_ids.into(),
			poll_id: poll_id.into(),
			user: None,
			voter_chat: None,
		}
	}
	pub fn add_option_id(mut self, option_id: impl Into<i64>) -> Self {
		self.option_ids.push(option_id.into());
		self
	}
	/** *Optional*. The user that changed the answer to the poll, if the voter isn't anonymous*/
	pub fn user(mut self, user: impl Into<User>) -> Self {
		self.user = Some(user.into());
		self
	}
	/** *Optional*. The chat that changed the answer to the poll, if the voter is anonymous*/
	pub fn voter_chat(mut self, voter_chat: impl Into<Chat>) -> Self {
		self.voter_chat = Some(voter_chat.into());
		self
	}
}
/**This object contains information about one answer option in a poll.

https://core.telegram.org/bots/api/#polloption*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PollOption {
	/**Option text, 1-100 characters
	Min len: 1
	Max len: 100*/
	pub text: String,
	/**Special entities that appear in the option *text*. Currently, only custom emoji entities are allowed in poll option texts*/
	pub text_entities: Vec<MessageEntity>,
	/**Number of users that voted for this option*/
	pub voter_count: i64,
}
impl PollOption {
	pub fn new(text: impl Into<String>, voter_count: impl Into<i64>) -> Self {
		Self {
			text: text.into(),
			text_entities: Vec::new(),
			voter_count: voter_count.into(),
		}
	}
	pub fn add_text_entity(mut self, text_entity: impl Into<MessageEntity>) -> Self {
		self.text_entities.push(text_entity.into());
		self
	}
	/** *Optional*. Special entities that appear in the option *text*. Currently, only custom emoji entities are allowed in poll option texts*/
	pub fn text_entities(mut self, text_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.text_entities = text_entities.into();
		self
	}
}
/**This object contains information about an incoming pre-checkout query.

https://core.telegram.org/bots/api/#precheckoutquery*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PreCheckoutQuery {
	/**Three-letter ISO 4217 [currency](https://core.telegram.org/bots/payments#supported-currencies) code, or “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90)*/
	pub currency: String,
	/**User who sent the query*/
	pub from: User,
	/**Unique query identifier*/
	pub id: String,
	/**Bot-specified invoice payload*/
	pub invoice_payload: String,
	/**Order information provided by the user*/
	pub order_info: Option<OrderInfo>,
	/**Identifier of the shipping option chosen by the user*/
	pub shipping_option_id: Option<String>,
	/**Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).*/
	pub total_amount: i64,
}
impl PreCheckoutQuery {
	pub fn new(currency: impl Into<String>, from: impl Into<User>, id: impl Into<String>, invoice_payload: impl Into<String>, total_amount: impl Into<i64>) -> Self {
		Self {
			currency: currency.into(),
			from: from.into(),
			id: id.into(),
			invoice_payload: invoice_payload.into(),
			order_info: None,
			shipping_option_id: None,
			total_amount: total_amount.into(),
		}
	}
	/** *Optional*. Order information provided by the user*/
	pub fn order_info(mut self, order_info: impl Into<OrderInfo>) -> Self {
		self.order_info = Some(order_info.into());
		self
	}
	/** *Optional*. Identifier of the shipping option chosen by the user*/
	pub fn shipping_option_id(mut self, shipping_option_id: impl Into<String>) -> Self {
		self.shipping_option_id = Some(shipping_option_id.into());
		self
	}
}
/**Describes an inline message to be sent by a user of a Mini App.

https://core.telegram.org/bots/api/#preparedinlinemessage*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct PreparedInlineMessage {
	/**Expiration date of the prepared message, in Unix time. Expired prepared messages can no longer be used*/
	pub expiration_date: i64,
	/**Unique identifier of the prepared message*/
	pub id: String,
}
impl PreparedInlineMessage {
	pub fn new(expiration_date: impl Into<i64>, id: impl Into<String>) -> Self {
		Self {
			expiration_date: expiration_date.into(),
			id: id.into(),
		}
	}
}
/**This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.

https://core.telegram.org/bots/api/#proximityalerttriggered*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ProximityAlertTriggered {
	/**The distance between the users*/
	pub distance: i64,
	/**User that triggered the alert*/
	pub traveler: User,
	/**User that set the alert*/
	pub watcher: User,
}
impl ProximityAlertTriggered {
	pub fn new(distance: impl Into<i64>, traveler: impl Into<User>, watcher: impl Into<User>) -> Self {
		Self {
			distance: distance.into(),
			traveler: traveler.into(),
			watcher: watcher.into(),
		}
	}
}
/**Represents a reaction added to a message along with the number of times it was added.

https://core.telegram.org/bots/api/#reactioncount*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ReactionCount {
	/**Type of the reaction*/
	pub r#type: ReactionType,
	/**Number of times the reaction was added*/
	pub total_count: i64,
}
impl ReactionCount {
	pub fn new(r#type: impl Into<ReactionType>, total_count: impl Into<i64>) -> Self {
		Self {
			r#type: r#type.into(),
			total_count: total_count.into(),
		}
	}
}
/**This object describes the type of a reaction. Currently, it can be one of

* [ReactionTypeEmoji](https://core.telegram.org/bots/api/#reactiontypeemoji)
* [ReactionTypeCustomEmoji](https://core.telegram.org/bots/api/#reactiontypecustomemoji)
* [ReactionTypePaid](https://core.telegram.org/bots/api/#reactiontypepaid)

https://core.telegram.org/bots/api/#reactiontype*/
#[derive(Clone, Debug, Serialize, Deserialize, From)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ReactionType {
	CustomEmoji(ReactionTypeCustomEmoji),
	Emoji(ReactionTypeEmoji),
	Paid(ReactionTypePaid),
}
/**The reaction is based on a custom emoji.

https://core.telegram.org/bots/api/#reactiontypecustomemoji*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReactionTypeCustomEmoji {
	/**Custom emoji identifier*/
	pub custom_emoji_id: String,
	/**Type of the reaction, always “custom\_emoji”
	Default: custom_emoji*/
	pub r#type: String,
}
impl ReactionTypeCustomEmoji {
	pub fn new(custom_emoji_id: impl Into<String>, r#type: impl Into<String>) -> Self {
		Self {
			custom_emoji_id: custom_emoji_id.into(),
			r#type: r#type.into(),
		}
	}
}
/**The reaction is based on an emoji.

https://core.telegram.org/bots/api/#reactiontypeemoji*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReactionTypeEmoji {
	/**Reaction emoji. Currently, it can be one of "👍", "👎", "❤", "🔥", "🥰", "👏", "😁", "🤔", "🤯", "😱", "🤬", "😢", "🎉", "🤩", "🤮", "💩", "🙏", "👌", "🕊", "🤡", "🥱", "🥴", "😍", "🐳", "❤‍🔥", "🌚", "🌭", "💯", "🤣", "⚡", "🍌", "🏆", "💔", "🤨", "😐", "🍓", "🍾", "💋", "🖕", "😈", "😴", "😭", "🤓", "👻", "👨‍💻", "👀", "🎃", "🙈", "😇", "😨", "🤝", "✍", "🤗", "🫡", "🎅", "🎄", "☃", "💅", "🤪", "🗿", "🆒", "💘", "🙉", "🦄", "😘", "💊", "🙊", "😎", "👾", "🤷‍♂", "🤷", "🤷‍♀", "😡"
	One of: 👍, 👎, ❤, 🔥, 🥰, 👏, 😁, 🤔, 🤯, 😱, 🤬, 😢, 🎉, 🤩, 🤮, 💩, 🙏, 👌, 🕊, 🤡, 🥱, 🥴, 😍, 🐳, ❤‍🔥, 🌚, 🌭, 💯, 🤣, ⚡, 🍌, 🏆, 💔, 🤨, 😐, 🍓, 🍾, 💋, 🖕, 😈, 😴, 😭, 🤓, 👻, 👨‍💻, 👀, 🎃, 🙈, 😇, 😨, 🤝, ✍, 🤗, 🫡, 🎅, 🎄, ☃, 💅, 🤪, 🗿, 🆒, 💘, 🙉, 🦄, 😘, 💊, 🙊, 😎, 👾, 🤷‍♂, 🤷, 🤷‍♀, 😡*/
	pub emoji: String,
	/**Type of the reaction, always “emoji”
	Default: emoji*/
	pub r#type: String,
}
impl ReactionTypeEmoji {
	pub fn new(emoji: impl Into<String>, r#type: impl Into<String>) -> Self {
		Self {
			emoji: emoji.into(),
			r#type: r#type.into(),
		}
	}
}
/**The reaction is paid.

https://core.telegram.org/bots/api/#reactiontypepaid*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReactionTypePaid {
	/**Type of the reaction, always “paid”
	Default: paid*/
	pub r#type: String,
}
impl ReactionTypePaid {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**This object contains basic information about a refunded payment.

https://core.telegram.org/bots/api/#refundedpayment*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct RefundedPayment {
	/**Three-letter ISO 4217 [currency](https://core.telegram.org/bots/payments#supported-currencies) code, or “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90). Currently, always “XTR”
	Default: XTR*/
	pub currency: String,
	/**Bot-specified invoice payload*/
	pub invoice_payload: String,
	/**Provider payment identifier*/
	pub provider_payment_charge_id: Option<String>,
	/**Telegram payment identifier*/
	pub telegram_payment_charge_id: String,
	/**Total refunded price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45`, `total_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).*/
	pub total_amount: i64,
}
impl RefundedPayment {
	pub fn new(currency: impl Into<String>, invoice_payload: impl Into<String>, telegram_payment_charge_id: impl Into<String>, total_amount: impl Into<i64>) -> Self {
		Self {
			currency: currency.into(),
			invoice_payload: invoice_payload.into(),
			provider_payment_charge_id: None,
			telegram_payment_charge_id: telegram_payment_charge_id.into(),
			total_amount: total_amount.into(),
		}
	}
	/** *Optional*. Provider payment identifier*/
	pub fn provider_payment_charge_id(mut self, provider_payment_charge_id: impl Into<String>) -> Self {
		self.provider_payment_charge_id = Some(provider_payment_charge_id.into());
		self
	}
}
/**This object represents a [custom keyboard](https://core.telegram.org/bots/features#keyboards) with reply options (see [Introduction to bots](https://core.telegram.org/bots/features#keyboards) for details and examples). Not supported in channels and for messages sent on behalf of a Telegram Business account.

https://core.telegram.org/bots/api/#replykeyboardmarkup*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct ReplyKeyboardMarkup {
	/**The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
	Min len: 1
	Max len: 64*/
	pub input_field_placeholder: Option<String>,
	/**Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to *false*, in which case the custom keyboard can be hidden and opened with a keyboard icon.
	Default value: false*/
	pub is_persistent: Option<bool>,
	/**Array of button rows, each represented by an Array of [KeyboardButton](https://core.telegram.org/bots/api/#keyboardbutton) objects*/
	pub keyboard: Vec<Vec<KeyboardButton>>,
	/**Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to *false*.
	Default value: false*/
	pub one_time_keyboard: Option<bool>,
	/**Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to *false*, in which case the custom keyboard is always of the same height as the app's standard keyboard.
	Default value: false*/
	pub resize_keyboard: Option<bool>,
	/**Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the *text* of the [Message](https://core.telegram.org/bots/api/#message) object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.  

	*Example:* A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language. Other users in the group don't see the keyboard.*/
	pub selective: Option<bool>,
}
impl ReplyKeyboardMarkup {
	pub fn new(keyboard: impl Into<Vec<Vec<KeyboardButton>>>) -> Self {
		Self {
			input_field_placeholder: None,
			is_persistent: None,
			keyboard: keyboard.into(),
			one_time_keyboard: None,
			resize_keyboard: None,
			selective: None,
		}
	}
	/** *Optional*. The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
	Min len: 1
	Max len: 64*/
	pub fn input_field_placeholder(mut self, input_field_placeholder: impl Into<String>) -> Self {
		self.input_field_placeholder = Some(input_field_placeholder.into());
		self
	}
	/** *Optional*. Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to *false*, in which case the custom keyboard can be hidden and opened with a keyboard icon.
	Default value: false*/
	pub fn is_persistent(mut self, is_persistent: bool) -> Self {
		self.is_persistent = Some(is_persistent);
		self
	}
	pub fn add_keyboard(mut self, keyboard: impl Into<Vec<KeyboardButton>>) -> Self {
		self.keyboard.push(keyboard.into());
		self
	}
	/** *Optional*. Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to *false*.
	Default value: false*/
	pub fn one_time_keyboard(mut self, one_time_keyboard: bool) -> Self {
		self.one_time_keyboard = Some(one_time_keyboard);
		self
	}
	/** *Optional*. Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to *false*, in which case the custom keyboard is always of the same height as the app's standard keyboard.
	Default value: false*/
	pub fn resize_keyboard(mut self, resize_keyboard: bool) -> Self {
		self.resize_keyboard = Some(resize_keyboard);
		self
	}
	/** *Optional*. Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the *text* of the [Message](https://core.telegram.org/bots/api/#message) object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.  

	*Example:* A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language. Other users in the group don't see the keyboard.*/
	pub fn selective(mut self, selective: bool) -> Self {
		self.selective = Some(selective);
		self
	}
}
/**Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see [ReplyKeyboardMarkup](https://core.telegram.org/bots/api/#replykeyboardmarkup)). Not supported in channels and for messages sent on behalf of a Telegram Business account.

https://core.telegram.org/bots/api/#replykeyboardremove*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct ReplyKeyboardRemove {
	/**Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use *one\_time\_keyboard* in [ReplyKeyboardMarkup](https://core.telegram.org/bots/api/#replykeyboardmarkup))
	Default value: true*/
	pub remove_keyboard: bool,
	/**Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the *text* of the [Message](https://core.telegram.org/bots/api/#message) object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.  

	*Example:* A user votes in a poll, bot returns confirmation message in reply to the vote and removes the keyboard for that user, while still showing the keyboard with poll options to users who haven't voted yet.*/
	pub selective: Option<bool>,
}
impl ReplyKeyboardRemove {
	pub fn new(remove_keyboard: bool) -> Self {
		Self {
			remove_keyboard: remove_keyboard,
			selective: None,
		}
	}
	/** *Optional*. Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the *text* of the [Message](https://core.telegram.org/bots/api/#message) object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.  

	*Example:* A user votes in a poll, bot returns confirmation message in reply to the vote and removes the keyboard for that user, while still showing the keyboard with poll options to users who haven't voted yet.*/
	pub fn selective(mut self, selective: bool) -> Self {
		self.selective = Some(selective);
		self
	}
}
/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
#[derive(Clone, Debug, Serialize, From)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum ReplyMarkup {
	ForceReply(ForceReply),
	InlineKeyboardMarkup(InlineKeyboardMarkup),
	ReplyKeyboardMarkup(ReplyKeyboardMarkup),
	ReplyKeyboardRemove(ReplyKeyboardRemove),
}
/**Describes reply parameters for the message that is being sent.

https://core.telegram.org/bots/api/#replyparameters*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct ReplyParameters {
	/**Pass *True* if the message should be sent even if the specified message to be replied to is not found. Always *False* for replies in another chat or forum topic. Always *True* for messages sent on behalf of a business account.*/
	pub allow_sending_without_reply: Option<bool>,
	/**If the message to be replied to is from a different chat, unique identifier for the chat or username of the channel (in the format `@channelusername`). Not supported for messages sent on behalf of a business account.*/
	pub chat_id: Option<ChatId>,
	/**Identifier of the message that will be replied to in the current chat, or in the chat *chat\_id* if it is specified*/
	pub message_id: i64,
	/**Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom\_emoji* entities. The message will fail to send if the quote isn't found in the original message.
	Min len: 0
	Max len: 1024*/
	pub quote: Option<String>,
	/**A JSON-serialized list of special entities that appear in the quote. It can be specified instead of *quote\_parse\_mode*.*/
	pub quote_entities: Vec<MessageEntity>,
	/**Mode for parsing entities in the quote. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub quote_parse_mode: Option<String>,
	/**Position of the quote in the original message in UTF-16 code units*/
	pub quote_position: Option<i64>,
}
impl ReplyParameters {
	pub fn new(message_id: impl Into<i64>) -> Self {
		Self {
			allow_sending_without_reply: None,
			chat_id: None,
			message_id: message_id.into(),
			quote: None,
			quote_entities: Vec::new(),
			quote_parse_mode: None,
			quote_position: None,
		}
	}
	/** *Optional*. Pass *True* if the message should be sent even if the specified message to be replied to is not found. Always *False* for replies in another chat or forum topic. Always *True* for messages sent on behalf of a business account.*/
	pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
		self.allow_sending_without_reply = Some(allow_sending_without_reply);
		self
	}
	/** *Optional*. If the message to be replied to is from a different chat, unique identifier for the chat or username of the channel (in the format `@channelusername`). Not supported for messages sent on behalf of a business account.*/
	pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
	/** *Optional*. Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom\_emoji* entities. The message will fail to send if the quote isn't found in the original message.
	Min len: 0
	Max len: 1024*/
	pub fn quote(mut self, quote: impl Into<String>) -> Self {
		self.quote = Some(quote.into());
		self
	}
	pub fn add_quote_entity(mut self, quote_entity: impl Into<MessageEntity>) -> Self {
		self.quote_entities.push(quote_entity.into());
		self
	}
	/** *Optional*. A JSON-serialized list of special entities that appear in the quote. It can be specified instead of *quote\_parse\_mode*.*/
	pub fn quote_entities(mut self, quote_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.quote_entities = quote_entities.into();
		self
	}
	/** *Optional*. Mode for parsing entities in the quote. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn quote_parse_mode(mut self, quote_parse_mode: impl Into<String>) -> Self {
		self.quote_parse_mode = Some(quote_parse_mode.into());
		self
	}
	/** *Optional*. Position of the quote in the original message in UTF-16 code units*/
	pub fn quote_position(mut self, quote_position: impl Into<i64>) -> Self {
		self.quote_position = Some(quote_position.into());
		self
	}
}
/**Describes why a request was unsuccessful.

https://core.telegram.org/bots/api/#responseparameters*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct ResponseParameters {
	/**The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.*/
	pub migrate_to_chat_id: Option<i64>,
	/**In case of exceeding flood control, the number of seconds left to wait before the request can be repeated*/
	pub retry_after: Option<i64>,
}
impl ResponseParameters {
	pub fn new() -> Self {
		Self {
			migrate_to_chat_id: None,
			retry_after: None,
		}
	}
	/** *Optional*. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.*/
	pub fn migrate_to_chat_id(mut self, migrate_to_chat_id: impl Into<i64>) -> Self {
		self.migrate_to_chat_id = Some(migrate_to_chat_id.into());
		self
	}
	/** *Optional*. In case of exceeding flood control, the number of seconds left to wait before the request can be repeated*/
	pub fn retry_after(mut self, retry_after: impl Into<i64>) -> Self {
		self.retry_after = Some(retry_after.into());
		self
	}
}
/**This object describes the state of a revenue withdrawal operation. Currently, it can be one of

* [RevenueWithdrawalStatePending](https://core.telegram.org/bots/api/#revenuewithdrawalstatepending)
* [RevenueWithdrawalStateSucceeded](https://core.telegram.org/bots/api/#revenuewithdrawalstatesucceeded)
* [RevenueWithdrawalStateFailed](https://core.telegram.org/bots/api/#revenuewithdrawalstatefailed)

https://core.telegram.org/bots/api/#revenuewithdrawalstate*/
#[derive(Clone, Debug, Deserialize, From)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum RevenueWithdrawalState {
	Failed(RevenueWithdrawalStateFailed),
	Pending(RevenueWithdrawalStatePending),
	Succeeded(RevenueWithdrawalStateSucceeded),
}
/**The withdrawal failed and the transaction was refunded.

https://core.telegram.org/bots/api/#revenuewithdrawalstatefailed*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct RevenueWithdrawalStateFailed {
	/**Type of the state, always “failed”
	Default: failed*/
	pub r#type: String,
}
impl RevenueWithdrawalStateFailed {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**The withdrawal is in progress.

https://core.telegram.org/bots/api/#revenuewithdrawalstatepending*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct RevenueWithdrawalStatePending {
	/**Type of the state, always “pending”
	Default: pending*/
	pub r#type: String,
}
impl RevenueWithdrawalStatePending {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**The withdrawal succeeded.

https://core.telegram.org/bots/api/#revenuewithdrawalstatesucceeded*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct RevenueWithdrawalStateSucceeded {
	/**Date the withdrawal was completed in Unix time*/
	pub date: i64,
	/**Type of the state, always “succeeded”
	Default: succeeded*/
	pub r#type: String,
	/**An HTTPS URL that can be used to see transaction details*/
	pub url: String,
}
impl RevenueWithdrawalStateSucceeded {
	pub fn new(date: impl Into<i64>, r#type: impl Into<String>, url: impl Into<String>) -> Self {
		Self {
			date: date.into(),
			r#type: r#type.into(),
			url: url.into(),
		}
	}
}
/**Describes an inline message sent by a [Web App](https://core.telegram.org/bots/webapps) on behalf of a user.

https://core.telegram.org/bots/api/#sentwebappmessage*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct SentWebAppMessage {
	/**Identifier of the sent inline message. Available only if there is an [inline keyboard](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) attached to the message.*/
	pub inline_message_id: Option<String>,
}
impl SentWebAppMessage {
	pub fn new() -> Self {
		Self {
			inline_message_id: None,
		}
	}
	/** *Optional*. Identifier of the sent inline message. Available only if there is an [inline keyboard](https://core.telegram.org/bots/api/#inlinekeyboardmarkup) attached to the message.*/
	pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
		self.inline_message_id = Some(inline_message_id.into());
		self
	}
}
/**setGameScore return value*/
#[derive(Clone, Debug, Deserialize, From)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum SetGameScoreResult {
	Bool(bool),
	Message(Message),
}
/**This object contains information about a user that was shared with the bot using a [KeyboardButtonRequestUsers](https://core.telegram.org/bots/api/#keyboardbuttonrequestusers) button.

https://core.telegram.org/bots/api/#shareduser*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct SharedUser {
	/**First name of the user, if the name was requested by the bot*/
	pub first_name: Option<String>,
	/**Last name of the user, if the name was requested by the bot*/
	pub last_name: Option<String>,
	/**Available sizes of the chat photo, if the photo was requested by the bot*/
	pub photo: Vec<PhotoSize>,
	/**Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means.*/
	pub user_id: i64,
	/**Username of the user, if the username was requested by the bot*/
	pub username: Option<String>,
}
impl SharedUser {
	pub fn new(user_id: impl Into<i64>) -> Self {
		Self {
			first_name: None,
			last_name: None,
			photo: Vec::new(),
			user_id: user_id.into(),
			username: None,
		}
	}
	/** *Optional*. First name of the user, if the name was requested by the bot*/
	pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
		self.first_name = Some(first_name.into());
		self
	}
	/** *Optional*. Last name of the user, if the name was requested by the bot*/
	pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
		self.last_name = Some(last_name.into());
		self
	}
	pub fn add_photo(mut self, photo: impl Into<PhotoSize>) -> Self {
		self.photo.push(photo.into());
		self
	}
	/** *Optional*. Available sizes of the chat photo, if the photo was requested by the bot*/
	pub fn photo(mut self, photo: impl Into<Vec<PhotoSize>>) -> Self {
		self.photo = photo.into();
		self
	}
	/** *Optional*. Username of the user, if the username was requested by the bot*/
	pub fn username(mut self, username: impl Into<String>) -> Self {
		self.username = Some(username.into());
		self
	}
}
/**This object represents a shipping address.

https://core.telegram.org/bots/api/#shippingaddress*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ShippingAddress {
	/**City*/
	pub city: String,
	/**Two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code*/
	pub country_code: String,
	/**Address post code*/
	pub post_code: String,
	/**State, if applicable*/
	pub state: String,
	/**First line for the address*/
	pub street_line1: String,
	/**Second line for the address*/
	pub street_line2: String,
}
impl ShippingAddress {
	pub fn new(city: impl Into<String>, country_code: impl Into<String>, post_code: impl Into<String>, state: impl Into<String>, street_line1: impl Into<String>, street_line2: impl Into<String>) -> Self {
		Self {
			city: city.into(),
			country_code: country_code.into(),
			post_code: post_code.into(),
			state: state.into(),
			street_line1: street_line1.into(),
			street_line2: street_line2.into(),
		}
	}
}
/**This object represents one shipping option.

https://core.telegram.org/bots/api/#shippingoption*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "serde-all", serde(Deserialize))]
pub struct ShippingOption {
	/**Shipping option identifier*/
	pub id: String,
	/**List of price portions*/
	pub prices: Vec<LabeledPrice>,
	/**Option title*/
	pub title: String,
}
impl ShippingOption {
	pub fn new(id: impl Into<String>, prices: impl Into<Vec<LabeledPrice>>, title: impl Into<String>) -> Self {
		Self {
			id: id.into(),
			prices: prices.into(),
			title: title.into(),
		}
	}
	pub fn add_price(mut self, price: impl Into<LabeledPrice>) -> Self {
		self.prices.push(price.into());
		self
	}
}
/**This object contains information about an incoming shipping query.

https://core.telegram.org/bots/api/#shippingquery*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct ShippingQuery {
	/**User who sent the query*/
	pub from: User,
	/**Unique query identifier*/
	pub id: String,
	/**Bot-specified invoice payload*/
	pub invoice_payload: String,
	/**User specified shipping address*/
	pub shipping_address: ShippingAddress,
}
impl ShippingQuery {
	pub fn new(from: impl Into<User>, id: impl Into<String>, invoice_payload: impl Into<String>, shipping_address: impl Into<ShippingAddress>) -> Self {
		Self {
			from: from.into(),
			id: id.into(),
			invoice_payload: invoice_payload.into(),
			shipping_address: shipping_address.into(),
		}
	}
}
/**Describes a Telegram Star transaction. Note that if the buyer initiates a chargeback with the payment provider from whom they acquired Stars (e.g., Apple, Google) following this transaction, the refunded Stars will be deducted from the bot's balance. This is outside of Telegram's control.

https://core.telegram.org/bots/api/#startransaction*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct StarTransaction {
	/**Integer amount of Telegram Stars transferred by the transaction*/
	pub amount: i64,
	/**Date the transaction was created in Unix time*/
	pub date: i64,
	/**Unique identifier of the transaction. Coincides with the identifier of the original transaction for refund transactions. Coincides with *SuccessfulPayment.telegram\_payment\_charge\_id* for successful incoming payments from users.*/
	pub id: String,
	/**The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999*/
	pub nanostar_amount: Option<i64>,
	/**Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions*/
	pub receiver: Option<TransactionPartner>,
	/**Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions*/
	pub source: Option<TransactionPartner>,
}
impl StarTransaction {
	pub fn new(amount: impl Into<i64>, date: impl Into<i64>, id: impl Into<String>) -> Self {
		Self {
			amount: amount.into(),
			date: date.into(),
			id: id.into(),
			nanostar_amount: None,
			receiver: None,
			source: None,
		}
	}
	/** *Optional*. The number of 1/1000000000 shares of Telegram Stars transferred by the transaction; from 0 to 999999999*/
	pub fn nanostar_amount(mut self, nanostar_amount: impl Into<i64>) -> Self {
		self.nanostar_amount = Some(nanostar_amount.into());
		self
	}
	/** *Optional*. Receiver of an outgoing transaction (e.g., a user for a purchase refund, Fragment for a withdrawal). Only for outgoing transactions*/
	pub fn receiver(mut self, receiver: impl Into<TransactionPartner>) -> Self {
		self.receiver = Some(receiver.into());
		self
	}
	/** *Optional*. Source of an incoming transaction (e.g., a user purchasing goods or services, Fragment refunding a failed withdrawal). Only for incoming transactions*/
	pub fn source(mut self, source: impl Into<TransactionPartner>) -> Self {
		self.source = Some(source.into());
		self
	}
}
/**Contains a list of Telegram Star transactions.

https://core.telegram.org/bots/api/#startransactions*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct StarTransactions {
	/**The list of transactions*/
	pub transactions: Vec<StarTransaction>,
}
impl StarTransactions {
	pub fn new(transactions: impl Into<Vec<StarTransaction>>) -> Self {
		Self {
			transactions: transactions.into(),
		}
	}
	pub fn add_transaction(mut self, transaction: impl Into<StarTransaction>) -> Self {
		self.transactions.push(transaction.into());
		self
	}
}
/**This object represents a sticker.

https://core.telegram.org/bots/api/#sticker*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Sticker {
	/**For custom emoji stickers, unique identifier of the custom emoji*/
	pub custom_emoji_id: Option<String>,
	/**Emoji associated with the sticker*/
	pub emoji: Option<String>,
	/**Identifier for this file, which can be used to download or reuse the file*/
	pub file_id: String,
	/**File size in bytes*/
	pub file_size: Option<i64>,
	/**Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub file_unique_id: String,
	/**Sticker height*/
	pub height: i64,
	/**if the sticker is [animated](https://telegram.org/blog/animated-stickers)*/
	pub is_animated: bool,
	/**if the sticker is a [video sticker](https://telegram.org/blog/video-stickers-better-reactions)*/
	pub is_video: bool,
	/**For mask stickers, the position where the mask should be placed*/
	pub mask_position: Option<MaskPosition>,
	/**if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
	Default value: true*/
	pub needs_repainting: Option<bool>,
	/**For premium regular stickers, premium animation for the sticker*/
	pub premium_animation: Option<File>,
	/**Type of the sticker, currently one of “regular”, “mask”, “custom\_emoji”. The type of the sticker is independent from its format, which is determined by the fields *is\_animated* and *is\_video*.
	One of: regular, mask, custom_emoji*/
	pub r#type: String,
	/**Name of the sticker set to which the sticker belongs*/
	pub set_name: Option<String>,
	/**Sticker thumbnail in the .WEBP or .JPG format*/
	pub thumbnail: Option<PhotoSize>,
	/**Sticker width*/
	pub width: i64,
}
impl Sticker {
	pub fn new(file_id: impl Into<String>, file_unique_id: impl Into<String>, height: impl Into<i64>, is_animated: bool, is_video: bool, r#type: impl Into<String>, width: impl Into<i64>) -> Self {
		Self {
			custom_emoji_id: None,
			emoji: None,
			file_id: file_id.into(),
			file_size: None,
			file_unique_id: file_unique_id.into(),
			height: height.into(),
			is_animated: is_animated,
			is_video: is_video,
			mask_position: None,
			needs_repainting: None,
			premium_animation: None,
			r#type: r#type.into(),
			set_name: None,
			thumbnail: None,
			width: width.into(),
		}
	}
	/** *Optional*. For custom emoji stickers, unique identifier of the custom emoji*/
	pub fn custom_emoji_id(mut self, custom_emoji_id: impl Into<String>) -> Self {
		self.custom_emoji_id = Some(custom_emoji_id.into());
		self
	}
	/** *Optional*. Emoji associated with the sticker*/
	pub fn emoji(mut self, emoji: impl Into<String>) -> Self {
		self.emoji = Some(emoji.into());
		self
	}
	/** *Optional*. File size in bytes*/
	pub fn file_size(mut self, file_size: impl Into<i64>) -> Self {
		self.file_size = Some(file_size.into());
		self
	}
	/** *Optional*. For mask stickers, the position where the mask should be placed*/
	pub fn mask_position(mut self, mask_position: impl Into<MaskPosition>) -> Self {
		self.mask_position = Some(mask_position.into());
		self
	}
	/** *Optional*. *True*, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
	Default value: true*/
	pub fn needs_repainting(mut self, needs_repainting: bool) -> Self {
		self.needs_repainting = Some(needs_repainting);
		self
	}
	/** *Optional*. For premium regular stickers, premium animation for the sticker*/
	pub fn premium_animation(mut self, premium_animation: impl Into<File>) -> Self {
		self.premium_animation = Some(premium_animation.into());
		self
	}
	/** *Optional*. Name of the sticker set to which the sticker belongs*/
	pub fn set_name(mut self, set_name: impl Into<String>) -> Self {
		self.set_name = Some(set_name.into());
		self
	}
	/** *Optional*. Sticker thumbnail in the .WEBP or .JPG format*/
	pub fn thumbnail(mut self, thumbnail: impl Into<PhotoSize>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
}
/**This object represents a sticker set.

https://core.telegram.org/bots/api/#stickerset*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct StickerSet {
	/**Sticker set name*/
	pub name: String,
	/**Type of stickers in the set, currently one of “regular”, “mask”, “custom\_emoji”
	One of: regular, mask, custom_emoji*/
	pub sticker_type: String,
	/**List of all set stickers*/
	pub stickers: Vec<Sticker>,
	/**Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format*/
	pub thumbnail: Option<PhotoSize>,
	/**Sticker set title*/
	pub title: String,
}
impl StickerSet {
	pub fn new(name: impl Into<String>, sticker_type: impl Into<String>, stickers: impl Into<Vec<Sticker>>, title: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			sticker_type: sticker_type.into(),
			stickers: stickers.into(),
			thumbnail: None,
			title: title.into(),
		}
	}
	pub fn add_sticker(mut self, sticker: impl Into<Sticker>) -> Self {
		self.stickers.push(sticker.into());
		self
	}
	/** *Optional*. Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format*/
	pub fn thumbnail(mut self, thumbnail: impl Into<PhotoSize>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
}
/**stopMessageLiveLocation return value*/
#[derive(Clone, Debug, Deserialize, From)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum StopMessageLiveLocationResult {
	Bool(bool),
	Message(Message),
}
/**This object represents a story.

https://core.telegram.org/bots/api/#story*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Story {
	/**Chat that posted the story*/
	pub chat: Chat,
	/**Unique identifier for the story in the chat*/
	pub id: i64,
}
impl Story {
	pub fn new(chat: impl Into<Chat>, id: impl Into<i64>) -> Self {
		Self {
			chat: chat.into(),
			id: id.into(),
		}
	}
}
/**This object contains basic information about a successful payment. Note that if the buyer initiates a chargeback with the relevant payment provider following this transaction, the funds may be debited from your balance. This is outside of Telegram's control.

https://core.telegram.org/bots/api/#successfulpayment*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct SuccessfulPayment {
	/**Three-letter ISO 4217 [currency](https://core.telegram.org/bots/payments#supported-currencies) code, or “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90)*/
	pub currency: String,
	/**Bot-specified invoice payload*/
	pub invoice_payload: String,
	/**True, if the payment is the first payment for a subscription
	Default value: true*/
	pub is_first_recurring: Option<bool>,
	/**True, if the payment is a recurring payment for a subscription
	Default value: true*/
	pub is_recurring: Option<bool>,
	/**Order information provided by the user*/
	pub order_info: Option<OrderInfo>,
	/**Provider payment identifier*/
	pub provider_payment_charge_id: String,
	/**Identifier of the shipping option chosen by the user*/
	pub shipping_option_id: Option<String>,
	/**Expiration date of the subscription, in Unix time; for recurring payments only*/
	pub subscription_expiration_date: Option<i64>,
	/**Telegram payment identifier*/
	pub telegram_payment_charge_id: String,
	/**Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).*/
	pub total_amount: i64,
}
impl SuccessfulPayment {
	pub fn new(currency: impl Into<String>, invoice_payload: impl Into<String>, provider_payment_charge_id: impl Into<String>, telegram_payment_charge_id: impl Into<String>, total_amount: impl Into<i64>) -> Self {
		Self {
			currency: currency.into(),
			invoice_payload: invoice_payload.into(),
			is_first_recurring: None,
			is_recurring: None,
			order_info: None,
			provider_payment_charge_id: provider_payment_charge_id.into(),
			shipping_option_id: None,
			subscription_expiration_date: None,
			telegram_payment_charge_id: telegram_payment_charge_id.into(),
			total_amount: total_amount.into(),
		}
	}
	/** *Optional*. True, if the payment is the first payment for a subscription
	Default value: true*/
	pub fn is_first_recurring(mut self, is_first_recurring: bool) -> Self {
		self.is_first_recurring = Some(is_first_recurring);
		self
	}
	/** *Optional*. True, if the payment is a recurring payment for a subscription
	Default value: true*/
	pub fn is_recurring(mut self, is_recurring: bool) -> Self {
		self.is_recurring = Some(is_recurring);
		self
	}
	/** *Optional*. Order information provided by the user*/
	pub fn order_info(mut self, order_info: impl Into<OrderInfo>) -> Self {
		self.order_info = Some(order_info.into());
		self
	}
	/** *Optional*. Identifier of the shipping option chosen by the user*/
	pub fn shipping_option_id(mut self, shipping_option_id: impl Into<String>) -> Self {
		self.shipping_option_id = Some(shipping_option_id.into());
		self
	}
	/** *Optional*. Expiration date of the subscription, in Unix time; for recurring payments only*/
	pub fn subscription_expiration_date(mut self, subscription_expiration_date: impl Into<i64>) -> Self {
		self.subscription_expiration_date = Some(subscription_expiration_date.into());
		self
	}
}
/**This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.

https://core.telegram.org/bots/api/#switchinlinequerychosenchat*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwitchInlineQueryChosenChat {
	/**True, if private chats with bots can be chosen*/
	pub allow_bot_chats: Option<bool>,
	/**True, if channel chats can be chosen*/
	pub allow_channel_chats: Option<bool>,
	/**True, if group and supergroup chats can be chosen*/
	pub allow_group_chats: Option<bool>,
	/**True, if private chats with users can be chosen*/
	pub allow_user_chats: Option<bool>,
	/**The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted*/
	pub query: Option<String>,
}
impl SwitchInlineQueryChosenChat {
	pub fn new() -> Self {
		Self {
			allow_bot_chats: None,
			allow_channel_chats: None,
			allow_group_chats: None,
			allow_user_chats: None,
			query: None,
		}
	}
	/** *Optional*. True, if private chats with bots can be chosen*/
	pub fn allow_bot_chats(mut self, allow_bot_chats: bool) -> Self {
		self.allow_bot_chats = Some(allow_bot_chats);
		self
	}
	/** *Optional*. True, if channel chats can be chosen*/
	pub fn allow_channel_chats(mut self, allow_channel_chats: bool) -> Self {
		self.allow_channel_chats = Some(allow_channel_chats);
		self
	}
	/** *Optional*. True, if group and supergroup chats can be chosen*/
	pub fn allow_group_chats(mut self, allow_group_chats: bool) -> Self {
		self.allow_group_chats = Some(allow_group_chats);
		self
	}
	/** *Optional*. True, if private chats with users can be chosen*/
	pub fn allow_user_chats(mut self, allow_user_chats: bool) -> Self {
		self.allow_user_chats = Some(allow_user_chats);
		self
	}
	/** *Optional*. The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted*/
	pub fn query(mut self, query: impl Into<String>) -> Self {
		self.query = Some(query.into());
		self
	}
}
/**This object contains information about the quoted part of a message that is replied to by the given message.

https://core.telegram.org/bots/api/#textquote*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct TextQuote {
	/**Special entities that appear in the quote. Currently, only *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom\_emoji* entities are kept in quotes.*/
	pub entities: Vec<MessageEntity>,
	/**True, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
	Default value: true*/
	pub is_manual: Option<bool>,
	/**Approximate quote position in the original message in UTF-16 code units as specified by the sender*/
	pub position: i64,
	/**Text of the quoted part of a message that is replied to by the given message*/
	pub text: String,
}
impl TextQuote {
	pub fn new(position: impl Into<i64>, text: impl Into<String>) -> Self {
		Self {
			entities: Vec::new(),
			is_manual: None,
			position: position.into(),
			text: text.into(),
		}
	}
	pub fn add_entity(mut self, entity: impl Into<MessageEntity>) -> Self {
		self.entities.push(entity.into());
		self
	}
	/** *Optional*. Special entities that appear in the quote. Currently, only *bold*, *italic*, *underline*, *strikethrough*, *spoiler*, and *custom\_emoji* entities are kept in quotes.*/
	pub fn entities(mut self, entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.entities = entities.into();
		self
	}
	/** *Optional*. True, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
	Default value: true*/
	pub fn is_manual(mut self, is_manual: bool) -> Self {
		self.is_manual = Some(is_manual);
		self
	}
}
/**This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of

* [TransactionPartnerUser](https://core.telegram.org/bots/api/#transactionpartneruser)
* [TransactionPartnerChat](https://core.telegram.org/bots/api/#transactionpartnerchat)
* [TransactionPartnerAffiliateProgram](https://core.telegram.org/bots/api/#transactionpartneraffiliateprogram)
* [TransactionPartnerFragment](https://core.telegram.org/bots/api/#transactionpartnerfragment)
* [TransactionPartnerTelegramAds](https://core.telegram.org/bots/api/#transactionpartnertelegramads)
* [TransactionPartnerTelegramApi](https://core.telegram.org/bots/api/#transactionpartnertelegramapi)
* [TransactionPartnerOther](https://core.telegram.org/bots/api/#transactionpartnerother)

https://core.telegram.org/bots/api/#transactionpartner*/
#[derive(Clone, Debug, Deserialize, From)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum TransactionPartner {
	AffiliateProgram(TransactionPartnerAffiliateProgram),
	Chat(TransactionPartnerChat),
	Fragment(TransactionPartnerFragment),
	Other(TransactionPartnerOther),
	TelegramAds(TransactionPartnerTelegramAds),
	TelegramApi(TransactionPartnerTelegramApi),
	User(TransactionPartnerUser),
}
/**Describes the affiliate program that issued the affiliate commission received via this transaction.

https://core.telegram.org/bots/api/#transactionpartneraffiliateprogram*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct TransactionPartnerAffiliateProgram {
	/**The number of Telegram Stars received by the bot for each 1000 Telegram Stars received by the affiliate program sponsor from referred users*/
	pub commission_per_mille: i64,
	/**Type of the transaction partner, always “affiliate\_program”
	Default: affiliate_program*/
	pub r#type: String,
	/**Information about the bot that sponsored the affiliate program*/
	pub sponsor_user: Option<User>,
}
impl TransactionPartnerAffiliateProgram {
	pub fn new(commission_per_mille: impl Into<i64>, r#type: impl Into<String>) -> Self {
		Self {
			commission_per_mille: commission_per_mille.into(),
			r#type: r#type.into(),
			sponsor_user: None,
		}
	}
	/** *Optional*. Information about the bot that sponsored the affiliate program*/
	pub fn sponsor_user(mut self, sponsor_user: impl Into<User>) -> Self {
		self.sponsor_user = Some(sponsor_user.into());
		self
	}
}
/**Describes a transaction with a chat.

https://core.telegram.org/bots/api/#transactionpartnerchat*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct TransactionPartnerChat {
	/**Information about the chat*/
	pub chat: Chat,
	/**The gift sent to the chat by the bot*/
	pub gift: Option<Gift>,
	/**Type of the transaction partner, always “chat”
	Default: chat*/
	pub r#type: String,
}
impl TransactionPartnerChat {
	pub fn new(chat: impl Into<Chat>, r#type: impl Into<String>) -> Self {
		Self {
			chat: chat.into(),
			gift: None,
			r#type: r#type.into(),
		}
	}
	/** *Optional*. The gift sent to the chat by the bot*/
	pub fn gift(mut self, gift: impl Into<Gift>) -> Self {
		self.gift = Some(gift.into());
		self
	}
}
/**Describes a withdrawal transaction with Fragment.

https://core.telegram.org/bots/api/#transactionpartnerfragment*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct TransactionPartnerFragment {
	/**Type of the transaction partner, always “fragment”
	Default: fragment*/
	pub r#type: String,
	/**State of the transaction if the transaction is outgoing*/
	pub withdrawal_state: Option<RevenueWithdrawalState>,
}
impl TransactionPartnerFragment {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
			withdrawal_state: None,
		}
	}
	/** *Optional*. State of the transaction if the transaction is outgoing*/
	pub fn withdrawal_state(mut self, withdrawal_state: impl Into<RevenueWithdrawalState>) -> Self {
		self.withdrawal_state = Some(withdrawal_state.into());
		self
	}
}
/**Describes a transaction with an unknown source or recipient.

https://core.telegram.org/bots/api/#transactionpartnerother*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct TransactionPartnerOther {
	/**Type of the transaction partner, always “other”
	Default: other*/
	pub r#type: String,
}
impl TransactionPartnerOther {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**Describes a withdrawal transaction to the Telegram Ads platform.

https://core.telegram.org/bots/api/#transactionpartnertelegramads*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct TransactionPartnerTelegramAds {
	/**Type of the transaction partner, always “telegram\_ads”
	Default: telegram_ads*/
	pub r#type: String,
}
impl TransactionPartnerTelegramAds {
	pub fn new(r#type: impl Into<String>) -> Self {
		Self {
			r#type: r#type.into(),
		}
	}
}
/**Describes a transaction with payment for [paid broadcasting](https://core.telegram.org/bots/api/#paid-broadcasts).

https://core.telegram.org/bots/api/#transactionpartnertelegramapi*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct TransactionPartnerTelegramApi {
	/**Type of the transaction partner, always “telegram\_api”
	Default: telegram_api*/
	pub r#type: String,
	/**The number of successful requests that exceeded regular limits and were therefore billed*/
	pub request_count: i64,
}
impl TransactionPartnerTelegramApi {
	pub fn new(r#type: impl Into<String>, request_count: impl Into<i64>) -> Self {
		Self {
			r#type: r#type.into(),
			request_count: request_count.into(),
		}
	}
}
/**Describes a transaction with a user.

https://core.telegram.org/bots/api/#transactionpartneruser*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct TransactionPartnerUser {
	/**Information about the affiliate that received a commission via this transaction*/
	pub affiliate: Option<AffiliateInfo>,
	/**The gift sent to the user by the bot*/
	pub gift: Option<Gift>,
	/**Bot-specified invoice payload*/
	pub invoice_payload: Option<String>,
	/**Information about the paid media bought by the user*/
	pub paid_media: Vec<PaidMedia>,
	/**Bot-specified paid media payload*/
	pub paid_media_payload: Option<String>,
	/**Type of the transaction partner, always “user”
	Default: user*/
	pub r#type: String,
	/**The duration of the paid subscription*/
	pub subscription_period: Option<i64>,
	/**Information about the user*/
	pub user: User,
}
impl TransactionPartnerUser {
	pub fn new(r#type: impl Into<String>, user: impl Into<User>) -> Self {
		Self {
			affiliate: None,
			gift: None,
			invoice_payload: None,
			paid_media: Vec::new(),
			paid_media_payload: None,
			r#type: r#type.into(),
			subscription_period: None,
			user: user.into(),
		}
	}
	/** *Optional*. Information about the affiliate that received a commission via this transaction*/
	pub fn affiliate(mut self, affiliate: impl Into<AffiliateInfo>) -> Self {
		self.affiliate = Some(affiliate.into());
		self
	}
	/** *Optional*. The gift sent to the user by the bot*/
	pub fn gift(mut self, gift: impl Into<Gift>) -> Self {
		self.gift = Some(gift.into());
		self
	}
	/** *Optional*. Bot-specified invoice payload*/
	pub fn invoice_payload(mut self, invoice_payload: impl Into<String>) -> Self {
		self.invoice_payload = Some(invoice_payload.into());
		self
	}
	pub fn add_paid_media(mut self, paid_media: impl Into<PaidMedia>) -> Self {
		self.paid_media.push(paid_media.into());
		self
	}
	/** *Optional*. Information about the paid media bought by the user*/
	pub fn paid_media(mut self, paid_media: impl Into<Vec<PaidMedia>>) -> Self {
		self.paid_media = paid_media.into();
		self
	}
	/** *Optional*. Bot-specified paid media payload*/
	pub fn paid_media_payload(mut self, paid_media_payload: impl Into<String>) -> Self {
		self.paid_media_payload = Some(paid_media_payload.into());
		self
	}
	/** *Optional*. The duration of the paid subscription*/
	pub fn subscription_period(mut self, subscription_period: impl Into<i64>) -> Self {
		self.subscription_period = Some(subscription_period.into());
		self
	}
}
/**This [object](https://core.telegram.org/bots/api/#available-types) represents an incoming update.  
At most **one** of the optional parameters can be present in any given update.

https://core.telegram.org/bots/api/#update*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Update {
	/**The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot*/
	pub business_connection: Option<BusinessConnection>,
	/**New message from a connected business account*/
	pub business_message: Option<Message>,
	/**New incoming callback query*/
	pub callback_query: Option<CallbackQuery>,
	/**New incoming channel post of any kind - text, photo, sticker, etc.*/
	pub channel_post: Option<Message>,
	/**A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.*/
	pub chat_boost: Option<ChatBoostUpdated>,
	/**A request to join the chat has been sent. The bot must have the *can\_invite\_users* administrator right in the chat to receive these updates.*/
	pub chat_join_request: Option<ChatJoinRequest>,
	/**A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify `"chat_member"` in the list of *allowed\_updates* to receive these updates.*/
	pub chat_member: Option<ChatMemberUpdated>,
	/**The result of an [inline](https://core.telegram.org/bots/api/#inline-mode) query that was chosen by a user and sent to their chat partner. Please see our documentation on the [feedback collecting](https://core.telegram.org/bots/inline#collecting-feedback) for details on how to enable these updates for your bot.*/
	pub chosen_inline_result: Option<ChosenInlineResult>,
	/**Messages were deleted from a connected business account*/
	pub deleted_business_messages: Option<BusinessMessagesDeleted>,
	/**New version of a message from a connected business account*/
	pub edited_business_message: Option<Message>,
	/**New version of a channel post that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.*/
	pub edited_channel_post: Option<Message>,
	/**New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.*/
	pub edited_message: Option<Message>,
	/**New incoming [inline](https://core.telegram.org/bots/api/#inline-mode) query*/
	pub inline_query: Option<InlineQuery>,
	/**New incoming message of any kind - text, photo, sticker, etc.*/
	pub message: Option<Message>,
	/**A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify `"message_reaction"` in the list of *allowed\_updates* to receive these updates. The update isn't received for reactions set by bots.*/
	pub message_reaction: Option<MessageReactionUpdated>,
	/**Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify `"message_reaction_count"` in the list of *allowed\_updates* to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.*/
	pub message_reaction_count: Option<MessageReactionCountUpdated>,
	/**The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.*/
	pub my_chat_member: Option<ChatMemberUpdated>,
	/**New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot*/
	pub poll: Option<Poll>,
	/**A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.*/
	pub poll_answer: Option<PollAnswer>,
	/**New incoming pre-checkout query. Contains full information about checkout*/
	pub pre_checkout_query: Option<PreCheckoutQuery>,
	/**A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat*/
	pub purchased_paid_media: Option<PaidMediaPurchased>,
	/**A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.*/
	pub removed_chat_boost: Option<ChatBoostRemoved>,
	/**New incoming shipping query. Only for invoices with flexible price*/
	pub shipping_query: Option<ShippingQuery>,
	/**The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using [webhooks](https://core.telegram.org/bots/api/#setwebhook), since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.*/
	pub update_id: i64,
}
impl Update {
	pub fn new(update_id: impl Into<i64>) -> Self {
		Self {
			business_connection: None,
			business_message: None,
			callback_query: None,
			channel_post: None,
			chat_boost: None,
			chat_join_request: None,
			chat_member: None,
			chosen_inline_result: None,
			deleted_business_messages: None,
			edited_business_message: None,
			edited_channel_post: None,
			edited_message: None,
			inline_query: None,
			message: None,
			message_reaction: None,
			message_reaction_count: None,
			my_chat_member: None,
			poll: None,
			poll_answer: None,
			pre_checkout_query: None,
			purchased_paid_media: None,
			removed_chat_boost: None,
			shipping_query: None,
			update_id: update_id.into(),
		}
	}
	/** *Optional*. The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot*/
	pub fn business_connection(mut self, business_connection: impl Into<BusinessConnection>) -> Self {
		self.business_connection = Some(business_connection.into());
		self
	}
	/** *Optional*. New message from a connected business account*/
	pub fn business_message(mut self, business_message: impl Into<Message>) -> Self {
		self.business_message = Some(business_message.into());
		self
	}
	/** *Optional*. New incoming callback query*/
	pub fn callback_query(mut self, callback_query: impl Into<CallbackQuery>) -> Self {
		self.callback_query = Some(callback_query.into());
		self
	}
	/** *Optional*. New incoming channel post of any kind - text, photo, sticker, etc.*/
	pub fn channel_post(mut self, channel_post: impl Into<Message>) -> Self {
		self.channel_post = Some(channel_post.into());
		self
	}
	/** *Optional*. A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.*/
	pub fn chat_boost(mut self, chat_boost: impl Into<ChatBoostUpdated>) -> Self {
		self.chat_boost = Some(chat_boost.into());
		self
	}
	/** *Optional*. A request to join the chat has been sent. The bot must have the *can\_invite\_users* administrator right in the chat to receive these updates.*/
	pub fn chat_join_request(mut self, chat_join_request: impl Into<ChatJoinRequest>) -> Self {
		self.chat_join_request = Some(chat_join_request.into());
		self
	}
	/** *Optional*. A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify `"chat_member"` in the list of *allowed\_updates* to receive these updates.*/
	pub fn chat_member(mut self, chat_member: impl Into<ChatMemberUpdated>) -> Self {
		self.chat_member = Some(chat_member.into());
		self
	}
	/** *Optional*. The result of an [inline](https://core.telegram.org/bots/api/#inline-mode) query that was chosen by a user and sent to their chat partner. Please see our documentation on the [feedback collecting](https://core.telegram.org/bots/inline#collecting-feedback) for details on how to enable these updates for your bot.*/
	pub fn chosen_inline_result(mut self, chosen_inline_result: impl Into<ChosenInlineResult>) -> Self {
		self.chosen_inline_result = Some(chosen_inline_result.into());
		self
	}
	/** *Optional*. Messages were deleted from a connected business account*/
	pub fn deleted_business_messages(mut self, deleted_business_messages: impl Into<BusinessMessagesDeleted>) -> Self {
		self.deleted_business_messages = Some(deleted_business_messages.into());
		self
	}
	/** *Optional*. New version of a message from a connected business account*/
	pub fn edited_business_message(mut self, edited_business_message: impl Into<Message>) -> Self {
		self.edited_business_message = Some(edited_business_message.into());
		self
	}
	/** *Optional*. New version of a channel post that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.*/
	pub fn edited_channel_post(mut self, edited_channel_post: impl Into<Message>) -> Self {
		self.edited_channel_post = Some(edited_channel_post.into());
		self
	}
	/** *Optional*. New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.*/
	pub fn edited_message(mut self, edited_message: impl Into<Message>) -> Self {
		self.edited_message = Some(edited_message.into());
		self
	}
	/** *Optional*. New incoming [inline](https://core.telegram.org/bots/api/#inline-mode) query*/
	pub fn inline_query(mut self, inline_query: impl Into<InlineQuery>) -> Self {
		self.inline_query = Some(inline_query.into());
		self
	}
	/** *Optional*. New incoming message of any kind - text, photo, sticker, etc.*/
	pub fn message(mut self, message: impl Into<Message>) -> Self {
		self.message = Some(message.into());
		self
	}
	/** *Optional*. A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify `"message_reaction"` in the list of *allowed\_updates* to receive these updates. The update isn't received for reactions set by bots.*/
	pub fn message_reaction(mut self, message_reaction: impl Into<MessageReactionUpdated>) -> Self {
		self.message_reaction = Some(message_reaction.into());
		self
	}
	/** *Optional*. Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify `"message_reaction_count"` in the list of *allowed\_updates* to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.*/
	pub fn message_reaction_count(mut self, message_reaction_count: impl Into<MessageReactionCountUpdated>) -> Self {
		self.message_reaction_count = Some(message_reaction_count.into());
		self
	}
	/** *Optional*. The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.*/
	pub fn my_chat_member(mut self, my_chat_member: impl Into<ChatMemberUpdated>) -> Self {
		self.my_chat_member = Some(my_chat_member.into());
		self
	}
	/** *Optional*. New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot*/
	pub fn poll(mut self, poll: impl Into<Poll>) -> Self {
		self.poll = Some(poll.into());
		self
	}
	/** *Optional*. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.*/
	pub fn poll_answer(mut self, poll_answer: impl Into<PollAnswer>) -> Self {
		self.poll_answer = Some(poll_answer.into());
		self
	}
	/** *Optional*. New incoming pre-checkout query. Contains full information about checkout*/
	pub fn pre_checkout_query(mut self, pre_checkout_query: impl Into<PreCheckoutQuery>) -> Self {
		self.pre_checkout_query = Some(pre_checkout_query.into());
		self
	}
	/** *Optional*. A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat*/
	pub fn purchased_paid_media(mut self, purchased_paid_media: impl Into<PaidMediaPurchased>) -> Self {
		self.purchased_paid_media = Some(purchased_paid_media.into());
		self
	}
	/** *Optional*. A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.*/
	pub fn removed_chat_boost(mut self, removed_chat_boost: impl Into<ChatBoostRemoved>) -> Self {
		self.removed_chat_boost = Some(removed_chat_boost.into());
		self
	}
	/** *Optional*. New incoming shipping query. Only for invoices with flexible price*/
	pub fn shipping_query(mut self, shipping_query: impl Into<ShippingQuery>) -> Self {
		self.shipping_query = Some(shipping_query.into());
		self
	}
}
/**This object represents a Telegram user or bot.

https://core.telegram.org/bots/api/#user*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
	/**if this user added the bot to the attachment menu
	Default value: true*/
	pub added_to_attachment_menu: Option<bool>,
	/**if the bot can be connected to a Telegram Business account to receive its messages. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).*/
	pub can_connect_to_business: Option<bool>,
	/**if the bot can be invited to groups. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).*/
	pub can_join_groups: Option<bool>,
	/**if [privacy mode](https://core.telegram.org/bots/features#privacy-mode) is disabled for the bot. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).*/
	pub can_read_all_group_messages: Option<bool>,
	/**User's or bot's first name*/
	pub first_name: String,
	/**if the bot has a main Web App. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).*/
	pub has_main_web_app: Option<bool>,
	/**Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.*/
	pub id: i64,
	/**if this user is a bot*/
	pub is_bot: bool,
	/**if this user is a Telegram Premium user
	Default value: true*/
	pub is_premium: Option<bool>,
	/**[IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language*/
	pub language_code: Option<String>,
	/**User's or bot's last name*/
	pub last_name: Option<String>,
	/**if the bot supports inline queries. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).*/
	pub supports_inline_queries: Option<bool>,
	/**User's or bot's username*/
	pub username: Option<String>,
}
impl User {
	pub fn new(first_name: impl Into<String>, id: impl Into<i64>, is_bot: bool) -> Self {
		Self {
			added_to_attachment_menu: None,
			can_connect_to_business: None,
			can_join_groups: None,
			can_read_all_group_messages: None,
			first_name: first_name.into(),
			has_main_web_app: None,
			id: id.into(),
			is_bot: is_bot,
			is_premium: None,
			language_code: None,
			last_name: None,
			supports_inline_queries: None,
			username: None,
		}
	}
	/** *Optional*. *True*, if this user added the bot to the attachment menu
	Default value: true*/
	pub fn added_to_attachment_menu(mut self, added_to_attachment_menu: bool) -> Self {
		self.added_to_attachment_menu = Some(added_to_attachment_menu);
		self
	}
	/** *Optional*. *True*, if the bot can be connected to a Telegram Business account to receive its messages. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).*/
	pub fn can_connect_to_business(mut self, can_connect_to_business: bool) -> Self {
		self.can_connect_to_business = Some(can_connect_to_business);
		self
	}
	/** *Optional*. *True*, if the bot can be invited to groups. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).*/
	pub fn can_join_groups(mut self, can_join_groups: bool) -> Self {
		self.can_join_groups = Some(can_join_groups);
		self
	}
	/** *Optional*. *True*, if [privacy mode](https://core.telegram.org/bots/features#privacy-mode) is disabled for the bot. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).*/
	pub fn can_read_all_group_messages(mut self, can_read_all_group_messages: bool) -> Self {
		self.can_read_all_group_messages = Some(can_read_all_group_messages);
		self
	}
	/** *Optional*. *True*, if the bot has a main Web App. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).*/
	pub fn has_main_web_app(mut self, has_main_web_app: bool) -> Self {
		self.has_main_web_app = Some(has_main_web_app);
		self
	}
	/** *Optional*. *True*, if this user is a Telegram Premium user
	Default value: true*/
	pub fn is_premium(mut self, is_premium: bool) -> Self {
		self.is_premium = Some(is_premium);
		self
	}
	/** *Optional*. [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language*/
	pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
		self.language_code = Some(language_code.into());
		self
	}
	/** *Optional*. User's or bot's last name*/
	pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
		self.last_name = Some(last_name.into());
		self
	}
	/** *Optional*. *True*, if the bot supports inline queries. Returned only in [getMe](https://core.telegram.org/bots/api/#getme).*/
	pub fn supports_inline_queries(mut self, supports_inline_queries: bool) -> Self {
		self.supports_inline_queries = Some(supports_inline_queries);
		self
	}
	/** *Optional*. User's or bot's username*/
	pub fn username(mut self, username: impl Into<String>) -> Self {
		self.username = Some(username.into());
		self
	}
}
/**This object represents a list of boosts added to a chat by a user.

https://core.telegram.org/bots/api/#userchatboosts*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct UserChatBoosts {
	/**The list of boosts added to the chat by the user*/
	pub boosts: Vec<ChatBoost>,
}
impl UserChatBoosts {
	pub fn new(boosts: impl Into<Vec<ChatBoost>>) -> Self {
		Self {
			boosts: boosts.into(),
		}
	}
	pub fn add_boost(mut self, boost: impl Into<ChatBoost>) -> Self {
		self.boosts.push(boost.into());
		self
	}
}
/**This object represent a user's profile pictures.

https://core.telegram.org/bots/api/#userprofilephotos*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct UserProfilePhotos {
	/**Requested profile pictures (in up to 4 sizes each)*/
	pub photos: Vec<Vec<PhotoSize>>,
	/**Total number of profile pictures the target user has*/
	pub total_count: i64,
}
impl UserProfilePhotos {
	pub fn new(photos: impl Into<Vec<Vec<PhotoSize>>>, total_count: impl Into<i64>) -> Self {
		Self {
			photos: photos.into(),
			total_count: total_count.into(),
		}
	}
	pub fn add_photo(mut self, photo: impl Into<Vec<PhotoSize>>) -> Self {
		self.photos.push(photo.into());
		self
	}
}
/**This object contains information about the users whose identifiers were shared with the bot using a [KeyboardButtonRequestUsers](https://core.telegram.org/bots/api/#keyboardbuttonrequestusers) button.

https://core.telegram.org/bots/api/#usersshared*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct UsersShared {
	/**Identifier of the request*/
	pub request_id: i64,
	/**Information about users shared with the bot.*/
	pub users: Vec<SharedUser>,
}
impl UsersShared {
	pub fn new(request_id: impl Into<i64>, users: impl Into<Vec<SharedUser>>) -> Self {
		Self {
			request_id: request_id.into(),
			users: users.into(),
		}
	}
	pub fn add_user(mut self, user: impl Into<SharedUser>) -> Self {
		self.users.push(user.into());
		self
	}
}
/**This object represents a venue.

https://core.telegram.org/bots/api/#venue*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Venue {
	/**Address of the venue*/
	pub address: String,
	/**Foursquare identifier of the venue*/
	pub foursquare_id: Option<String>,
	/**Foursquare type of the venue. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)*/
	pub foursquare_type: Option<String>,
	/**Google Places identifier of the venue*/
	pub google_place_id: Option<String>,
	/**Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)*/
	pub google_place_type: Option<String>,
	/**Venue location. Can't be a live location*/
	pub location: Location,
	/**Name of the venue*/
	pub title: String,
}
impl Venue {
	pub fn new(address: impl Into<String>, location: impl Into<Location>, title: impl Into<String>) -> Self {
		Self {
			address: address.into(),
			foursquare_id: None,
			foursquare_type: None,
			google_place_id: None,
			google_place_type: None,
			location: location.into(),
			title: title.into(),
		}
	}
	/** *Optional*. Foursquare identifier of the venue*/
	pub fn foursquare_id(mut self, foursquare_id: impl Into<String>) -> Self {
		self.foursquare_id = Some(foursquare_id.into());
		self
	}
	/** *Optional*. Foursquare type of the venue. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)*/
	pub fn foursquare_type(mut self, foursquare_type: impl Into<String>) -> Self {
		self.foursquare_type = Some(foursquare_type.into());
		self
	}
	/** *Optional*. Google Places identifier of the venue*/
	pub fn google_place_id(mut self, google_place_id: impl Into<String>) -> Self {
		self.google_place_id = Some(google_place_id.into());
		self
	}
	/** *Optional*. Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)*/
	pub fn google_place_type(mut self, google_place_type: impl Into<String>) -> Self {
		self.google_place_type = Some(google_place_type.into());
		self
	}
}
/**This object represents a video file.

https://core.telegram.org/bots/api/#video*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Video {
	/**Available sizes of the cover of the video in the message*/
	pub cover: Vec<PhotoSize>,
	/**Duration of the video in seconds as defined by the sender*/
	pub duration: i64,
	/**Identifier for this file, which can be used to download or reuse the file*/
	pub file_id: String,
	/**Original filename as defined by the sender*/
	pub file_name: Option<String>,
	/**File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub file_size: Option<i64>,
	/**Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub file_unique_id: String,
	/**Video height as defined by the sender*/
	pub height: i64,
	/**MIME type of the file as defined by the sender*/
	pub mime_type: Option<String>,
	/**Timestamp in seconds from which the video will play in the message*/
	pub start_timestamp: Option<i64>,
	/**Video thumbnail*/
	pub thumbnail: Option<PhotoSize>,
	/**Video width as defined by the sender*/
	pub width: i64,
}
impl Video {
	pub fn new(duration: impl Into<i64>, file_id: impl Into<String>, file_unique_id: impl Into<String>, height: impl Into<i64>, width: impl Into<i64>) -> Self {
		Self {
			cover: Vec::new(),
			duration: duration.into(),
			file_id: file_id.into(),
			file_name: None,
			file_size: None,
			file_unique_id: file_unique_id.into(),
			height: height.into(),
			mime_type: None,
			start_timestamp: None,
			thumbnail: None,
			width: width.into(),
		}
	}
	pub fn add_cover(mut self, cover: impl Into<PhotoSize>) -> Self {
		self.cover.push(cover.into());
		self
	}
	/** *Optional*. Available sizes of the cover of the video in the message*/
	pub fn cover(mut self, cover: impl Into<Vec<PhotoSize>>) -> Self {
		self.cover = cover.into();
		self
	}
	/** *Optional*. Original filename as defined by the sender*/
	pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
		self.file_name = Some(file_name.into());
		self
	}
	/** *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub fn file_size(mut self, file_size: impl Into<i64>) -> Self {
		self.file_size = Some(file_size.into());
		self
	}
	/** *Optional*. MIME type of the file as defined by the sender*/
	pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
		self.mime_type = Some(mime_type.into());
		self
	}
	/** *Optional*. Timestamp in seconds from which the video will play in the message*/
	pub fn start_timestamp(mut self, start_timestamp: impl Into<i64>) -> Self {
		self.start_timestamp = Some(start_timestamp.into());
		self
	}
	/** *Optional*. Video thumbnail*/
	pub fn thumbnail(mut self, thumbnail: impl Into<PhotoSize>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
}
/**This object represents a service message about a video chat ended in the chat.

https://core.telegram.org/bots/api/#videochatended*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct VideoChatEnded {
	/**Video chat duration in seconds*/
	pub duration: i64,
}
impl VideoChatEnded {
	pub fn new(duration: impl Into<i64>) -> Self {
		Self {
			duration: duration.into(),
		}
	}
}
/**This object represents a service message about new members invited to a video chat.

https://core.telegram.org/bots/api/#videochatparticipantsinvited*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct VideoChatParticipantsInvited {
	/**New members that were invited to the video chat*/
	pub users: Vec<User>,
}
impl VideoChatParticipantsInvited {
	pub fn new(users: impl Into<Vec<User>>) -> Self {
		Self {
			users: users.into(),
		}
	}
	pub fn add_user(mut self, user: impl Into<User>) -> Self {
		self.users.push(user.into());
		self
	}
}
/**This object represents a service message about a video chat scheduled in the chat.

https://core.telegram.org/bots/api/#videochatscheduled*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct VideoChatScheduled {
	/**Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator*/
	pub start_date: i64,
}
impl VideoChatScheduled {
	pub fn new(start_date: impl Into<i64>) -> Self {
		Self {
			start_date: start_date.into(),
		}
	}
}
/**This object represents a service message about a video chat started in the chat. Currently holds no information.

https://core.telegram.org/bots/api/#videochatstarted*/
pub type VideoChatStarted = ();
/**This object represents a [video message](https://telegram.org/blog/video-messages-and-telescope) (available in Telegram apps as of [v.4.0](https://telegram.org/blog/video-messages-and-telescope)).

https://core.telegram.org/bots/api/#videonote*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct VideoNote {
	/**Duration of the video in seconds as defined by the sender*/
	pub duration: i64,
	/**Identifier for this file, which can be used to download or reuse the file*/
	pub file_id: String,
	/**File size in bytes*/
	pub file_size: Option<i64>,
	/**Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub file_unique_id: String,
	/**Video width and height (diameter of the video message) as defined by the sender*/
	pub length: i64,
	/**Video thumbnail*/
	pub thumbnail: Option<PhotoSize>,
}
impl VideoNote {
	pub fn new(duration: impl Into<i64>, file_id: impl Into<String>, file_unique_id: impl Into<String>, length: impl Into<i64>) -> Self {
		Self {
			duration: duration.into(),
			file_id: file_id.into(),
			file_size: None,
			file_unique_id: file_unique_id.into(),
			length: length.into(),
			thumbnail: None,
		}
	}
	/** *Optional*. File size in bytes*/
	pub fn file_size(mut self, file_size: impl Into<i64>) -> Self {
		self.file_size = Some(file_size.into());
		self
	}
	/** *Optional*. Video thumbnail*/
	pub fn thumbnail(mut self, thumbnail: impl Into<PhotoSize>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
}
/**This object represents a voice note.

https://core.telegram.org/bots/api/#voice*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct Voice {
	/**Duration of the audio in seconds as defined by the sender*/
	pub duration: i64,
	/**Identifier for this file, which can be used to download or reuse the file*/
	pub file_id: String,
	/**File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub file_size: Option<i64>,
	/**Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.*/
	pub file_unique_id: String,
	/**MIME type of the file as defined by the sender*/
	pub mime_type: Option<String>,
}
impl Voice {
	pub fn new(duration: impl Into<i64>, file_id: impl Into<String>, file_unique_id: impl Into<String>) -> Self {
		Self {
			duration: duration.into(),
			file_id: file_id.into(),
			file_size: None,
			file_unique_id: file_unique_id.into(),
			mime_type: None,
		}
	}
	/** *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.*/
	pub fn file_size(mut self, file_size: impl Into<i64>) -> Self {
		self.file_size = Some(file_size.into());
		self
	}
	/** *Optional*. MIME type of the file as defined by the sender*/
	pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
		self.mime_type = Some(mime_type.into());
		self
	}
}
/**Describes data sent from a [Web App](https://core.telegram.org/bots/webapps) to the bot.

https://core.telegram.org/bots/api/#webappdata*/
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct WebAppData {
	/**Text of the *web\_app* keyboard button from which the Web App was opened. Be aware that a bad client can send arbitrary data in this field.*/
	pub button_text: String,
	/**The data. Be aware that a bad client can send arbitrary data in this field.*/
	pub data: String,
}
impl WebAppData {
	pub fn new(button_text: impl Into<String>, data: impl Into<String>) -> Self {
		Self {
			button_text: button_text.into(),
			data: data.into(),
		}
	}
}
/**Describes a [Web App](https://core.telegram.org/bots/webapps).

https://core.telegram.org/bots/api/#webappinfo*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebAppInfo {
	/**An HTTPS URL of a Web App to be opened with additional data as specified in [Initializing Web Apps](https://core.telegram.org/bots/webapps#initializing-mini-apps)*/
	pub url: String,
}
impl WebAppInfo {
	pub fn new(url: impl Into<String>) -> Self {
		Self {
			url: url.into(),
		}
	}
}
/**Describes the current status of a webhook.

https://core.telegram.org/bots/api/#webhookinfo*/
#[apply(
	Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct WebhookInfo {
	/**A list of update types the bot is subscribed to. Defaults to all update types except *chat\_member**/
	pub allowed_updates: Vec<String>,
	/**if a custom certificate was provided for webhook certificate checks*/
	pub has_custom_certificate: bool,
	/**Currently used webhook IP address*/
	pub ip_address: Option<String>,
	/**Unix time for the most recent error that happened when trying to deliver an update via webhook*/
	pub last_error_date: Option<i64>,
	/**Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook*/
	pub last_error_message: Option<String>,
	/**Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters*/
	pub last_synchronization_error_date: Option<i64>,
	/**The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery*/
	pub max_connections: Option<i64>,
	/**Number of updates awaiting delivery*/
	pub pending_update_count: i64,
	/**Webhook URL, may be empty if webhook is not set up*/
	pub url: String,
}
impl WebhookInfo {
	pub fn new(has_custom_certificate: bool, pending_update_count: impl Into<i64>, url: impl Into<String>) -> Self {
		Self {
			allowed_updates: Vec::new(),
			has_custom_certificate: has_custom_certificate,
			ip_address: None,
			last_error_date: None,
			last_error_message: None,
			last_synchronization_error_date: None,
			max_connections: None,
			pending_update_count: pending_update_count.into(),
			url: url.into(),
		}
	}
	pub fn add_allowed_update(mut self, allowed_update: impl Into<String>) -> Self {
		self.allowed_updates.push(allowed_update.into());
		self
	}
	/** *Optional*. A list of update types the bot is subscribed to. Defaults to all update types except *chat\_member**/
	pub fn allowed_updates(mut self, allowed_updates: impl Into<Vec<String>>) -> Self {
		self.allowed_updates = allowed_updates.into();
		self
	}
	/** *Optional*. Currently used webhook IP address*/
	pub fn ip_address(mut self, ip_address: impl Into<String>) -> Self {
		self.ip_address = Some(ip_address.into());
		self
	}
	/** *Optional*. Unix time for the most recent error that happened when trying to deliver an update via webhook*/
	pub fn last_error_date(mut self, last_error_date: impl Into<i64>) -> Self {
		self.last_error_date = Some(last_error_date.into());
		self
	}
	/** *Optional*. Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook*/
	pub fn last_error_message(mut self, last_error_message: impl Into<String>) -> Self {
		self.last_error_message = Some(last_error_message.into());
		self
	}
	/** *Optional*. Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters*/
	pub fn last_synchronization_error_date(mut self, last_synchronization_error_date: impl Into<i64>) -> Self {
		self.last_synchronization_error_date = Some(last_synchronization_error_date.into());
		self
	}
	/** *Optional*. The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery*/
	pub fn max_connections(mut self, max_connections: impl Into<i64>) -> Self {
		self.max_connections = Some(max_connections.into());
		self
	}
}
/**This object represents a service message about a user allowing a bot to write messages after adding it to the attachment menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps).

https://core.telegram.org/bots/api/#writeaccessallowed*/
#[apply(
	Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serde-all", serde(Serialize))]
pub struct WriteAccessAllowed {
	/**True, if the access was granted when the bot was added to the attachment or side menu*/
	pub from_attachment_menu: Option<bool>,
	/**True, if the access was granted after the user accepted an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps)*/
	pub from_request: Option<bool>,
	/**Name of the Web App, if the access was granted when the Web App was launched from a link*/
	pub web_app_name: Option<String>,
}
impl WriteAccessAllowed {
	pub fn new() -> Self {
		Self {
			from_attachment_menu: None,
			from_request: None,
			web_app_name: None,
		}
	}
	/** *Optional*. True, if the access was granted when the bot was added to the attachment or side menu*/
	pub fn from_attachment_menu(mut self, from_attachment_menu: bool) -> Self {
		self.from_attachment_menu = Some(from_attachment_menu);
		self
	}
	/** *Optional*. True, if the access was granted after the user accepted an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps)*/
	pub fn from_request(mut self, from_request: bool) -> Self {
		self.from_request = Some(from_request);
		self
	}
	/** *Optional*. Name of the Web App, if the access was granted when the Web App was launched from a link*/
	pub fn web_app_name(mut self, web_app_name: impl Into<String>) -> Self {
		self.web_app_name = Some(web_app_name.into());
		self
	}
}
/**Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns *True* on success.

https://core.telegram.org/bots/api/#addstickertoset*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct AddStickerToSet {
	/**Sticker set name*/
	pub name: String,
	/**A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.*/
	pub sticker: InputSticker,
	/**User identifier of sticker set owner*/
	pub user_id: i64,
}
impl AddStickerToSet {
	pub fn new(name: impl Into<String>, sticker: impl Into<InputSticker>, user_id: impl Into<i64>) -> Self {
		Self {
			name: name.into(),
			sticker: sticker.into(),
			user_id: user_id.into(),
		}
	}
}
impl Executable for AddStickerToSet {
	type Response = bool;
	const METHOD_NAME: &str = "addStickerToSet";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		parts.add_string("name", self.name);
		parts.add_attachable("sticker", self.sticker);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots/features#inline-keyboards). The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, *True* is returned.

Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via [@BotFather](https://t.me/botfather) and accept the terms. Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.

https://core.telegram.org/bots/api/#answercallbackquery*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct AnswerCallbackQuery {
	/**The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.*/
	pub cache_time: Option<i64>,
	/**Unique identifier for the query to be answered*/
	pub callback_query_id: String,
	/**If *True*, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to *false*.*/
	pub show_alert: Option<bool>,
	/**Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters*/
	pub text: Option<String>,
	/**URL that will be opened by the user's client. If you have created a [Game](https://core.telegram.org/bots/api/#game) and accepted the conditions via [@BotFather](https://t.me/botfather), specify the URL that opens your game - note that this will only work if the query comes from a [*callback\_game*](https://core.telegram.org/bots/api/#inlinekeyboardbutton) button.  

	Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.*/
	pub url: Option<String>,
}
impl AnswerCallbackQuery {
	pub fn new(callback_query_id: impl Into<String>) -> Self {
		Self {
			cache_time: None,
			callback_query_id: callback_query_id.into(),
			show_alert: None,
			text: None,
			url: None,
		}
	}
	/** The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.*/
	pub fn cache_time(mut self, cache_time: impl Into<i64>) -> Self {
		self.cache_time = Some(cache_time.into());
		self
	}
	/** If *True*, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to *false*.*/
	pub fn show_alert(mut self, show_alert: bool) -> Self {
		self.show_alert = Some(show_alert);
		self
	}
	/** Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters*/
	pub fn text(mut self, text: impl Into<String>) -> Self {
		self.text = Some(text.into());
		self
	}
	/** URL that will be opened by the user's client. If you have created a [Game](https://core.telegram.org/bots/api/#game) and accepted the conditions via [@BotFather](https://t.me/botfather), specify the URL that opens your game - note that this will only work if the query comes from a [*callback\_game*](https://core.telegram.org/bots/api/#inlinekeyboardbutton) button.  

	Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.*/
	pub fn url(mut self, url: impl Into<String>) -> Self {
		self.url = Some(url.into());
		self
	}
}
impl Executable for AnswerCallbackQuery {
	type Response = bool;
	const METHOD_NAME: &str = "answerCallbackQuery";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(5);
		parts.add_i64("cache_time", self.cache_time);
		parts.add_string("callback_query_id", self.callback_query_id);
		parts.add_bool("show_alert", self.show_alert);
		parts.add_string("text", self.text);
		parts.add_string("url", self.url);
		parts
	}
}
/**Use this method to send answers to an inline query. On success, *True* is returned.  
No more than **50** results per query are allowed.

https://core.telegram.org/bots/api/#answerinlinequery*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct AnswerInlineQuery {
	/**A JSON-serialized object describing a button to be shown above inline query results*/
	pub button: Option<InlineQueryResultsButton>,
	/**The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.*/
	pub cache_time: Option<i64>,
	/**Unique identifier for the answered query*/
	pub inline_query_id: String,
	/**Pass *True* if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.*/
	pub is_personal: Option<bool>,
	/**Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.*/
	pub next_offset: Option<String>,
	/**A JSON-serialized array of results for the inline query*/
	pub results: Vec<InlineQueryResult>,
}
impl AnswerInlineQuery {
	pub fn new(inline_query_id: impl Into<String>, results: impl Into<Vec<InlineQueryResult>>) -> Self {
		Self {
			button: None,
			cache_time: None,
			inline_query_id: inline_query_id.into(),
			is_personal: None,
			next_offset: None,
			results: results.into(),
		}
	}
	/** A JSON-serialized object describing a button to be shown above inline query results*/
	pub fn button(mut self, button: impl Into<InlineQueryResultsButton>) -> Self {
		self.button = Some(button.into());
		self
	}
	/** The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.*/
	pub fn cache_time(mut self, cache_time: impl Into<i64>) -> Self {
		self.cache_time = Some(cache_time.into());
		self
	}
	/** Pass *True* if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query.*/
	pub fn is_personal(mut self, is_personal: bool) -> Self {
		self.is_personal = Some(is_personal);
		self
	}
	/** Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.*/
	pub fn next_offset(mut self, next_offset: impl Into<String>) -> Self {
		self.next_offset = Some(next_offset.into());
		self
	}
	pub fn add_result(mut self, result: impl Into<InlineQueryResult>) -> Self {
		self.results.push(result.into());
		self
	}
}
impl Executable for AnswerInlineQuery {
	type Response = bool;
	const METHOD_NAME: &str = "answerInlineQuery";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(6);
		if let Some(button) = self.button { parts.add_object("button", button); }
		parts.add_i64("cache_time", self.cache_time);
		parts.add_string("inline_query_id", self.inline_query_id);
		parts.add_bool("is_personal", self.is_personal);
		parts.add_string("next_offset", self.next_offset);
		if self.results.len() > 0 { parts.add_object("results", self.results) }
		parts
	}
}
/**Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an [Update](https://core.telegram.org/bots/api/#update) with the field *pre\_checkout\_query*. Use this method to respond to such pre-checkout queries. On success, *True* is returned. **Note:** The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.

https://core.telegram.org/bots/api/#answerprecheckoutquery*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct AnswerPreCheckoutQuery {
	/**Required if *ok* is *False*. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.*/
	pub error_message: Option<String>,
	/**Specify *True* if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use *False* if there are any problems.*/
	pub ok: bool,
	/**Unique identifier for the query to be answered*/
	pub pre_checkout_query_id: String,
}
impl AnswerPreCheckoutQuery {
	pub fn new(ok: bool, pre_checkout_query_id: impl Into<String>) -> Self {
		Self {
			error_message: None,
			ok: ok,
			pre_checkout_query_id: pre_checkout_query_id.into(),
		}
	}
	/** Required if *ok* is *False*. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.*/
	pub fn error_message(mut self, error_message: impl Into<String>) -> Self {
		self.error_message = Some(error_message.into());
		self
	}
}
impl Executable for AnswerPreCheckoutQuery {
	type Response = bool;
	const METHOD_NAME: &str = "answerPreCheckoutQuery";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		parts.add_string("error_message", self.error_message);
		parts.add_bool("ok", self.ok);
		parts.add_string("pre_checkout_query_id", self.pre_checkout_query_id);
		parts
	}
}
/**If you sent an invoice requesting a shipping address and the parameter *is\_flexible* was specified, the Bot API will send an [Update](https://core.telegram.org/bots/api/#update) with a *shipping\_query* field to the bot. Use this method to reply to shipping queries. On success, *True* is returned.

https://core.telegram.org/bots/api/#answershippingquery*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct AnswerShippingQuery {
	/**Required if *ok* is *False*. Error message in human readable form that explains why it is impossible to complete the order (e.g. “Sorry, delivery to your desired address is unavailable”). Telegram will display this message to the user.*/
	pub error_message: Option<String>,
	/**Pass *True* if delivery to the specified address is possible and *False* if there are any problems (for example, if delivery to the specified address is not possible)*/
	pub ok: bool,
	/**Required if *ok* is *True*. A JSON-serialized array of available shipping options.*/
	pub shipping_options: Vec<ShippingOption>,
	/**Unique identifier for the query to be answered*/
	pub shipping_query_id: String,
}
impl AnswerShippingQuery {
	pub fn new(ok: bool, shipping_query_id: impl Into<String>) -> Self {
		Self {
			error_message: None,
			ok: ok,
			shipping_options: Vec::new(),
			shipping_query_id: shipping_query_id.into(),
		}
	}
	/** Required if *ok* is *False*. Error message in human readable form that explains why it is impossible to complete the order (e.g. “Sorry, delivery to your desired address is unavailable”). Telegram will display this message to the user.*/
	pub fn error_message(mut self, error_message: impl Into<String>) -> Self {
		self.error_message = Some(error_message.into());
		self
	}
	pub fn add_shipping_option(mut self, shipping_option: impl Into<ShippingOption>) -> Self {
		self.shipping_options.push(shipping_option.into());
		self
	}
	/** Required if *ok* is *True*. A JSON-serialized array of available shipping options.*/
	pub fn shipping_options(mut self, shipping_options: impl Into<Vec<ShippingOption>>) -> Self {
		self.shipping_options = shipping_options.into();
		self
	}
}
impl Executable for AnswerShippingQuery {
	type Response = bool;
	const METHOD_NAME: &str = "answerShippingQuery";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_string("error_message", self.error_message);
		parts.add_bool("ok", self.ok);
		if self.shipping_options.len() > 0 { parts.add_object("shipping_options", self.shipping_options) }
		parts.add_string("shipping_query_id", self.shipping_query_id);
		parts
	}
}
/**Use this method to set the result of an interaction with a [Web App](https://core.telegram.org/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a [SentWebAppMessage](https://core.telegram.org/bots/api/#sentwebappmessage) object is returned.

https://core.telegram.org/bots/api/#answerwebappquery*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct AnswerWebAppQuery {
	/**A JSON-serialized object describing the message to be sent*/
	pub result: InlineQueryResult,
	/**Unique identifier for the query to be answered*/
	pub web_app_query_id: String,
}
impl AnswerWebAppQuery {
	pub fn new(result: impl Into<InlineQueryResult>, web_app_query_id: impl Into<String>) -> Self {
		Self {
			result: result.into(),
			web_app_query_id: web_app_query_id.into(),
		}
	}
}
impl Executable for AnswerWebAppQuery {
	type Response = SentWebAppMessage;
	const METHOD_NAME: &str = "answerWebAppQuery";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_object("result", self.result);
		parts.add_string("web_app_query_id", self.web_app_query_id);
		parts
	}
}
/**Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\_invite\_users* administrator right. Returns *True* on success.

https://core.telegram.org/bots/api/#approvechatjoinrequest*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct ApproveChatJoinRequest {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl ApproveChatJoinRequest {
	pub fn new(chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			user_id: user_id.into(),
		}
	}
}
impl Executable for ApproveChatJoinRequest {
	type Response = bool;
	const METHOD_NAME: &str = "approveChatJoinRequest";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [unbanned](https://core.telegram.org/bots/api/#unbanchatmember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#banchatmember*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct BanChatMember {
	/**Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Pass *True* to delete all messages from the chat for the user that is being removed. If *False*, the user will be able to see messages in the group that were sent before the user was removed. Always *True* for supergroups and channels.*/
	pub revoke_messages: Option<bool>,
	/**Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.*/
	pub until_date: Option<i64>,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl BanChatMember {
	pub fn new(chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			revoke_messages: None,
			until_date: None,
			user_id: user_id.into(),
		}
	}
	/** Pass *True* to delete all messages from the chat for the user that is being removed. If *False*, the user will be able to see messages in the group that were sent before the user was removed. Always *True* for supergroups and channels.*/
	pub fn revoke_messages(mut self, revoke_messages: bool) -> Self {
		self.revoke_messages = Some(revoke_messages);
		self
	}
	/** Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.*/
	pub fn until_date(mut self, until_date: impl Into<i64>) -> Self {
		self.until_date = Some(until_date.into());
		self
	}
}
impl Executable for BanChatMember {
	type Response = bool;
	const METHOD_NAME: &str = "banChatMember";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("revoke_messages", self.revoke_messages);
		parts.add_i64("until_date", self.until_date);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [unbanned](https://core.telegram.org/bots/api/#unbanchatsenderchat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#banchatsenderchat*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct BanChatSenderChat {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Unique identifier of the target sender chat*/
	pub sender_chat_id: i64,
}
impl BanChatSenderChat {
	pub fn new(chat_id: impl Into<ChatId>, sender_chat_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			sender_chat_id: sender_chat_id.into(),
		}
	}
}
impl Executable for BanChatSenderChat {
	type Response = bool;
	const METHOD_NAME: &str = "banChatSenderChat";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("sender_chat_id", self.sender_chat_id);
		parts
	}
}
/**Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns *True* on success. Requires no parameters.

https://core.telegram.org/bots/api/#close*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct Close {
}
impl Close {
	pub fn new() -> Self {
		Self {
		}
	}
}
impl Executable for Close {
	type Response = bool;
	const METHOD_NAME: &str = "close";
	fn into_parts(self) -> FormParts {
		FormParts::new(0)
	}
}
/**Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.

https://core.telegram.org/bots/api/#closeforumtopic*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct CloseForumTopic {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**Unique identifier for the target message thread of the forum topic*/
	pub message_thread_id: i64,
}
impl CloseForumTopic {
	pub fn new(chat_id: impl Into<ChatId>, message_thread_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			message_thread_id: message_thread_id.into(),
		}
	}
}
impl Executable for CloseForumTopic {
	type Response = bool;
	const METHOD_NAME: &str = "closeForumTopic";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts
	}
}
/**Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#closegeneralforumtopic*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct CloseGeneralForumTopic {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
}
impl CloseGeneralForumTopic {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for CloseGeneralForumTopic {
	type Response = bool;
	const METHOD_NAME: &str = "closeGeneralForumTopic";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to copy messages of any kind. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\_option\_id* is known to the bot. The method is analogous to the method [forwardMessage](https://core.telegram.org/bots/api/#forwardmessage), but the copied message doesn't have a link to the original message. Returns the [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent message on success.

https://core.telegram.org/bots/api/#copymessage*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct CopyMessage {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept*/
	pub caption: Option<String>,
	/**A JSON-serialized list of special entities that appear in the new caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)*/
	pub from_chat_id: FromChatId,
	/**Message identifier in the chat specified in *from\_chat\_id**/
	pub message_id: i64,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Mode for parsing entities in the new caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Pass if the caption must be shown above the message media. Ignored if a new caption isn't specified.*/
	pub show_caption_above_media: Option<bool>,
	/**New start timestamp for the copied video in the message*/
	pub video_start_timestamp: Option<i64>,
}
impl CopyMessage {
	pub fn new(chat_id: impl Into<ChatId>, from_chat_id: impl Into<FromChatId>, message_id: impl Into<i64>) -> Self {
		Self {
			allow_paid_broadcast: None,
			caption: None,
			caption_entities: Vec::new(),
			chat_id: chat_id.into(),
			disable_notification: None,
			from_chat_id: from_chat_id.into(),
			message_id: message_id.into(),
			message_thread_id: None,
			parse_mode: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			show_caption_above_media: None,
			video_start_timestamp: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the new caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Mode for parsing entities in the new caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
	/** Pass *True*, if the caption must be shown above the message media. Ignored if a new caption isn't specified.*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** New start timestamp for the copied video in the message*/
	pub fn video_start_timestamp(mut self, video_start_timestamp: impl Into<i64>) -> Self {
		self.video_start_timestamp = Some(video_start_timestamp.into());
		self
	}
}
impl Executable for CopyMessage {
	type Response = MessageId;
	const METHOD_NAME: &str = "copyMessage";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(14);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("caption", self.caption);
		if self.caption_entities.len() > 0 { parts.add_object("caption_entities", self.caption_entities) }
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_string("from_chat_id", self.from_chat_id.to_string());
		parts.add_i64("message_id", self.message_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_string("parse_mode", self.parse_mode);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts.add_bool("show_caption_above_media", self.show_caption_above_media);
		parts.add_i64("video_start_timestamp", self.video_start_timestamp);
		parts
	}
}
/**Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz [poll](https://core.telegram.org/bots/api/#poll) can be copied only if the value of the field *correct\_option\_id* is known to the bot. The method is analogous to the method [forwardMessages](https://core.telegram.org/bots/api/#forwardmessages), but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.

https://core.telegram.org/bots/api/#copymessages*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct CopyMessages {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Unique identifier for the chat where the original messages were sent (or channel username in the format `@channelusername`)*/
	pub from_chat_id: FromChatId,
	/**A JSON-serialized list of 1-100 identifiers of messages in the chat *from\_chat\_id* to copy. The identifiers must be specified in a strictly increasing order.*/
	pub message_ids: Vec<i64>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Protects the contents of the sent messages from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Pass *True* to copy the messages without their captions*/
	pub remove_caption: Option<bool>,
}
impl CopyMessages {
	pub fn new(chat_id: impl Into<ChatId>, from_chat_id: impl Into<FromChatId>, message_ids: impl Into<Vec<i64>>) -> Self {
		Self {
			chat_id: chat_id.into(),
			disable_notification: None,
			from_chat_id: from_chat_id.into(),
			message_ids: message_ids.into(),
			message_thread_id: None,
			protect_content: None,
			remove_caption: None,
		}
	}
	/** Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	pub fn add_message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_ids.push(message_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Protects the contents of the sent messages from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Pass *True* to copy the messages without their captions*/
	pub fn remove_caption(mut self, remove_caption: bool) -> Self {
		self.remove_caption = Some(remove_caption);
		self
	}
}
impl Executable for CopyMessages {
	type Response = Vec<MessageId>;
	const METHOD_NAME: &str = "copyMessages";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(7);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_string("from_chat_id", self.from_chat_id.to_string());
		if self.message_ids.len() > 0 { parts.add_object("message_ids", self.message_ids) }
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_bool("protect_content", self.protect_content);
		parts.add_bool("remove_caption", self.remove_caption);
		parts
	}
}
/**Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.

https://core.telegram.org/bots/api/#createchatinvitelink*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct CreateChatInviteLink {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**if users joining the chat via the link need to be approved by chat administrators. If *True*, *member\_limit* can't be specified*/
	pub creates_join_request: Option<bool>,
	/**Point in time (Unix timestamp) when the link will expire*/
	pub expire_date: Option<i64>,
	/**The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999*/
	pub member_limit: Option<i64>,
	/**Invite link name; 0-32 characters*/
	pub name: Option<String>,
}
impl CreateChatInviteLink {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
			creates_join_request: None,
			expire_date: None,
			member_limit: None,
			name: None,
		}
	}
	/** *True*, if users joining the chat via the link need to be approved by chat administrators. If *True*, *member\_limit* can't be specified*/
	pub fn creates_join_request(mut self, creates_join_request: bool) -> Self {
		self.creates_join_request = Some(creates_join_request);
		self
	}
	/** Point in time (Unix timestamp) when the link will expire*/
	pub fn expire_date(mut self, expire_date: impl Into<i64>) -> Self {
		self.expire_date = Some(expire_date.into());
		self
	}
	/** The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999*/
	pub fn member_limit(mut self, member_limit: impl Into<i64>) -> Self {
		self.member_limit = Some(member_limit.into());
		self
	}
	/** Invite link name; 0-32 characters*/
	pub fn name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}
}
impl Executable for CreateChatInviteLink {
	type Response = ChatInviteLink;
	const METHOD_NAME: &str = "createChatInviteLink";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(5);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("creates_join_request", self.creates_join_request);
		parts.add_i64("expire_date", self.expire_date);
		parts.add_i64("member_limit", self.member_limit);
		parts.add_string("name", self.name);
		parts
	}
}
/**Use this method to create a [subscription invite link](https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions) for a channel chat. The bot must have the *can\_invite\_users* administrator rights. The link can be edited using the method [editChatSubscriptionInviteLink](https://core.telegram.org/bots/api/#editchatsubscriptioninvitelink) or revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.

https://core.telegram.org/bots/api/#createchatsubscriptioninvitelink*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct CreateChatSubscriptionInviteLink {
	/**Unique identifier for the target channel chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Invite link name; 0-32 characters*/
	pub name: Option<String>,
	/**The number of seconds the subscription will be active for before the next payment. Currently, it must always be 2592000 (30 days).*/
	pub subscription_period: i64,
	/**The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat; 1-2500*/
	pub subscription_price: i64,
}
impl CreateChatSubscriptionInviteLink {
	pub fn new(chat_id: impl Into<ChatId>, subscription_period: impl Into<i64>, subscription_price: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			name: None,
			subscription_period: subscription_period.into(),
			subscription_price: subscription_price.into(),
		}
	}
	/** Invite link name; 0-32 characters*/
	pub fn name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}
}
impl Executable for CreateChatSubscriptionInviteLink {
	type Response = ChatInviteLink;
	const METHOD_NAME: &str = "createChatSubscriptionInviteLink";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_string("name", self.name);
		parts.add_i64("subscription_period", self.subscription_period);
		parts.add_i64("subscription_price", self.subscription_price);
		parts
	}
}
/**Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. Returns information about the created topic as a [ForumTopic](https://core.telegram.org/bots/api/#forumtopic) object.

https://core.telegram.org/bots/api/#createforumtopic*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct CreateForumTopic {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)*/
	pub icon_color: Option<i64>,
	/**Unique identifier of the custom emoji shown as the topic icon. Use [getForumTopicIconStickers](https://core.telegram.org/bots/api/#getforumtopiciconstickers) to get all allowed custom emoji identifiers.*/
	pub icon_custom_emoji_id: Option<String>,
	/**Topic name, 1-128 characters*/
	pub name: String,
}
impl CreateForumTopic {
	pub fn new(chat_id: impl Into<ChatId>, name: impl Into<String>) -> Self {
		Self {
			chat_id: chat_id.into(),
			icon_color: None,
			icon_custom_emoji_id: None,
			name: name.into(),
		}
	}
	/** Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)*/
	pub fn icon_color(mut self, icon_color: impl Into<i64>) -> Self {
		self.icon_color = Some(icon_color.into());
		self
	}
	/** Unique identifier of the custom emoji shown as the topic icon. Use [getForumTopicIconStickers](https://core.telegram.org/bots/api/#getforumtopiciconstickers) to get all allowed custom emoji identifiers.*/
	pub fn icon_custom_emoji_id(mut self, icon_custom_emoji_id: impl Into<String>) -> Self {
		self.icon_custom_emoji_id = Some(icon_custom_emoji_id.into());
		self
	}
}
impl Executable for CreateForumTopic {
	type Response = ForumTopic;
	const METHOD_NAME: &str = "createForumTopic";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("icon_color", self.icon_color);
		parts.add_string("icon_custom_emoji_id", self.icon_custom_emoji_id);
		parts.add_string("name", self.name);
		parts
	}
}
/**Use this method to create a link for an invoice. Returns the created invoice link as *String* on success.

https://core.telegram.org/bots/api/#createinvoicelink*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct CreateInvoiceLink {
	/**Unique identifier of the business connection on behalf of which the link will be created. For payments in [Telegram Stars](https://t.me/BotNews/90) only.*/
	pub business_connection_id: Option<String>,
	/**Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub currency: String,
	/**Product description, 1-255 characters*/
	pub description: String,
	/**Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub is_flexible: Option<bool>,
	/**The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub max_tip_amount: Option<i64>,
	/**Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_email: Option<bool>,
	/**Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_name: Option<bool>,
	/**Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_phone_number: Option<bool>,
	/**Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_shipping_address: Option<bool>,
	/**Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.*/
	pub payload: String,
	/**Photo height*/
	pub photo_height: Option<i64>,
	/**Photo size in bytes*/
	pub photo_size: Option<i64>,
	/**URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.*/
	pub photo_url: Option<String>,
	/**Photo width*/
	pub photo_width: Option<i64>,
	/**Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub prices: Vec<LabeledPrice>,
	/**JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.*/
	pub provider_data: Option<String>,
	/**Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub provider_token: Option<String>,
	/**Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub send_email_to_provider: Option<bool>,
	/**Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub send_phone_number_to_provider: Option<bool>,
	/**The number of seconds the subscription will be active for before the next payment. The currency must be set to “XTR” (Telegram Stars) if the parameter is used. Currently, it must always be 2592000 (30 days) if specified. Any number of subscriptions can be active for a given bot at the same time, including multiple concurrent subscriptions from the same user. Subscription price must no exceed 2500 Telegram Stars.*/
	pub subscription_period: Option<i64>,
	/**A JSON-serialized array of suggested amounts of tips in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\_tip\_amount*.*/
	pub suggested_tip_amounts: Vec<i64>,
	/**Product name, 1-32 characters*/
	pub title: String,
}
impl CreateInvoiceLink {
	pub fn new(currency: impl Into<String>, description: impl Into<String>, payload: impl Into<String>, prices: impl Into<Vec<LabeledPrice>>, title: impl Into<String>) -> Self {
		Self {
			business_connection_id: None,
			currency: currency.into(),
			description: description.into(),
			is_flexible: None,
			max_tip_amount: None,
			need_email: None,
			need_name: None,
			need_phone_number: None,
			need_shipping_address: None,
			payload: payload.into(),
			photo_height: None,
			photo_size: None,
			photo_url: None,
			photo_width: None,
			prices: prices.into(),
			provider_data: None,
			provider_token: None,
			send_email_to_provider: None,
			send_phone_number_to_provider: None,
			subscription_period: None,
			suggested_tip_amounts: Vec::new(),
			title: title.into(),
		}
	}
	/** Unique identifier of the business connection on behalf of which the link will be created. For payments in [Telegram Stars](https://t.me/BotNews/90) only.*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn is_flexible(mut self, is_flexible: bool) -> Self {
		self.is_flexible = Some(is_flexible);
		self
	}
	/** The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn max_tip_amount(mut self, max_tip_amount: impl Into<i64>) -> Self {
		self.max_tip_amount = Some(max_tip_amount.into());
		self
	}
	/** Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_email(mut self, need_email: bool) -> Self {
		self.need_email = Some(need_email);
		self
	}
	/** Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_name(mut self, need_name: bool) -> Self {
		self.need_name = Some(need_name);
		self
	}
	/** Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_phone_number(mut self, need_phone_number: bool) -> Self {
		self.need_phone_number = Some(need_phone_number);
		self
	}
	/** Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_shipping_address(mut self, need_shipping_address: bool) -> Self {
		self.need_shipping_address = Some(need_shipping_address);
		self
	}
	/** Photo height*/
	pub fn photo_height(mut self, photo_height: impl Into<i64>) -> Self {
		self.photo_height = Some(photo_height.into());
		self
	}
	/** Photo size in bytes*/
	pub fn photo_size(mut self, photo_size: impl Into<i64>) -> Self {
		self.photo_size = Some(photo_size.into());
		self
	}
	/** URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.*/
	pub fn photo_url(mut self, photo_url: impl Into<String>) -> Self {
		self.photo_url = Some(photo_url.into());
		self
	}
	/** Photo width*/
	pub fn photo_width(mut self, photo_width: impl Into<i64>) -> Self {
		self.photo_width = Some(photo_width.into());
		self
	}
	pub fn add_price(mut self, price: impl Into<LabeledPrice>) -> Self {
		self.prices.push(price.into());
		self
	}
	/** JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.*/
	pub fn provider_data(mut self, provider_data: impl Into<String>) -> Self {
		self.provider_data = Some(provider_data.into());
		self
	}
	/** Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn provider_token(mut self, provider_token: impl Into<String>) -> Self {
		self.provider_token = Some(provider_token.into());
		self
	}
	/** Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn send_email_to_provider(mut self, send_email_to_provider: bool) -> Self {
		self.send_email_to_provider = Some(send_email_to_provider);
		self
	}
	/** Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn send_phone_number_to_provider(mut self, send_phone_number_to_provider: bool) -> Self {
		self.send_phone_number_to_provider = Some(send_phone_number_to_provider);
		self
	}
	/** The number of seconds the subscription will be active for before the next payment. The currency must be set to “XTR” (Telegram Stars) if the parameter is used. Currently, it must always be 2592000 (30 days) if specified. Any number of subscriptions can be active for a given bot at the same time, including multiple concurrent subscriptions from the same user. Subscription price must no exceed 2500 Telegram Stars.*/
	pub fn subscription_period(mut self, subscription_period: impl Into<i64>) -> Self {
		self.subscription_period = Some(subscription_period.into());
		self
	}
	pub fn add_suggested_tip_amount(mut self, suggested_tip_amount: impl Into<i64>) -> Self {
		self.suggested_tip_amounts.push(suggested_tip_amount.into());
		self
	}
	/** A JSON-serialized array of suggested amounts of tips in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\_tip\_amount*.*/
	pub fn suggested_tip_amounts(mut self, suggested_tip_amounts: impl Into<Vec<i64>>) -> Self {
		self.suggested_tip_amounts = suggested_tip_amounts.into();
		self
	}
}
impl Executable for CreateInvoiceLink {
	type Response = String;
	const METHOD_NAME: &str = "createInvoiceLink";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(22);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("currency", self.currency);
		parts.add_string("description", self.description);
		parts.add_bool("is_flexible", self.is_flexible);
		parts.add_i64("max_tip_amount", self.max_tip_amount);
		parts.add_bool("need_email", self.need_email);
		parts.add_bool("need_name", self.need_name);
		parts.add_bool("need_phone_number", self.need_phone_number);
		parts.add_bool("need_shipping_address", self.need_shipping_address);
		parts.add_string("payload", self.payload);
		parts.add_i64("photo_height", self.photo_height);
		parts.add_i64("photo_size", self.photo_size);
		parts.add_string("photo_url", self.photo_url);
		parts.add_i64("photo_width", self.photo_width);
		if self.prices.len() > 0 { parts.add_object("prices", self.prices) }
		parts.add_string("provider_data", self.provider_data);
		parts.add_string("provider_token", self.provider_token);
		parts.add_bool("send_email_to_provider", self.send_email_to_provider);
		parts.add_bool("send_phone_number_to_provider", self.send_phone_number_to_provider);
		parts.add_i64("subscription_period", self.subscription_period);
		if self.suggested_tip_amounts.len() > 0 { parts.add_object("suggested_tip_amounts", self.suggested_tip_amounts) }
		parts.add_string("title", self.title);
		parts
	}
}
/**Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns *True* on success.

https://core.telegram.org/bots/api/#createnewstickerset*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct CreateNewStickerSet {
	/**Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., *animals*). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `"_by_<bot_username>"`. `<bot_username>` is case insensitive. 1-64 characters.*/
	pub name: String,
	/**Pass *True* if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only*/
	pub needs_repainting: Option<bool>,
	/**Type of stickers in the set, pass “regular”, “mask”, or “custom\_emoji”. By default, a regular sticker set is created.*/
	pub sticker_type: Option<String>,
	/**A JSON-serialized list of 1-50 initial stickers to be added to the sticker set*/
	pub stickers: Vec<InputSticker>,
	/**Sticker set title, 1-64 characters*/
	pub title: String,
	/**User identifier of created sticker set owner*/
	pub user_id: i64,
}
impl CreateNewStickerSet {
	pub fn new(name: impl Into<String>, stickers: impl Into<Vec<InputSticker>>, title: impl Into<String>, user_id: impl Into<i64>) -> Self {
		Self {
			name: name.into(),
			needs_repainting: None,
			sticker_type: None,
			stickers: stickers.into(),
			title: title.into(),
			user_id: user_id.into(),
		}
	}
	/** Pass *True* if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only*/
	pub fn needs_repainting(mut self, needs_repainting: bool) -> Self {
		self.needs_repainting = Some(needs_repainting);
		self
	}
	/** Type of stickers in the set, pass “regular”, “mask”, or “custom\_emoji”. By default, a regular sticker set is created.*/
	pub fn sticker_type(mut self, sticker_type: impl Into<String>) -> Self {
		self.sticker_type = Some(sticker_type.into());
		self
	}
	pub fn add_sticker(mut self, sticker: impl Into<InputSticker>) -> Self {
		self.stickers.push(sticker.into());
		self
	}
}
impl Executable for CreateNewStickerSet {
	type Response = bool;
	const METHOD_NAME: &str = "createNewStickerSet";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(6);
		parts.add_string("name", self.name);
		parts.add_bool("needs_repainting", self.needs_repainting);
		parts.add_string("sticker_type", self.sticker_type);
		parts.add_attachable("stickers", self.stickers);
		parts.add_string("title", self.title);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the *can\_invite\_users* administrator right. Returns *True* on success.

https://core.telegram.org/bots/api/#declinechatjoinrequest*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct DeclineChatJoinRequest {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl DeclineChatJoinRequest {
	pub fn new(chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			user_id: user_id.into(),
		}
	}
}
impl Executable for DeclineChatJoinRequest {
	type Response = bool;
	const METHOD_NAME: &str = "declineChatJoinRequest";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#deletechatphoto*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct DeleteChatPhoto {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
}
impl DeleteChatPhoto {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for DeleteChatPhoto {
	type Response = bool;
	const METHOD_NAME: &str = "deleteChatPhoto";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\_set\_sticker\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.

https://core.telegram.org/bots/api/#deletechatstickerset*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct DeleteChatStickerSet {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
}
impl DeleteChatStickerSet {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for DeleteChatStickerSet {
	type Response = bool;
	const METHOD_NAME: &str = "deleteChatStickerSet";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to delete a forum topic along with all its messages in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_delete\_messages* administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#deleteforumtopic*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct DeleteForumTopic {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**Unique identifier for the target message thread of the forum topic*/
	pub message_thread_id: i64,
}
impl DeleteForumTopic {
	pub fn new(chat_id: impl Into<ChatId>, message_thread_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			message_thread_id: message_thread_id.into(),
		}
	}
}
impl Executable for DeleteForumTopic {
	type Response = bool;
	const METHOD_NAME: &str = "deleteForumTopic";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts
	}
}
/**Use this method to delete a message, including service messages, with the following limitations:  
\- A message can only be deleted if it was sent less than 48 hours ago.  
\- Service messages about a supergroup, channel, or forum topic creation can't be deleted.  
\- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.  
\- Bots can delete outgoing messages in private chats, groups, and supergroups.  
\- Bots can delete incoming messages in private chats.  
\- Bots granted *can\_post\_messages* permissions can delete outgoing messages in channels.  
\- If the bot is an administrator of a group, it can delete any message there.  
\- If the bot has *can\_delete\_messages* permission in a supergroup or a channel, it can delete any message there.  
Returns *True* on success.

https://core.telegram.org/bots/api/#deletemessage*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct DeleteMessage {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Identifier of the message to delete*/
	pub message_id: i64,
}
impl DeleteMessage {
	pub fn new(chat_id: impl Into<ChatId>, message_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			message_id: message_id.into(),
		}
	}
}
impl Executable for DeleteMessage {
	type Response = bool;
	const METHOD_NAME: &str = "deleteMessage";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("message_id", self.message_id);
		parts
	}
}
/**Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns *True* on success.

https://core.telegram.org/bots/api/#deletemessages*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct DeleteMessages {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**A JSON-serialized list of 1-100 identifiers of messages to delete. See [deleteMessage](https://core.telegram.org/bots/api/#deletemessage) for limitations on which messages can be deleted*/
	pub message_ids: Vec<i64>,
}
impl DeleteMessages {
	pub fn new(chat_id: impl Into<ChatId>, message_ids: impl Into<Vec<i64>>) -> Self {
		Self {
			chat_id: chat_id.into(),
			message_ids: message_ids.into(),
		}
	}
	pub fn add_message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_ids.push(message_id.into());
		self
	}
}
impl Executable for DeleteMessages {
	type Response = bool;
	const METHOD_NAME: &str = "deleteMessages";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		if self.message_ids.len() > 0 { parts.add_object("message_ids", self.message_ids) }
		parts
	}
}
/**Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, [higher level commands](https://core.telegram.org/bots/api/#determining-list-of-commands) will be shown to affected users. Returns *True* on success.

https://core.telegram.org/bots/api/#deletemycommands*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct DeleteMyCommands {
	/**A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands*/
	pub language_code: Option<String>,
	/**A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault).*/
	pub scope: Option<BotCommandScope>,
}
impl DeleteMyCommands {
	pub fn new() -> Self {
		Self {
			language_code: None,
			scope: None,
		}
	}
	/** A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands*/
	pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
		self.language_code = Some(language_code.into());
		self
	}
	/** A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault).*/
	pub fn scope(mut self, scope: impl Into<BotCommandScope>) -> Self {
		self.scope = Some(scope.into());
		self
	}
}
impl Executable for DeleteMyCommands {
	type Response = bool;
	const METHOD_NAME: &str = "deleteMyCommands";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("language_code", self.language_code);
		if let Some(scope) = self.scope { parts.add_object("scope", scope); }
		parts
	}
}
/**Use this method to delete a sticker from a set created by the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#deletestickerfromset*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct DeleteStickerFromSet {
	/**File identifier of the sticker*/
	pub sticker: String,
}
impl DeleteStickerFromSet {
	pub fn new(sticker: impl Into<String>) -> Self {
		Self {
			sticker: sticker.into(),
		}
	}
}
impl Executable for DeleteStickerFromSet {
	type Response = bool;
	const METHOD_NAME: &str = "deleteStickerFromSet";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("sticker", self.sticker);
		parts
	}
}
/**Use this method to delete a sticker set that was created by the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#deletestickerset*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct DeleteStickerSet {
	/**Sticker set name*/
	pub name: String,
}
impl DeleteStickerSet {
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
		}
	}
}
impl Executable for DeleteStickerSet {
	type Response = bool;
	const METHOD_NAME: &str = "deleteStickerSet";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("name", self.name);
		parts
	}
}
/**Use this method to remove webhook integration if you decide to switch back to [getUpdates](https://core.telegram.org/bots/api/#getupdates). Returns *True* on success.

https://core.telegram.org/bots/api/#deletewebhook*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct DeleteWebhook {
	/**Pass *True* to drop all pending updates*/
	pub drop_pending_updates: Option<bool>,
}
impl DeleteWebhook {
	pub fn new() -> Self {
		Self {
			drop_pending_updates: None,
		}
	}
	/** Pass *True* to drop all pending updates*/
	pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
		self.drop_pending_updates = Some(drop_pending_updates);
		self
	}
}
impl Executable for DeleteWebhook {
	type Response = bool;
	const METHOD_NAME: &str = "deleteWebhook";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_bool("drop_pending_updates", self.drop_pending_updates);
		parts
	}
}
/**Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.

https://core.telegram.org/bots/api/#editchatinvitelink*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct EditChatInviteLink {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**if users joining the chat via the link need to be approved by chat administrators. If *True*, *member\_limit* can't be specified*/
	pub creates_join_request: Option<bool>,
	/**Point in time (Unix timestamp) when the link will expire*/
	pub expire_date: Option<i64>,
	/**The invite link to edit*/
	pub invite_link: String,
	/**The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999*/
	pub member_limit: Option<i64>,
	/**Invite link name; 0-32 characters*/
	pub name: Option<String>,
}
impl EditChatInviteLink {
	pub fn new(chat_id: impl Into<ChatId>, invite_link: impl Into<String>) -> Self {
		Self {
			chat_id: chat_id.into(),
			creates_join_request: None,
			expire_date: None,
			invite_link: invite_link.into(),
			member_limit: None,
			name: None,
		}
	}
	/** *True*, if users joining the chat via the link need to be approved by chat administrators. If *True*, *member\_limit* can't be specified*/
	pub fn creates_join_request(mut self, creates_join_request: bool) -> Self {
		self.creates_join_request = Some(creates_join_request);
		self
	}
	/** Point in time (Unix timestamp) when the link will expire*/
	pub fn expire_date(mut self, expire_date: impl Into<i64>) -> Self {
		self.expire_date = Some(expire_date.into());
		self
	}
	/** The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999*/
	pub fn member_limit(mut self, member_limit: impl Into<i64>) -> Self {
		self.member_limit = Some(member_limit.into());
		self
	}
	/** Invite link name; 0-32 characters*/
	pub fn name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}
}
impl Executable for EditChatInviteLink {
	type Response = ChatInviteLink;
	const METHOD_NAME: &str = "editChatInviteLink";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(6);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("creates_join_request", self.creates_join_request);
		parts.add_i64("expire_date", self.expire_date);
		parts.add_string("invite_link", self.invite_link);
		parts.add_i64("member_limit", self.member_limit);
		parts.add_string("name", self.name);
		parts
	}
}
/**Use this method to edit a subscription invite link created by the bot. The bot must have the *can\_invite\_users* administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.

https://core.telegram.org/bots/api/#editchatsubscriptioninvitelink*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct EditChatSubscriptionInviteLink {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**The invite link to edit*/
	pub invite_link: String,
	/**Invite link name; 0-32 characters*/
	pub name: Option<String>,
}
impl EditChatSubscriptionInviteLink {
	pub fn new(chat_id: impl Into<ChatId>, invite_link: impl Into<String>) -> Self {
		Self {
			chat_id: chat_id.into(),
			invite_link: invite_link.into(),
			name: None,
		}
	}
	/** Invite link name; 0-32 characters*/
	pub fn name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}
}
impl Executable for EditChatSubscriptionInviteLink {
	type Response = ChatInviteLink;
	const METHOD_NAME: &str = "editChatSubscriptionInviteLink";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_string("invite_link", self.invite_link);
		parts.add_string("name", self.name);
		parts
	}
}
/**Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.

https://core.telegram.org/bots/api/#editforumtopic*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct EditForumTopic {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**New unique identifier of the custom emoji shown as the topic icon. Use [getForumTopicIconStickers](https://core.telegram.org/bots/api/#getforumtopiciconstickers) to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept*/
	pub icon_custom_emoji_id: Option<String>,
	/**Unique identifier for the target message thread of the forum topic*/
	pub message_thread_id: i64,
	/**New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept*/
	pub name: Option<String>,
}
impl EditForumTopic {
	pub fn new(chat_id: impl Into<ChatId>, message_thread_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			icon_custom_emoji_id: None,
			message_thread_id: message_thread_id.into(),
			name: None,
		}
	}
	/** New unique identifier of the custom emoji shown as the topic icon. Use [getForumTopicIconStickers](https://core.telegram.org/bots/api/#getforumtopiciconstickers) to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept*/
	pub fn icon_custom_emoji_id(mut self, icon_custom_emoji_id: impl Into<String>) -> Self {
		self.icon_custom_emoji_id = Some(icon_custom_emoji_id.into());
		self
	}
	/** New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept*/
	pub fn name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}
}
impl Executable for EditForumTopic {
	type Response = bool;
	const METHOD_NAME: &str = "editForumTopic";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_string("icon_custom_emoji_id", self.icon_custom_emoji_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_string("name", self.name);
		parts
	}
}
/**Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#editgeneralforumtopic*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct EditGeneralForumTopic {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**New topic name, 1-128 characters*/
	pub name: String,
}
impl EditGeneralForumTopic {
	pub fn new(chat_id: impl Into<ChatId>, name: impl Into<String>) -> Self {
		Self {
			chat_id: chat_id.into(),
			name: name.into(),
		}
	}
}
impl Executable for EditGeneralForumTopic {
	type Response = bool;
	const METHOD_NAME: &str = "editGeneralForumTopic";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_string("name", self.name);
		parts
	}
}
/**Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.

https://core.telegram.org/bots/api/#editmessagecaption*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct EditMessageCaption {
	/**Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub business_connection_id: Option<String>,
	/**New caption of the message, 0-1024 characters after entities parsing*/
	pub caption: Option<String>,
	/**A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: Option<ChatId>,
	/**Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub inline_message_id: Option<String>,
	/**Required if *inline\_message\_id* is not specified. Identifier of the message to edit*/
	pub message_id: Option<i64>,
	/**Mode for parsing entities in the message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Pass if the caption must be shown above the message media. Supported only for animation, photo and video messages.*/
	pub show_caption_above_media: Option<bool>,
}
impl EditMessageCaption {
	pub fn new() -> Self {
		Self {
			business_connection_id: None,
			caption: None,
			caption_entities: Vec::new(),
			chat_id: None,
			inline_message_id: None,
			message_id: None,
			parse_mode: None,
			reply_markup: None,
			show_caption_above_media: None,
		}
	}
	/** Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** New caption of the message, 0-1024 characters after entities parsing*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
	/** Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
		self.inline_message_id = Some(inline_message_id.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Identifier of the message to edit*/
	pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_id = Some(message_id.into());
		self
	}
	/** Mode for parsing entities in the message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Pass *True*, if the caption must be shown above the message media. Supported only for animation, photo and video messages.*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
}
impl Executable for EditMessageCaption {
	type Response = EditMessageResult;
	const METHOD_NAME: &str = "editMessageCaption";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(9);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("caption", self.caption);
		if self.caption_entities.len() > 0 { parts.add_object("caption_entities", self.caption_entities) }
		parts.add_string("chat_id", self.chat_id.map(|x| x.to_string()));
		parts.add_string("inline_message_id", self.inline_message_id);
		parts.add_i64("message_id", self.message_id);
		parts.add_string("parse_mode", self.parse_mode);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		parts.add_bool("show_caption_above_media", self.show_caption_above_media);
		parts
	}
}
/**Use this method to edit live location messages. A location can be edited until its *live\_period* expires or editing is explicitly disabled by a call to [stopMessageLiveLocation](https://core.telegram.org/bots/api/#stopmessagelivelocation). On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.

https://core.telegram.org/bots/api/#editmessagelivelocation*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct EditMessageLiveLocation {
	/**Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub business_connection_id: Option<String>,
	/**Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: Option<ChatId>,
	/**Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.*/
	pub heading: Option<i64>,
	/**The radius of uncertainty for the location, measured in meters; 0-1500*/
	pub horizontal_accuracy: Option<f32>,
	/**Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub inline_message_id: Option<String>,
	/**Latitude of new location*/
	pub latitude: f32,
	/**New period in seconds during which the location can be updated, starting from the message send date. If 0x7FFFFFFF is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current *live\_period* by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then *live\_period* remains unchanged*/
	pub live_period: Option<i64>,
	/**Longitude of new location*/
	pub longitude: f32,
	/**Required if *inline\_message\_id* is not specified. Identifier of the message to edit*/
	pub message_id: Option<i64>,
	/**The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.*/
	pub proximity_alert_radius: Option<i64>,
	/**A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl EditMessageLiveLocation {
	pub fn new(latitude: impl Into<f32>, longitude: impl Into<f32>) -> Self {
		Self {
			business_connection_id: None,
			chat_id: None,
			heading: None,
			horizontal_accuracy: None,
			inline_message_id: None,
			latitude: latitude.into(),
			live_period: None,
			longitude: longitude.into(),
			message_id: None,
			proximity_alert_radius: None,
			reply_markup: None,
		}
	}
	/** Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
	/** Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.*/
	pub fn heading(mut self, heading: impl Into<i64>) -> Self {
		self.heading = Some(heading.into());
		self
	}
	/** The radius of uncertainty for the location, measured in meters; 0-1500*/
	pub fn horizontal_accuracy(mut self, horizontal_accuracy: impl Into<f32>) -> Self {
		self.horizontal_accuracy = Some(horizontal_accuracy.into());
		self
	}
	/** Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
		self.inline_message_id = Some(inline_message_id.into());
		self
	}
	/** New period in seconds during which the location can be updated, starting from the message send date. If 0x7FFFFFFF is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current *live\_period* by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then *live\_period* remains unchanged*/
	pub fn live_period(mut self, live_period: impl Into<i64>) -> Self {
		self.live_period = Some(live_period.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Identifier of the message to edit*/
	pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_id = Some(message_id.into());
		self
	}
	/** The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.*/
	pub fn proximity_alert_radius(mut self, proximity_alert_radius: impl Into<i64>) -> Self {
		self.proximity_alert_radius = Some(proximity_alert_radius.into());
		self
	}
	/** A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
impl Executable for EditMessageLiveLocation {
	type Response = EditMessageResult;
	const METHOD_NAME: &str = "editMessageLiveLocation";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(11);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.map(|x| x.to_string()));
		parts.add_i64("heading", self.heading);
		parts.add_f32("horizontal_accuracy", self.horizontal_accuracy);
		parts.add_string("inline_message_id", self.inline_message_id);
		parts.add_f32("latitude", self.latitude);
		parts.add_i64("live_period", self.live_period);
		parts.add_f32("longitude", self.longitude);
		parts.add_i64("message_id", self.message_id);
		parts.add_i64("proximity_alert_radius", self.proximity_alert_radius);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		parts
	}
}
/**Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file\_id or specify a URL. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.

https://core.telegram.org/bots/api/#editmessagemedia*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct EditMessageMedia {
	/**Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub business_connection_id: Option<String>,
	/**Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: Option<ChatId>,
	/**Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub inline_message_id: Option<String>,
	/**A JSON-serialized object for a new media content of the message*/
	pub media: InputMedia,
	/**Required if *inline\_message\_id* is not specified. Identifier of the message to edit*/
	pub message_id: Option<i64>,
	/**A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl EditMessageMedia {
	pub fn new(media: impl Into<InputMedia>) -> Self {
		Self {
			business_connection_id: None,
			chat_id: None,
			inline_message_id: None,
			media: media.into(),
			message_id: None,
			reply_markup: None,
		}
	}
	/** Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
	/** Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
		self.inline_message_id = Some(inline_message_id.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Identifier of the message to edit*/
	pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_id = Some(message_id.into());
		self
	}
	/** A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
impl Executable for EditMessageMedia {
	type Response = EditMessageResult;
	const METHOD_NAME: &str = "editMessageMedia";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(6);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.map(|x| x.to_string()));
		parts.add_string("inline_message_id", self.inline_message_id);
		parts.add_attachable("media", self.media);
		parts.add_i64("message_id", self.message_id);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		parts
	}
}
/**Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.

https://core.telegram.org/bots/api/#editmessagereplymarkup*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct EditMessageReplyMarkup {
	/**Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub business_connection_id: Option<String>,
	/**Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: Option<ChatId>,
	/**Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub inline_message_id: Option<String>,
	/**Required if *inline\_message\_id* is not specified. Identifier of the message to edit*/
	pub message_id: Option<i64>,
	/**A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl EditMessageReplyMarkup {
	pub fn new() -> Self {
		Self {
			business_connection_id: None,
			chat_id: None,
			inline_message_id: None,
			message_id: None,
			reply_markup: None,
		}
	}
	/** Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
	/** Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
		self.inline_message_id = Some(inline_message_id.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Identifier of the message to edit*/
	pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_id = Some(message_id.into());
		self
	}
	/** A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
impl Executable for EditMessageReplyMarkup {
	type Response = EditMessageResult;
	const METHOD_NAME: &str = "editMessageReplyMarkup";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(5);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.map(|x| x.to_string()));
		parts.add_string("inline_message_id", self.inline_message_id);
		parts.add_i64("message_id", self.message_id);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		parts
	}
}
/**Use this method to edit text and [game](https://core.telegram.org/bots/api/#games) messages. On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within **48 hours** from the time they were sent.

https://core.telegram.org/bots/api/#editmessagetext*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct EditMessageText {
	/**Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub business_connection_id: Option<String>,
	/**Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: Option<ChatId>,
	/**A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse\_mode**/
	pub entities: Vec<MessageEntity>,
	/**Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub inline_message_id: Option<String>,
	/**Link preview generation options for the message*/
	pub link_preview_options: Option<LinkPreviewOptions>,
	/**Required if *inline\_message\_id* is not specified. Identifier of the message to edit*/
	pub message_id: Option<i64>,
	/**Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**New text of the message, 1-4096 characters after entities parsing*/
	pub text: String,
}
impl EditMessageText {
	pub fn new(text: impl Into<String>) -> Self {
		Self {
			business_connection_id: None,
			chat_id: None,
			entities: Vec::new(),
			inline_message_id: None,
			link_preview_options: None,
			message_id: None,
			parse_mode: None,
			reply_markup: None,
			text: text.into(),
		}
	}
	/** Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
	pub fn add_entity(mut self, entity: impl Into<MessageEntity>) -> Self {
		self.entities.push(entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse\_mode**/
	pub fn entities(mut self, entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.entities = entities.into();
		self
	}
	/** Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
		self.inline_message_id = Some(inline_message_id.into());
		self
	}
	/** Link preview generation options for the message*/
	pub fn link_preview_options(mut self, link_preview_options: impl Into<LinkPreviewOptions>) -> Self {
		self.link_preview_options = Some(link_preview_options.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Identifier of the message to edit*/
	pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_id = Some(message_id.into());
		self
	}
	/** Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
impl Executable for EditMessageText {
	type Response = EditMessageResult;
	const METHOD_NAME: &str = "editMessageText";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(9);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.map(|x| x.to_string()));
		if self.entities.len() > 0 { parts.add_object("entities", self.entities) }
		parts.add_string("inline_message_id", self.inline_message_id);
		if let Some(link_preview_options) = self.link_preview_options { parts.add_object("link_preview_options", link_preview_options); }
		parts.add_i64("message_id", self.message_id);
		parts.add_string("parse_mode", self.parse_mode);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		parts.add_string("text", self.text);
		parts
	}
}
/**Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns *True* on success.

https://core.telegram.org/bots/api/#edituserstarsubscription*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct EditUserStarSubscription {
	/**Pass *True* to cancel extension of the user subscription; the subscription must be active up to the end of the current subscription period. Pass *False* to allow the user to re-enable a subscription that was previously canceled by the bot.*/
	pub is_canceled: bool,
	/**Telegram payment identifier for the subscription*/
	pub telegram_payment_charge_id: String,
	/**Identifier of the user whose subscription will be edited*/
	pub user_id: i64,
}
impl EditUserStarSubscription {
	pub fn new(is_canceled: bool, telegram_payment_charge_id: impl Into<String>, user_id: impl Into<i64>) -> Self {
		Self {
			is_canceled: is_canceled,
			telegram_payment_charge_id: telegram_payment_charge_id.into(),
			user_id: user_id.into(),
		}
	}
}
impl Executable for EditUserStarSubscription {
	type Response = bool;
	const METHOD_NAME: &str = "editUserStarSubscription";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		parts.add_bool("is_canceled", self.is_canceled);
		parts.add_string("telegram_payment_charge_id", self.telegram_payment_charge_id);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as *String* on success.

https://core.telegram.org/bots/api/#exportchatinvitelink*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct ExportChatInviteLink {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
}
impl ExportChatInviteLink {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for ExportChatInviteLink {
	type Response = String;
	const METHOD_NAME: &str = "exportChatInviteLink";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#forwardmessage*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct ForwardMessage {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)*/
	pub from_chat_id: FromChatId,
	/**Message identifier in the chat specified in *from\_chat\_id**/
	pub message_id: i64,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Protects the contents of the forwarded message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**New start timestamp for the forwarded video in the message*/
	pub video_start_timestamp: Option<i64>,
}
impl ForwardMessage {
	pub fn new(chat_id: impl Into<ChatId>, from_chat_id: impl Into<FromChatId>, message_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			disable_notification: None,
			from_chat_id: from_chat_id.into(),
			message_id: message_id.into(),
			message_thread_id: None,
			protect_content: None,
			video_start_timestamp: None,
		}
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Protects the contents of the forwarded message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** New start timestamp for the forwarded video in the message*/
	pub fn video_start_timestamp(mut self, video_start_timestamp: impl Into<i64>) -> Self {
		self.video_start_timestamp = Some(video_start_timestamp.into());
		self
	}
}
impl Executable for ForwardMessage {
	type Response = Message;
	const METHOD_NAME: &str = "forwardMessage";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(7);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_string("from_chat_id", self.from_chat_id.to_string());
		parts.add_i64("message_id", self.message_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_bool("protect_content", self.protect_content);
		parts.add_i64("video_start_timestamp", self.video_start_timestamp);
		parts
	}
}
/**Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [MessageId](https://core.telegram.org/bots/api/#messageid) of the sent messages is returned.

https://core.telegram.org/bots/api/#forwardmessages*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct ForwardMessages {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Unique identifier for the chat where the original messages were sent (or channel username in the format `@channelusername`)*/
	pub from_chat_id: FromChatId,
	/**A JSON-serialized list of 1-100 identifiers of messages in the chat *from\_chat\_id* to forward. The identifiers must be specified in a strictly increasing order.*/
	pub message_ids: Vec<i64>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Protects the contents of the forwarded messages from forwarding and saving*/
	pub protect_content: Option<bool>,
}
impl ForwardMessages {
	pub fn new(chat_id: impl Into<ChatId>, from_chat_id: impl Into<FromChatId>, message_ids: impl Into<Vec<i64>>) -> Self {
		Self {
			chat_id: chat_id.into(),
			disable_notification: None,
			from_chat_id: from_chat_id.into(),
			message_ids: message_ids.into(),
			message_thread_id: None,
			protect_content: None,
		}
	}
	/** Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	pub fn add_message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_ids.push(message_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Protects the contents of the forwarded messages from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
}
impl Executable for ForwardMessages {
	type Response = Vec<MessageId>;
	const METHOD_NAME: &str = "forwardMessages";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(6);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_string("from_chat_id", self.from_chat_id.to_string());
		if self.message_ids.len() > 0 { parts.add_object("message_ids", self.message_ids) }
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_bool("protect_content", self.protect_content);
		parts
	}
}
/**Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a [Gifts](https://core.telegram.org/bots/api/#gifts) object.

https://core.telegram.org/bots/api/#getavailablegifts*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetAvailableGifts {
}
impl GetAvailableGifts {
	pub fn new() -> Self {
		Self {
		}
	}
}
impl Executable for GetAvailableGifts {
	type Response = Gifts;
	const METHOD_NAME: &str = "getAvailableGifts";
	fn into_parts(self) -> FormParts {
		FormParts::new(0)
	}
}
/**Use this method to get information about the connection of the bot with a business account. Returns a [BusinessConnection](https://core.telegram.org/bots/api/#businessconnection) object on success.

https://core.telegram.org/bots/api/#getbusinessconnection*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetBusinessConnection {
	/**Unique identifier of the business connection*/
	pub business_connection_id: String,
}
impl GetBusinessConnection {
	pub fn new(business_connection_id: impl Into<String>) -> Self {
		Self {
			business_connection_id: business_connection_id.into(),
		}
	}
}
impl Executable for GetBusinessConnection {
	type Response = BusinessConnection;
	const METHOD_NAME: &str = "getBusinessConnection";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts
	}
}
/**Use this method to get up-to-date information about the chat. Returns a [ChatFullInfo](https://core.telegram.org/bots/api/#chatfullinfo) object on success.

https://core.telegram.org/bots/api/#getchat*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetChat {
	/**Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
}
impl GetChat {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for GetChat {
	type Response = ChatFullInfo;
	const METHOD_NAME: &str = "getChat";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of [ChatMember](https://core.telegram.org/bots/api/#chatmember) objects.

https://core.telegram.org/bots/api/#getchatadministrators*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetChatAdministrators {
	/**Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
}
impl GetChatAdministrators {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for GetChatAdministrators {
	type Response = Vec<ChatMember>;
	const METHOD_NAME: &str = "getChatAdministrators";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a [ChatMember](https://core.telegram.org/bots/api/#chatmember) object on success.

https://core.telegram.org/bots/api/#getchatmember*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetChatMember {
	/**Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl GetChatMember {
	pub fn new(chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			user_id: user_id.into(),
		}
	}
}
impl Executable for GetChatMember {
	type Response = ChatMember;
	const METHOD_NAME: &str = "getChatMember";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to get the number of members in a chat. Returns *Int* on success.

https://core.telegram.org/bots/api/#getchatmembercount*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetChatMemberCount {
	/**Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
}
impl GetChatMemberCount {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for GetChatMemberCount {
	type Response = i64;
	const METHOD_NAME: &str = "getChatMemberCount";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns [MenuButton](https://core.telegram.org/bots/api/#menubutton) on success.

https://core.telegram.org/bots/api/#getchatmenubutton*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetChatMenuButton {
	/**Unique identifier for the target private chat. If not specified, default bot's menu button will be returned*/
	pub chat_id: Option<i64>,
}
impl GetChatMenuButton {
	pub fn new() -> Self {
		Self {
			chat_id: None,
		}
	}
	/** Unique identifier for the target private chat. If not specified, default bot's menu button will be returned*/
	pub fn chat_id(mut self, chat_id: impl Into<i64>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
}
impl Executable for GetChatMenuButton {
	type Response = MenuButton;
	const METHOD_NAME: &str = "getChatMenuButton";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_i64("chat_id", self.chat_id);
		parts
	}
}
/**Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.

https://core.telegram.org/bots/api/#getcustomemojistickers*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetCustomEmojiStickers {
	/**A JSON-serialized list of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.*/
	pub custom_emoji_ids: Vec<String>,
}
impl GetCustomEmojiStickers {
	pub fn new(custom_emoji_ids: impl Into<Vec<String>>) -> Self {
		Self {
			custom_emoji_ids: custom_emoji_ids.into(),
		}
	}
	pub fn add_custom_emoji_id(mut self, custom_emoji_id: impl Into<String>) -> Self {
		self.custom_emoji_ids.push(custom_emoji_id.into());
		self
	}
}
impl Executable for GetCustomEmojiStickers {
	type Response = Vec<Sticker>;
	const METHOD_NAME: &str = "getCustomEmojiStickers";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		if self.custom_emoji_ids.len() > 0 { parts.add_object("custom_emoji_ids", self.custom_emoji_ids) }
		parts
	}
}
/**Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a [File](https://core.telegram.org/bots/api/#file) object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api/#getfile) again.

https://core.telegram.org/bots/api/#getfile*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetFile {
	/**File identifier to get information about*/
	pub file_id: String,
}
impl GetFile {
	pub fn new(file_id: impl Into<String>) -> Self {
		Self {
			file_id: file_id.into(),
		}
	}
}
impl Executable for GetFile {
	type Response = File;
	const METHOD_NAME: &str = "getFile";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("file_id", self.file_id);
		parts
	}
}
/**Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.

https://core.telegram.org/bots/api/#getforumtopiciconstickers*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetForumTopicIconStickers {
}
impl GetForumTopicIconStickers {
	pub fn new() -> Self {
		Self {
		}
	}
}
impl Executable for GetForumTopicIconStickers {
	type Response = Vec<Sticker>;
	const METHOD_NAME: &str = "getForumTopicIconStickers";
	fn into_parts(self) -> FormParts {
		FormParts::new(0)
	}
}
/**Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of [GameHighScore](https://core.telegram.org/bots/api/#gamehighscore) objects.

This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and their neighbors are not among them. Please note that this behavior is subject to change.

https://core.telegram.org/bots/api/#getgamehighscores*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetGameHighScores {
	/**Required if *inline\_message\_id* is not specified. Unique identifier for the target chat*/
	pub chat_id: Option<i64>,
	/**Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub inline_message_id: Option<String>,
	/**Required if *inline\_message\_id* is not specified. Identifier of the sent message*/
	pub message_id: Option<i64>,
	/**Target user id*/
	pub user_id: i64,
}
impl GetGameHighScores {
	pub fn new(user_id: impl Into<i64>) -> Self {
		Self {
			chat_id: None,
			inline_message_id: None,
			message_id: None,
			user_id: user_id.into(),
		}
	}
	/** Required if *inline\_message\_id* is not specified. Unique identifier for the target chat*/
	pub fn chat_id(mut self, chat_id: impl Into<i64>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
	/** Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
		self.inline_message_id = Some(inline_message_id.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Identifier of the sent message*/
	pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_id = Some(message_id.into());
		self
	}
}
impl Executable for GetGameHighScores {
	type Response = Vec<GameHighScore>;
	const METHOD_NAME: &str = "getGameHighScores";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_i64("chat_id", self.chat_id);
		parts.add_string("inline_message_id", self.inline_message_id);
		parts.add_i64("message_id", self.message_id);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a [User](https://core.telegram.org/bots/api/#user) object.

https://core.telegram.org/bots/api/#getme*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetMe {
}
impl GetMe {
	pub fn new() -> Self {
		Self {
		}
	}
}
impl Executable for GetMe {
	type Response = User;
	const METHOD_NAME: &str = "getMe";
	fn into_parts(self) -> FormParts {
		FormParts::new(0)
	}
}
/**Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of [BotCommand](https://core.telegram.org/bots/api/#botcommand) objects. If commands aren't set, an empty list is returned.

https://core.telegram.org/bots/api/#getmycommands*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetMyCommands {
	/**A two-letter ISO 639-1 language code or an empty string*/
	pub language_code: Option<String>,
	/**A JSON-serialized object, describing scope of users. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault).*/
	pub scope: Option<BotCommandScope>,
}
impl GetMyCommands {
	pub fn new() -> Self {
		Self {
			language_code: None,
			scope: None,
		}
	}
	/** A two-letter ISO 639-1 language code or an empty string*/
	pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
		self.language_code = Some(language_code.into());
		self
	}
	/** A JSON-serialized object, describing scope of users. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault).*/
	pub fn scope(mut self, scope: impl Into<BotCommandScope>) -> Self {
		self.scope = Some(scope.into());
		self
	}
}
impl Executable for GetMyCommands {
	type Response = Vec<BotCommand>;
	const METHOD_NAME: &str = "getMyCommands";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("language_code", self.language_code);
		if let Some(scope) = self.scope { parts.add_object("scope", scope); }
		parts
	}
}
/**Use this method to get the current default administrator rights of the bot. Returns [ChatAdministratorRights](https://core.telegram.org/bots/api/#chatadministratorrights) on success.

https://core.telegram.org/bots/api/#getmydefaultadministratorrights*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetMyDefaultAdministratorRights {
	/**Pass *True* to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.*/
	pub for_channels: Option<bool>,
}
impl GetMyDefaultAdministratorRights {
	pub fn new() -> Self {
		Self {
			for_channels: None,
		}
	}
	/** Pass *True* to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.*/
	pub fn for_channels(mut self, for_channels: bool) -> Self {
		self.for_channels = Some(for_channels);
		self
	}
}
impl Executable for GetMyDefaultAdministratorRights {
	type Response = ChatAdministratorRights;
	const METHOD_NAME: &str = "getMyDefaultAdministratorRights";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_bool("for_channels", self.for_channels);
		parts
	}
}
/**Use this method to get the current bot description for the given user language. Returns [BotDescription](https://core.telegram.org/bots/api/#botdescription) on success.

https://core.telegram.org/bots/api/#getmydescription*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetMyDescription {
	/**A two-letter ISO 639-1 language code or an empty string*/
	pub language_code: Option<String>,
}
impl GetMyDescription {
	pub fn new() -> Self {
		Self {
			language_code: None,
		}
	}
	/** A two-letter ISO 639-1 language code or an empty string*/
	pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
		self.language_code = Some(language_code.into());
		self
	}
}
impl Executable for GetMyDescription {
	type Response = BotDescription;
	const METHOD_NAME: &str = "getMyDescription";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("language_code", self.language_code);
		parts
	}
}
/**Use this method to get the current bot name for the given user language. Returns [BotName](https://core.telegram.org/bots/api/#botname) on success.

https://core.telegram.org/bots/api/#getmyname*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetMyName {
	/**A two-letter ISO 639-1 language code or an empty string*/
	pub language_code: Option<String>,
}
impl GetMyName {
	pub fn new() -> Self {
		Self {
			language_code: None,
		}
	}
	/** A two-letter ISO 639-1 language code or an empty string*/
	pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
		self.language_code = Some(language_code.into());
		self
	}
}
impl Executable for GetMyName {
	type Response = BotName;
	const METHOD_NAME: &str = "getMyName";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("language_code", self.language_code);
		parts
	}
}
/**Use this method to get the current bot short description for the given user language. Returns [BotShortDescription](https://core.telegram.org/bots/api/#botshortdescription) on success.

https://core.telegram.org/bots/api/#getmyshortdescription*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetMyShortDescription {
	/**A two-letter ISO 639-1 language code or an empty string*/
	pub language_code: Option<String>,
}
impl GetMyShortDescription {
	pub fn new() -> Self {
		Self {
			language_code: None,
		}
	}
	/** A two-letter ISO 639-1 language code or an empty string*/
	pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
		self.language_code = Some(language_code.into());
		self
	}
}
impl Executable for GetMyShortDescription {
	type Response = BotShortDescription;
	const METHOD_NAME: &str = "getMyShortDescription";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("language_code", self.language_code);
		parts
	}
}
/**Returns the bot's Telegram Star transactions in chronological order. On success, returns a [StarTransactions](https://core.telegram.org/bots/api/#startransactions) object.

https://core.telegram.org/bots/api/#getstartransactions*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetStarTransactions {
	/**The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.*/
	pub limit: Option<i64>,
	/**Number of transactions to skip in the response*/
	pub offset: Option<i64>,
}
impl GetStarTransactions {
	pub fn new() -> Self {
		Self {
			limit: None,
			offset: None,
		}
	}
	/** The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.*/
	pub fn limit(mut self, limit: impl Into<i64>) -> Self {
		self.limit = Some(limit.into());
		self
	}
	/** Number of transactions to skip in the response*/
	pub fn offset(mut self, offset: impl Into<i64>) -> Self {
		self.offset = Some(offset.into());
		self
	}
}
impl Executable for GetStarTransactions {
	type Response = StarTransactions;
	const METHOD_NAME: &str = "getStarTransactions";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_i64("limit", self.limit);
		parts.add_i64("offset", self.offset);
		parts
	}
}
/**Use this method to get a sticker set. On success, a [StickerSet](https://core.telegram.org/bots/api/#stickerset) object is returned.

https://core.telegram.org/bots/api/#getstickerset*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetStickerSet {
	/**Name of the sticker set*/
	pub name: String,
}
impl GetStickerSet {
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
		}
	}
}
impl Executable for GetStickerSet {
	type Response = StickerSet;
	const METHOD_NAME: &str = "getStickerSet";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("name", self.name);
		parts
	}
}
/**Use this method to receive incoming updates using long polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)). Returns an Array of [Update](https://core.telegram.org/bots/api/#update) objects.

https://core.telegram.org/bots/api/#getupdates*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetUpdates {
	/**A JSON-serialized list of the update types you want your bot to receive. For example, specify `["message", "edited_channel_post", "callback_query"]` to only receive updates of these types. See [Update](https://core.telegram.org/bots/api/#update) for a complete list of available update types. Specify an empty list to receive all update types except *chat\_member*, *message\_reaction*, and *message\_reaction\_count* (default). If not specified, the previous setting will be used.  

	Please note that this parameter doesn't affect updates created before the call to getUpdates, so unwanted updates may be received for a short period of time.*/
	pub allowed_updates: Vec<String>,
	/**Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.*/
	pub limit: Option<i64>,
	/**Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as [getUpdates](https://core.telegram.org/bots/api/#getupdates) is called with an *offset* higher than its *update\_id*. The negative offset can be specified to retrieve updates starting from *-offset* update from the end of the updates queue. All previous updates will be forgotten.*/
	pub offset: Option<i64>,
	/**Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.*/
	pub timeout: Option<i64>,
}
impl GetUpdates {
	pub fn new() -> Self {
		Self {
			allowed_updates: Vec::new(),
			limit: None,
			offset: None,
			timeout: None,
		}
	}
	pub fn add_allowed_update(mut self, allowed_update: impl Into<String>) -> Self {
		self.allowed_updates.push(allowed_update.into());
		self
	}
	/** A JSON-serialized list of the update types you want your bot to receive. For example, specify `["message", "edited_channel_post", "callback_query"]` to only receive updates of these types. See [Update](https://core.telegram.org/bots/api/#update) for a complete list of available update types. Specify an empty list to receive all update types except *chat\_member*, *message\_reaction*, and *message\_reaction\_count* (default). If not specified, the previous setting will be used.  

	Please note that this parameter doesn't affect updates created before the call to getUpdates, so unwanted updates may be received for a short period of time.*/
	pub fn allowed_updates(mut self, allowed_updates: impl Into<Vec<String>>) -> Self {
		self.allowed_updates = allowed_updates.into();
		self
	}
	/** Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.*/
	pub fn limit(mut self, limit: impl Into<i64>) -> Self {
		self.limit = Some(limit.into());
		self
	}
	/** Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as [getUpdates](https://core.telegram.org/bots/api/#getupdates) is called with an *offset* higher than its *update\_id*. The negative offset can be specified to retrieve updates starting from *-offset* update from the end of the updates queue. All previous updates will be forgotten.*/
	pub fn offset(mut self, offset: impl Into<i64>) -> Self {
		self.offset = Some(offset.into());
		self
	}
	/** Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.*/
	pub fn timeout(mut self, timeout: impl Into<i64>) -> Self {
		self.timeout = Some(timeout.into());
		self
	}
}
impl Executable for GetUpdates {
	type Response = Vec<Update>;
	const METHOD_NAME: &str = "getUpdates";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		if self.allowed_updates.len() > 0 { parts.add_object("allowed_updates", self.allowed_updates) }
		parts.add_i64("limit", self.limit);
		parts.add_i64("offset", self.offset);
		parts.add_i64("timeout", self.timeout);
		parts
	}
}
/**Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a [UserChatBoosts](https://core.telegram.org/bots/api/#userchatboosts) object.

https://core.telegram.org/bots/api/#getuserchatboosts*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetUserChatBoosts {
	/**Unique identifier for the chat or username of the channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl GetUserChatBoosts {
	pub fn new(chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			user_id: user_id.into(),
		}
	}
}
impl Executable for GetUserChatBoosts {
	type Response = UserChatBoosts;
	const METHOD_NAME: &str = "getUserChatBoosts";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to get a list of profile pictures for a user. Returns a [UserProfilePhotos](https://core.telegram.org/bots/api/#userprofilephotos) object.

https://core.telegram.org/bots/api/#getuserprofilephotos*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetUserProfilePhotos {
	/**Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.*/
	pub limit: Option<i64>,
	/**Sequential number of the first photo to be returned. By default, all photos are returned.*/
	pub offset: Option<i64>,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl GetUserProfilePhotos {
	pub fn new(user_id: impl Into<i64>) -> Self {
		Self {
			limit: None,
			offset: None,
			user_id: user_id.into(),
		}
	}
	/** Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.*/
	pub fn limit(mut self, limit: impl Into<i64>) -> Self {
		self.limit = Some(limit.into());
		self
	}
	/** Sequential number of the first photo to be returned. By default, all photos are returned.*/
	pub fn offset(mut self, offset: impl Into<i64>) -> Self {
		self.offset = Some(offset.into());
		self
	}
}
impl Executable for GetUserProfilePhotos {
	type Response = UserProfilePhotos;
	const METHOD_NAME: &str = "getUserProfilePhotos";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		parts.add_i64("limit", self.limit);
		parts.add_i64("offset", self.offset);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to get current webhook status. Requires no parameters. On success, returns a [WebhookInfo](https://core.telegram.org/bots/api/#webhookinfo) object. If the bot is using [getUpdates](https://core.telegram.org/bots/api/#getupdates), will return an object with the *url* field empty.

https://core.telegram.org/bots/api/#getwebhookinfo*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct GetWebhookInfo {
}
impl GetWebhookInfo {
	pub fn new() -> Self {
		Self {
		}
	}
}
impl Executable for GetWebhookInfo {
	type Response = WebhookInfo;
	const METHOD_NAME: &str = "getWebhookInfo";
	fn into_parts(self) -> FormParts {
		FormParts::new(0)
	}
}
/**Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. The topic will be automatically closed if it was open. Returns *True* on success.

https://core.telegram.org/bots/api/#hidegeneralforumtopic*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct HideGeneralForumTopic {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
}
impl HideGeneralForumTopic {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for HideGeneralForumTopic {
	type Response = bool;
	const METHOD_NAME: &str = "hideGeneralForumTopic";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method for your bot to leave a group, supergroup or channel. Returns *True* on success.

https://core.telegram.org/bots/api/#leavechat*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct LeaveChat {
	/**Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
}
impl LeaveChat {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for LeaveChat {
	type Response = bool;
	const METHOD_NAME: &str = "leaveChat";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to log out from the cloud Bot API server before launching the bot locally. You **must** log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns *True* on success. Requires no parameters.

https://core.telegram.org/bots/api/#logout*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct LogOut {
}
impl LogOut {
	pub fn new() -> Self {
		Self {
		}
	}
}
impl Executable for LogOut {
	type Response = bool;
	const METHOD_NAME: &str = "logOut";
	fn into_parts(self) -> FormParts {
		FormParts::new(0)
	}
}
/**Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.

https://core.telegram.org/bots/api/#pinchatmessage*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct PinChatMessage {
	/**Unique identifier of the business connection on behalf of which the message will be pinned*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Pass *True* if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.*/
	pub disable_notification: Option<bool>,
	/**Identifier of a message to pin*/
	pub message_id: i64,
}
impl PinChatMessage {
	pub fn new(chat_id: impl Into<ChatId>, message_id: impl Into<i64>) -> Self {
		Self {
			business_connection_id: None,
			chat_id: chat_id.into(),
			disable_notification: None,
			message_id: message_id.into(),
		}
	}
	/** Unique identifier of the business connection on behalf of which the message will be pinned*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Pass *True* if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
}
impl Executable for PinChatMessage {
	type Response = bool;
	const METHOD_NAME: &str = "pinChatMessage";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_i64("message_id", self.message_id);
		parts
	}
}
/**Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass *False* for all boolean parameters to demote a user. Returns *True* on success.

https://core.telegram.org/bots/api/#promotechatmember*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct PromoteChatMember {
	/**Pass *True* if the administrator can change chat title, photo and other settings*/
	pub can_change_info: Option<bool>,
	/**Pass *True* if the administrator can delete messages of other users*/
	pub can_delete_messages: Option<bool>,
	/**Pass *True* if the administrator can delete stories posted by other users*/
	pub can_delete_stories: Option<bool>,
	/**Pass *True* if the administrator can edit messages of other users and can pin messages; for channels only*/
	pub can_edit_messages: Option<bool>,
	/**Pass *True* if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive*/
	pub can_edit_stories: Option<bool>,
	/**Pass *True* if the administrator can invite new users to the chat*/
	pub can_invite_users: Option<bool>,
	/**Pass *True* if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages and ignore slow mode. Implied by any other administrator privilege.*/
	pub can_manage_chat: Option<bool>,
	/**Pass *True* if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only*/
	pub can_manage_topics: Option<bool>,
	/**Pass *True* if the administrator can manage video chats*/
	pub can_manage_video_chats: Option<bool>,
	/**Pass *True* if the administrator can pin messages; for supergroups only*/
	pub can_pin_messages: Option<bool>,
	/**Pass *True* if the administrator can post messages in the channel, or access channel statistics; for channels only*/
	pub can_post_messages: Option<bool>,
	/**Pass *True* if the administrator can post stories to the chat*/
	pub can_post_stories: Option<bool>,
	/**Pass *True* if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)*/
	pub can_promote_members: Option<bool>,
	/**Pass *True* if the administrator can restrict, ban or unban chat members, or access supergroup statistics*/
	pub can_restrict_members: Option<bool>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Pass *True* if the administrator's presence in the chat is hidden*/
	pub is_anonymous: Option<bool>,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl PromoteChatMember {
	pub fn new(chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
		Self {
			can_change_info: None,
			can_delete_messages: None,
			can_delete_stories: None,
			can_edit_messages: None,
			can_edit_stories: None,
			can_invite_users: None,
			can_manage_chat: None,
			can_manage_topics: None,
			can_manage_video_chats: None,
			can_pin_messages: None,
			can_post_messages: None,
			can_post_stories: None,
			can_promote_members: None,
			can_restrict_members: None,
			chat_id: chat_id.into(),
			is_anonymous: None,
			user_id: user_id.into(),
		}
	}
	/** Pass *True* if the administrator can change chat title, photo and other settings*/
	pub fn can_change_info(mut self, can_change_info: bool) -> Self {
		self.can_change_info = Some(can_change_info);
		self
	}
	/** Pass *True* if the administrator can delete messages of other users*/
	pub fn can_delete_messages(mut self, can_delete_messages: bool) -> Self {
		self.can_delete_messages = Some(can_delete_messages);
		self
	}
	/** Pass *True* if the administrator can delete stories posted by other users*/
	pub fn can_delete_stories(mut self, can_delete_stories: bool) -> Self {
		self.can_delete_stories = Some(can_delete_stories);
		self
	}
	/** Pass *True* if the administrator can edit messages of other users and can pin messages; for channels only*/
	pub fn can_edit_messages(mut self, can_edit_messages: bool) -> Self {
		self.can_edit_messages = Some(can_edit_messages);
		self
	}
	/** Pass *True* if the administrator can edit stories posted by other users, post stories to the chat page, pin chat stories, and access the chat's story archive*/
	pub fn can_edit_stories(mut self, can_edit_stories: bool) -> Self {
		self.can_edit_stories = Some(can_edit_stories);
		self
	}
	/** Pass *True* if the administrator can invite new users to the chat*/
	pub fn can_invite_users(mut self, can_invite_users: bool) -> Self {
		self.can_invite_users = Some(can_invite_users);
		self
	}
	/** Pass *True* if the administrator can access the chat event log, get boost list, see hidden supergroup and channel members, report spam messages and ignore slow mode. Implied by any other administrator privilege.*/
	pub fn can_manage_chat(mut self, can_manage_chat: bool) -> Self {
		self.can_manage_chat = Some(can_manage_chat);
		self
	}
	/** Pass *True* if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only*/
	pub fn can_manage_topics(mut self, can_manage_topics: bool) -> Self {
		self.can_manage_topics = Some(can_manage_topics);
		self
	}
	/** Pass *True* if the administrator can manage video chats*/
	pub fn can_manage_video_chats(mut self, can_manage_video_chats: bool) -> Self {
		self.can_manage_video_chats = Some(can_manage_video_chats);
		self
	}
	/** Pass *True* if the administrator can pin messages; for supergroups only*/
	pub fn can_pin_messages(mut self, can_pin_messages: bool) -> Self {
		self.can_pin_messages = Some(can_pin_messages);
		self
	}
	/** Pass *True* if the administrator can post messages in the channel, or access channel statistics; for channels only*/
	pub fn can_post_messages(mut self, can_post_messages: bool) -> Self {
		self.can_post_messages = Some(can_post_messages);
		self
	}
	/** Pass *True* if the administrator can post stories to the chat*/
	pub fn can_post_stories(mut self, can_post_stories: bool) -> Self {
		self.can_post_stories = Some(can_post_stories);
		self
	}
	/** Pass *True* if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by him)*/
	pub fn can_promote_members(mut self, can_promote_members: bool) -> Self {
		self.can_promote_members = Some(can_promote_members);
		self
	}
	/** Pass *True* if the administrator can restrict, ban or unban chat members, or access supergroup statistics*/
	pub fn can_restrict_members(mut self, can_restrict_members: bool) -> Self {
		self.can_restrict_members = Some(can_restrict_members);
		self
	}
	/** Pass *True* if the administrator's presence in the chat is hidden*/
	pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
		self.is_anonymous = Some(is_anonymous);
		self
	}
}
impl Executable for PromoteChatMember {
	type Response = bool;
	const METHOD_NAME: &str = "promoteChatMember";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(17);
		parts.add_bool("can_change_info", self.can_change_info);
		parts.add_bool("can_delete_messages", self.can_delete_messages);
		parts.add_bool("can_delete_stories", self.can_delete_stories);
		parts.add_bool("can_edit_messages", self.can_edit_messages);
		parts.add_bool("can_edit_stories", self.can_edit_stories);
		parts.add_bool("can_invite_users", self.can_invite_users);
		parts.add_bool("can_manage_chat", self.can_manage_chat);
		parts.add_bool("can_manage_topics", self.can_manage_topics);
		parts.add_bool("can_manage_video_chats", self.can_manage_video_chats);
		parts.add_bool("can_pin_messages", self.can_pin_messages);
		parts.add_bool("can_post_messages", self.can_post_messages);
		parts.add_bool("can_post_stories", self.can_post_stories);
		parts.add_bool("can_promote_members", self.can_promote_members);
		parts.add_bool("can_restrict_members", self.can_restrict_members);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("is_anonymous", self.is_anonymous);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Refunds a successful payment in [Telegram Stars](https://t.me/BotNews/90). Returns *True* on success.

https://core.telegram.org/bots/api/#refundstarpayment*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct RefundStarPayment {
	/**Telegram payment identifier*/
	pub telegram_payment_charge_id: String,
	/**Identifier of the user whose payment will be refunded*/
	pub user_id: i64,
}
impl RefundStarPayment {
	pub fn new(telegram_payment_charge_id: impl Into<String>, user_id: impl Into<i64>) -> Self {
		Self {
			telegram_payment_charge_id: telegram_payment_charge_id.into(),
			user_id: user_id.into(),
		}
	}
}
impl Executable for RefundStarPayment {
	type Response = bool;
	const METHOD_NAME: &str = "refundStarPayment";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("telegram_payment_charge_id", self.telegram_payment_charge_id);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Removes verification from a chat that is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#removechatverification*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct RemoveChatVerification {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
}
impl RemoveChatVerification {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for RemoveChatVerification {
	type Response = bool;
	const METHOD_NAME: &str = "removeChatVerification";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Removes verification from a user who is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#removeuserverification*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct RemoveUserVerification {
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl RemoveUserVerification {
	pub fn new(user_id: impl Into<i64>) -> Self {
		Self {
			user_id: user_id.into(),
		}
	}
}
impl Executable for RemoveUserVerification {
	type Response = bool;
	const METHOD_NAME: &str = "removeUserVerification";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.

https://core.telegram.org/bots/api/#reopenforumtopic*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct ReopenForumTopic {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**Unique identifier for the target message thread of the forum topic*/
	pub message_thread_id: i64,
}
impl ReopenForumTopic {
	pub fn new(chat_id: impl Into<ChatId>, message_thread_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			message_thread_id: message_thread_id.into(),
		}
	}
}
impl Executable for ReopenForumTopic {
	type Response = bool;
	const METHOD_NAME: &str = "reopenForumTopic";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts
	}
}
/**Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. The topic will be automatically unhidden if it was hidden. Returns *True* on success.

https://core.telegram.org/bots/api/#reopengeneralforumtopic*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct ReopenGeneralForumTopic {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
}
impl ReopenGeneralForumTopic {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for ReopenGeneralForumTopic {
	type Response = bool;
	const METHOD_NAME: &str = "reopenGeneralForumTopic";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling [deleteStickerFromSet](https://core.telegram.org/bots/api/#deletestickerfromset), then [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), then [setStickerPositionInSet](https://core.telegram.org/bots/api/#setstickerpositioninset). Returns *True* on success.

https://core.telegram.org/bots/api/#replacestickerinset*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct ReplaceStickerInSet {
	/**Sticker set name*/
	pub name: String,
	/**File identifier of the replaced sticker*/
	pub old_sticker: String,
	/**A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set remains unchanged.*/
	pub sticker: InputSticker,
	/**User identifier of the sticker set owner*/
	pub user_id: i64,
}
impl ReplaceStickerInSet {
	pub fn new(name: impl Into<String>, old_sticker: impl Into<String>, sticker: impl Into<InputSticker>, user_id: impl Into<i64>) -> Self {
		Self {
			name: name.into(),
			old_sticker: old_sticker.into(),
			sticker: sticker.into(),
			user_id: user_id.into(),
		}
	}
}
impl Executable for ReplaceStickerInSet {
	type Response = bool;
	const METHOD_NAME: &str = "replaceStickerInSet";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_string("name", self.name);
		parts.add_string("old_sticker", self.old_sticker);
		parts.add_attachable("sticker", self.sticker);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass *True* for all permissions to lift restrictions from a user. Returns *True* on success.

https://core.telegram.org/bots/api/#restrictchatmember*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct RestrictChatMember {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**A JSON-serialized object for new user permissions*/
	pub permissions: ChatPermissions,
	/**Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever*/
	pub until_date: Option<i64>,
	/**Pass *True* if chat permissions are set independently. Otherwise, the *can\_send\_other\_messages* and *can\_add\_web\_page\_previews* permissions will imply the *can\_send\_messages*, *can\_send\_audios*, *can\_send\_documents*, *can\_send\_photos*, *can\_send\_videos*, *can\_send\_video\_notes*, and *can\_send\_voice\_notes* permissions; the *can\_send\_polls* permission will imply the *can\_send\_messages* permission.*/
	pub use_independent_chat_permissions: Option<bool>,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl RestrictChatMember {
	pub fn new(chat_id: impl Into<ChatId>, permissions: impl Into<ChatPermissions>, user_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			permissions: permissions.into(),
			until_date: None,
			use_independent_chat_permissions: None,
			user_id: user_id.into(),
		}
	}
	/** Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever*/
	pub fn until_date(mut self, until_date: impl Into<i64>) -> Self {
		self.until_date = Some(until_date.into());
		self
	}
	/** Pass *True* if chat permissions are set independently. Otherwise, the *can\_send\_other\_messages* and *can\_add\_web\_page\_previews* permissions will imply the *can\_send\_messages*, *can\_send\_audios*, *can\_send\_documents*, *can\_send\_photos*, *can\_send\_videos*, *can\_send\_video\_notes*, and *can\_send\_voice\_notes* permissions; the *can\_send\_polls* permission will imply the *can\_send\_messages* permission.*/
	pub fn use_independent_chat_permissions(mut self, use_independent_chat_permissions: bool) -> Self {
		self.use_independent_chat_permissions = Some(use_independent_chat_permissions);
		self
	}
}
impl Executable for RestrictChatMember {
	type Response = bool;
	const METHOD_NAME: &str = "restrictChatMember";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(5);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_object("permissions", self.permissions);
		parts.add_i64("until_date", self.until_date);
		parts.add_bool("use_independent_chat_permissions", self.use_independent_chat_permissions);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.

https://core.telegram.org/bots/api/#revokechatinvitelink*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct RevokeChatInviteLink {
	/**Unique identifier of the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**The invite link to revoke*/
	pub invite_link: String,
}
impl RevokeChatInviteLink {
	pub fn new(chat_id: impl Into<ChatId>, invite_link: impl Into<String>) -> Self {
		Self {
			chat_id: chat_id.into(),
			invite_link: invite_link.into(),
		}
	}
}
impl Executable for RevokeChatInviteLink {
	type Response = ChatInviteLink;
	const METHOD_NAME: &str = "revokeChatInviteLink";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_string("invite_link", self.invite_link);
		parts
	}
}
/**Stores a message that can be sent by a user of a Mini App. Returns a [PreparedInlineMessage](https://core.telegram.org/bots/api/#preparedinlinemessage) object.

https://core.telegram.org/bots/api/#savepreparedinlinemessage*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SavePreparedInlineMessage {
	/**Pass *True* if the message can be sent to private chats with bots*/
	pub allow_bot_chats: Option<bool>,
	/**Pass *True* if the message can be sent to channel chats*/
	pub allow_channel_chats: Option<bool>,
	/**Pass *True* if the message can be sent to group and supergroup chats*/
	pub allow_group_chats: Option<bool>,
	/**Pass *True* if the message can be sent to private chats with users*/
	pub allow_user_chats: Option<bool>,
	/**A JSON-serialized object describing the message to be sent*/
	pub result: InlineQueryResult,
	/**Unique identifier of the target user that can use the prepared message*/
	pub user_id: i64,
}
impl SavePreparedInlineMessage {
	pub fn new(result: impl Into<InlineQueryResult>, user_id: impl Into<i64>) -> Self {
		Self {
			allow_bot_chats: None,
			allow_channel_chats: None,
			allow_group_chats: None,
			allow_user_chats: None,
			result: result.into(),
			user_id: user_id.into(),
		}
	}
	/** Pass *True* if the message can be sent to private chats with bots*/
	pub fn allow_bot_chats(mut self, allow_bot_chats: bool) -> Self {
		self.allow_bot_chats = Some(allow_bot_chats);
		self
	}
	/** Pass *True* if the message can be sent to channel chats*/
	pub fn allow_channel_chats(mut self, allow_channel_chats: bool) -> Self {
		self.allow_channel_chats = Some(allow_channel_chats);
		self
	}
	/** Pass *True* if the message can be sent to group and supergroup chats*/
	pub fn allow_group_chats(mut self, allow_group_chats: bool) -> Self {
		self.allow_group_chats = Some(allow_group_chats);
		self
	}
	/** Pass *True* if the message can be sent to private chats with users*/
	pub fn allow_user_chats(mut self, allow_user_chats: bool) -> Self {
		self.allow_user_chats = Some(allow_user_chats);
		self
	}
}
impl Executable for SavePreparedInlineMessage {
	type Response = PreparedInlineMessage;
	const METHOD_NAME: &str = "savePreparedInlineMessage";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(6);
		parts.add_bool("allow_bot_chats", self.allow_bot_chats);
		parts.add_bool("allow_channel_chats", self.allow_channel_chats);
		parts.add_bool("allow_group_chats", self.allow_group_chats);
		parts.add_bool("allow_user_chats", self.allow_user_chats);
		parts.add_object("result", self.result);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.

https://core.telegram.org/bots/api/#sendanimation*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendAnimation {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Animation to send. Pass a file\_id as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub animation: Asset,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Animation caption (may also be used when resending animation by *file\_id*), 0-1024 characters after entities parsing*/
	pub caption: Option<String>,
	/**A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Duration of sent animation in seconds*/
	pub duration: Option<i64>,
	/**Pass *True* if the animation needs to be covered with a spoiler animation*/
	pub has_spoiler: Option<bool>,
	/**Animation height*/
	pub height: Option<i64>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Mode for parsing entities in the animation caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub thumbnail: Option<Asset>,
	/**Animation width*/
	pub width: Option<i64>,
}
impl SendAnimation {
	pub fn new(animation: impl Into<Asset>, chat_id: impl Into<ChatId>) -> Self {
		Self {
			allow_paid_broadcast: None,
			animation: animation.into(),
			business_connection_id: None,
			caption: None,
			caption_entities: Vec::new(),
			chat_id: chat_id.into(),
			disable_notification: None,
			duration: None,
			has_spoiler: None,
			height: None,
			message_effect_id: None,
			message_thread_id: None,
			parse_mode: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			show_caption_above_media: None,
			thumbnail: None,
			width: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Animation caption (may also be used when resending animation by *file\_id*), 0-1024 characters after entities parsing*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Duration of sent animation in seconds*/
	pub fn duration(mut self, duration: impl Into<i64>) -> Self {
		self.duration = Some(duration.into());
		self
	}
	/** Pass *True* if the animation needs to be covered with a spoiler animation*/
	pub fn has_spoiler(mut self, has_spoiler: bool) -> Self {
		self.has_spoiler = Some(has_spoiler);
		self
	}
	/** Animation height*/
	pub fn height(mut self, height: impl Into<i64>) -> Self {
		self.height = Some(height.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Mode for parsing entities in the animation caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
	/** Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn thumbnail(mut self, thumbnail: impl Into<Asset>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
	/** Animation width*/
	pub fn width(mut self, width: impl Into<i64>) -> Self {
		self.width = Some(width.into());
		self
	}
}
impl Executable for SendAnimation {
	type Response = Message;
	const METHOD_NAME: &str = "sendAnimation";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(19);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_attachable("animation", self.animation);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("caption", self.caption);
		if self.caption_entities.len() > 0 { parts.add_object("caption_entities", self.caption_entities) }
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_i64("duration", self.duration);
		parts.add_bool("has_spoiler", self.has_spoiler);
		parts.add_i64("height", self.height);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_string("parse_mode", self.parse_mode);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts.add_bool("show_caption_above_media", self.show_caption_above_media);
		if let Some(thumbnail) = self.thumbnail { parts.add_attachable("thumbnail", thumbnail); }
		parts.add_i64("width", self.width);
		parts
	}
}
/**Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.

For sending voice messages, use the [sendVoice](https://core.telegram.org/bots/api/#sendvoice) method instead.

https://core.telegram.org/bots/api/#sendaudio*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendAudio {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Audio file to send. Pass a file\_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub audio: Asset,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Audio caption, 0-1024 characters after entities parsing*/
	pub caption: Option<String>,
	/**A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Duration of the audio in seconds*/
	pub duration: Option<i64>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Performer*/
	pub performer: Option<String>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub thumbnail: Option<Asset>,
	/**Track name*/
	pub title: Option<String>,
}
impl SendAudio {
	pub fn new(audio: impl Into<Asset>, chat_id: impl Into<ChatId>) -> Self {
		Self {
			allow_paid_broadcast: None,
			audio: audio.into(),
			business_connection_id: None,
			caption: None,
			caption_entities: Vec::new(),
			chat_id: chat_id.into(),
			disable_notification: None,
			duration: None,
			message_effect_id: None,
			message_thread_id: None,
			parse_mode: None,
			performer: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			thumbnail: None,
			title: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Audio caption, 0-1024 characters after entities parsing*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Duration of the audio in seconds*/
	pub fn duration(mut self, duration: impl Into<i64>) -> Self {
		self.duration = Some(duration.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Mode for parsing entities in the audio caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** Performer*/
	pub fn performer(mut self, performer: impl Into<String>) -> Self {
		self.performer = Some(performer.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
	/** Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn thumbnail(mut self, thumbnail: impl Into<Asset>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
	/** Track name*/
	pub fn title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}
}
impl Executable for SendAudio {
	type Response = Message;
	const METHOD_NAME: &str = "sendAudio";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(17);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_attachable("audio", self.audio);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("caption", self.caption);
		if self.caption_entities.len() > 0 { parts.add_object("caption_entities", self.caption_entities) }
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_i64("duration", self.duration);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_string("parse_mode", self.parse_mode);
		parts.add_string("performer", self.performer);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		if let Some(thumbnail) = self.thumbnail { parts.add_attachable("thumbnail", thumbnail); }
		parts.add_string("title", self.title);
		parts
	}
}
/**Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns *True* on success.

Example: The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [sendChatAction](https://core.telegram.org/bots/api/#sendchataction) with *action* = *upload\_photo*. The user will see a “sending photo” status for the bot.

We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.

https://core.telegram.org/bots/api/#sendchataction*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendChatAction {
	/**Type of action to broadcast. Choose one, depending on what the user is about to receive: *typing* for [text messages](https://core.telegram.org/bots/api/#sendmessage), *upload\_photo* for [photos](https://core.telegram.org/bots/api/#sendphoto), *record\_video* or *upload\_video* for [videos](https://core.telegram.org/bots/api/#sendvideo), *record\_voice* or *upload\_voice* for [voice notes](https://core.telegram.org/bots/api/#sendvoice), *upload\_document* for [general files](https://core.telegram.org/bots/api/#senddocument), *choose\_sticker* for [stickers](https://core.telegram.org/bots/api/#sendsticker), *find\_location* for [location data](https://core.telegram.org/bots/api/#sendlocation), *record\_video\_note* or *upload\_video\_note* for [video notes](https://core.telegram.org/bots/api/#sendvideonote).*/
	pub action: String,
	/**Unique identifier of the business connection on behalf of which the action will be sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Unique identifier for the target message thread; for supergroups only*/
	pub message_thread_id: Option<i64>,
}
impl SendChatAction {
	pub fn new(action: impl Into<String>, chat_id: impl Into<ChatId>) -> Self {
		Self {
			action: action.into(),
			business_connection_id: None,
			chat_id: chat_id.into(),
			message_thread_id: None,
		}
	}
	/** Unique identifier of the business connection on behalf of which the action will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Unique identifier for the target message thread; for supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
}
impl Executable for SendChatAction {
	type Response = bool;
	const METHOD_NAME: &str = "sendChatAction";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_string("action", self.action);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts
	}
}
/**Use this method to send phone contacts. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#sendcontact*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendContact {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Contact's first name*/
	pub first_name: String,
	/**Contact's last name*/
	pub last_name: Option<String>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Contact's phone number*/
	pub phone_number: String,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes*/
	pub vcard: Option<String>,
}
impl SendContact {
	pub fn new(chat_id: impl Into<ChatId>, first_name: impl Into<String>, phone_number: impl Into<String>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			chat_id: chat_id.into(),
			disable_notification: None,
			first_name: first_name.into(),
			last_name: None,
			message_effect_id: None,
			message_thread_id: None,
			phone_number: phone_number.into(),
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			vcard: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Contact's last name*/
	pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
		self.last_name = Some(last_name.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
	/** Additional data about the contact in the form of a [vCard](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes*/
	pub fn vcard(mut self, vcard: impl Into<String>) -> Self {
		self.vcard = Some(vcard.into());
		self
	}
}
impl Executable for SendContact {
	type Response = Message;
	const METHOD_NAME: &str = "sendContact";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(13);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_string("first_name", self.first_name);
		parts.add_string("last_name", self.last_name);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_string("phone_number", self.phone_number);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts.add_string("vcard", self.vcard);
		parts
	}
}
/**Use this method to send an animated emoji that will display a random value. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#senddice*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendDice {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Emoji on which the dice throw animation is based. Currently, must be one of “🎲”, “🎯”, “🏀”, “⚽”, “🎳”, or “🎰”. Dice can have values 1-6 for “🎲”, “🎯” and “🎳”, values 1-5 for “🏀” and “⚽”, and values 1-64 for “🎰”. Defaults to “🎲”*/
	pub emoji: Option<String>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Protects the contents of the sent message from forwarding*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
}
impl SendDice {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			chat_id: chat_id.into(),
			disable_notification: None,
			emoji: None,
			message_effect_id: None,
			message_thread_id: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Emoji on which the dice throw animation is based. Currently, must be one of “🎲”, “🎯”, “🏀”, “⚽”, “🎳”, or “🎰”. Dice can have values 1-6 for “🎲”, “🎯” and “🎳”, values 1-5 for “🏀” and “⚽”, and values 1-64 for “🎰”. Defaults to “🎲”*/
	pub fn emoji(mut self, emoji: impl Into<String>) -> Self {
		self.emoji = Some(emoji.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Protects the contents of the sent message from forwarding*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
}
impl Executable for SendDice {
	type Response = Message;
	const METHOD_NAME: &str = "sendDice";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(10);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_string("emoji", self.emoji);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts
	}
}
/**Use this method to send general files. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.

https://core.telegram.org/bots/api/#senddocument*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendDocument {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Document caption (may also be used when resending documents by *file\_id*), 0-1024 characters after entities parsing*/
	pub caption: Option<String>,
	/**A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Disables automatic server-side content type detection for files uploaded using multipart/form-data*/
	pub disable_content_type_detection: Option<bool>,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**File to send. Pass a file\_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub document: Asset,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub thumbnail: Option<Asset>,
}
impl SendDocument {
	pub fn new(chat_id: impl Into<ChatId>, document: impl Into<Asset>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			caption: None,
			caption_entities: Vec::new(),
			chat_id: chat_id.into(),
			disable_content_type_detection: None,
			disable_notification: None,
			document: document.into(),
			message_effect_id: None,
			message_thread_id: None,
			parse_mode: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			thumbnail: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Document caption (may also be used when resending documents by *file\_id*), 0-1024 characters after entities parsing*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** Disables automatic server-side content type detection for files uploaded using multipart/form-data*/
	pub fn disable_content_type_detection(mut self, disable_content_type_detection: bool) -> Self {
		self.disable_content_type_detection = Some(disable_content_type_detection);
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Mode for parsing entities in the document caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
	/** Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn thumbnail(mut self, thumbnail: impl Into<Asset>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
}
impl Executable for SendDocument {
	type Response = Message;
	const METHOD_NAME: &str = "sendDocument";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(15);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("caption", self.caption);
		if self.caption_entities.len() > 0 { parts.add_object("caption_entities", self.caption_entities) }
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_content_type_detection", self.disable_content_type_detection);
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_attachable("document", self.document);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_string("parse_mode", self.parse_mode);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		if let Some(thumbnail) = self.thumbnail { parts.add_attachable("thumbnail", thumbnail); }
		parts
	}
}
/**Use this method to send a game. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#sendgame*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendGame {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat*/
	pub chat_id: i64,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Short name of the game, serves as the unique identifier for the game. Set up your games via [@BotFather](https://t.me/botfather).*/
	pub game_short_name: String,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If empty, one 'Play game\_title' button will be shown. If not empty, the first button must launch the game.*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
}
impl SendGame {
	pub fn new(chat_id: impl Into<i64>, game_short_name: impl Into<String>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			chat_id: chat_id.into(),
			disable_notification: None,
			game_short_name: game_short_name.into(),
			message_effect_id: None,
			message_thread_id: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If empty, one 'Play game\_title' button will be shown. If not empty, the first button must launch the game.*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
}
impl Executable for SendGame {
	type Response = Message;
	const METHOD_NAME: &str = "sendGame";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(10);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_i64("chat_id", self.chat_id);
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_string("game_short_name", self.game_short_name);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts
	}
}
/**Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns *True* on success.

https://core.telegram.org/bots/api/#sendgift*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendGift {
	/**Required if *user\_id* is not specified. Unique identifier for the chat or username of the channel (in the format `@channelusername`) that will receive the gift.*/
	pub chat_id: Option<ChatId>,
	/**Identifier of the gift*/
	pub gift_id: String,
	/**Pass *True* to pay for the gift upgrade from the bot's balance, thereby making the upgrade free for the receiver*/
	pub pay_for_upgrade: Option<bool>,
	/**Text that will be shown along with the gift; 0-128 characters*/
	pub text: Option<String>,
	/**A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of *text\_parse\_mode*. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\_emoji” are ignored.*/
	pub text_entities: Vec<MessageEntity>,
	/**Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\_emoji” are ignored.*/
	pub text_parse_mode: Option<String>,
	/**Required if *chat\_id* is not specified. Unique identifier of the target user who will receive the gift.*/
	pub user_id: Option<i64>,
}
impl SendGift {
	pub fn new(gift_id: impl Into<String>) -> Self {
		Self {
			chat_id: None,
			gift_id: gift_id.into(),
			pay_for_upgrade: None,
			text: None,
			text_entities: Vec::new(),
			text_parse_mode: None,
			user_id: None,
		}
	}
	/** Required if *user\_id* is not specified. Unique identifier for the chat or username of the channel (in the format `@channelusername`) that will receive the gift.*/
	pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
	/** Pass *True* to pay for the gift upgrade from the bot's balance, thereby making the upgrade free for the receiver*/
	pub fn pay_for_upgrade(mut self, pay_for_upgrade: bool) -> Self {
		self.pay_for_upgrade = Some(pay_for_upgrade);
		self
	}
	/** Text that will be shown along with the gift; 0-128 characters*/
	pub fn text(mut self, text: impl Into<String>) -> Self {
		self.text = Some(text.into());
		self
	}
	pub fn add_text_entity(mut self, text_entity: impl Into<MessageEntity>) -> Self {
		self.text_entities.push(text_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of *text\_parse\_mode*. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\_emoji” are ignored.*/
	pub fn text_entities(mut self, text_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.text_entities = text_entities.into();
		self
	}
	/** Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\_emoji” are ignored.*/
	pub fn text_parse_mode(mut self, text_parse_mode: impl Into<String>) -> Self {
		self.text_parse_mode = Some(text_parse_mode.into());
		self
	}
	/** Required if *chat\_id* is not specified. Unique identifier of the target user who will receive the gift.*/
	pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
		self.user_id = Some(user_id.into());
		self
	}
}
impl Executable for SendGift {
	type Response = bool;
	const METHOD_NAME: &str = "sendGift";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(7);
		parts.add_string("chat_id", self.chat_id.map(|x| x.to_string()));
		parts.add_string("gift_id", self.gift_id);
		parts.add_bool("pay_for_upgrade", self.pay_for_upgrade);
		parts.add_string("text", self.text);
		if self.text_entities.len() > 0 { parts.add_object("text_entities", self.text_entities) }
		parts.add_string("text_parse_mode", self.text_parse_mode);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to send invoices. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#sendinvoice*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendInvoice {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub currency: String,
	/**Product description, 1-255 characters*/
	pub description: String,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub is_flexible: Option<bool>,
	/**The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub max_tip_amount: Option<i64>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_email: Option<bool>,
	/**Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_name: Option<bool>,
	/**Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_phone_number: Option<bool>,
	/**Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub need_shipping_address: Option<bool>,
	/**Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.*/
	pub payload: String,
	/**Photo height*/
	pub photo_height: Option<i64>,
	/**Photo size in bytes*/
	pub photo_size: Option<i64>,
	/**URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.*/
	pub photo_url: Option<String>,
	/**Photo width*/
	pub photo_width: Option<i64>,
	/**Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub prices: Vec<LabeledPrice>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.*/
	pub provider_data: Option<String>,
	/**Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub provider_token: Option<String>,
	/**A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If empty, one 'Pay `total price`' button will be shown. If not empty, the first button must be a Pay button.*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub send_email_to_provider: Option<bool>,
	/**Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub send_phone_number_to_provider: Option<bool>,
	/**Unique deep-linking parameter. If left empty, **forwarded copies** of the sent message will have a *Pay* button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a *URL* button with a deep link to the bot (instead of a *Pay* button), with the value used as the start parameter*/
	pub start_parameter: Option<String>,
	/**A JSON-serialized array of suggested amounts of tips in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\_tip\_amount*.*/
	pub suggested_tip_amounts: Vec<i64>,
	/**Product name, 1-32 characters*/
	pub title: String,
}
impl SendInvoice {
	pub fn new(chat_id: impl Into<ChatId>, currency: impl Into<String>, description: impl Into<String>, payload: impl Into<String>, prices: impl Into<Vec<LabeledPrice>>, title: impl Into<String>) -> Self {
		Self {
			allow_paid_broadcast: None,
			chat_id: chat_id.into(),
			currency: currency.into(),
			description: description.into(),
			disable_notification: None,
			is_flexible: None,
			max_tip_amount: None,
			message_effect_id: None,
			message_thread_id: None,
			need_email: None,
			need_name: None,
			need_phone_number: None,
			need_shipping_address: None,
			payload: payload.into(),
			photo_height: None,
			photo_size: None,
			photo_url: None,
			photo_width: None,
			prices: prices.into(),
			protect_content: None,
			provider_data: None,
			provider_token: None,
			reply_markup: None,
			reply_parameters: None,
			send_email_to_provider: None,
			send_phone_number_to_provider: None,
			start_parameter: None,
			suggested_tip_amounts: Vec::new(),
			title: title.into(),
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn is_flexible(mut self, is_flexible: bool) -> Self {
		self.is_flexible = Some(is_flexible);
		self
	}
	/** The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn max_tip_amount(mut self, max_tip_amount: impl Into<i64>) -> Self {
		self.max_tip_amount = Some(max_tip_amount.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_email(mut self, need_email: bool) -> Self {
		self.need_email = Some(need_email);
		self
	}
	/** Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_name(mut self, need_name: bool) -> Self {
		self.need_name = Some(need_name);
		self
	}
	/** Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_phone_number(mut self, need_phone_number: bool) -> Self {
		self.need_phone_number = Some(need_phone_number);
		self
	}
	/** Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn need_shipping_address(mut self, need_shipping_address: bool) -> Self {
		self.need_shipping_address = Some(need_shipping_address);
		self
	}
	/** Photo height*/
	pub fn photo_height(mut self, photo_height: impl Into<i64>) -> Self {
		self.photo_height = Some(photo_height.into());
		self
	}
	/** Photo size in bytes*/
	pub fn photo_size(mut self, photo_size: impl Into<i64>) -> Self {
		self.photo_size = Some(photo_size.into());
		self
	}
	/** URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.*/
	pub fn photo_url(mut self, photo_url: impl Into<String>) -> Self {
		self.photo_url = Some(photo_url.into());
		self
	}
	/** Photo width*/
	pub fn photo_width(mut self, photo_width: impl Into<i64>) -> Self {
		self.photo_width = Some(photo_width.into());
		self
	}
	pub fn add_price(mut self, price: impl Into<LabeledPrice>) -> Self {
		self.prices.push(price.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.*/
	pub fn provider_data(mut self, provider_data: impl Into<String>) -> Self {
		self.provider_data = Some(provider_data.into());
		self
	}
	/** Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn provider_token(mut self, provider_token: impl Into<String>) -> Self {
		self.provider_token = Some(provider_token.into());
		self
	}
	/** A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If empty, one 'Pay `total price`' button will be shown. If not empty, the first button must be a Pay button.*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
	/** Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn send_email_to_provider(mut self, send_email_to_provider: bool) -> Self {
		self.send_email_to_provider = Some(send_email_to_provider);
		self
	}
	/** Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).*/
	pub fn send_phone_number_to_provider(mut self, send_phone_number_to_provider: bool) -> Self {
		self.send_phone_number_to_provider = Some(send_phone_number_to_provider);
		self
	}
	/** Unique deep-linking parameter. If left empty, **forwarded copies** of the sent message will have a *Pay* button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a *URL* button with a deep link to the bot (instead of a *Pay* button), with the value used as the start parameter*/
	pub fn start_parameter(mut self, start_parameter: impl Into<String>) -> Self {
		self.start_parameter = Some(start_parameter.into());
		self
	}
	pub fn add_suggested_tip_amount(mut self, suggested_tip_amount: impl Into<i64>) -> Self {
		self.suggested_tip_amounts.push(suggested_tip_amount.into());
		self
	}
	/** A JSON-serialized array of suggested amounts of tips in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\_tip\_amount*.*/
	pub fn suggested_tip_amounts(mut self, suggested_tip_amounts: impl Into<Vec<i64>>) -> Self {
		self.suggested_tip_amounts = suggested_tip_amounts.into();
		self
	}
}
impl Executable for SendInvoice {
	type Response = Message;
	const METHOD_NAME: &str = "sendInvoice";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(29);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_string("currency", self.currency);
		parts.add_string("description", self.description);
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_bool("is_flexible", self.is_flexible);
		parts.add_i64("max_tip_amount", self.max_tip_amount);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_bool("need_email", self.need_email);
		parts.add_bool("need_name", self.need_name);
		parts.add_bool("need_phone_number", self.need_phone_number);
		parts.add_bool("need_shipping_address", self.need_shipping_address);
		parts.add_string("payload", self.payload);
		parts.add_i64("photo_height", self.photo_height);
		parts.add_i64("photo_size", self.photo_size);
		parts.add_string("photo_url", self.photo_url);
		parts.add_i64("photo_width", self.photo_width);
		if self.prices.len() > 0 { parts.add_object("prices", self.prices) }
		parts.add_bool("protect_content", self.protect_content);
		parts.add_string("provider_data", self.provider_data);
		parts.add_string("provider_token", self.provider_token);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts.add_bool("send_email_to_provider", self.send_email_to_provider);
		parts.add_bool("send_phone_number_to_provider", self.send_phone_number_to_provider);
		parts.add_string("start_parameter", self.start_parameter);
		if self.suggested_tip_amounts.len() > 0 { parts.add_object("suggested_tip_amounts", self.suggested_tip_amounts) }
		parts.add_string("title", self.title);
		parts
	}
}
/**Use this method to send point on the map. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#sendlocation*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendLocation {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.*/
	pub heading: Option<i64>,
	/**The radius of uncertainty for the location, measured in meters; 0-1500*/
	pub horizontal_accuracy: Option<f32>,
	/**Latitude of the location*/
	pub latitude: f32,
	/**Period in seconds during which the location will be updated (see [Live Locations](https://telegram.org/blog/live-locations), should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.*/
	pub live_period: Option<i64>,
	/**Longitude of the location*/
	pub longitude: f32,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.*/
	pub proximity_alert_radius: Option<i64>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
}
impl SendLocation {
	pub fn new(chat_id: impl Into<ChatId>, latitude: impl Into<f32>, longitude: impl Into<f32>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			chat_id: chat_id.into(),
			disable_notification: None,
			heading: None,
			horizontal_accuracy: None,
			latitude: latitude.into(),
			live_period: None,
			longitude: longitude.into(),
			message_effect_id: None,
			message_thread_id: None,
			protect_content: None,
			proximity_alert_radius: None,
			reply_markup: None,
			reply_parameters: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.*/
	pub fn heading(mut self, heading: impl Into<i64>) -> Self {
		self.heading = Some(heading.into());
		self
	}
	/** The radius of uncertainty for the location, measured in meters; 0-1500*/
	pub fn horizontal_accuracy(mut self, horizontal_accuracy: impl Into<f32>) -> Self {
		self.horizontal_accuracy = Some(horizontal_accuracy.into());
		self
	}
	/** Period in seconds during which the location will be updated (see [Live Locations](https://telegram.org/blog/live-locations), should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.*/
	pub fn live_period(mut self, live_period: impl Into<i64>) -> Self {
		self.live_period = Some(live_period.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.*/
	pub fn proximity_alert_radius(mut self, proximity_alert_radius: impl Into<i64>) -> Self {
		self.proximity_alert_radius = Some(proximity_alert_radius.into());
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
}
impl Executable for SendLocation {
	type Response = Message;
	const METHOD_NAME: &str = "sendLocation";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(15);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_i64("heading", self.heading);
		parts.add_f32("horizontal_accuracy", self.horizontal_accuracy);
		parts.add_f32("latitude", self.latitude);
		parts.add_i64("live_period", self.live_period);
		parts.add_f32("longitude", self.longitude);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_bool("protect_content", self.protect_content);
		parts.add_i64("proximity_alert_radius", self.proximity_alert_radius);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts
	}
}
/**Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of [Messages](https://core.telegram.org/bots/api/#message) that were sent is returned.

https://core.telegram.org/bots/api/#sendmediagroup*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendMediaGroup {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**A JSON-serialized array describing messages to be sent, must include 2-10 items*/
	pub media: Vec<Media>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Protects the contents of the sent messages from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
}
impl SendMediaGroup {
	pub fn new(chat_id: impl Into<ChatId>, media: impl Into<Vec<Media>>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			chat_id: chat_id.into(),
			disable_notification: None,
			media: media.into(),
			message_effect_id: None,
			message_thread_id: None,
			protect_content: None,
			reply_parameters: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Sends messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	pub fn add_media(mut self, media: impl Into<Media>) -> Self {
		self.media.push(media.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Protects the contents of the sent messages from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
}
impl Executable for SendMediaGroup {
	type Response = Vec<Message>;
	const METHOD_NAME: &str = "sendMediaGroup";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(9);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_attachable("media", self.media);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts
	}
}
/**Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#sendmessage*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendMessage {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse\_mode**/
	pub entities: Vec<MessageEntity>,
	/**Link preview generation options for the message*/
	pub link_preview_options: Option<LinkPreviewOptions>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Text of the message to be sent, 1-4096 characters after entities parsing*/
	pub text: String,
}
impl SendMessage {
	pub fn new(chat_id: impl Into<ChatId>, text: impl Into<String>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			chat_id: chat_id.into(),
			disable_notification: None,
			entities: Vec::new(),
			link_preview_options: None,
			message_effect_id: None,
			message_thread_id: None,
			parse_mode: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			text: text.into(),
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	pub fn add_entity(mut self, entity: impl Into<MessageEntity>) -> Self {
		self.entities.push(entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in message text, which can be specified instead of *parse\_mode**/
	pub fn entities(mut self, entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.entities = entities.into();
		self
	}
	/** Link preview generation options for the message*/
	pub fn link_preview_options(mut self, link_preview_options: impl Into<LinkPreviewOptions>) -> Self {
		self.link_preview_options = Some(link_preview_options.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Mode for parsing entities in the message text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
}
impl Executable for SendMessage {
	type Response = Message;
	const METHOD_NAME: &str = "sendMessage";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(13);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		if self.entities.len() > 0 { parts.add_object("entities", self.entities) }
		if let Some(link_preview_options) = self.link_preview_options { parts.add_object("link_preview_options", link_preview_options); }
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_string("parse_mode", self.parse_mode);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts.add_string("text", self.text);
		parts
	}
}
/**Use this method to send paid media. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#sendpaidmedia*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendPaidMedia {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Media caption, 0-1024 characters after entities parsing*/
	pub caption: Option<String>,
	/**A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`). If the chat is a channel, all Telegram Star proceeds from this media will be credited to the chat's balance. Otherwise, they will be credited to the bot's balance.*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**A JSON-serialized array describing the media to be sent; up to 10 items*/
	pub media: Vec<InputPaidMedia>,
	/**Mode for parsing entities in the media caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Bot-defined paid media payload, 0-128 bytes. This will not be displayed to the user, use it for your internal processes.*/
	pub payload: Option<String>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**The number of Telegram Stars that must be paid to buy access to the media; 1-2500*/
	pub star_count: i64,
}
impl SendPaidMedia {
	pub fn new(chat_id: impl Into<ChatId>, media: impl Into<Vec<InputPaidMedia>>, star_count: impl Into<i64>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			caption: None,
			caption_entities: Vec::new(),
			chat_id: chat_id.into(),
			disable_notification: None,
			media: media.into(),
			parse_mode: None,
			payload: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			show_caption_above_media: None,
			star_count: star_count.into(),
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Media caption, 0-1024 characters after entities parsing*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	pub fn add_media(mut self, media: impl Into<InputPaidMedia>) -> Self {
		self.media.push(media.into());
		self
	}
	/** Mode for parsing entities in the media caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** Bot-defined paid media payload, 0-128 bytes. This will not be displayed to the user, use it for your internal processes.*/
	pub fn payload(mut self, payload: impl Into<String>) -> Self {
		self.payload = Some(payload.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
	/** Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
}
impl Executable for SendPaidMedia {
	type Response = Message;
	const METHOD_NAME: &str = "sendPaidMedia";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(14);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("caption", self.caption);
		if self.caption_entities.len() > 0 { parts.add_object("caption_entities", self.caption_entities) }
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_attachable("media", self.media);
		parts.add_string("parse_mode", self.parse_mode);
		parts.add_string("payload", self.payload);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts.add_bool("show_caption_above_media", self.show_caption_above_media);
		parts.add_i64("star_count", self.star_count);
		parts
	}
}
/**Use this method to send photos. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#sendphoto*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendPhoto {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Photo caption (may also be used when resending photos by *file\_id*), 0-1024 characters after entities parsing*/
	pub caption: Option<String>,
	/**A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Pass *True* if the photo needs to be covered with a spoiler animation*/
	pub has_spoiler: Option<bool>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Photo to send. Pass a file\_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub photo: Asset,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
}
impl SendPhoto {
	pub fn new(chat_id: impl Into<ChatId>, photo: impl Into<Asset>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			caption: None,
			caption_entities: Vec::new(),
			chat_id: chat_id.into(),
			disable_notification: None,
			has_spoiler: None,
			message_effect_id: None,
			message_thread_id: None,
			parse_mode: None,
			photo: photo.into(),
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			show_caption_above_media: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Photo caption (may also be used when resending photos by *file\_id*), 0-1024 characters after entities parsing*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Pass *True* if the photo needs to be covered with a spoiler animation*/
	pub fn has_spoiler(mut self, has_spoiler: bool) -> Self {
		self.has_spoiler = Some(has_spoiler);
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Mode for parsing entities in the photo caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
	/** Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
}
impl Executable for SendPhoto {
	type Response = Message;
	const METHOD_NAME: &str = "sendPhoto";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(15);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("caption", self.caption);
		if self.caption_entities.len() > 0 { parts.add_object("caption_entities", self.caption_entities) }
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_bool("has_spoiler", self.has_spoiler);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_string("parse_mode", self.parse_mode);
		parts.add_attachable("photo", self.photo);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts.add_bool("show_caption_above_media", self.show_caption_above_media);
		parts
	}
}
/**Use this method to send a native poll. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#sendpoll*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendPoll {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**if the poll allows multiple answers, ignored for polls in quiz mode, defaults to *False**/
	pub allows_multiple_answers: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with *open\_period*.*/
	pub close_date: Option<i64>,
	/**0-based identifier of the correct answer option, required for polls in quiz mode*/
	pub correct_option_id: Option<i64>,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing*/
	pub explanation: Option<String>,
	/**A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of *explanation\_parse\_mode**/
	pub explanation_entities: Vec<MessageEntity>,
	/**Mode for parsing entities in the explanation. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub explanation_parse_mode: Option<String>,
	/**if the poll needs to be anonymous, defaults to *True**/
	pub is_anonymous: Option<bool>,
	/**Pass *True* if the poll needs to be immediately closed. This can be useful for poll preview.*/
	pub is_closed: Option<bool>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with *close\_date*.*/
	pub open_period: Option<i64>,
	/**A JSON-serialized list of 2-10 answer options*/
	pub options: Vec<InputPollOption>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Poll question, 1-300 characters*/
	pub question: String,
	/**A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of *question\_parse\_mode**/
	pub question_entities: Vec<MessageEntity>,
	/**Mode for parsing entities in the question. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Currently, only custom emoji entities are allowed*/
	pub question_parse_mode: Option<String>,
	/**Poll type, “quiz” or “regular”, defaults to “regular”*/
	pub r#type: Option<String>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
}
impl SendPoll {
	pub fn new(chat_id: impl Into<ChatId>, options: impl Into<Vec<InputPollOption>>, question: impl Into<String>) -> Self {
		Self {
			allow_paid_broadcast: None,
			allows_multiple_answers: None,
			business_connection_id: None,
			chat_id: chat_id.into(),
			close_date: None,
			correct_option_id: None,
			disable_notification: None,
			explanation: None,
			explanation_entities: Vec::new(),
			explanation_parse_mode: None,
			is_anonymous: None,
			is_closed: None,
			message_effect_id: None,
			message_thread_id: None,
			open_period: None,
			options: options.into(),
			protect_content: None,
			question: question.into(),
			question_entities: Vec::new(),
			question_parse_mode: None,
			r#type: None,
			reply_markup: None,
			reply_parameters: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** *True*, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to *False**/
	pub fn allows_multiple_answers(mut self, allows_multiple_answers: bool) -> Self {
		self.allows_multiple_answers = Some(allows_multiple_answers);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with *open\_period*.*/
	pub fn close_date(mut self, close_date: impl Into<i64>) -> Self {
		self.close_date = Some(close_date.into());
		self
	}
	/** 0-based identifier of the correct answer option, required for polls in quiz mode*/
	pub fn correct_option_id(mut self, correct_option_id: impl Into<i64>) -> Self {
		self.correct_option_id = Some(correct_option_id.into());
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing*/
	pub fn explanation(mut self, explanation: impl Into<String>) -> Self {
		self.explanation = Some(explanation.into());
		self
	}
	pub fn add_explanation_entity(mut self, explanation_entity: impl Into<MessageEntity>) -> Self {
		self.explanation_entities.push(explanation_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of *explanation\_parse\_mode**/
	pub fn explanation_entities(mut self, explanation_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.explanation_entities = explanation_entities.into();
		self
	}
	/** Mode for parsing entities in the explanation. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn explanation_parse_mode(mut self, explanation_parse_mode: impl Into<String>) -> Self {
		self.explanation_parse_mode = Some(explanation_parse_mode.into());
		self
	}
	/** *True*, if the poll needs to be anonymous, defaults to *True**/
	pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
		self.is_anonymous = Some(is_anonymous);
		self
	}
	/** Pass *True* if the poll needs to be immediately closed. This can be useful for poll preview.*/
	pub fn is_closed(mut self, is_closed: bool) -> Self {
		self.is_closed = Some(is_closed);
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with *close\_date*.*/
	pub fn open_period(mut self, open_period: impl Into<i64>) -> Self {
		self.open_period = Some(open_period.into());
		self
	}
	pub fn add_option(mut self, option: impl Into<InputPollOption>) -> Self {
		self.options.push(option.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	pub fn add_question_entity(mut self, question_entity: impl Into<MessageEntity>) -> Self {
		self.question_entities.push(question_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of *question\_parse\_mode**/
	pub fn question_entities(mut self, question_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.question_entities = question_entities.into();
		self
	}
	/** Mode for parsing entities in the question. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Currently, only custom emoji entities are allowed*/
	pub fn question_parse_mode(mut self, question_parse_mode: impl Into<String>) -> Self {
		self.question_parse_mode = Some(question_parse_mode.into());
		self
	}
	/** Poll type, “quiz” or “regular”, defaults to “regular”*/
	pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
		self.r#type = Some(r#type.into());
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
}
impl Executable for SendPoll {
	type Response = Message;
	const METHOD_NAME: &str = "sendPoll";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(23);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_bool("allows_multiple_answers", self.allows_multiple_answers);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("close_date", self.close_date);
		parts.add_i64("correct_option_id", self.correct_option_id);
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_string("explanation", self.explanation);
		if self.explanation_entities.len() > 0 { parts.add_object("explanation_entities", self.explanation_entities) }
		parts.add_string("explanation_parse_mode", self.explanation_parse_mode);
		parts.add_bool("is_anonymous", self.is_anonymous);
		parts.add_bool("is_closed", self.is_closed);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_i64("open_period", self.open_period);
		if self.options.len() > 0 { parts.add_object("options", self.options) }
		parts.add_bool("protect_content", self.protect_content);
		parts.add_string("question", self.question);
		if self.question_entities.len() > 0 { parts.add_object("question_entities", self.question_entities) }
		parts.add_string("question_parse_mode", self.question_parse_mode);
		parts.add_string("r#type", self.r#type);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts
	}
}
/**Use this method to send static .WEBP, [animated](https://telegram.org/blog/animated-stickers) .TGS, or [video](https://telegram.org/blog/video-stickers-better-reactions) .WEBM stickers. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#sendsticker*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendSticker {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Emoji associated with the sticker; only for just uploaded stickers*/
	pub emoji: Option<String>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Sticker to send. Pass a file\_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP sticker from the Internet, or upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Video and animated stickers can't be sent via an HTTP URL.*/
	pub sticker: Asset,
}
impl SendSticker {
	pub fn new(chat_id: impl Into<ChatId>, sticker: impl Into<Asset>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			chat_id: chat_id.into(),
			disable_notification: None,
			emoji: None,
			message_effect_id: None,
			message_thread_id: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			sticker: sticker.into(),
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Emoji associated with the sticker; only for just uploaded stickers*/
	pub fn emoji(mut self, emoji: impl Into<String>) -> Self {
		self.emoji = Some(emoji.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
}
impl Executable for SendSticker {
	type Response = Message;
	const METHOD_NAME: &str = "sendSticker";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(11);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_string("emoji", self.emoji);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts.add_attachable("sticker", self.sticker);
		parts
	}
}
/**Use this method to send information about a venue. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#sendvenue*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendVenue {
	/**Address of the venue*/
	pub address: String,
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Foursquare identifier of the venue*/
	pub foursquare_id: Option<String>,
	/**Foursquare type of the venue, if known. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)*/
	pub foursquare_type: Option<String>,
	/**Google Places identifier of the venue*/
	pub google_place_id: Option<String>,
	/**Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)*/
	pub google_place_type: Option<String>,
	/**Latitude of the venue*/
	pub latitude: f32,
	/**Longitude of the venue*/
	pub longitude: f32,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Name of the venue*/
	pub title: String,
}
impl SendVenue {
	pub fn new(address: impl Into<String>, chat_id: impl Into<ChatId>, latitude: impl Into<f32>, longitude: impl Into<f32>, title: impl Into<String>) -> Self {
		Self {
			address: address.into(),
			allow_paid_broadcast: None,
			business_connection_id: None,
			chat_id: chat_id.into(),
			disable_notification: None,
			foursquare_id: None,
			foursquare_type: None,
			google_place_id: None,
			google_place_type: None,
			latitude: latitude.into(),
			longitude: longitude.into(),
			message_effect_id: None,
			message_thread_id: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			title: title.into(),
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Foursquare identifier of the venue*/
	pub fn foursquare_id(mut self, foursquare_id: impl Into<String>) -> Self {
		self.foursquare_id = Some(foursquare_id.into());
		self
	}
	/** Foursquare type of the venue, if known. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)*/
	pub fn foursquare_type(mut self, foursquare_type: impl Into<String>) -> Self {
		self.foursquare_type = Some(foursquare_type.into());
		self
	}
	/** Google Places identifier of the venue*/
	pub fn google_place_id(mut self, google_place_id: impl Into<String>) -> Self {
		self.google_place_id = Some(google_place_id.into());
		self
	}
	/** Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)*/
	pub fn google_place_type(mut self, google_place_type: impl Into<String>) -> Self {
		self.google_place_type = Some(google_place_type.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
}
impl Executable for SendVenue {
	type Response = Message;
	const METHOD_NAME: &str = "sendVenue";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(17);
		parts.add_string("address", self.address);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_string("foursquare_id", self.foursquare_id);
		parts.add_string("foursquare_type", self.foursquare_type);
		parts.add_string("google_place_id", self.google_place_id);
		parts.add_string("google_place_type", self.google_place_type);
		parts.add_f32("latitude", self.latitude);
		parts.add_f32("longitude", self.longitude);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts.add_string("title", self.title);
		parts
	}
}
/**Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.

https://core.telegram.org/bots/api/#sendvideo*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendVideo {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Video caption (may also be used when resending videos by *file\_id*), 0-1024 characters after entities parsing*/
	pub caption: Option<String>,
	/**A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Cover for the video in the message. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub cover: Option<Asset>,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Duration of sent video in seconds*/
	pub duration: Option<i64>,
	/**Pass *True* if the video needs to be covered with a spoiler animation*/
	pub has_spoiler: Option<bool>,
	/**Video height*/
	pub height: Option<i64>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Pass if the caption must be shown above the message media*/
	pub show_caption_above_media: Option<bool>,
	/**Start timestamp for the video in the message*/
	pub start_timestamp: Option<i64>,
	/**Pass *True* if the uploaded video is suitable for streaming*/
	pub supports_streaming: Option<bool>,
	/**Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub thumbnail: Option<Asset>,
	/**Video to send. Pass a file\_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub video: Asset,
	/**Video width*/
	pub width: Option<i64>,
}
impl SendVideo {
	pub fn new(chat_id: impl Into<ChatId>, video: impl Into<Asset>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			caption: None,
			caption_entities: Vec::new(),
			chat_id: chat_id.into(),
			cover: None,
			disable_notification: None,
			duration: None,
			has_spoiler: None,
			height: None,
			message_effect_id: None,
			message_thread_id: None,
			parse_mode: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			show_caption_above_media: None,
			start_timestamp: None,
			supports_streaming: None,
			thumbnail: None,
			video: video.into(),
			width: None,
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Video caption (may also be used when resending videos by *file\_id*), 0-1024 characters after entities parsing*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** Cover for the video in the message. Pass a file\_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://\<file\_attach\_name\>” to upload a new one using multipart/form-data under \<file\_attach\_name\> name. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn cover(mut self, cover: impl Into<Asset>) -> Self {
		self.cover = Some(cover.into());
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Duration of sent video in seconds*/
	pub fn duration(mut self, duration: impl Into<i64>) -> Self {
		self.duration = Some(duration.into());
		self
	}
	/** Pass *True* if the video needs to be covered with a spoiler animation*/
	pub fn has_spoiler(mut self, has_spoiler: bool) -> Self {
		self.has_spoiler = Some(has_spoiler);
		self
	}
	/** Video height*/
	pub fn height(mut self, height: impl Into<i64>) -> Self {
		self.height = Some(height.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Mode for parsing entities in the video caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
	/** Pass *True*, if the caption must be shown above the message media*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
		self
	}
	/** Start timestamp for the video in the message*/
	pub fn start_timestamp(mut self, start_timestamp: impl Into<i64>) -> Self {
		self.start_timestamp = Some(start_timestamp.into());
		self
	}
	/** Pass *True* if the uploaded video is suitable for streaming*/
	pub fn supports_streaming(mut self, supports_streaming: bool) -> Self {
		self.supports_streaming = Some(supports_streaming);
		self
	}
	/** Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn thumbnail(mut self, thumbnail: impl Into<Asset>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
	/** Video width*/
	pub fn width(mut self, width: impl Into<i64>) -> Self {
		self.width = Some(width.into());
		self
	}
}
impl Executable for SendVideo {
	type Response = Message;
	const METHOD_NAME: &str = "sendVideo";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(22);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("caption", self.caption);
		if self.caption_entities.len() > 0 { parts.add_object("caption_entities", self.caption_entities) }
		parts.add_string("chat_id", self.chat_id.to_string());
		if let Some(cover) = self.cover { parts.add_attachable("cover", cover); }
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_i64("duration", self.duration);
		parts.add_bool("has_spoiler", self.has_spoiler);
		parts.add_i64("height", self.height);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_string("parse_mode", self.parse_mode);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts.add_bool("show_caption_above_media", self.show_caption_above_media);
		parts.add_i64("start_timestamp", self.start_timestamp);
		parts.add_bool("supports_streaming", self.supports_streaming);
		if let Some(thumbnail) = self.thumbnail { parts.add_attachable("thumbnail", thumbnail); }
		parts.add_attachable("video", self.video);
		parts.add_i64("width", self.width);
		parts
	}
}
/**As of [v.4.0](https://telegram.org/blog/video-messages-and-telescope), Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.

https://core.telegram.org/bots/api/#sendvideonote*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendVideoNote {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Duration of sent video in seconds*/
	pub duration: Option<i64>,
	/**Video width and height, i.e. diameter of the video message*/
	pub length: Option<i64>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub thumbnail: Option<Asset>,
	/**Video note to send. Pass a file\_id as String to send a video note that exists on the Telegram servers (recommended) or upload a new video using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Sending video notes by a URL is currently unsupported*/
	pub video_note: Asset,
}
impl SendVideoNote {
	pub fn new(chat_id: impl Into<ChatId>, video_note: impl Into<Asset>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			chat_id: chat_id.into(),
			disable_notification: None,
			duration: None,
			length: None,
			message_effect_id: None,
			message_thread_id: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			thumbnail: None,
			video_note: video_note.into(),
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Duration of sent video in seconds*/
	pub fn duration(mut self, duration: impl Into<i64>) -> Self {
		self.duration = Some(duration.into());
		self
	}
	/** Video width and height, i.e. diameter of the video message*/
	pub fn length(mut self, length: impl Into<i64>) -> Self {
		self.length = Some(length.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
	/** Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://\<file\_attach\_name\>” if the thumbnail was uploaded using multipart/form-data under \<file\_attach\_name\>. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub fn thumbnail(mut self, thumbnail: impl Into<Asset>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
}
impl Executable for SendVideoNote {
	type Response = Message;
	const METHOD_NAME: &str = "sendVideoNote";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(13);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_i64("duration", self.duration);
		parts.add_i64("length", self.length);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		if let Some(thumbnail) = self.thumbnail { parts.add_attachable("thumbnail", thumbnail); }
		parts.add_attachable("video_note", self.video_note);
		parts
	}
}
/**Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS, or in .MP3 format, or in .M4A format (other formats may be sent as [Audio](https://core.telegram.org/bots/api/#audio) or [Document](https://core.telegram.org/bots/api/#document)). On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.

https://core.telegram.org/bots/api/#sendvoice*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SendVoice {
	/**Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub allow_paid_broadcast: Option<bool>,
	/**Unique identifier of the business connection on behalf of which the message will be sent*/
	pub business_connection_id: Option<String>,
	/**Voice message caption, 0-1024 characters after entities parsing*/
	pub caption: Option<String>,
	/**A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub caption_entities: Vec<MessageEntity>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub disable_notification: Option<bool>,
	/**Duration of the voice message in seconds*/
	pub duration: Option<i64>,
	/**Unique identifier of the message effect to be added to the message; for private chats only*/
	pub message_effect_id: Option<String>,
	/**Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub message_thread_id: Option<i64>,
	/**Mode for parsing entities in the voice message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub parse_mode: Option<String>,
	/**Protects the contents of the sent message from forwarding and saving*/
	pub protect_content: Option<bool>,
	/**Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub reply_markup: Option<ReplyMarkup>,
	/**Description of the message to reply to*/
	pub reply_parameters: Option<ReplyParameters>,
	/**Audio file to send. Pass a file\_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub voice: Asset,
}
impl SendVoice {
	pub fn new(chat_id: impl Into<ChatId>, voice: impl Into<Asset>) -> Self {
		Self {
			allow_paid_broadcast: None,
			business_connection_id: None,
			caption: None,
			caption_entities: Vec::new(),
			chat_id: chat_id.into(),
			disable_notification: None,
			duration: None,
			message_effect_id: None,
			message_thread_id: None,
			parse_mode: None,
			protect_content: None,
			reply_markup: None,
			reply_parameters: None,
			voice: voice.into(),
		}
	}
	/** Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance*/
	pub fn allow_paid_broadcast(mut self, allow_paid_broadcast: bool) -> Self {
		self.allow_paid_broadcast = Some(allow_paid_broadcast);
		self
	}
	/** Unique identifier of the business connection on behalf of which the message will be sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Voice message caption, 0-1024 characters after entities parsing*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode**/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.*/
	pub fn disable_notification(mut self, disable_notification: bool) -> Self {
		self.disable_notification = Some(disable_notification);
		self
	}
	/** Duration of the voice message in seconds*/
	pub fn duration(mut self, duration: impl Into<i64>) -> Self {
		self.duration = Some(duration.into());
		self
	}
	/** Unique identifier of the message effect to be added to the message; for private chats only*/
	pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
		self.message_effect_id = Some(message_effect_id.into());
		self
	}
	/** Unique identifier for the target message thread (topic) of the forum; for forum supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** Mode for parsing entities in the voice message caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.*/
	pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
		self.parse_mode = Some(parse_mode.into());
		self
	}
	/** Protects the contents of the sent message from forwarding and saving*/
	pub fn protect_content(mut self, protect_content: bool) -> Self {
		self.protect_content = Some(protect_content);
		self
	}
	/** Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user*/
	pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** Description of the message to reply to*/
	pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
		self.reply_parameters = Some(reply_parameters.into());
		self
	}
}
impl Executable for SendVoice {
	type Response = Message;
	const METHOD_NAME: &str = "sendVoice";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(14);
		parts.add_bool("allow_paid_broadcast", self.allow_paid_broadcast);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("caption", self.caption);
		if self.caption_entities.len() > 0 { parts.add_object("caption_entities", self.caption_entities) }
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("disable_notification", self.disable_notification);
		parts.add_i64("duration", self.duration);
		parts.add_string("message_effect_id", self.message_effect_id);
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts.add_string("parse_mode", self.parse_mode);
		parts.add_bool("protect_content", self.protect_content);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		if let Some(reply_parameters) = self.reply_parameters { parts.add_object("reply_parameters", reply_parameters); }
		parts.add_attachable("voice", self.voice);
		parts
	}
}
/**Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#setchatadministratorcustomtitle*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetChatAdministratorCustomTitle {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**New custom title for the administrator; 0-16 characters, emoji are not allowed*/
	pub custom_title: String,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl SetChatAdministratorCustomTitle {
	pub fn new(chat_id: impl Into<ChatId>, custom_title: impl Into<String>, user_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			custom_title: custom_title.into(),
			user_id: user_id.into(),
		}
	}
}
impl Executable for SetChatAdministratorCustomTitle {
	type Response = bool;
	const METHOD_NAME: &str = "setChatAdministratorCustomTitle";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_string("custom_title", self.custom_title);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#setchatdescription*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetChatDescription {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**New chat description, 0-255 characters*/
	pub description: Option<String>,
}
impl SetChatDescription {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
			description: None,
		}
	}
	/** New chat description, 0-255 characters*/
	pub fn description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}
}
impl Executable for SetChatDescription {
	type Response = bool;
	const METHOD_NAME: &str = "setChatDescription";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_string("description", self.description);
		parts
	}
}
/**Use this method to change the bot's menu button in a private chat, or the default menu button. Returns *True* on success.

https://core.telegram.org/bots/api/#setchatmenubutton*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetChatMenuButton {
	/**Unique identifier for the target private chat. If not specified, default bot's menu button will be changed*/
	pub chat_id: Option<i64>,
	/**A JSON-serialized object for the bot's new menu button. Defaults to [MenuButtonDefault](https://core.telegram.org/bots/api/#menubuttondefault)*/
	pub menu_button: Option<MenuButton>,
}
impl SetChatMenuButton {
	pub fn new() -> Self {
		Self {
			chat_id: None,
			menu_button: None,
		}
	}
	/** Unique identifier for the target private chat. If not specified, default bot's menu button will be changed*/
	pub fn chat_id(mut self, chat_id: impl Into<i64>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
	/** A JSON-serialized object for the bot's new menu button. Defaults to [MenuButtonDefault](https://core.telegram.org/bots/api/#menubuttondefault)*/
	pub fn menu_button(mut self, menu_button: impl Into<MenuButton>) -> Self {
		self.menu_button = Some(menu_button.into());
		self
	}
}
impl Executable for SetChatMenuButton {
	type Response = bool;
	const METHOD_NAME: &str = "setChatMenuButton";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_i64("chat_id", self.chat_id);
		if let Some(menu_button) = self.menu_button { parts.add_object("menu_button", menu_button); }
		parts
	}
}
/**Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the *can\_restrict\_members* administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#setchatpermissions*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetChatPermissions {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**A JSON-serialized object for new default chat permissions*/
	pub permissions: ChatPermissions,
	/**Pass *True* if chat permissions are set independently. Otherwise, the *can\_send\_other\_messages* and *can\_add\_web\_page\_previews* permissions will imply the *can\_send\_messages*, *can\_send\_audios*, *can\_send\_documents*, *can\_send\_photos*, *can\_send\_videos*, *can\_send\_video\_notes*, and *can\_send\_voice\_notes* permissions; the *can\_send\_polls* permission will imply the *can\_send\_messages* permission.*/
	pub use_independent_chat_permissions: Option<bool>,
}
impl SetChatPermissions {
	pub fn new(chat_id: impl Into<ChatId>, permissions: impl Into<ChatPermissions>) -> Self {
		Self {
			chat_id: chat_id.into(),
			permissions: permissions.into(),
			use_independent_chat_permissions: None,
		}
	}
	/** Pass *True* if chat permissions are set independently. Otherwise, the *can\_send\_other\_messages* and *can\_add\_web\_page\_previews* permissions will imply the *can\_send\_messages*, *can\_send\_audios*, *can\_send\_documents*, *can\_send\_photos*, *can\_send\_videos*, *can\_send\_video\_notes*, and *can\_send\_voice\_notes* permissions; the *can\_send\_polls* permission will imply the *can\_send\_messages* permission.*/
	pub fn use_independent_chat_permissions(mut self, use_independent_chat_permissions: bool) -> Self {
		self.use_independent_chat_permissions = Some(use_independent_chat_permissions);
		self
	}
}
impl Executable for SetChatPermissions {
	type Response = bool;
	const METHOD_NAME: &str = "setChatPermissions";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_object("permissions", self.permissions);
		parts.add_bool("use_independent_chat_permissions", self.use_independent_chat_permissions);
		parts
	}
}
/**Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#setchatphoto*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetChatPhoto {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**New chat photo, uploaded using multipart/form-data*/
	pub photo: InputFile,
}
impl SetChatPhoto {
	pub fn new(chat_id: impl Into<ChatId>, photo: impl Into<InputFile>) -> Self {
		Self {
			chat_id: chat_id.into(),
			photo: photo.into(),
		}
	}
}
impl Executable for SetChatPhoto {
	type Response = bool;
	const METHOD_NAME: &str = "setChatPhoto";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_attachable("photo", self.photo);
		parts
	}
}
/**Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field *can\_set\_sticker\_set* optionally returned in [getChat](https://core.telegram.org/bots/api/#getchat) requests to check if the bot can use this method. Returns *True* on success.

https://core.telegram.org/bots/api/#setchatstickerset*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetChatStickerSet {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**Name of the sticker set to be set as the group sticker set*/
	pub sticker_set_name: String,
}
impl SetChatStickerSet {
	pub fn new(chat_id: impl Into<ChatId>, sticker_set_name: impl Into<String>) -> Self {
		Self {
			chat_id: chat_id.into(),
			sticker_set_name: sticker_set_name.into(),
		}
	}
}
impl Executable for SetChatStickerSet {
	type Response = bool;
	const METHOD_NAME: &str = "setChatStickerSet";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_string("sticker_set_name", self.sticker_set_name);
		parts
	}
}
/**Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#setchattitle*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetChatTitle {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**New chat title, 1-128 characters*/
	pub title: String,
}
impl SetChatTitle {
	pub fn new(chat_id: impl Into<ChatId>, title: impl Into<String>) -> Self {
		Self {
			chat_id: chat_id.into(),
			title: title.into(),
		}
	}
}
impl Executable for SetChatTitle {
	type Response = bool;
	const METHOD_NAME: &str = "setChatTitle";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_string("title", self.title);
		parts
	}
}
/**Use this method to set the thumbnail of a custom emoji sticker set. Returns *True* on success.

https://core.telegram.org/bots/api/#setcustomemojistickersetthumbnail*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetCustomEmojiStickerSetThumbnail {
	/**Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.*/
	pub custom_emoji_id: Option<String>,
	/**Sticker set name*/
	pub name: String,
}
impl SetCustomEmojiStickerSetThumbnail {
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			custom_emoji_id: None,
			name: name.into(),
		}
	}
	/** Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.*/
	pub fn custom_emoji_id(mut self, custom_emoji_id: impl Into<String>) -> Self {
		self.custom_emoji_id = Some(custom_emoji_id.into());
		self
	}
}
impl Executable for SetCustomEmojiStickerSetThumbnail {
	type Response = bool;
	const METHOD_NAME: &str = "setCustomEmojiStickerSetThumbnail";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("custom_emoji_id", self.custom_emoji_id);
		parts.add_string("name", self.name);
		parts
	}
}
/**Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned. Returns an error, if the new score is not greater than the user's current score in the chat and *force* is *False*.

https://core.telegram.org/bots/api/#setgamescore*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetGameScore {
	/**Required if *inline\_message\_id* is not specified. Unique identifier for the target chat*/
	pub chat_id: Option<i64>,
	/**Pass *True* if the game message should not be automatically edited to include the current scoreboard*/
	pub disable_edit_message: Option<bool>,
	/**Pass *True* if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters*/
	pub force: Option<bool>,
	/**Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub inline_message_id: Option<String>,
	/**Required if *inline\_message\_id* is not specified. Identifier of the sent message*/
	pub message_id: Option<i64>,
	/**New score, must be non-negative*/
	pub score: i64,
	/**User identifier*/
	pub user_id: i64,
}
impl SetGameScore {
	pub fn new(score: impl Into<i64>, user_id: impl Into<i64>) -> Self {
		Self {
			chat_id: None,
			disable_edit_message: None,
			force: None,
			inline_message_id: None,
			message_id: None,
			score: score.into(),
			user_id: user_id.into(),
		}
	}
	/** Required if *inline\_message\_id* is not specified. Unique identifier for the target chat*/
	pub fn chat_id(mut self, chat_id: impl Into<i64>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
	/** Pass *True* if the game message should not be automatically edited to include the current scoreboard*/
	pub fn disable_edit_message(mut self, disable_edit_message: bool) -> Self {
		self.disable_edit_message = Some(disable_edit_message);
		self
	}
	/** Pass *True* if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters*/
	pub fn force(mut self, force: bool) -> Self {
		self.force = Some(force);
		self
	}
	/** Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
		self.inline_message_id = Some(inline_message_id.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Identifier of the sent message*/
	pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_id = Some(message_id.into());
		self
	}
}
impl Executable for SetGameScore {
	type Response = SetGameScoreResult;
	const METHOD_NAME: &str = "setGameScore";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(7);
		parts.add_i64("chat_id", self.chat_id);
		parts.add_bool("disable_edit_message", self.disable_edit_message);
		parts.add_bool("force", self.force);
		parts.add_string("inline_message_id", self.inline_message_id);
		parts.add_i64("message_id", self.message_id);
		parts.add_i64("score", self.score);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns *True* on success.

https://core.telegram.org/bots/api/#setmessagereaction*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetMessageReaction {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Pass *True* to set the reaction with a big animation*/
	pub is_big: Option<bool>,
	/**Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.*/
	pub message_id: i64,
	/**A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.*/
	pub reaction: Vec<ReactionType>,
}
impl SetMessageReaction {
	pub fn new(chat_id: impl Into<ChatId>, message_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			is_big: None,
			message_id: message_id.into(),
			reaction: Vec::new(),
		}
	}
	/** Pass *True* to set the reaction with a big animation*/
	pub fn is_big(mut self, is_big: bool) -> Self {
		self.is_big = Some(is_big);
		self
	}
	pub fn add_reaction(mut self, reaction: impl Into<ReactionType>) -> Self {
		self.reaction.push(reaction.into());
		self
	}
	/** A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.*/
	pub fn reaction(mut self, reaction: impl Into<Vec<ReactionType>>) -> Self {
		self.reaction = reaction.into();
		self
	}
}
impl Executable for SetMessageReaction {
	type Response = bool;
	const METHOD_NAME: &str = "setMessageReaction";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("is_big", self.is_big);
		parts.add_i64("message_id", self.message_id);
		if self.reaction.len() > 0 { parts.add_object("reaction", self.reaction) }
		parts
	}
}
/**Use this method to change the list of the bot's commands. See [this manual](https://core.telegram.org/bots/features#commands) for more details about bot commands. Returns *True* on success.

https://core.telegram.org/bots/api/#setmycommands*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetMyCommands {
	/**A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.*/
	pub commands: Vec<BotCommand>,
	/**A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands*/
	pub language_code: Option<String>,
	/**A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault).*/
	pub scope: Option<BotCommandScope>,
}
impl SetMyCommands {
	pub fn new(commands: impl Into<Vec<BotCommand>>) -> Self {
		Self {
			commands: commands.into(),
			language_code: None,
			scope: None,
		}
	}
	pub fn add_command(mut self, command: impl Into<BotCommand>) -> Self {
		self.commands.push(command.into());
		self
	}
	/** A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands*/
	pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
		self.language_code = Some(language_code.into());
		self
	}
	/** A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault).*/
	pub fn scope(mut self, scope: impl Into<BotCommandScope>) -> Self {
		self.scope = Some(scope.into());
		self
	}
}
impl Executable for SetMyCommands {
	type Response = bool;
	const METHOD_NAME: &str = "setMyCommands";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		if self.commands.len() > 0 { parts.add_object("commands", self.commands) }
		parts.add_string("language_code", self.language_code);
		if let Some(scope) = self.scope { parts.add_object("scope", scope); }
		parts
	}
}
/**Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#setmydefaultadministratorrights*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetMyDefaultAdministratorRights {
	/**Pass *True* to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.*/
	pub for_channels: Option<bool>,
	/**A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.*/
	pub rights: Option<ChatAdministratorRights>,
}
impl SetMyDefaultAdministratorRights {
	pub fn new() -> Self {
		Self {
			for_channels: None,
			rights: None,
		}
	}
	/** Pass *True* to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.*/
	pub fn for_channels(mut self, for_channels: bool) -> Self {
		self.for_channels = Some(for_channels);
		self
	}
	/** A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.*/
	pub fn rights(mut self, rights: impl Into<ChatAdministratorRights>) -> Self {
		self.rights = Some(rights.into());
		self
	}
}
impl Executable for SetMyDefaultAdministratorRights {
	type Response = bool;
	const METHOD_NAME: &str = "setMyDefaultAdministratorRights";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_bool("for_channels", self.for_channels);
		if let Some(rights) = self.rights { parts.add_object("rights", rights); }
		parts
	}
}
/**Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns *True* on success.

https://core.telegram.org/bots/api/#setmydescription*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetMyDescription {
	/**New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.*/
	pub description: Option<String>,
	/**A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.*/
	pub language_code: Option<String>,
}
impl SetMyDescription {
	pub fn new() -> Self {
		Self {
			description: None,
			language_code: None,
		}
	}
	/** New bot description; 0-512 characters. Pass an empty string to remove the dedicated description for the given language.*/
	pub fn description(mut self, description: impl Into<String>) -> Self {
		self.description = Some(description.into());
		self
	}
	/** A two-letter ISO 639-1 language code. If empty, the description will be applied to all users for whose language there is no dedicated description.*/
	pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
		self.language_code = Some(language_code.into());
		self
	}
}
impl Executable for SetMyDescription {
	type Response = bool;
	const METHOD_NAME: &str = "setMyDescription";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("description", self.description);
		parts.add_string("language_code", self.language_code);
		parts
	}
}
/**Use this method to change the bot's name. Returns *True* on success.

https://core.telegram.org/bots/api/#setmyname*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetMyName {
	/**A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.*/
	pub language_code: Option<String>,
	/**New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.*/
	pub name: Option<String>,
}
impl SetMyName {
	pub fn new() -> Self {
		Self {
			language_code: None,
			name: None,
		}
	}
	/** A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.*/
	pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
		self.language_code = Some(language_code.into());
		self
	}
	/** New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.*/
	pub fn name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}
}
impl Executable for SetMyName {
	type Response = bool;
	const METHOD_NAME: &str = "setMyName";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("language_code", self.language_code);
		parts.add_string("name", self.name);
		parts
	}
}
/**Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#setmyshortdescription*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetMyShortDescription {
	/**A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.*/
	pub language_code: Option<String>,
	/**New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.*/
	pub short_description: Option<String>,
}
impl SetMyShortDescription {
	pub fn new() -> Self {
		Self {
			language_code: None,
			short_description: None,
		}
	}
	/** A two-letter ISO 639-1 language code. If empty, the short description will be applied to all users for whose language there is no dedicated short description.*/
	pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
		self.language_code = Some(language_code.into());
		self
	}
	/** New short description for the bot; 0-120 characters. Pass an empty string to remove the dedicated short description for the given language.*/
	pub fn short_description(mut self, short_description: impl Into<String>) -> Self {
		self.short_description = Some(short_description.into());
		self
	}
}
impl Executable for SetMyShortDescription {
	type Response = bool;
	const METHOD_NAME: &str = "setMyShortDescription";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("language_code", self.language_code);
		parts.add_string("short_description", self.short_description);
		parts
	}
}
/**Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns *True* on success.

Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.

https://core.telegram.org/bots/api/#setpassportdataerrors*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetPassportDataErrors {
	/**A JSON-serialized array describing the errors*/
	pub errors: Vec<PassportElementError>,
	/**User identifier*/
	pub user_id: i64,
}
impl SetPassportDataErrors {
	pub fn new(errors: impl Into<Vec<PassportElementError>>, user_id: impl Into<i64>) -> Self {
		Self {
			errors: errors.into(),
			user_id: user_id.into(),
		}
	}
	pub fn add_error(mut self, error: impl Into<PassportElementError>) -> Self {
		self.errors.push(error.into());
		self
	}
}
impl Executable for SetPassportDataErrors {
	type Response = bool;
	const METHOD_NAME: &str = "setPassportDataErrors";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		if self.errors.len() > 0 { parts.add_object("errors", self.errors) }
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#setstickeremojilist*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetStickerEmojiList {
	/**A JSON-serialized list of 1-20 emoji associated with the sticker*/
	pub emoji_list: Vec<String>,
	/**File identifier of the sticker*/
	pub sticker: String,
}
impl SetStickerEmojiList {
	pub fn new(emoji_list: impl Into<Vec<String>>, sticker: impl Into<String>) -> Self {
		Self {
			emoji_list: emoji_list.into(),
			sticker: sticker.into(),
		}
	}
	pub fn add_emoji_list(mut self, emoji_list: impl Into<String>) -> Self {
		self.emoji_list.push(emoji_list.into());
		self
	}
}
impl Executable for SetStickerEmojiList {
	type Response = bool;
	const METHOD_NAME: &str = "setStickerEmojiList";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		if self.emoji_list.len() > 0 { parts.add_object("emoji_list", self.emoji_list) }
		parts.add_string("sticker", self.sticker);
		parts
	}
}
/**Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#setstickerkeywords*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetStickerKeywords {
	/**A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters*/
	pub keywords: Vec<String>,
	/**File identifier of the sticker*/
	pub sticker: String,
}
impl SetStickerKeywords {
	pub fn new(sticker: impl Into<String>) -> Self {
		Self {
			keywords: Vec::new(),
			sticker: sticker.into(),
		}
	}
	pub fn add_keyword(mut self, keyword: impl Into<String>) -> Self {
		self.keywords.push(keyword.into());
		self
	}
	/** A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters*/
	pub fn keywords(mut self, keywords: impl Into<Vec<String>>) -> Self {
		self.keywords = keywords.into();
		self
	}
}
impl Executable for SetStickerKeywords {
	type Response = bool;
	const METHOD_NAME: &str = "setStickerKeywords";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		if self.keywords.len() > 0 { parts.add_object("keywords", self.keywords) }
		parts.add_string("sticker", self.sticker);
		parts
	}
}
/**Use this method to change the [mask position](https://core.telegram.org/bots/api/#maskposition) of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#setstickermaskposition*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetStickerMaskPosition {
	/**A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.*/
	pub mask_position: Option<MaskPosition>,
	/**File identifier of the sticker*/
	pub sticker: String,
}
impl SetStickerMaskPosition {
	pub fn new(sticker: impl Into<String>) -> Self {
		Self {
			mask_position: None,
			sticker: sticker.into(),
		}
	}
	/** A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.*/
	pub fn mask_position(mut self, mask_position: impl Into<MaskPosition>) -> Self {
		self.mask_position = Some(mask_position.into());
		self
	}
}
impl Executable for SetStickerMaskPosition {
	type Response = bool;
	const METHOD_NAME: &str = "setStickerMaskPosition";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		if let Some(mask_position) = self.mask_position { parts.add_object("mask_position", mask_position); }
		parts.add_string("sticker", self.sticker);
		parts
	}
}
/**Use this method to move a sticker in a set created by the bot to a specific position. Returns *True* on success.

https://core.telegram.org/bots/api/#setstickerpositioninset*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetStickerPositionInSet {
	/**New sticker position in the set, zero-based*/
	pub position: i64,
	/**File identifier of the sticker*/
	pub sticker: String,
}
impl SetStickerPositionInSet {
	pub fn new(position: impl Into<i64>, sticker: impl Into<String>) -> Self {
		Self {
			position: position.into(),
			sticker: sticker.into(),
		}
	}
}
impl Executable for SetStickerPositionInSet {
	type Response = bool;
	const METHOD_NAME: &str = "setStickerPositionInSet";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_i64("position", self.position);
		parts.add_string("sticker", self.sticker);
		parts
	}
}
/**Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns *True* on success.

https://core.telegram.org/bots/api/#setstickersetthumbnail*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetStickerSetThumbnail {
	/**Format of the thumbnail, must be one of “static” for a **.WEBP** or **.PNG** image, “animated” for a **.TGS** animation, or “video” for a **.WEBM** video*/
	pub format: String,
	/**Sticker set name*/
	pub name: String,
	/**A **.WEBP** or **.PNG** image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a **.TGS** animation with a thumbnail up to 32 kilobytes in size (see [https://core.telegram.org/stickers#animation-requirements](https://core.telegram.org/stickers#animation-requirements) for animated sticker technical requirements), or a **.WEBM** video with the thumbnail up to 32 kilobytes in size; see [https://core.telegram.org/stickers#video-requirements](https://core.telegram.org/stickers#video-requirements) for video sticker technical requirements. Pass a *file\_id* as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.*/
	pub thumbnail: Option<Asset>,
	/**User identifier of the sticker set owner*/
	pub user_id: i64,
}
impl SetStickerSetThumbnail {
	pub fn new(format: impl Into<String>, name: impl Into<String>, user_id: impl Into<i64>) -> Self {
		Self {
			format: format.into(),
			name: name.into(),
			thumbnail: None,
			user_id: user_id.into(),
		}
	}
	/** A **.WEBP** or **.PNG** image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a **.TGS** animation with a thumbnail up to 32 kilobytes in size (see [https://core.telegram.org/stickers#animation-requirements](https://core.telegram.org/stickers#animation-requirements) for animated sticker technical requirements), or a **.WEBM** video with the thumbnail up to 32 kilobytes in size; see [https://core.telegram.org/stickers#video-requirements](https://core.telegram.org/stickers#video-requirements) for video sticker technical requirements. Pass a *file\_id* as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files). Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.*/
	pub fn thumbnail(mut self, thumbnail: impl Into<Asset>) -> Self {
		self.thumbnail = Some(thumbnail.into());
		self
	}
}
impl Executable for SetStickerSetThumbnail {
	type Response = bool;
	const METHOD_NAME: &str = "setStickerSetThumbnail";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_string("format", self.format);
		parts.add_string("name", self.name);
		if let Some(thumbnail) = self.thumbnail { parts.add_attachable("thumbnail", thumbnail); }
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to set the title of a created sticker set. Returns *True* on success.

https://core.telegram.org/bots/api/#setstickersettitle*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetStickerSetTitle {
	/**Sticker set name*/
	pub name: String,
	/**Sticker set title, 1-64 characters*/
	pub title: String,
}
impl SetStickerSetTitle {
	pub fn new(name: impl Into<String>, title: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			title: title.into(),
		}
	}
}
impl Executable for SetStickerSetTitle {
	type Response = bool;
	const METHOD_NAME: &str = "setStickerSetTitle";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("name", self.name);
		parts.add_string("title", self.title);
		parts
	}
}
/**Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method [requestEmojiStatusAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps). Returns *True* on success.

https://core.telegram.org/bots/api/#setuseremojistatus*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetUserEmojiStatus {
	/**Custom emoji identifier of the emoji status to set. Pass an empty string to remove the status.*/
	pub emoji_status_custom_emoji_id: Option<String>,
	/**Expiration date of the emoji status, if any*/
	pub emoji_status_expiration_date: Option<i64>,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl SetUserEmojiStatus {
	pub fn new(user_id: impl Into<i64>) -> Self {
		Self {
			emoji_status_custom_emoji_id: None,
			emoji_status_expiration_date: None,
			user_id: user_id.into(),
		}
	}
	/** Custom emoji identifier of the emoji status to set. Pass an empty string to remove the status.*/
	pub fn emoji_status_custom_emoji_id(mut self, emoji_status_custom_emoji_id: impl Into<String>) -> Self {
		self.emoji_status_custom_emoji_id = Some(emoji_status_custom_emoji_id.into());
		self
	}
	/** Expiration date of the emoji status, if any*/
	pub fn emoji_status_expiration_date(mut self, emoji_status_expiration_date: impl Into<i64>) -> Self {
		self.emoji_status_expiration_date = Some(emoji_status_expiration_date.into());
		self
	}
}
impl Executable for SetUserEmojiStatus {
	type Response = bool;
	const METHOD_NAME: &str = "setUserEmojiStatus";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		parts.add_string("emoji_status_custom_emoji_id", self.emoji_status_custom_emoji_id);
		parts.add_i64("emoji_status_expiration_date", self.emoji_status_expiration_date);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized [Update](https://core.telegram.org/bots/api/#update). In case of an unsuccessful request (a request with response [HTTP status code](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes) different from `2XY`), we will repeat the request and give up after a reasonable amount of attempts. Returns *True* on success.

If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter *secret\_token*. If specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret token as content.

https://core.telegram.org/bots/api/#setwebhook*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct SetWebhook {
	/**A JSON-serialized list of the update types you want your bot to receive. For example, specify `["message", "edited_channel_post", "callback_query"]` to only receive updates of these types. See [Update](https://core.telegram.org/bots/api/#update) for a complete list of available update types. Specify an empty list to receive all update types except *chat\_member*, *message\_reaction*, and *message\_reaction\_count* (default). If not specified, the previous setting will be used.  
	Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.*/
	pub allowed_updates: Vec<String>,
	/**Upload your public key certificate so that the root certificate in use can be checked. See our [self-signed guide](https://core.telegram.org/bots/self-signed) for details.*/
	pub certificate: Option<InputFile>,
	/**Pass *True* to drop all pending updates*/
	pub drop_pending_updates: Option<bool>,
	/**The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS*/
	pub ip_address: Option<String>,
	/**The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to *40*. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.*/
	pub max_connections: Option<i64>,
	/**A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token” in every webhook request, 1-256 characters. Only characters `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed. The header is useful to ensure that the request comes from a webhook set by you.*/
	pub secret_token: Option<String>,
	/**HTTPS URL to send updates to. Use an empty string to remove webhook integration*/
	pub url: String,
}
impl SetWebhook {
	pub fn new(url: impl Into<String>) -> Self {
		Self {
			allowed_updates: Vec::new(),
			certificate: None,
			drop_pending_updates: None,
			ip_address: None,
			max_connections: None,
			secret_token: None,
			url: url.into(),
		}
	}
	pub fn add_allowed_update(mut self, allowed_update: impl Into<String>) -> Self {
		self.allowed_updates.push(allowed_update.into());
		self
	}
	/** A JSON-serialized list of the update types you want your bot to receive. For example, specify `["message", "edited_channel_post", "callback_query"]` to only receive updates of these types. See [Update](https://core.telegram.org/bots/api/#update) for a complete list of available update types. Specify an empty list to receive all update types except *chat\_member*, *message\_reaction*, and *message\_reaction\_count* (default). If not specified, the previous setting will be used.  
	Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.*/
	pub fn allowed_updates(mut self, allowed_updates: impl Into<Vec<String>>) -> Self {
		self.allowed_updates = allowed_updates.into();
		self
	}
	/** Upload your public key certificate so that the root certificate in use can be checked. See our [self-signed guide](https://core.telegram.org/bots/self-signed) for details.*/
	pub fn certificate(mut self, certificate: impl Into<InputFile>) -> Self {
		self.certificate = Some(certificate.into());
		self
	}
	/** Pass *True* to drop all pending updates*/
	pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
		self.drop_pending_updates = Some(drop_pending_updates);
		self
	}
	/** The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS*/
	pub fn ip_address(mut self, ip_address: impl Into<String>) -> Self {
		self.ip_address = Some(ip_address.into());
		self
	}
	/** The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to *40*. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.*/
	pub fn max_connections(mut self, max_connections: impl Into<i64>) -> Self {
		self.max_connections = Some(max_connections.into());
		self
	}
	/** A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token” in every webhook request, 1-256 characters. Only characters `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed. The header is useful to ensure that the request comes from a webhook set by you.*/
	pub fn secret_token(mut self, secret_token: impl Into<String>) -> Self {
		self.secret_token = Some(secret_token.into());
		self
	}
}
impl Executable for SetWebhook {
	type Response = bool;
	const METHOD_NAME: &str = "setWebhook";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(7);
		if self.allowed_updates.len() > 0 { parts.add_object("allowed_updates", self.allowed_updates) }
		if let Some(certificate) = self.certificate { parts.add_attachable("certificate", certificate); }
		parts.add_bool("drop_pending_updates", self.drop_pending_updates);
		parts.add_string("ip_address", self.ip_address);
		parts.add_i64("max_connections", self.max_connections);
		parts.add_string("secret_token", self.secret_token);
		parts.add_string("url", self.url);
		parts
	}
}
/**Use this method to stop updating a live location message before *live\_period* expires. On success, if the message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.

https://core.telegram.org/bots/api/#stopmessagelivelocation*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct StopMessageLiveLocation {
	/**Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub business_connection_id: Option<String>,
	/**Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: Option<ChatId>,
	/**Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub inline_message_id: Option<String>,
	/**Required if *inline\_message\_id* is not specified. Identifier of the message with live location to stop*/
	pub message_id: Option<i64>,
	/**A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl StopMessageLiveLocation {
	pub fn new() -> Self {
		Self {
			business_connection_id: None,
			chat_id: None,
			inline_message_id: None,
			message_id: None,
			reply_markup: None,
		}
	}
	/** Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
		self.chat_id = Some(chat_id.into());
		self
	}
	/** Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message*/
	pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
		self.inline_message_id = Some(inline_message_id.into());
		self
	}
	/** Required if *inline\_message\_id* is not specified. Identifier of the message with live location to stop*/
	pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_id = Some(message_id.into());
		self
	}
	/** A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
impl Executable for StopMessageLiveLocation {
	type Response = StopMessageLiveLocationResult;
	const METHOD_NAME: &str = "stopMessageLiveLocation";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(5);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.map(|x| x.to_string()));
		parts.add_string("inline_message_id", self.inline_message_id);
		parts.add_i64("message_id", self.message_id);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		parts
	}
}
/**Use this method to stop a poll which was sent by the bot. On success, the stopped [Poll](https://core.telegram.org/bots/api/#poll) is returned.

https://core.telegram.org/bots/api/#stoppoll*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct StopPoll {
	/**Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Identifier of the original message with the poll*/
	pub message_id: i64,
	/**A JSON-serialized object for a new message [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl StopPoll {
	pub fn new(chat_id: impl Into<ChatId>, message_id: impl Into<i64>) -> Self {
		Self {
			business_connection_id: None,
			chat_id: chat_id.into(),
			message_id: message_id.into(),
			reply_markup: None,
		}
	}
	/** Unique identifier of the business connection on behalf of which the message to be edited was sent*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** A JSON-serialized object for a new message [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
}
impl Executable for StopPoll {
	type Response = Poll;
	const METHOD_NAME: &str = "stopPoll";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(4);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("message_id", self.message_id);
		if let Some(reply_markup) = self.reply_markup { parts.add_object("reply_markup", reply_markup); }
		parts
	}
}
/**Use this method to unban a previously banned user in a supergroup or channel. The user will **not** return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be **removed** from the chat. If you don't want this, use the parameter *only\_if\_banned*. Returns *True* on success.

https://core.telegram.org/bots/api/#unbanchatmember*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct UnbanChatMember {
	/**Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Do nothing if the user is not banned*/
	pub only_if_banned: Option<bool>,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl UnbanChatMember {
	pub fn new(chat_id: impl Into<ChatId>, user_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			only_if_banned: None,
			user_id: user_id.into(),
		}
	}
	/** Do nothing if the user is not banned*/
	pub fn only_if_banned(mut self, only_if_banned: bool) -> Self {
		self.only_if_banned = Some(only_if_banned);
		self
	}
}
impl Executable for UnbanChatMember {
	type Response = bool;
	const METHOD_NAME: &str = "unbanChatMember";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_bool("only_if_banned", self.only_if_banned);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#unbanchatsenderchat*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct UnbanChatSenderChat {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Unique identifier of the target sender chat*/
	pub sender_chat_id: i64,
}
impl UnbanChatSenderChat {
	pub fn new(chat_id: impl Into<ChatId>, sender_chat_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			sender_chat_id: sender_chat_id.into(),
		}
	}
}
impl Executable for UnbanChatSenderChat {
	type Response = bool;
	const METHOD_NAME: &str = "unbanChatSenderChat";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("sender_chat_id", self.sender_chat_id);
		parts
	}
}
/**Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. Returns *True* on success.

https://core.telegram.org/bots/api/#unhidegeneralforumtopic*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct UnhideGeneralForumTopic {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
}
impl UnhideGeneralForumTopic {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for UnhideGeneralForumTopic {
	type Response = bool;
	const METHOD_NAME: &str = "unhideGeneralForumTopic";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.

https://core.telegram.org/bots/api/#unpinallchatmessages*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct UnpinAllChatMessages {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
}
impl UnpinAllChatMessages {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for UnpinAllChatMessages {
	type Response = bool;
	const METHOD_NAME: &str = "unpinAllChatMessages";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the *can\_pin\_messages* administrator right in the supergroup. Returns *True* on success.

https://core.telegram.org/bots/api/#unpinallforumtopicmessages*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct UnpinAllForumTopicMessages {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
	/**Unique identifier for the target message thread of the forum topic*/
	pub message_thread_id: i64,
}
impl UnpinAllForumTopicMessages {
	pub fn new(chat_id: impl Into<ChatId>, message_thread_id: impl Into<i64>) -> Self {
		Self {
			chat_id: chat_id.into(),
			message_thread_id: message_thread_id.into(),
		}
	}
}
impl Executable for UnpinAllForumTopicMessages {
	type Response = bool;
	const METHOD_NAME: &str = "unpinAllForumTopicMessages";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("message_thread_id", self.message_thread_id);
		parts
	}
}
/**Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the *can\_pin\_messages* administrator right in the supergroup. Returns *True* on success.

https://core.telegram.org/bots/api/#unpinallgeneralforumtopicmessages*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct UnpinAllGeneralForumTopicMessages {
	/**Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)*/
	pub chat_id: ChatId,
}
impl UnpinAllGeneralForumTopicMessages {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
		}
	}
}
impl Executable for UnpinAllGeneralForumTopicMessages {
	type Response = bool;
	const METHOD_NAME: &str = "unpinAllGeneralForumTopicMessages";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(1);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts
	}
}
/**Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can\_pin\_messages' administrator right in a supergroup or 'can\_edit\_messages' administrator right in a channel. Returns *True* on success.

https://core.telegram.org/bots/api/#unpinchatmessage*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct UnpinChatMessage {
	/**Unique identifier of the business connection on behalf of which the message will be unpinned*/
	pub business_connection_id: Option<String>,
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Identifier of the message to unpin. Required if *business\_connection\_id* is specified. If not specified, the most recent pinned message (by sending date) will be unpinned.*/
	pub message_id: Option<i64>,
}
impl UnpinChatMessage {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			business_connection_id: None,
			chat_id: chat_id.into(),
			message_id: None,
		}
	}
	/** Unique identifier of the business connection on behalf of which the message will be unpinned*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** Identifier of the message to unpin. Required if *business\_connection\_id* is specified. If not specified, the most recent pinned message (by sending date) will be unpinned.*/
	pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
		self.message_id = Some(message_id.into());
		self
	}
}
impl Executable for UnpinChatMessage {
	type Response = bool;
	const METHOD_NAME: &str = "unpinChatMessage";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		parts.add_string("business_connection_id", self.business_connection_id);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_i64("message_id", self.message_id);
		parts
	}
}
/**Use this method to upload a file with a sticker for later use in the [createNewStickerSet](https://core.telegram.org/bots/api/#createnewstickerset), [addStickerToSet](https://core.telegram.org/bots/api/#addstickertoset), or [replaceStickerInSet](https://core.telegram.org/bots/api/#replacestickerinset) methods (the file can be used multiple times). Returns the uploaded [File](https://core.telegram.org/bots/api/#file) on success.

https://core.telegram.org/bots/api/#uploadstickerfile*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct UploadStickerFile {
	/**A file with the sticker in .WEBP, .PNG, .TGS, or .WEBM format. See [https://core.telegram.org/stickers](https://core.telegram.org/stickers) for technical requirements. [More information on Sending Files »](https://core.telegram.org/bots/api/#sending-files)*/
	pub sticker: InputFile,
	/**Format of the sticker, must be one of “static”, “animated”, “video”*/
	pub sticker_format: String,
	/**User identifier of sticker file owner*/
	pub user_id: i64,
}
impl UploadStickerFile {
	pub fn new(sticker: impl Into<InputFile>, sticker_format: impl Into<String>, user_id: impl Into<i64>) -> Self {
		Self {
			sticker: sticker.into(),
			sticker_format: sticker_format.into(),
			user_id: user_id.into(),
		}
	}
}
impl Executable for UploadStickerFile {
	type Response = File;
	const METHOD_NAME: &str = "uploadStickerFile";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(3);
		parts.add_attachable("sticker", self.sticker);
		parts.add_string("sticker_format", self.sticker_format);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
/**Verifies a chat [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#verifychat*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct VerifyChat {
	/**Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)*/
	pub chat_id: ChatId,
	/**Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.*/
	pub custom_description: Option<String>,
}
impl VerifyChat {
	pub fn new(chat_id: impl Into<ChatId>) -> Self {
		Self {
			chat_id: chat_id.into(),
			custom_description: None,
		}
	}
	/** Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.*/
	pub fn custom_description(mut self, custom_description: impl Into<String>) -> Self {
		self.custom_description = Some(custom_description.into());
		self
	}
}
impl Executable for VerifyChat {
	type Response = bool;
	const METHOD_NAME: &str = "verifyChat";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("chat_id", self.chat_id.to_string());
		parts.add_string("custom_description", self.custom_description);
		parts
	}
}
/**Verifies a user [on behalf of the organization](https://telegram.org/verify#third-party-verification) which is represented by the bot. Returns *True* on success.

https://core.telegram.org/bots/api/#verifyuser*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-all", serde(Serialize, Deserialize))]
pub struct VerifyUser {
	/**Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.*/
	pub custom_description: Option<String>,
	/**Unique identifier of the target user*/
	pub user_id: i64,
}
impl VerifyUser {
	pub fn new(user_id: impl Into<i64>) -> Self {
		Self {
			custom_description: None,
			user_id: user_id.into(),
		}
	}
	/** Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.*/
	pub fn custom_description(mut self, custom_description: impl Into<String>) -> Self {
		self.custom_description = Some(custom_description.into());
		self
	}
}
impl Executable for VerifyUser {
	type Response = bool;
	const METHOD_NAME: &str = "verifyUser";
	fn into_parts(self) -> FormParts {
		let mut parts = FormParts::new(2);
		parts.add_string("custom_description", self.custom_description);
		parts.add_i64("user_id", self.user_id);
		parts
	}
}
