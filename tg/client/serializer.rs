#![allow(unused_variables)]

use core::str;
use serde::ser::{self, Impossible, Serialize};
use serde_json::{Serializer as JsonSerializer, Error as JsonSerializationError, ser::{CompactFormatter, Compound}};
use super::{FormParts, Part};

pub struct Serializer<'a> {
	map: &'a mut FormParts,
}
impl<'a> Serializer<'a> {
	pub fn new(map: &'a mut FormParts) -> Self { 
		Serializer { map }
	}
}

pub type SerError = JsonSerializationError;
pub type SerResult<T> = Result<T, SerError>;

impl<'a> ser::Serializer for &'a mut Serializer<'a> {
	type Ok = (); type Error = SerError;
 
	type SerializeStruct = Self;
	fn serialize_struct(self, name: &'static str, len: usize) -> SerResult<Self::SerializeStruct> { 
		Ok(self)
	}
	type SerializeMap = Impossible<(), Self::Error>;
	fn serialize_map(self, len: Option<usize>) -> SerResult<Self::SerializeMap> { unimplemented!() }

	type SerializeSeq = Impossible<(), Self::Error>;
	fn serialize_seq(self, len: Option<usize>) -> SerResult<Self::SerializeSeq> { unimplemented!() }

	type SerializeTuple = Impossible<(), Self::Error>;
	fn serialize_tuple(self, len: usize) -> SerResult<Self::SerializeTuple> { unimplemented!() }

	type SerializeTupleStruct = Impossible<(), Self::Error>;
	fn serialize_tuple_struct(self, name: &'static str, len: usize) -> SerResult<Self::SerializeTupleStruct> { unimplemented!() }

	type SerializeTupleVariant = Impossible<(), Self::Error>;
	fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> SerResult<Self::SerializeTupleVariant> { unimplemented!() }

	type SerializeStructVariant = Impossible<(), Self::Error>;
	fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> SerResult<Self::SerializeStructVariant> { unimplemented!() }

	fn serialize_bool(self, v: bool) -> SerResult<()> { unimplemented!() }
	fn serialize_i8(self, v: i8) -> SerResult<()> { unimplemented!() }
	fn serialize_i16(self, v: i16) -> SerResult<()> { unimplemented!() }
	fn serialize_i32(self, v: i32) -> SerResult<()> { unimplemented!() }
	fn serialize_i64(self, v: i64) -> SerResult<()> { unimplemented!() }
	fn serialize_u8(self, v: u8) -> SerResult<()> { unimplemented!() }
	fn serialize_u16(self, v: u16) -> SerResult<()> { unimplemented!() }
	fn serialize_u32(self, v: u32) -> SerResult<()> { unimplemented!() }
	fn serialize_u64(self, v: u64) -> SerResult<()> { unimplemented!() }
	fn serialize_f32(self, v: f32) -> SerResult<()> { unimplemented!() }
	fn serialize_f64(self, v: f64) -> SerResult<()> { unimplemented!() }
	fn serialize_char(self, v: char) -> SerResult<()> { unimplemented!() }
	fn serialize_str(self, v: &str) -> SerResult<()> { unimplemented!() }
	fn serialize_bytes(self, v: &[u8]) -> SerResult<()> { unimplemented!() }
	fn serialize_none(self) -> SerResult<()> { unimplemented!() }
	fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> SerResult<()> { unimplemented!() } 
	fn serialize_unit(self) -> SerResult<()> { unimplemented!() }
	fn serialize_unit_struct(self, name: &'static str) -> SerResult<()> { unimplemented!() }
	fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> SerResult<()> { unimplemented!() }
	fn serialize_newtype_struct<T: ?Sized + Serialize>(self, name: &'static str, value: &T) -> SerResult<()> { unimplemented!() }
	fn serialize_newtype_variant<T: ?Sized + Serialize>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> SerResult<()> { unimplemented!() }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer<'a> {
	type Ok = (); type Error = SerError;
	fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> SerResult<()> {
		let mut buffer = Vec::with_capacity(128);
		let mut serializer = FieldSerializer { buffer: &mut buffer, captured_type: FieldType::RawValue, json: Option::default() };
		value.serialize(&mut serializer)?;
		let value = match serializer.captured_type {
			FieldType::RawValue | FieldType::Json => unsafe { String::from_utf8_unchecked(buffer) }.into(),
			FieldType::Bytes => {
				unsafe { std::mem::transmute::<_, &Vec<u8>>(value), }
			}
		};

		self.map.push((key.into(), value));
		return Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
}

enum FieldType {
	RawValue,
	Bytes,
	Json,
	Stream,
}

struct FieldSerializer<'a> {
	pub buffer: &'a mut Vec<u8>,
	pub captured_type: FieldType,
	pub json: Option<JsonSerializer<&'a mut Vec<u8>, CompactFormatter>>
}
impl<'a> ser::Serializer for &'a mut FieldSerializer<'a> {
	type Ok = (); type Error = SerError;

	type SerializeStruct = Compound<'a, &'a mut Vec<u8>, CompactFormatter>;
	#[inline] fn serialize_struct(self, name: &'static str, len: usize) -> SerResult<Self::SerializeStruct> {
		self.captured_type = FieldType::Json;
		let json = self.json.get_or_insert_with(|| JsonSerializer::new(self.buffer));
		json.serialize_struct(name, len)
	}

	type SerializeMap = Compound<'a, &'a mut Vec<u8>, CompactFormatter>;
	#[inline] fn serialize_map(self, len: Option<usize>) -> SerResult<Self::SerializeMap> {
		self.captured_type = FieldType::Json;
		let json = self.json.get_or_insert_with(|| JsonSerializer::new(self.buffer));
		json.serialize_map(len)
	}

	type SerializeSeq = Compound<'a, &'a mut Vec<u8>, CompactFormatter>;
	#[inline] fn serialize_seq(self, len: Option<usize>) -> SerResult<Self::SerializeSeq> {
		self.captured_type = FieldType::Json;
		let json = self.json.get_or_insert_with(|| JsonSerializer::new(self.buffer));
		json.serialize_seq(len)
	}

	type SerializeTuple = Compound<'a, &'a mut Vec<u8>, CompactFormatter>;
	#[inline] fn serialize_tuple(self, len: usize) -> SerResult<Self::SerializeTuple> {
		self.captured_type = FieldType::Json;
		let json = self.json.get_or_insert_with(|| JsonSerializer::new(self.buffer));
		json.serialize_tuple(len)	
	}

	type SerializeTupleStruct = Compound<'a, &'a mut Vec<u8>, CompactFormatter>;
	#[inline] fn serialize_tuple_struct(self, name: &'static str, len: usize) -> SerResult<Self::SerializeTupleStruct> {
		self.captured_type = FieldType::Json;
		let json = self.json.get_or_insert_with(|| JsonSerializer::new(self.buffer));
		json.serialize_tuple_struct(name, len)
	}

	type SerializeTupleVariant = Compound<'a, &'a mut Vec<u8>, CompactFormatter>;
	#[inline] fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> SerResult<Self::SerializeTupleVariant> {
		self.captured_type = FieldType::Json;
		let json = self.json.get_or_insert_with(|| JsonSerializer::new(self.buffer));
		json.serialize_tuple_variant(name, variant_index, variant, len)
	}

	type SerializeStructVariant = Compound<'a, &'a mut Vec<u8>, CompactFormatter>;
	#[inline] fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> SerResult<Self::SerializeStructVariant> {
		self.captured_type = FieldType::Json;
		let serializer = self.json.get_or_insert_with(|| JsonSerializer::new(self.buffer));
		serializer.serialize_struct_variant(name, variant_index, variant, len)
	}

	#[inline] fn serialize_bool(self, v: bool) -> SerResult<()> {
		if v { self.buffer.extend(b"true") } else { self.buffer.extend(b"false") };
		Ok(())
	}
	#[inline] fn serialize_newtype_variant<T: ?Sized + Serialize>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> SerResult<()> {
		if name == "InputFile" {
			if variant == "Stream" {
				self.captured_type = FieldType::Stream;
			}
			else if variant == "Bytes" {
				self.captured_type = FieldType::Bytes;
			}
			return Ok(());
		}
		value.serialize(self)
	}
	#[inline] fn serialize_i8(self, v: i8) -> SerResult<()> {
		self.buffer.extend(itoa::Buffer::new().format(v).as_bytes()); Ok(())
	}
	#[inline] fn serialize_i16(self, v: i16) -> SerResult<()> {
		self.buffer.extend(itoa::Buffer::new().format(v).as_bytes()); Ok(())
	}
	#[inline] fn serialize_i32(self, v: i32) -> SerResult<()> {
		self.buffer.extend(itoa::Buffer::new().format(v).as_bytes()); Ok(())
	}
	#[inline] fn serialize_i64(self, v: i64) -> SerResult<()> {
		self.buffer.extend(itoa::Buffer::new().format(v).as_bytes()); Ok(())
	}
	#[inline] fn serialize_u8(self, v: u8) -> SerResult<()> {
		self.buffer.extend(itoa::Buffer::new().format(v).as_bytes()); Ok(())
	}
	#[inline] fn serialize_u16(self, v: u16) -> SerResult<()> {
		self.buffer.extend(itoa::Buffer::new().format(v).as_bytes()); Ok(())
	}
	#[inline] fn serialize_u32(self, v: u32) -> SerResult<()> {
		self.buffer.extend(itoa::Buffer::new().format(v).as_bytes()); Ok(())
	}
	#[inline] fn serialize_u64(self, v: u64) -> SerResult<()> {
		self.buffer.extend(itoa::Buffer::new().format(v).as_bytes()); Ok(())
	}
	#[inline] fn serialize_f32(self, v: f32) -> SerResult<()> {
		self.buffer.extend(ryu::Buffer::new().format_finite(v).as_bytes()); Ok(())
	}
	#[inline] fn serialize_f64(self, v: f64) -> SerResult<()> {
		self.buffer.extend(ryu::Buffer::new().format_finite(v).as_bytes()); Ok(())
	}
	#[inline] fn serialize_char(self, v: char) -> SerResult<()> {
		let mut buf = [0; 4];
		self.serialize_str(v.encode_utf8(&mut buf))?; Ok(())
	}
	#[inline] fn serialize_str(self, v: &str) -> SerResult<()> {
		self.buffer.extend(v.as_bytes()); Ok(())
	}
	#[inline] fn serialize_bytes(self, v: &[u8]) -> SerResult<()> {
		self.captured_type = FieldType::Bytes;
		self.buffer.extend(v); Ok(())
	}
	#[inline] fn serialize_none(self) -> SerResult<()> {
		Ok(())
	}
	#[inline] fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> SerResult<()> {
		value.serialize(self)
		// unimplemented!()
	}
	#[inline] fn serialize_newtype_struct<T: ?Sized + Serialize>(self, name: &'static str, value: &T) -> SerResult<()> {
		value.serialize(self)
		// unimplemented!()
	}
	fn serialize_unit(self) -> SerResult<()> {
		unimplemented!()
	}
	fn serialize_unit_struct(self, name: &'static str) -> SerResult<()> {
		unimplemented!()
	}
	fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> SerResult<()> {
		unimplemented!()
	}
}