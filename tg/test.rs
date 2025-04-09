use std::assert_matches::debug_assert_matches;
use serde_json::from_str;
use crate::*;


#[test]
fn test_chatmember_banned() {
		let json = r#"
		{
		  "status": "kicked",
		  "until_date": 8179542492,
		  "user": {
			"first_name": "Chat Boss",
			"id": 8179542492,
			"is_bot": true,
			"username": "asdfsadf"
		  }
		}
	"#;
	let member: ChatMember = from_str(json).unwrap();
	debug_assert_matches!(member, ChatMember::Banned {..});
}

#[test]
fn test_chatmember_member() {
	let json = r#"
		{
		  "status": "member",
		  "user": {
			"first_name": "Chat Boss",
			"id": 8179542492,
			"is_bot": true,
			"username": "asdfsadf"
		  }
		}
	"#;
	let member: ChatMember = from_str(json).unwrap();
	debug_assert_matches!(member, ChatMember::Member {..});
}

#[test]
fn test_chatmember_update() {
	let json = r#"
		{
			"chat": {
			  "id": -1002291315165,
			  "title": "ChatBoss test group",
			  "type": "supergroup"
			},
			"date": 1743324791,
			"from": {
			  "first_name": "redacted_first_name",
			  "id": 42453245,
			  "is_bot": false,
			  "language_code": "en",
			  "username": "redacted_username"
			},
			"new_chat_member": {
			  "status": "left",
			  "user": {
				"first_name": "Chat Boss",
				"id": 8179542492,
				"is_bot": true,
				"username": "sdafasdf"
			  }
			},
			"old_chat_member": {
			  "status": "member",
			  "user": {
				"first_name": "Chat Boss",
				"id": 8179542492,
				"is_bot": true,
				"username": "asdfsadf"
			  }
			}
		  }
	"#;
	let update: ChatMemberUpdated = from_str(json).unwrap();
	debug_assert_matches!(update, ChatMemberUpdated { new_chat_member: ChatMember::Left {..}, old_chat_member: ChatMember::Member {..}, .. });
}