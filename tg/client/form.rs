use std::{borrow::Cow, fmt};
use futures_util::AsyncRead;
use derive_more::derive::From;
use serde::Serialize;
use crate::{Asset, InputFile};

pub struct FormParts {
	pub inner: Vec<(Cow<'static, str>, Part)>
}
impl FormParts {
	pub fn new(capacity: usize) -> Self {
		Self { inner: Vec::with_capacity(capacity) }
	}
	#[inline] pub fn add_bool(&mut self, k: &'static str, v: impl Into<Option<bool>>) {
		if let Some(v) = v.into() { self.inner.push((k.into(), if v { "true" } else { "false" }.into())) }
	}
	#[inline] pub fn add_string(&mut self, k: &'static str, v: impl Into<Option<String>>) {
		if let Some(v) = v.into() { self.inner.push((k.into(), v.into())) }
	}
	#[inline] pub fn add_i64(&mut self, k: &'static str, v: impl Into<Option<i64>>) {
		if let Some(v) = v.into() { self.inner.push((k.into(), String::from(itoa::Buffer::new().format(v)).into())) }
	}
	#[inline] pub fn add_f32(&mut self, k: &'static str, v: impl Into<Option<f32>>) {
		if let Some(v) = v.into() { self.inner.push((k.into(), String::from(ryu::Buffer::new().format(v)).into())) }
	}
	#[inline] pub fn add_object<T: Serialize>(&mut self, k: &'static str, v: T) { 
		self.inner.push((k.into(), serde_json::to_string(&v).unwrap().into()));
	}
	#[inline] pub fn add_file(&mut self, k: &'static str, v: impl Into<Option<InputFile>>) {
		let Some(v) = v.into() else { return };
		let id = self.inner.len();
		match v {
			InputFile::Bytes(bytes) => self.inner.push((id.to_string().into(), bytes.into())),
			InputFile::Stream(Some(stream)) => self.inner.push((id.to_string().into(), stream.into())),
			_ => return
		}
		self.inner.push((k.into(), format!("attach://{id}").into()));
	}
	#[inline] pub fn add_asset(&mut self, k: &'static str, v: impl Into<Option<Asset>>) {
		let Some(v) = v.into() else { return };
		match v {
			Asset::Url(url) => self.add_string(k, url),
			Asset::File(file) => self.add_file(k, file),
		}
	}
}


#[derive(From)]
pub enum Part {
	Text(Cow<'static, str>),
	Bytes(Vec<u8>),
	Stream(Box<dyn AsyncRead + Send + Sync + Unpin>),
}
impl fmt::Debug for Part {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	    writeln!(f, "kek")
	}
}

impl From<&'static str> for Part {
	fn from(value: &'static str) -> Self { Part::Text(value.into()) }
}
impl From<String> for Part {
	fn from(value: String) -> Self { Part::Text(value.into()) }
}
