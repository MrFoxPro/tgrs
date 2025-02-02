use std::{error::Error, fmt};

pub trait Method: serde::Serialize + Sized + Send + 'static {
	const NAME: &str;
	type Response: serde::de::DeserializeOwned + Send + 'static;
	fn exec(self, client: &impl TelegramBotClient) -> impl Future<Output = Result<Self::Response, ExecuteError>> {
		client.execute(self)
	}
}

pub trait TelegramBotClient {
	fn execute<M: Method>(&self, method: M) -> impl Future<Output = Result<M::Response, ExecuteError>>;
}

#[derive(Clone, Debug)]
pub struct ResponseError {
    description: String,
    error_code: Option<i64>,
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<i64>,
}
impl Error for ResponseError {}
impl fmt::Display for ResponseError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "a telegram error has occurred: description={}", self.description)?;
        if let Some(code) = self.error_code {
            write!(out, "; error_code={}", code)?;
        }
        if let Some(chat_id) = self.migrate_to_chat_id {
            write!(out, "; migrate_to_chat_id={}", chat_id)?;
        }
        if let Some(retry_after) = self.retry_after {
            write!(out, "; retry_after={}", retry_after)?;
        }
        Ok(())
    }
}

#[derive(Debug, derive_more::From)]
pub enum ExecuteError {
    Http(reqwest::Error),
    Response(ResponseError),
    TooManyRequests,
}

impl Error for ExecuteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::ExecuteError::*;
        Some(match self {
            Http(err) => err,
            Response(err) => err,
            TooManyRequests => return None,
        })
    }
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::ExecuteError::*;
        write!(out, "failed to execute method: {}",
            match self {
                Http(err) => err.to_string(),
                Response(err) => err.to_string(),
                TooManyRequests => "too many requests".to_string(),
            }
        )
    }
}