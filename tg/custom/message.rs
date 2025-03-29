
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
impl Message {
	pub fn new(chat: impl Into<Chat>, date: impl Into<i64>, message_id: impl Into<i64>) -> Self {
		Self {
			animation: None,
			audio: None,
			author_signature: None,
			boost_added: None,
			business_connection_id: None,
			caption: None,
			caption_entities: Vec::new(),
			channel_chat_created: None,
			chat: chat.into(),
			chat_background_set: None,
			chat_shared: None,
			connected_website: None,
			contact: None,
			date: date.into(),
			delete_chat_photo: None,
			dice: None,
			document: None,
			edit_date: None,
			effect_id: None,
			entities: Vec::new(),
			external_reply: None,
			forum_topic_closed: None,
			forum_topic_created: None,
			forum_topic_edited: None,
			forum_topic_reopened: None,
			forward_origin: None,
			from: None,
			game: None,
			general_forum_topic_hidden: None,
			general_forum_topic_unhidden: None,
			giveaway: None,
			giveaway_completed: None,
			giveaway_created: None,
			giveaway_winners: None,
			group_chat_created: None,
			has_media_spoiler: None,
			has_protected_content: None,
			invoice: None,
			is_automatic_forward: None,
			is_from_offline: None,
			is_topic_message: None,
			left_chat_member: None,
			link_preview_options: None,
			location: None,
			media_group_id: None,
			message_auto_delete_timer_changed: None,
			message_id: message_id.into(),
			message_thread_id: None,
			migrate_from_chat_id: None,
			migrate_to_chat_id: None,
			new_chat_members: Vec::new(),
			new_chat_photo: Vec::new(),
			new_chat_title: None,
			paid_media: None,
			passport_data: None,
			photo: Vec::new(),
			pinned_message: None,
			poll: None,
			proximity_alert_triggered: None,
			quote: None,
			refunded_payment: None,
			reply_markup: None,
			reply_to_message: None,
			reply_to_story: None,
			sender_boost_count: None,
			sender_business_bot: None,
			sender_chat: None,
			show_caption_above_media: None,
			sticker: None,
			story: None,
			successful_payment: None,
			supergroup_chat_created: None,
			text: None,
			users_shared: None,
			venue: None,
			via_bot: None,
			video: None,
			video_chat_ended: None,
			video_chat_participants_invited: None,
			video_chat_scheduled: None,
			video_chat_started: None,
			video_note: None,
			voice: None,
			web_app_data: None,
			write_access_allowed: None,
		}
	}
	/** *Optional*. Message is an animation, information about the animation. For backward compatibility, when this field is set, the *document* field will also be set*/
	pub fn animation(mut self, animation: impl Into<Animation>) -> Self {
		self.animation = Some(animation.into());
		self
	}
	/** *Optional*. Message is an audio file, information about the file*/
	pub fn audio(mut self, audio: impl Into<Audio>) -> Self {
		self.audio = Some(audio.into());
		self
	}
	/** *Optional*. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator*/
	pub fn author_signature(mut self, author_signature: impl Into<String>) -> Self {
		self.author_signature = Some(author_signature.into());
		self
	}
	/** *Optional*. Service message: user boosted the chat*/
	pub fn boost_added(mut self, boost_added: impl Into<ChatBoostAdded>) -> Self {
		self.boost_added = Some(boost_added.into());
		self
	}
	/** *Optional*. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.*/
	pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
		self.business_connection_id = Some(business_connection_id.into());
		self
	}
	/** *Optional*. Caption for the animation, audio, document, paid media, photo, video or voice*/
	pub fn caption(mut self, caption: impl Into<String>) -> Self {
		self.caption = Some(caption.into());
		self
	}
	pub fn add_caption_entity(mut self, caption_entity: impl Into<MessageEntity>) -> Self {
		self.caption_entities.push(caption_entity.into());
		self
	}
	/** *Optional*. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption*/
	pub fn caption_entities(mut self, caption_entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.caption_entities = caption_entities.into();
		self
	}
	/** *Optional*. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply\_to\_message if someone replies to a very first message in a channel.
	Default value: true*/
	pub fn channel_chat_created(mut self, channel_chat_created: bool) -> Self {
		self.channel_chat_created = Some(channel_chat_created);
		self
	}
	/** *Optional*. Service message: chat background set*/
	pub fn chat_background_set(mut self, chat_background_set: impl Into<ChatBackground>) -> Self {
		self.chat_background_set = Some(chat_background_set.into());
		self
	}
	/** *Optional*. Service message: a chat was shared with the bot*/
	pub fn chat_shared(mut self, chat_shared: impl Into<ChatShared>) -> Self {
		self.chat_shared = Some(chat_shared.into());
		self
	}
	/** *Optional*. The domain name of the website on which the user has logged in. [More about Telegram Login »](https://core.telegram.org/widgets/login)*/
	pub fn connected_website(mut self, connected_website: impl Into<String>) -> Self {
		self.connected_website = Some(connected_website.into());
		self
	}
	/** *Optional*. Message is a shared contact, information about the contact*/
	pub fn contact(mut self, contact: impl Into<Contact>) -> Self {
		self.contact = Some(contact.into());
		self
	}
	/** *Optional*. Service message: the chat photo was deleted
	Default value: true*/
	pub fn delete_chat_photo(mut self, delete_chat_photo: bool) -> Self {
		self.delete_chat_photo = Some(delete_chat_photo);
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
	/** *Optional*. Date the message was last edited in Unix time*/
	pub fn edit_date(mut self, edit_date: impl Into<i64>) -> Self {
		self.edit_date = Some(edit_date.into());
		self
	}
	/** *Optional*. Unique identifier of the message effect added to the message*/
	pub fn effect_id(mut self, effect_id: impl Into<String>) -> Self {
		self.effect_id = Some(effect_id.into());
		self
	}
	pub fn add_entity(mut self, entity: impl Into<MessageEntity>) -> Self {
		self.entities.push(entity.into());
		self
	}
	/** *Optional*. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text*/
	pub fn entities(mut self, entities: impl Into<Vec<MessageEntity>>) -> Self {
		self.entities = entities.into();
		self
	}
	/** *Optional*. Information about the message that is being replied to, which may come from another chat or forum topic*/
	pub fn external_reply(mut self, external_reply: impl Into<ExternalReplyInfo>) -> Self {
		self.external_reply = Some(external_reply.into());
		self
	}
	/** *Optional*. Service message: forum topic closed*/
	pub fn forum_topic_closed(mut self, forum_topic_closed: impl Into<ForumTopicClosed>) -> Self {
		self.forum_topic_closed = Some(forum_topic_closed.into());
		self
	}
	/** *Optional*. Service message: forum topic created*/
	pub fn forum_topic_created(mut self, forum_topic_created: impl Into<ForumTopicCreated>) -> Self {
		self.forum_topic_created = Some(forum_topic_created.into());
		self
	}
	/** *Optional*. Service message: forum topic edited*/
	pub fn forum_topic_edited(mut self, forum_topic_edited: impl Into<ForumTopicEdited>) -> Self {
		self.forum_topic_edited = Some(forum_topic_edited.into());
		self
	}
	/** *Optional*. Service message: forum topic reopened*/
	pub fn forum_topic_reopened(mut self, forum_topic_reopened: impl Into<ForumTopicReopened>) -> Self {
		self.forum_topic_reopened = Some(forum_topic_reopened.into());
		self
	}
	/** *Optional*. Information about the original message for forwarded messages*/
	pub fn forward_origin(mut self, forward_origin: impl Into<MessageOrigin>) -> Self {
		self.forward_origin = Some(forward_origin.into());
		self
	}
	/** *Optional*. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats*/
	pub fn from(mut self, from: impl Into<User>) -> Self {
		self.from = Some(from.into());
		self
	}
	/** *Optional*. Message is a game, information about the game. [More about games »](https://core.telegram.org/bots/api/#games)*/
	pub fn game(mut self, game: impl Into<Game>) -> Self {
		self.game = Some(game.into());
		self
	}
	/** *Optional*. Service message: the 'General' forum topic hidden*/
	pub fn general_forum_topic_hidden(mut self, general_forum_topic_hidden: impl Into<GeneralForumTopicHidden>) -> Self {
		self.general_forum_topic_hidden = Some(general_forum_topic_hidden.into());
		self
	}
	/** *Optional*. Service message: the 'General' forum topic unhidden*/
	pub fn general_forum_topic_unhidden(mut self, general_forum_topic_unhidden: impl Into<GeneralForumTopicUnhidden>) -> Self {
		self.general_forum_topic_unhidden = Some(general_forum_topic_unhidden.into());
		self
	}
	/** *Optional*. The message is a scheduled giveaway message*/
	pub fn giveaway(mut self, giveaway: impl Into<Giveaway>) -> Self {
		self.giveaway = Some(giveaway.into());
		self
	}
	/** *Optional*. Service message: a giveaway without public winners was completed*/
	pub fn giveaway_completed(mut self, giveaway_completed: impl Into<GiveawayCompleted>) -> Self {
		self.giveaway_completed = Some(giveaway_completed.into());
		self
	}
	/** *Optional*. Service message: a scheduled giveaway was created*/
	pub fn giveaway_created(mut self, giveaway_created: impl Into<GiveawayCreated>) -> Self {
		self.giveaway_created = Some(giveaway_created.into());
		self
	}
	/** *Optional*. A giveaway with public winners was completed*/
	pub fn giveaway_winners(mut self, giveaway_winners: impl Into<GiveawayWinners>) -> Self {
		self.giveaway_winners = Some(giveaway_winners.into());
		self
	}
	/** *Optional*. Service message: the group has been created
	Default value: true*/
	pub fn group_chat_created(mut self, group_chat_created: bool) -> Self {
		self.group_chat_created = Some(group_chat_created);
		self
	}
	/** *Optional*. *True*, if the message media is covered by a spoiler animation
	Default value: true*/
	pub fn has_media_spoiler(mut self, has_media_spoiler: bool) -> Self {
		self.has_media_spoiler = Some(has_media_spoiler);
		self
	}
	/** *Optional*. *True*, if the message can't be forwarded
	Default value: true*/
	pub fn has_protected_content(mut self, has_protected_content: bool) -> Self {
		self.has_protected_content = Some(has_protected_content);
		self
	}
	/** *Optional*. Message is an invoice for a [payment](https://core.telegram.org/bots/api/#payments), information about the invoice. [More about payments »](https://core.telegram.org/bots/api/#payments)*/
	pub fn invoice(mut self, invoice: impl Into<Invoice>) -> Self {
		self.invoice = Some(invoice.into());
		self
	}
	/** *Optional*. *True*, if the message is a channel post that was automatically forwarded to the connected discussion group
	Default value: true*/
	pub fn is_automatic_forward(mut self, is_automatic_forward: bool) -> Self {
		self.is_automatic_forward = Some(is_automatic_forward);
		self
	}
	/** *Optional*. True, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
	Default value: true*/
	pub fn is_from_offline(mut self, is_from_offline: bool) -> Self {
		self.is_from_offline = Some(is_from_offline);
		self
	}
	/** *Optional*. *True*, if the message is sent to a forum topic
	Default value: true*/
	pub fn is_topic_message(mut self, is_topic_message: bool) -> Self {
		self.is_topic_message = Some(is_topic_message);
		self
	}
	/** *Optional*. A member was removed from the group, information about them (this member may be the bot itself)*/
	pub fn left_chat_member(mut self, left_chat_member: impl Into<User>) -> Self {
		self.left_chat_member = Some(left_chat_member.into());
		self
	}
	/** *Optional*. Options used for link preview generation for the message, if it is a text message and link preview options were changed*/
	pub fn link_preview_options(mut self, link_preview_options: impl Into<LinkPreviewOptions>) -> Self {
		self.link_preview_options = Some(link_preview_options.into());
		self
	}
	/** *Optional*. Message is a shared location, information about the location*/
	pub fn location(mut self, location: impl Into<Location>) -> Self {
		self.location = Some(location.into());
		self
	}
	/** *Optional*. The unique identifier of a media message group this message belongs to*/
	pub fn media_group_id(mut self, media_group_id: impl Into<String>) -> Self {
		self.media_group_id = Some(media_group_id.into());
		self
	}
	/** *Optional*. Service message: auto-delete timer settings changed in the chat*/
	pub fn message_auto_delete_timer_changed(mut self, message_auto_delete_timer_changed: impl Into<MessageAutoDeleteTimerChanged>) -> Self {
		self.message_auto_delete_timer_changed = Some(message_auto_delete_timer_changed.into());
		self
	}
	/** *Optional*. Unique identifier of a message thread to which the message belongs; for supergroups only*/
	pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
		self.message_thread_id = Some(message_thread_id.into());
		self
	}
	/** *Optional*. The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.*/
	pub fn migrate_from_chat_id(mut self, migrate_from_chat_id: impl Into<i64>) -> Self {
		self.migrate_from_chat_id = Some(migrate_from_chat_id.into());
		self
	}
	/** *Optional*. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.*/
	pub fn migrate_to_chat_id(mut self, migrate_to_chat_id: impl Into<i64>) -> Self {
		self.migrate_to_chat_id = Some(migrate_to_chat_id.into());
		self
	}
	pub fn add_new_chat_member(mut self, new_chat_member: impl Into<User>) -> Self {
		self.new_chat_members.push(new_chat_member.into());
		self
	}
	/** *Optional*. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)*/
	pub fn new_chat_members(mut self, new_chat_members: impl Into<Vec<User>>) -> Self {
		self.new_chat_members = new_chat_members.into();
		self
	}
	pub fn add_new_chat_photo(mut self, new_chat_photo: impl Into<PhotoSize>) -> Self {
		self.new_chat_photo.push(new_chat_photo.into());
		self
	}
	/** *Optional*. A chat photo was change to this value*/
	pub fn new_chat_photo(mut self, new_chat_photo: impl Into<Vec<PhotoSize>>) -> Self {
		self.new_chat_photo = new_chat_photo.into();
		self
	}
	/** *Optional*. A chat title was changed to this value*/
	pub fn new_chat_title(mut self, new_chat_title: impl Into<String>) -> Self {
		self.new_chat_title = Some(new_chat_title.into());
		self
	}
	/** *Optional*. Message contains paid media; information about the paid media*/
	pub fn paid_media(mut self, paid_media: impl Into<PaidMediaInfo>) -> Self {
		self.paid_media = Some(paid_media.into());
		self
	}
	/** *Optional*. Telegram Passport data*/
	pub fn passport_data(mut self, passport_data: impl Into<PassportData>) -> Self {
		self.passport_data = Some(passport_data.into());
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
	/** *Optional*. Specified message was pinned. Note that the Message object in this field will not contain further *reply\_to\_message* fields even if it itself is a reply.*/
	pub fn pinned_message(mut self, pinned_message: impl Into<MaybeInaccessibleMessage>) -> Self {
		self.pinned_message = Some(pinned_message.into());
		self
	}
	/** *Optional*. Message is a native poll, information about the poll*/
	pub fn poll(mut self, poll: impl Into<Poll>) -> Self {
		self.poll = Some(poll.into());
		self
	}
	/** *Optional*. Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.*/
	pub fn proximity_alert_triggered(mut self, proximity_alert_triggered: impl Into<ProximityAlertTriggered>) -> Self {
		self.proximity_alert_triggered = Some(proximity_alert_triggered.into());
		self
	}
	/** *Optional*. For replies that quote part of the original message, the quoted part of the message*/
	pub fn quote(mut self, quote: impl Into<TextQuote>) -> Self {
		self.quote = Some(quote.into());
		self
	}
	/** *Optional*. Message is a service message about a refunded payment, information about the payment. [More about payments »](https://core.telegram.org/bots/api/#payments)*/
	pub fn refunded_payment(mut self, refunded_payment: impl Into<RefundedPayment>) -> Self {
		self.refunded_payment = Some(refunded_payment.into());
		self
	}
	/** *Optional*. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.*/
	pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
		self.reply_markup = Some(reply_markup.into());
		self
	}
	/** *Optional*. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further *reply\_to\_message* fields even if it itself is a reply.*/
	pub fn reply_to_message(mut self, reply_to_message: impl Into<Box<Message>>) -> Self {
		self.reply_to_message = Some(reply_to_message.into());
		self
	}
	/** *Optional*. For replies to a story, the original story*/
	pub fn reply_to_story(mut self, reply_to_story: impl Into<Story>) -> Self {
		self.reply_to_story = Some(reply_to_story.into());
		self
	}
	/** *Optional*. If the sender of the message boosted the chat, the number of boosts added by the user*/
	pub fn sender_boost_count(mut self, sender_boost_count: impl Into<i64>) -> Self {
		self.sender_boost_count = Some(sender_boost_count.into());
		self
	}
	/** *Optional*. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.*/
	pub fn sender_business_bot(mut self, sender_business_bot: impl Into<User>) -> Self {
		self.sender_business_bot = Some(sender_business_bot.into());
		self
	}
	/** *Optional*. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field *from* contains a fake sender user in non-channel chats.*/
	pub fn sender_chat(mut self, sender_chat: impl Into<Chat>) -> Self {
		self.sender_chat = Some(sender_chat.into());
		self
	}
	/** *Optional*. True, if the caption must be shown above the message media
	Default value: true*/
	pub fn show_caption_above_media(mut self, show_caption_above_media: bool) -> Self {
		self.show_caption_above_media = Some(show_caption_above_media);
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
	/** *Optional*. Message is a service message about a successful payment, information about the payment. [More about payments »](https://core.telegram.org/bots/api/#payments)*/
	pub fn successful_payment(mut self, successful_payment: impl Into<SuccessfulPayment>) -> Self {
		self.successful_payment = Some(successful_payment.into());
		self
	}
	/** *Optional*. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply\_to\_message if someone replies to a very first message in a directly created supergroup.
	Default value: true*/
	pub fn supergroup_chat_created(mut self, supergroup_chat_created: bool) -> Self {
		self.supergroup_chat_created = Some(supergroup_chat_created);
		self
	}
	/** *Optional*. For text messages, the actual UTF-8 text of the message*/
	pub fn text(mut self, text: impl Into<String>) -> Self {
		self.text = Some(text.into());
		self
	}
	/** *Optional*. Service message: users were shared with the bot*/
	pub fn users_shared(mut self, users_shared: impl Into<UsersShared>) -> Self {
		self.users_shared = Some(users_shared.into());
		self
	}
	/** *Optional*. Message is a venue, information about the venue. For backward compatibility, when this field is set, the *location* field will also be set*/
	pub fn venue(mut self, venue: impl Into<Venue>) -> Self {
		self.venue = Some(venue.into());
		self
	}
	/** *Optional*. Bot through which the message was sent*/
	pub fn via_bot(mut self, via_bot: impl Into<User>) -> Self {
		self.via_bot = Some(via_bot.into());
		self
	}
	/** *Optional*. Message is a video, information about the video*/
	pub fn video(mut self, video: impl Into<Video>) -> Self {
		self.video = Some(video.into());
		self
	}
	/** *Optional*. Service message: video chat ended*/
	pub fn video_chat_ended(mut self, video_chat_ended: impl Into<VideoChatEnded>) -> Self {
		self.video_chat_ended = Some(video_chat_ended.into());
		self
	}
	/** *Optional*. Service message: new participants invited to a video chat*/
	pub fn video_chat_participants_invited(mut self, video_chat_participants_invited: impl Into<VideoChatParticipantsInvited>) -> Self {
		self.video_chat_participants_invited = Some(video_chat_participants_invited.into());
		self
	}
	/** *Optional*. Service message: video chat scheduled*/
	pub fn video_chat_scheduled(mut self, video_chat_scheduled: impl Into<VideoChatScheduled>) -> Self {
		self.video_chat_scheduled = Some(video_chat_scheduled.into());
		self
	}
	/** *Optional*. Service message: video chat started*/
	pub fn video_chat_started(mut self, video_chat_started: impl Into<VideoChatStarted>) -> Self {
		self.video_chat_started = Some(video_chat_started.into());
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
	/** *Optional*. Service message: data sent by a Web App*/
	pub fn web_app_data(mut self, web_app_data: impl Into<WebAppData>) -> Self {
		self.web_app_data = Some(web_app_data.into());
		self
	}
	/** *Optional*. Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps)*/
	pub fn write_access_allowed(mut self, write_access_allowed: impl Into<WriteAccessAllowed>) -> Self {
		self.write_access_allowed = Some(write_access_allowed.into());
		self
	}
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
