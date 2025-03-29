
#[cfg(not(feature = "custom-message"))]
pub use original_message::*;
#[cfg(not(feature = "custom-message"))]
mod original_message {
	use serde::Deserialize;
	use serde_with::apply;
	use crate::*;

	/**This object represents a message.

	https://core.telegram.org/bots/api/#message*/
	#[apply(
		Vec => #[serde(skip_serializing_if = "Vec::is_empty")],
		Option => #[serde(skip_serializing_if = "Option::is_none")],
	)]
	#[derive(Clone, Debug, Deserialize)]
	pub struct Message {
		/**Message is an animation, information about the animation. For backward compatibility, when this field is set, the *document* field will also be set*/
		pub animation: Option<Animation>,
		/**Message is an audio file, information about the file*/
		pub audio: Option<Audio>,
		/**Signature of the post author for messages in channels, or the custom title of an anonymous group administrator*/
		pub author_signature: Option<String>,
		/**Service message: user boosted the chat*/
		pub boost_added: Option<ChatBoostAdded>,
		/**Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.*/
		pub business_connection_id: Option<String>,
		/**Caption for the animation, audio, document, paid media, photo, video or voice*/
		pub caption: Option<String>,
		/**For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption*/
		pub caption_entities: Vec<MessageEntity>,
		/**Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply\_to\_message if someone replies to a very first message in a channel.
		Default value: true*/
		pub channel_chat_created: Option<bool>,
		/**Chat the message belongs to*/
		pub chat: Chat,
		/**Service message: chat background set*/
		pub chat_background_set: Option<ChatBackground>,
		/**Service message: a chat was shared with the bot*/
		pub chat_shared: Option<ChatShared>,
		/**The domain name of the website on which the user has logged in. [More about Telegram Login »](https://core.telegram.org/widgets/login)*/
		pub connected_website: Option<String>,
		/**Message is a shared contact, information about the contact*/
		pub contact: Option<Contact>,
		/**Date the message was sent in Unix time. It is always a positive number, representing a valid date.*/
		pub date: i64,
		/**Service message: the chat photo was deleted
		Default value: true*/
		pub delete_chat_photo: Option<bool>,
		/**Message is a dice with random value*/
		pub dice: Option<Dice>,
		/**Message is a general file, information about the file*/
		pub document: Option<Document>,
		/**Date the message was last edited in Unix time*/
		pub edit_date: Option<i64>,
		/**Unique identifier of the message effect added to the message*/
		pub effect_id: Option<String>,
		/**For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text*/
		pub entities: Vec<MessageEntity>,
		/**Information about the message that is being replied to, which may come from another chat or forum topic*/
		pub external_reply: Option<ExternalReplyInfo>,
		/**Service message: forum topic closed*/
		pub forum_topic_closed: Option<ForumTopicClosed>,
		/**Service message: forum topic created*/
		pub forum_topic_created: Option<ForumTopicCreated>,
		/**Service message: forum topic edited*/
		pub forum_topic_edited: Option<ForumTopicEdited>,
		/**Service message: forum topic reopened*/
		pub forum_topic_reopened: Option<ForumTopicReopened>,
		/**Information about the original message for forwarded messages*/
		pub forward_origin: Option<MessageOrigin>,
		/**Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats*/
		pub from: Option<User>,
		/**Message is a game, information about the game. [More about games »](https://core.telegram.org/bots/api/#games)*/
		pub game: Option<Game>,
		/**Service message: the 'General' forum topic hidden*/
		pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
		/**Service message: the 'General' forum topic unhidden*/
		pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
		/**The message is a scheduled giveaway message*/
		pub giveaway: Option<Giveaway>,
		/**Service message: a giveaway without public winners was completed*/
		pub giveaway_completed: Option<GiveawayCompleted>,
		/**Service message: a scheduled giveaway was created*/
		pub giveaway_created: Option<GiveawayCreated>,
		/**A giveaway with public winners was completed*/
		pub giveaway_winners: Option<GiveawayWinners>,
		/**Service message: the group has been created
		Default value: true*/
		pub group_chat_created: Option<bool>,
		/**if the message media is covered by a spoiler animation
		Default value: true*/
		pub has_media_spoiler: Option<bool>,
		/**if the message can't be forwarded
		Default value: true*/
		pub has_protected_content: Option<bool>,
		/**Message is an invoice for a [payment](https://core.telegram.org/bots/api/#payments), information about the invoice. [More about payments »](https://core.telegram.org/bots/api/#payments)*/
		pub invoice: Option<Invoice>,
		/**if the message is a channel post that was automatically forwarded to the connected discussion group
		Default value: true*/
		pub is_automatic_forward: Option<bool>,
		/**True, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
		Default value: true*/
		pub is_from_offline: Option<bool>,
		/**if the message is sent to a forum topic
		Default value: true*/
		pub is_topic_message: Option<bool>,
		/**A member was removed from the group, information about them (this member may be the bot itself)*/
		pub left_chat_member: Option<User>,
		/**Options used for link preview generation for the message, if it is a text message and link preview options were changed*/
		pub link_preview_options: Option<LinkPreviewOptions>,
		/**Message is a shared location, information about the location*/
		pub location: Option<Location>,
		/**The unique identifier of a media message group this message belongs to*/
		pub media_group_id: Option<String>,
		/**Service message: auto-delete timer settings changed in the chat*/
		pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
		/**Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent*/
		pub message_id: i64,
		/**Unique identifier of a message thread to which the message belongs; for supergroups only*/
		pub message_thread_id: Option<i64>,
		/**The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.*/
		pub migrate_from_chat_id: Option<i64>,
		/**The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.*/
		pub migrate_to_chat_id: Option<i64>,
		/**New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)*/
		pub new_chat_members: Vec<User>,
		/**A chat photo was change to this value*/
		pub new_chat_photo: Vec<PhotoSize>,
		/**A chat title was changed to this value*/
		pub new_chat_title: Option<String>,
		/**Message contains paid media; information about the paid media*/
		pub paid_media: Option<PaidMediaInfo>,
		/**Telegram Passport data*/
		pub passport_data: Option<PassportData>,
		/**Message is a photo, available sizes of the photo*/
		pub photo: Vec<PhotoSize>,
		/**Specified message was pinned. Note that the Message object in this field will not contain further *reply\_to\_message* fields even if it itself is a reply.*/
		pub pinned_message: Option<MaybeInaccessibleMessage>,
		/**Message is a native poll, information about the poll*/
		pub poll: Option<Poll>,
		/**Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.*/
		pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
		/**For replies that quote part of the original message, the quoted part of the message*/
		pub quote: Option<TextQuote>,
		/**Message is a service message about a refunded payment, information about the payment. [More about payments »](https://core.telegram.org/bots/api/#payments)*/
		pub refunded_payment: Option<RefundedPayment>,
		/**Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.*/
		pub reply_markup: Option<InlineKeyboardMarkup>,
		/**For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further *reply\_to\_message* fields even if it itself is a reply.*/
		pub reply_to_message: Option<Box<Message>>,
		/**For replies to a story, the original story*/
		pub reply_to_story: Option<Story>,
		/**If the sender of the message boosted the chat, the number of boosts added by the user*/
		pub sender_boost_count: Option<i64>,
		/**The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.*/
		pub sender_business_bot: Option<User>,
		/**Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field *from* contains a fake sender user in non-channel chats.*/
		pub sender_chat: Option<Chat>,
		/**True, if the caption must be shown above the message media
		Default value: true*/
		pub show_caption_above_media: Option<bool>,
		/**Message is a sticker, information about the sticker*/
		pub sticker: Option<Sticker>,
		/**Message is a forwarded story*/
		pub story: Option<Story>,
		/**Message is a service message about a successful payment, information about the payment. [More about payments »](https://core.telegram.org/bots/api/#payments)*/
		pub successful_payment: Option<SuccessfulPayment>,
		/**Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply\_to\_message if someone replies to a very first message in a directly created supergroup.
		Default value: true*/
		pub supergroup_chat_created: Option<bool>,
		/**For text messages, the actual UTF-8 text of the message*/
		pub text: Option<String>,
		/**Service message: users were shared with the bot*/
		pub users_shared: Option<UsersShared>,
		/**Message is a venue, information about the venue. For backward compatibility, when this field is set, the *location* field will also be set*/
		pub venue: Option<Venue>,
		/**Bot through which the message was sent*/
		pub via_bot: Option<User>,
		/**Message is a video, information about the video*/
		pub video: Option<Video>,
		/**Service message: video chat ended*/
		pub video_chat_ended: Option<VideoChatEnded>,
		/**Service message: new participants invited to a video chat*/
		pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
		/**Service message: video chat scheduled*/
		pub video_chat_scheduled: Option<VideoChatScheduled>,
		/**Service message: video chat started*/
		pub video_chat_started: Option<VideoChatStarted>,
		/**Message is a [video note](https://telegram.org/blog/video-messages-and-telescope), information about the video message*/
		pub video_note: Option<VideoNote>,
		/**Message is a voice message, information about the file*/
		pub voice: Option<Voice>,
		/**Service message: data sent by a Web App*/
		pub web_app_data: Option<WebAppData>,
		/**Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps)*/
		pub write_access_allowed: Option<WriteAccessAllowed>,
	}
}

