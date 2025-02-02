use core::fmt;
use std::{error::Error, string::FromUtf16Error};
use crate::Message;

#[derive(Clone, Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub message: Message,
}
impl Command {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_args(&self) -> &[String] {
        &self.args
    }
    pub fn get_message(&self) -> &Message {
        &self.message
    }
}
impl TryFrom<Message> for Command {
    type Error = CommandError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_text().map(|text| (text.get_bot_commands(), text)) {
            Some((Some(commands), text)) => {
                // just take first command and ignore others
                let command = &commands[0];
                let name = command.command.clone();
                // assume that all text after command is arguments
                let offset = text.data.find(&name).unwrap_or(0);
                // bot suffix is 1 character longer due to '@' symbol
                let length = name.len() + command.bot_name.as_ref().map(|x| x.len() + 1).unwrap_or(0);
                let pos = offset + length;
                // pos is UTF-16 offset
                let raw_args: Vec<u16> = text.data.encode_utf16().skip(pos).collect();
                let raw_args = String::from_utf16(&raw_args)?;
				let args = raw_args.split_whitespace().map(ToOwned::to_owned).collect();
                Ok(Command { name, args, message })
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
        write!(
            out,
            "failed to parse command: {}",
            match self {
                CommandError::NotFound => String::from("not found"),
                CommandError::Utf16(err) => err.to_string(),
                CommandError::MismatchedQuotes => String::from("mismatched quotes"),
            }
        )
    }
}
