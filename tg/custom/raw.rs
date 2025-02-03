#![allow(dead_code)]

use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::{Error, Unexpected}};
use super::{Text, TextEntities};

#[derive(Deserialize, Serialize)]
pub struct RawBoostAdded {
    boost_count: i64,
}
impl RawBoostAdded {
    pub fn deserialize_value<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
        RawBoostAdded::deserialize(deserializer).map(|RawBoostAdded { boost_count }| boost_count)
    }
    pub fn serialize_value<S: Serializer>(value: &i64, serializer: S) -> Result<S::Ok, S::Error> {
        let value = RawBoostAdded { boost_count: *value, };
        value.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
pub struct RawCaption {
    pub caption: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<TextEntities>,
}
impl RawCaption {
    pub fn deserialize_value<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Text>, D::Error> {
        Option::<RawCaption>::deserialize(deserializer).map(|wrapper| {
            wrapper.map(|RawCaption { caption: data, caption_entities: entities }| Text { data, entities })
        })
    }
    pub fn serialize_value<S: Serializer>(value: &Option<Text>, serializer: S) -> Result<S::Ok, S::Error> {
        let value = value.clone().map(|value| RawCaption { caption: value.data, caption_entities: value.entities });
        value.serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawEmpty {}
impl RawEmpty {
    pub fn deserialize_value<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
        RawEmpty::deserialize(deserializer).map(|_| ())
    }
    pub fn serialize_value<S: Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
        RawEmpty {}.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
pub struct RawFlag;
impl RawFlag {
    pub fn deserialize_value<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
        True::deserialize(deserializer).map(|_| ())
    }
    pub fn serialize_value<S: Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
        True.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
pub struct RawText {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<TextEntities>,
}

impl RawText {
    pub fn deserialize_value<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Text, D::Error> {
        RawText::deserialize(deserializer).map(|x| Text {
            data: x.text,
            entities: x.entities,
        })
    }
    pub fn serialize_value<S: Serializer>(value: &Text, serializer: S) -> Result<S::Ok, S::Error> {
        RawText { text: value.data.clone(), entities: value.entities.clone(), }.serialize(serializer)
    }
}

macro_rules! impl_bool_type {
    ($name:ident = $value:ident) => {
        #[derive(Clone, Copy, PartialEq, PartialOrd)]
        pub(crate) struct $name;
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_bool($value)
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let b = bool::deserialize(deserializer)?;
                if b == $value {
                    Ok(Self)
                } else {
                    Err(D::Error::invalid_value(Unexpected::Bool(b), &stringify!($value)))
                }
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(&$value, formatter)
            }
        }
    };
}

impl_bool_type!(True = true);
impl_bool_type!(False = false);
