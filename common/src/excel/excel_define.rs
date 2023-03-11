use std::str::FromStr;

use strum::IntoEnumIterator;

use crate::excel::excel_define::KeyType::{All, AllKey, Server, ServerKey};

#[derive(strum::EnumString, strum::Display, strum::EnumIter, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum CellType {
    #[strum(serialize = "uint")]
    UInt,
    Int,
    Long,
    String,
    Bool,
    Vector3ArrayInt,
    Vector3Int,
    #[strum(serialize = "vector2_int", serialize = "vector[int,int]")]
    Vector2Int,
    #[strum(serialize = "vector3_uint")]
    Vector3UInt,
    #[strum(serialize = "vector2_uint")]
    Vector2UInt,
    Vector2ArrayInt,
    ArrayInt,
    #[strum(serialize = "array_uint")]
    ArrayUInt,
    DictionaryStringFloat,
    DictionaryStringInt,
    Lang,
    Float,
    Double,
    #[strum(serialize = "vector[float,float]")]
    Vector2Float,
    #[strum(serialize = "vector[float,float,float]")]
    Vector3Float,
    #[strum(serialize = "vector[string,string]")]
    Vector2String,
}

#[derive(strum::EnumString, strum::Display, strum::EnumIter, Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub enum KeyType {
    #[strum(serialize = "allkey")]
    AllKey,
    All,
    Client,
    #[strum(serialize = "clientkey")]
    ClientKey,
    Server,
    #[strum(serialize = "serverkey")]
    ServerKey,
}

impl KeyType {
    pub fn all_server_key() -> Vec<KeyType> {
        vec![AllKey, All, Server, ServerKey]
    }

    pub fn all_client_key() -> Vec<KeyType> {
        let server_keys = Self::all_server_key();
        KeyType::iter().filter(|x| {
            !server_keys.contains(x)
        }).collect()
    }
}

#[derive(typed_builder::TypedBuilder, Debug, serde::Deserialize, serde::Serialize)]
pub struct GameConfig {
    pub name: String,
    pub cell_name: Vec<String>,
    pub key_type: Vec<KeyType>,
    pub cell_type: Vec<CellType>,
    pub data: Vec<Vec<String>>,
}

#[derive(typed_builder::TypedBuilder, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GameConfigs {
    pub data: Vec<GameConfig>,
}