#[cfg(feature = "custom-message")]
pub use custom_message::*;
#[cfg(feature = "custom-message")]
mod custom_message {
	use serde::Deserialize;
	use serde_with::apply;
	use serde_json::Value as JValue;
	use crate::*;
	use self::raw::*;

	/**This object represents a message.

	https://core.telegram.org/bots/api/#message*/
	#[apply(
		Vec => #[serde(skip_serializing_if = "Vec::is_empty")],
		Option => #[serde(skip_serializing_if = "Option::is_none")],
	)]
	#[derive(Clone, Debug, Deserialize)]
	pub struct Message {
		/**Signature of the post author for messages in channels, or the custom title of an anonymous group administrator*/
		pub author_signature: Option<String>,
		/**Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.*/
		pub business_connection_id: Option<String>,
		/**Chat the message belongs to*/
		pub chat: Chat,
		/**Date the message was sent in Unix time. It is always a positive number, representing a valid date.*/
		pub date: i64,
		/**Service message: the chat photo was deleted
		Default value: true*/
		pub delete_chat_photo: Option<bool>,
		/**Date the message was last edited in Unix time*/
		pub edit_date: Option<i64>,
		/**Unique identifier of the message effect added to the message*/
		pub effect_id: Option<String>,
		/**Information about the message that is being replied to, which may come from another chat or forum topic*/
		pub external_reply: Option<ExternalReplyInfo>,
		/**Information about the original message for forwarded messages*/
		pub forward_origin: Option<MessageOrigin>,
		/**Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats*/
		pub from: Option<User>,
		/**if the message media is covered by a spoiler animation
		Default value: true*/
		pub has_media_spoiler: Option<bool>,
		/**if the message can't be forwarded
		Default value: true*/
		pub has_protected_content: Option<bool>,
		/**if the message is a channel post that was automatically forwarded to the connected discussion group
		Default value: true*/
		pub is_automatic_forward: Option<bool>,
		/**True, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
		Default value: true*/
		pub is_from_offline: Option<bool>,
		/**if the message is sent to a forum topic
		Default value: true*/
		pub is_topic_message: Option<bool>,
		/**Options used for link preview generation for the message, if it is a text message and link preview options were changed*/
		pub link_preview_options: Option<LinkPreviewOptions>,
		/**The unique identifier of a media message group this message belongs to*/
		pub media_group_id: Option<String>,
		/**Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent*/
		pub message_id: i64,
		/**Unique identifier of a message thread to which the message belongs; for supergroups only*/
		pub message_thread_id: Option<i64>,
		/**For replies that quote part of the original message, the quoted part of the message*/
		pub quote: Option<TextQuote>,
		/**Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.*/
		pub reply_markup: Option<InlineKeyboardMarkup>,
		/**For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further *reply\_to\_message* fields even if it itself is a reply.*/
		// pub reply_to_message: Option<Box<Message>>,
		/**For replies to a story, the original story*/
		pub reply_to_story: Option<Story>,
		/**If the sender of the message boosted the chat, the number of boosts added by the user*/
		pub sender_boost_count: Option<i64>,
		/**The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.*/
		pub sender_business_bot: Option<User>,
		/**Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field *from* contains a fake sender user in non-channel chats.*/
		pub sender_chat: Option<Chat>,
		/**True, if the caption must be shown above the message media
		Default value: true*/
		pub show_caption_above_media: Option<bool>,
		/**Bot through which the message was sent*/
		pub via_bot: Option<User>,

		#[serde(flatten)]
		pub data: MessageData,
	}

	impl Message {
		pub fn get_text(&self) -> Option<&Text> {
			match self.data {
				MessageData::Text(ref text)
				| MessageData::Audio {
					caption: Some(ref text), ..
				}
				| MessageData::Photo {
					caption: Some(ref text), ..
				}
				| MessageData::Video {
					caption: Some(ref text), ..
				}
				| MessageData::Document {
					caption: Some(ref text), ..
				}
				| MessageData::Voice {
					caption: Some(ref text), ..
				} => Some(text),
				_ => None,
			}
		}
	}

	#[derive(Clone, Debug, Deserialize)]
	#[serde(rename_all = "snake_case")]
	pub enum MessageData {
		/**Message is an animation, information about the animation. For backward compatibility, when this field is set, the *document* field will also be set*/
		Animation(Animation),

		/**Service message: auto-delete timer settings changed in the chat*/
		MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChanged),
		/**Service message: user boosted the chat*/
		#[serde(deserialize_with = "RawBoostAdded::deserialize_value", serialize_with = "RawBoostAdded::serialize_value")]
		BoostAdded(i64),

		/**Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply\_to\_message if someone replies to a very first message in a channel.
		Default value: true*/
		#[serde(deserialize_with = "RawFlag::deserialize_value", serialize_with = "RawFlag::serialize_value")]
		ChannelChatCreated,

		/**Service message: chat background set*/
		ChatBackgroundSet(ChatBackground),

		/**Service message: a chat was shared with the bot*/
		ChatShared(ChatShared),

		/**The domain name of the website on which the user has logged in. [More about Telegram Login »](https://core.telegram.org/widgets/login)*/
		ConnectedWebsite(String),

		/**Message is a shared contact, information about the contact*/
		Contact(Contact),

		#[serde(deserialize_with = "RawFlag::deserialize_value", serialize_with = "RawFlag::serialize_value")]
		DeleteChatPhoto,

		/**Message is a dice with random value*/
		Dice(Dice),

		#[serde(deserialize_with = "RawEmpty::deserialize_value", serialize_with = "RawEmpty::serialize_value")]
		/**Service message: forum topic closed*/
		ForumTopicClosed,

		/**Service message: forum topic created*/
		ForumTopicCreated(ForumTopicCreated),

		/**Service message: forum topic edited*/
		ForumTopicEdited(ForumTopicEdited),

		#[serde(deserialize_with = "RawEmpty::deserialize_value", serialize_with = "RawEmpty::serialize_value")]
		/**Service message: forum topic reopened*/
		ForumTopicReopened,

		/**Message is a game, information about the game. [More about games »](https://core.telegram.org/bots/api/#games)*/
		Game(Game),
		
		/**Service message: the 'General' forum topic hidden*/
		#[serde(deserialize_with = "RawEmpty::deserialize_value", serialize_with = "RawEmpty::serialize_value")]
		GeneralForumTopicHidden,

		/**Service message: the 'General' forum topic unhidden*/
		#[serde(deserialize_with = "RawEmpty::deserialize_value", serialize_with = "RawEmpty::serialize_value")]
		GeneralForumTopicUnhidden,

		/**The message is a scheduled giveaway message*/
		Giveaway(Giveaway),

		/**Service message: a scheduled giveaway was created*/
		GiveawayCreated(GiveawayCreated),
		/**Service message: a giveaway without public winners was completed*/
		GiveawayCompleted(GiveawayCompleted),

		/**A giveaway with public winners was completed*/
		GiveawayWinners(GiveawayWinners),

		/**Service message: the group has been created
		Default value: true*/
		#[serde(deserialize_with = "RawFlag::deserialize_value", serialize_with = "RawFlag::serialize_value")]
		GroupChatCreated,

		/**Message is an invoice for a [payment](https://core.telegram.org/bots/api/#payments), information about the invoice. [More about payments »](https://core.telegram.org/bots/api/#payments)*/
		Invoice(Invoice),

		/**A member was removed from the group, information about them (this member may be the bot itself)*/
		LeftChatMember(User),

		/**Message is a shared location, information about the location*/
		Location(Location),
		
		/**The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.*/
		MigrateFromChatId(i64),

		/**The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.*/
		MigrateToChatId(i64),

		/**New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)*/
		NewChatMembers(Vec<User>),

		/**A chat photo was change to this value*/
		NewChatPhoto(Vec<PhotoSize>),

		/**A chat title was changed to this value*/
		NewChatTitle(String),

		/**Message contains paid media; information about the paid media*/
		PaidMedia(PaidMediaInfo),

		/**Telegram Passport data*/
		PassportData(PassportData),

		/**Specified message was pinned. Note that the Message object in this field will not contain further *reply\_to\_message* fields even if it itself is a reply.*/
		PinnedMessage(MaybeInaccessibleMessage),

		/**Message is a native poll, information about the poll*/
		Poll(Poll),

		/**Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.*/
		ProximityAlertTriggered(ProximityAlertTriggered),

		/**Message is a service message about a refunded payment, information about the payment. [More about payments »](https://core.telegram.org/bots/api/#payments)*/
		RefundedPayment(RefundedPayment),

		/**Message is a sticker, information about the sticker*/
		Sticker(Sticker),

		/**Message is a forwarded story*/
		Story(Story),

		/**Message is a service message about a successful payment, information about the payment. [More about payments »](https://core.telegram.org/bots/api/#payments)*/
		SuccessfulPayment(SuccessfulPayment),

		/**Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply\_to\_message if someone replies to a very first message in a directly created supergroup.
		Default value: true*/
		#[serde(
			deserialize_with = "RawFlag::deserialize_value",
			serialize_with = "RawFlag::serialize_value"
		)]
		SupergroupChatCreated,

		/**Service message: users were shared with the bot*/
		UsersShared(UsersShared),

		/**Message is a venue, information about the venue. For backward compatibility, when this field is set, the *location* field will also be set*/
		Venue(Venue),

		/**Message is a [video note](https://telegram.org/blog/video-messages-and-telescope), information about the video message*/
		VideoNote(VideoNote),

		/**Service message: video chat ended*/
		VideoChatEnded(VideoChatEnded),

		/**Service message: new participants invited to a video chat*/
		VideoChatParticipantsInvited(VideoChatParticipantsInvited),

		/**Service message: video chat scheduled*/
		VideoChatScheduled(VideoChatScheduled),

		/**Service message: video chat started*/
		#[serde(
			deserialize_with = "RawEmpty::deserialize_value",
			serialize_with = "RawEmpty::serialize_value"
		)]
		VideoChatStarted,

		/**Service message: data sent by a Web App*/
		WebAppData(WebAppData),

		/**Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps)*/
		WriteAccessAllowed(WriteAccessAllowed),

		/**Message is an audio file, information about the file*/
		#[serde(untagged)]
		Audio {
			audio: Audio,
			#[serde(flatten, deserialize_with = "RawCaption::deserialize_value", serialize_with = "RawCaption::serialize_value", skip_serializing_if = "Option::is_none")]
			caption: Option<Text>,
		},
		
		/**Message is a general file, information about the file*/
		#[serde(untagged)]
		Document {
			document: Document,
			#[serde(
				flatten,
				deserialize_with = "RawCaption::deserialize_value",
				serialize_with = "RawCaption::serialize_value",
				skip_serializing_if = "Option::is_none"
			)]
			caption: Option<Text>,
		},

		/**Message is a photo, available sizes of the photo*/
		#[serde(untagged)]
		Photo {
			photo: Vec<PhotoSize>,
			#[serde(
				flatten,
				deserialize_with = "RawCaption::deserialize_value",
				serialize_with = "RawCaption::serialize_value",
				skip_serializing_if = "Option::is_none"
			)]
			caption: Option<Text>,
		},

		/**Message is a video, information about the video*/
		#[serde(untagged)]
		Video {
			video: Video,
			caption: Option<Text>,
		},

		/**Message is a voice message, information about the file*/
		#[serde(untagged)]
		Voice {
			voice: Video,
			caption: Option<Text>,
		},

		/**For text messages, the actual UTF-8 text of the message*/
		#[serde(
			deserialize_with = "RawText::deserialize_value",
			serialize_with = "RawText::serialize_value",
			untagged
		)]
		Text(Text),

		#[serde(untagged)]
		Unknown(JValue),
	}
}
