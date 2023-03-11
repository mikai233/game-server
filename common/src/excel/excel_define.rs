use std::collections::HashMap;
use std::str::FromStr;

use anyhow::anyhow;
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

macro_rules! default_data {
    ($str:expr) => {
        if $str.is_empty() || $str == "0" {
            return Ok(Default::default());
        }
    };
}

#[derive(Default)]
pub struct Vector3ArrayInt(Vec<(i32, i32, i32)>);

impl FromStr for Vector3ArrayInt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        default_data!(s);
        let split_array = s.split(";").map(ToString::to_string).collect::<Vec<String>>();
        let mut result = vec![];
        for str in split_array {
            let split_str = str.split(",").map(ToString::to_string).collect::<Vec<String>>();
            if split_str.len() != 3 {
                return Err(anyhow!(format!("incorrect data: {}",s)));
            }
            let split_i32 = split_str.into_iter().map(|x| { x.parse::<i32>() }).collect::<Vec<_>>();
            result.push((split_i32[0].clone()?, split_i32[1].clone()?, split_i32[2].clone()?));
        }
        Ok(Vector3ArrayInt { 0: result })
    }
}

#[derive(Default)]
pub struct Vector3Int((i32, i32, i32));

impl FromStr for Vector3Int {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        default_data!(s);
        let split_str = s.split(",").map(ToString::to_string).collect::<Vec<String>>();
        if split_str.len() != 3 {
            return Err(anyhow!(format!("incorrect data: {}",s)));
        }
        let split_i32 = split_str.iter().map(|x| { x.parse::<i32>() }).collect::<Vec<_>>();
        Ok(Vector3Int {
            0: (split_i32[0].clone()?, split_i32[1].clone()?, split_i32[2].clone()?),
        })
    }
}

#[derive(Default)]
pub struct Vector2Int((i32, i32));

impl FromStr for Vector2Int {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        default_data!(s);
        let split_str = s.split(",").map(ToString::to_string).collect::<Vec<String>>();
        if split_str.len() != 2 {
            return Err(anyhow!(format!("incorrect data: {}",s)));
        }
        let split_i32 = split_str.iter().map(|x| { x.parse::<i32>() }).collect::<Vec<_>>();
        Ok(Vector2Int {
            0: (split_i32[0].clone()?, split_i32[1].clone()?),
        })
    }
}

#[derive(Default)]
pub struct Vector3UInt((u32, u32, u32));

impl FromStr for Vector3UInt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        default_data!(s);
        let split_str = s.split(",").map(ToString::to_string).collect::<Vec<String>>();
        if split_str.len() != 3 {
            return Err(anyhow!(format!("incorrect data: {}",s)));
        }
        let split_u32 = split_str.iter().map(|x| { x.parse::<u32>() }).collect::<Vec<_>>();
        Ok(Vector3UInt {
            0: (split_u32[0].clone()?, split_u32[1].clone()?, split_u32[2].clone()?),
        })
    }
}

#[derive(Default)]
pub struct Vector2UInt((u32, u32));

impl FromStr for Vector2UInt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        default_data!(s);
        let split_str = s.split(",").map(ToString::to_string).collect::<Vec<String>>();
        if split_str.len() != 2 {
            return Err(anyhow!(format!("incorrect data: {}",s)));
        }
        let split_u32 = split_str.iter().map(|x| { x.parse::<u32>() }).collect::<Vec<_>>();
        Ok(Vector2UInt {
            0: (split_u32[0].clone()?, split_u32[1].clone()?),
        })
    }
}

#[derive(Default)]
pub struct Vector2ArrayInt(Vec<(i32, i32)>);

impl FromStr for Vector2ArrayInt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        default_data!(s);
        let split_array = s.trim().split(";").map(ToString::to_string).collect::<Vec<String>>();
        let mut result = vec![];
        for str in split_array {
            let split_str = str.split(",").map(ToString::to_string).collect::<Vec<String>>();
            if split_str.len() != 2 {
                return Err(anyhow!(format!("incorrect data: {}",s)));
            }
            let split_i32 = split_str.iter().map(|x| { x.parse::<i32>() }).collect::<Vec<_>>();
            result.push((split_i32[0].clone()?, split_i32[1].clone()?));
        }
        Ok(Vector2ArrayInt { 0: result })
    }
}

#[derive(Default)]
pub struct ArrayInt(Vec<i32>);

impl FromStr for ArrayInt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        default_data!(s);
        let split_str = s.trim().split(",").map(ToString::to_string).collect::<Vec<String>>();
        let split_i32 = split_str.iter().map(|x| { x.parse::<i32>() }).collect::<Vec<_>>();
        let mut result = vec![];
        for x in split_i32 {
            let x = x?;
            result.push(x);
        }
        Ok(ArrayInt {
            0: result,
        })
    }
}

#[derive(Default)]
pub struct ArrayUInt(Vec<u32>);

impl FromStr for ArrayUInt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        default_data!(s);
        if s.is_empty() {
            return Ok(Default::default());
        }
        let split_str = s.split(",").map(ToString::to_string).collect::<Vec<String>>();
        let split_u32 = split_str.iter().map(|x| { x.parse::<u32>() }).collect::<Vec<_>>();
        let mut result = vec![];
        for x in split_u32 {
            let x = x?;
            result.push(x);
        }
        Ok(ArrayUInt {
            0: result,
        })
    }
}

#[derive(Default)]
pub struct DictionaryStringFloat(HashMap<String, f32>);

impl FromStr for DictionaryStringFloat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Default)]
pub struct DictionaryStringInt(HashMap<String, i32>);

impl FromStr for DictionaryStringInt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Default)]
pub struct Vector2Float((f32, f32));

impl FromStr for Vector2Float {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        default_data!(s);
        let split_str = s.split(",").map(ToString::to_string).collect::<Vec<String>>();
        if split_str.len() != 2 {
            return Err(anyhow!(format!("incorrect data: {}",s)));
        }
        let split_f32 = split_str.iter().map(|x| { x.parse::<f32>() }).collect::<Vec<_>>();
        Ok(Vector2Float {
            0: (split_f32[0].clone()?, split_f32[1].clone()?),
        })
    }
}

#[derive(Default)]
pub struct Vector3Float((f32, f32, f32));

impl FromStr for Vector3Float {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        default_data!(s);
        let split_str = s.split(",").map(ToString::to_string).collect::<Vec<String>>();
        if split_str.len() != 3 {
            return Err(anyhow!(format!("incorrect data: {}",s)));
        }
        let split_f32 = split_str.iter().map(|x| { x.parse::<f32>() }).collect::<Vec<_>>();
        Ok(Vector3Float {
            0: (split_f32[0].clone()?, split_f32[1].clone()?, split_f32[2].clone()?),
        })
    }
}

#[derive(Default)]
pub struct Vector2String((String, String));

impl FromStr for Vector2String {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        default_data!(s);
        let split_str = s.split(",").map(ToString::to_string).collect::<Vec<String>>();
        if split_str.len() != 2 {
            return Err(anyhow!(format!("incorrect data: {}",s)));
        }
        let split_string = split_str.iter().map(|x| { x.parse::<String>() }).collect::<Vec<_>>();
        Ok(Vector2String {
            0: (split_string[0].clone()?, split_string[1].clone()?),
        })
    }
}