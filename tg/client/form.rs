use std::{borrow::Cow, fmt};
use futures_util::AsyncRead;
use derive_more::derive::From;
use serde::Serialize;
use crate::{Attachable, InputFile, InputFileInner};

pub struct FormParts {
	pub inner: Vec<(Cow<'static, str>, Part)>
}
impl FormParts {
	pub fn new(capacity: usize) -> Self {
		Self { inner: Vec::with_capacity(capacity) }
	}
	#[inline] pub fn add_bool(&mut self, key: &'static str, v: impl Into<Option<bool>>) {
		if let Some(v) = v.into() { self.inner.push((key.into(), if v { "true" } else { "false" }.into())) }
	}
	#[inline] pub fn add_string(&mut self, key: &'static str, v: impl Into<Option<String>>) {
		if let Some(v) = v.into() { self.inner.push((key.into(), v.into())) }
	}
	#[inline] pub fn add_i64(&mut self, key: &'static str, v: impl Into<Option<i64>>) {
		if let Some(v) = v.into() { self.inner.push((key.into(), String::from(itoa::Buffer::new().format(v)).into())) }
	}
	#[inline] pub fn add_f32(&mut self, key: &'static str, v: impl Into<Option<f32>>) {
		if let Some(v) = v.into() { self.inner.push((key.into(), String::from(ryu::Buffer::new().format(v)).into())) }
	}
	#[inline] pub fn add_object(&mut self, key: &'static str, v: impl Serialize) { 
		self.inner.push((key.into(), serde_json::to_string(&v).unwrap().into()));
	}
	#[inline] pub fn add_file(&mut self, v: impl Into<Option<InputFile>>) {
		let Some(v) = v.into() else { return };
		match v.inner {
			InputFileInner::Bytes(bytes) => self.inner.push((v.filename.into(), bytes.into())),
			InputFileInner::Stream(Some(stream)) => self.inner.push((v.filename.into(), stream.into())),
			_ => return
		}
	}
	#[inline] pub fn add_attachable(&mut self, key: &'static str, v: impl Attachable) {
		v.attach(key, self)
	}
}

#[derive(From)]
pub enum Part {
	Text(Cow<'static, str>),
	Bytes(bytes::Bytes),
	Stream(Box<dyn AsyncRead + Send + Sync + Unpin>),
}
impl fmt::Debug for Part {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Part::Text(text) => writeln!(f, "Part::Text({text})"),
			Part::Bytes(_)   => writeln!(f, "Part::Bytes(<...>)"),
			Part::Stream(_)  => writeln!(f, "Part::Stream(<...>)"),
		}
	}
}

impl From<&'static str> for Part {
	fn from(value: &'static str) -> Self { Part::Text(value.into()) }
}
impl From<String> for Part {
	fn from(value: String) -> Self { Part::Text(value.into()) }
}
