use core::str;
use serde::ser::{self, Impossible, Serialize};
use serde_json::{Serializer as JsonSerializer, Error as JsonSerializationError};

type OutputMap = Vec<(&'static str, String)>;

pub struct Serializer<'a> {
	map: &'a mut OutputMap
}
impl<'a> Serializer<'a> {
	pub fn new(map: &'a mut OutputMap) -> Self { 
		Serializer { map }
	}
}

type SerError = JsonSerializationError;
type Result<T> = std::result::Result<T, SerError>;

impl<'a> ser::Serializer for &'a mut Serializer<'a> {
	type Ok = ();
	type Error = SerError;

	type SerializeStruct = Self;
	fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
		Ok(self)
	}

	type SerializeMap = Impossible<(), Self::Error>;
	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
		unimplemented!()
	}

	type SerializeSeq = Impossible<(), Self::Error>;
	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
		unimplemented!()
	}

	type SerializeTuple = Impossible<(), Self::Error>;
	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
		unimplemented!()
	}

	type SerializeTupleStruct = Impossible<(), Self::Error>;
	fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
		unimplemented!()
	}

	type SerializeTupleVariant = Impossible<(), Self::Error>;
	fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant> {
		unimplemented!()
	}

	type SerializeStructVariant = Impossible<(), Self::Error>;
	fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant> {
		unimplemented!()
	}

	fn serialize_bool(self, v: bool) -> Result<()> {
		unimplemented!()
	}
	fn serialize_i8(self, v: i8) -> Result<()> {
		unimplemented!()
	}
	fn serialize_i16(self, v: i16) -> Result<()> {
		unimplemented!()
	}
	fn serialize_i32(self, v: i32) -> Result<()> {
		unimplemented!()
	}
	fn serialize_i64(self, v: i64) -> Result<()> {
		unimplemented!()
	}
	fn serialize_u8(self, v: u8) -> Result<()> {
		unimplemented!()
	}
	fn serialize_u16(self, v: u16) -> Result<()> {
		unimplemented!()
	}
	fn serialize_u32(self, v: u32) -> Result<()> {
		unimplemented!()
	}
	fn serialize_u64(self, v: u64) -> Result<()> {
		unimplemented!()
	}
	fn serialize_f32(self, v: f32) -> Result<()> {
		unimplemented!()
	}
	fn serialize_f64(self, v: f64) -> Result<()> {
		unimplemented!()
	}
	fn serialize_char(self, v: char) -> Result<()> {
		unimplemented!()
	}
	fn serialize_str(self, v: &str) -> Result<()> {
		unimplemented!()
	}
	fn serialize_bytes(self, v: &[u8]) -> Result<()> {
		unimplemented!()
	}
	fn serialize_none(self) -> Result<()> {
		unimplemented!()
	}
	fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
		unimplemented!()
	}
	fn serialize_unit(self) -> Result<()> {
		unimplemented!()
	}
	fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
		unimplemented!()
	}
	fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<()> {
		unimplemented!()
	}
	fn serialize_newtype_struct<T: ?Sized + Serialize>(self, name: &'static str, value: &T) -> Result<()> {
		unimplemented!()
	}
	fn serialize_newtype_variant<T: ?Sized + Serialize>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<()> {
		unimplemented!()
	}
}

impl<'a> ser::SerializeStruct for &'a mut Serializer<'a> {
	type Ok = (); type Error = SerError;
	fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<()> {
		let mut buffer = Vec::with_capacity(128);
		value.serialize(&mut JsonSerializer::new(&mut buffer))?;
		self.map.push((key, unsafe { String::from_utf8_unchecked(buffer) }));
		Ok(())
	}
	fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
		Ok(())
	}
}

// struct FieldSerializer;
// impl FieldSerializer {
// 	}
// 	pub fn into_inner(self) -> OutputMap {
// 		self.map
// 	}
// }

// type SerError = std::fmt::Error;
// type Result<T> = std::result::Result<T, SerError>;

// impl<'a> ser::Serializer for &'a mut Serializer {
// 	type Ok = ();
// 	type Error = SerError;

// 	type SerializeStruct = Self;
// 	fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
// 		Ok(self)
// 	}

// 	type SerializeMap = Impossible<(), Self::Error>;
// 	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
// 		unimplemented!()
// 	}

// 	type SerializeSeq = Impossible<(), Self::Error>;
// 	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
// 		unimplemented!()
// 	}

// 	type SerializeTuple = Impossible<(), Self::Error>;
// 	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
// 		unimplemented!()
// 	}

// 	type SerializeTupleStruct = Impossible<(), Self::Error>;
// 	fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
// 		unimplemented!()
// 	}

// 	type SerializeTupleVariant = Impossible<(), Self::Error>;
// 	fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant> {
// 		unimplemented!()
// 	}

// 	type SerializeStructVariant = Impossible<(), Self::Error>;
// 	fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant> {
// 		unimplemented!()
// 	}

// 	fn serialize_bool(self, v: bool) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_i8(self, v: i8) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_i16(self, v: i16) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_i32(self, v: i32) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_i64(self, v: i64) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_u8(self, v: u8) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_u16(self, v: u16) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_u32(self, v: u32) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_u64(self, v: u64) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_f32(self, v: f32) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_f64(self, v: f64) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_char(self, v: char) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_str(self, v: &str) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_bytes(self, v: &[u8]) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_none(self) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_unit(self) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_newtype_struct<T: ?Sized + Serialize>(self, name: &'static str, value: &T) -> Result<()> {
// 		unimplemented!()
// 	}
// 	fn serialize_newtype_variant<T: ?Sized + Serialize>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<()> {
// 		unimplemented!()
// 	}
// }

// impl<'a> ser::SerializeStruct for &'a mut Serializer {
// 	type Ok = (); type Error = SerError;
// 	fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<()> {
// 		let mut buffer = Vec::with_capacity(128);
// 		value.serialize(&mut JsonSerializer::new(&mut buffer)).unwrap();
// 		self.map.push((key, unsafe { String::from_utf8_unchecked(buffer) }));
// 		Ok(())
// 	}
// 	fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
// 		Ok(())
// 	}
// }