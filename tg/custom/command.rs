use crate::Message;
use super::Text;
use std::{fmt, error::Error, string::FromUtf16Error};

#[derive(Clone, Debug)]
pub struct Command {
	pub name: String,
	pub args: Vec<String>,
	pub message: Message,
}
impl TryFrom<Message> for Command {
	type Error = CommandError;
	fn try_from(message: Message) -> Result<Self, Self::Error> {
		#[cfg(not(feature = "custom-message"))]
		let target = message.text.as_ref()
			.map(|text| Text { data: text.clone(), entities: message.entities.clone() })
			.map(|text| (text.get_bot_commands(), text));
		#[cfg(feature = "custom-message")]
		let target = message.get_text().map(|text| (text.get_bot_commands(), text));
		match target {
			Some((commands, text)) if commands.len() > 0 => {
				// just take first command and ignore others
				let command = &commands[0];
				let name = command.command.clone();
				// assume that all text after command is arguments
				let offset = text.data.find(&name).unwrap_or(0);
				// bot suffix is 1 character longer due to '@' symbol
				let length = name.len() + command.bot_name.as_ref().map(|x| x.len() + 1).unwrap_or(0);
				let pos = offset + length;
				// pos is UTF-16 offset
				let raw_args = text.data.encode_utf16().skip(pos).collect::<Vec<u16>>();
				let raw_args = String::from_utf16(&raw_args)?;
				let args = raw_args.split_whitespace().map(ToOwned::to_owned).collect();
				return Ok(Command { name, args, message });
			}
			_ => Err(CommandError::NotFound),
		}
	}
}

#[derive(Debug)]
pub enum CommandError {
	NotFound,
	Utf16(FromUtf16Error),
	MismatchedQuotes,
}
impl From<FromUtf16Error> for CommandError {
	fn from(err: FromUtf16Error) -> Self {
		Self::Utf16(err)
	}
}
impl Error for CommandError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			CommandError::NotFound => None,
			CommandError::Utf16(err) => Some(err),
			CommandError::MismatchedQuotes => None,
		}
	}
}
impl fmt::Display for CommandError {
	fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
		write!(out, "failed to parse command: {}",
			match self {
				CommandError::NotFound => String::from("not found"),
				CommandError::Utf16(err) => err.to_string(),
				CommandError::MismatchedQuotes => String::from("mismatched quotes"),
			}
		)
	}
}
