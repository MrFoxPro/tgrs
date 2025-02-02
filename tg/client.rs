use std::{error::Error, fmt};
use serde::Deserialize;

pub trait Method: serde::Serialize + Sized + Send + 'static {
	const NAME: &str;
	type Response: serde::de::DeserializeOwned + Send + 'static;
	fn exec<C: TelegramBotClient>(self, client: &C) -> impl Future<Output = Result<Self::Response, C::Error>> {
		client.execute(self)
	}
}

pub trait TelegramBotClient {
	type Error;
	fn execute<M: Method>(&self, method: M) -> impl Future<Output = Result<M::Response, Self::Error>>;
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
enum RawResponse<T> {
    Success {
        result: T,
    },
    Error {
        description: String,
        error_code: Option<i64>,
        parameters: Option<RawResponseParameters>,
    },
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct RawResponseParameters {
    migrate_to_chat_id: Option<i64>,
    /// Number of seconds left to wait before the request can be repeated.
    retry_after: Option<i64>,
}

/// Represents an API Response.
#[derive(Clone, Debug, Deserialize)]
#[serde(from = "RawResponse<T>")]
pub enum Response<T> { Success(T), Error(ResponseError) }

impl<T> Response<T> {
    /// Returns a number of seconds left to wait before the request can be repeated.
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            Response::Success(_) => None,
            Response::Error(err) => err.retry_after.and_then(|x| x.try_into().ok()),
        }
    }
    pub fn into_result(self) -> Result<T, ResponseError> {
        match self {
            Response::Success(obj) => Ok(obj),
            Response::Error(err) => Err(err),
        }
    }
}

impl<T> From<RawResponse<T>> for Response<T> {
    fn from(raw: RawResponse<T>) -> Self {
        match raw {
            RawResponse::Success { result, .. } => Response::Success(result),
            RawResponse::Error { description, error_code, parameters, .. } => Response::Error(ResponseError {
                description,
                error_code,
                migrate_to_chat_id: parameters.and_then(|x| x.migrate_to_chat_id),
                retry_after: parameters.and_then(|x| x.retry_after),
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResponseError {
    pub description: String,
    pub error_code: Option<i64>,
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<i64>,
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
