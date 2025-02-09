pub mod serializer;

use std::{borrow::Cow, error::Error, fmt, marker::PhantomData, future::Future};
use derive_more::derive::From;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(From)]
pub enum Part {
	Text(Cow<'static, str>),
	Bytes(Vec<u8>),
	// Reader(Box<dyn Read + Send>),
	// Stream(Box<dyn  + Unpin + Send>),
}
impl From<&'static str> for Part {
	fn from(value: &'static str) -> Self { Part::Text(value.into()) }
}
impl From<String> for Part {
	fn from(value: String) -> Self { Part::Text(value.into()) }
}

pub trait Executable: Sized + Send + 'static {
	type Response: serde::de::DeserializeOwned + Send + 'static;
	fn method_name(&self) -> &'static str;
	fn into_parts(self) -> Parts<Self::Response>;

	fn insert(self, key: impl Into<Cow<'static, str>>, value: impl Into<Part>) -> impl Executable<Response = Self::Response> {
		let mut parts = self.into_parts();
		parts.inner.push((key.into(), value.into()));
		parts
	}
	fn exec<C: TelegramBotClient>(self, client: &C) -> impl Future<Output = Result<Self::Response, C::Error>> + Send {
		client.exec(self)
	}
}

macro_rules! method {
	($name:ty, $id:literal, $response:ty) => {
		impl crate::client::Executable for $name {
			type Response = $response;
			fn method_name(&self) -> &'static str { $id }
			fn into_parts(self) -> crate::client::Parts<Self::Response> {
				let mut container = Vec::with_capacity(32);
				self.serialize(&mut crate::client::serializer::Serializer::new(&mut container)).unwrap();
				crate::client::Parts::from_serialized_method($id, container)
			}
		}
	};
}

pub struct Parts<R: DeserializeOwned + Send + 'static> {
	pub method_name: &'static str,
	pub inner: Vec<(Cow<'static, str>, Part)>,
	_response_type: std::marker::PhantomData<R>,
}
impl<R: DeserializeOwned + Send + 'static> Parts<R> {
	pub fn from_serialized_method(method_name: &'static str, parts: Vec<(Cow<'static, str>, String)>) -> Self {
		Self { method_name, inner: parts.into_iter().map(|(k, v)| (k, Part::Text(v.into()))).collect(), _response_type: PhantomData::default() }
	}
}
impl<R: DeserializeOwned + Send + 'static> Executable for Parts<R> {
	type Response = R;
	fn method_name(&self) -> &'static str { self.method_name }
	fn into_parts(self) -> Parts<Self::Response> { self }
}

pub trait TelegramBotClient {
	type Error;
	fn exec<E: Executable>(&self, method: E) -> impl Future<Output = Result<E::Response, Self::Error>> + Send;
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
