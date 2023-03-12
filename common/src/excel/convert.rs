use std::collections::HashMap;

use crate::excel::excel_define::CellType;

pub type Vector3ArrayInt = Vec<(i32, i32, i32)>;
pub type Vector3Int = (i32, i32, i32);
pub type Vector2Int = (i32, i32);
pub type Vector3UInt = (u32, u32, u32);
pub type Vector2UInt = (u32, u32);
pub type Vector2ArrayInt = Vec<(i32, i32)>;
pub type ArrayInt = Vec<i32>;
pub type ArrayUInt = Vec<u32>;
pub type DictionaryStringFloat = HashMap<String, f32>;
pub type DictionaryStringInt = HashMap<String, i32>;
pub type Lang = String;
pub type Vector2Float = (f32, f32);
pub type Vector3Float = (f32, f32, f32);
pub type Vector2String = (String, String);

pub trait ToLua {
    type Output;
    fn to_lua(&self) -> anyhow::Result<Self::Output>;
}

pub trait Parse<T>: Sized {
    fn parse(&self) -> anyhow::Result<T>;
}

macro_rules! to_lua {
    ($ty:ty) => {
        impl ToLua for $ty {
            type Output = String;

            fn to_lua(&self) -> anyhow::Result<Self::Output> {
                Ok(self.to_string())
            }
        }
    };
}

to_lua!(u32);
to_lua!(i32);
to_lua!(i64);
// to_lua!(String);
impl ToLua for String {
    type Output = String;

    fn to_lua(&self) -> anyhow::Result<Self::Output> {
        let escape_string = self.replace("\"", "\\\"").replace("\r", "\\\r").replace("\n", "\\\n");
        Ok(format!("\"{}\"", escape_string))
    }
}
to_lua!(bool);
to_lua!(f32);
to_lua!(f64);

macro_rules! array_to_lua {
    ($name:ident) => {
        impl ToLua for $name {
            type Output = String;

            fn to_lua(&self) -> anyhow::Result<Self::Output> {
                let string_vec = self.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
                Ok(format!("{{ {} }}", string_vec))
            }
        }
    };
}

array_to_lua!(ArrayInt);
array_to_lua!(ArrayUInt);

macro_rules! vector3_to_lua {
    ($name:ident) => {
        impl ToLua for $name {
            type Output = String;

            fn to_lua(&self) -> anyhow::Result<Self::Output> {
                Ok(format!("{{ {}, {}, {} }}", self.0.to_lua()?, self.1.to_lua()?, self.2.to_lua()?))
            }
        }
    };
}

vector3_to_lua!(Vector3Int);
vector3_to_lua!(Vector3UInt);
vector3_to_lua!(Vector3Float);

macro_rules! vector_array_to_lua {
    ($name:ident) => {
        impl ToLua for $name {
            type Output = String;

            fn to_lua(&self) -> anyhow::Result<Self::Output> {
                let mut vector_vec = vec![];
                for vector in self {
                    let v = vector.to_lua()?;
                    vector_vec.push(v);
                }
                Ok(format!("{{ {} }}",vector_vec.join(", ")))
            }
        }
    };
}

vector_array_to_lua!(Vector3ArrayInt);
vector_array_to_lua!(Vector2ArrayInt);

macro_rules! vector2_to_lua {
    ($name:ident) => {
        impl ToLua for $name {
            type Output = String;

            fn to_lua(&self) -> anyhow::Result<Self::Output> {
                Ok(format!("{{ {}, {} }}", self.0.to_lua()?, self.1.to_lua()?))
            }
        }
    };
}

vector2_to_lua!(Vector2Int);
vector2_to_lua!(Vector2UInt);
vector2_to_lua!(Vector2Float);
vector2_to_lua!(Vector2String);

macro_rules! map_to_lua {
    ($name:ident) => {
        impl ToLua for $name {
            type Output = String;

            fn to_lua(&self) -> anyhow::Result<Self::Output> {
                let mut kv = vec![];
                for (k, v) in self {
                    kv.push(format!("{{{} = {}}}", k, v));
                }
                Ok(format!("{{ {} }}", kv.join(", ")))
            }
        }
    };
}

map_to_lua!(DictionaryStringFloat);
map_to_lua!(DictionaryStringInt);

macro_rules! default {
    ($str:expr) => {
        if $str.is_empty() || $str == "0" {
            return Ok(Default::default());
        }
    };
}

macro_rules! parse {
    ($ty:ty) => {
        impl Parse<$ty> for String where $ty: Default {
            fn parse(&self) -> anyhow::Result<$ty> {
                default!(self);
                Ok(std::str::FromStr::from_str(self)?)
            }
        }
    };
}

parse!(i32);
parse!(u32);
parse!(f32);
parse!(f64);
parse!(i64);
parse!(u64);
// parse!(bool);
impl Parse<bool> for String {
    fn parse(&self) -> anyhow::Result<bool> {
        default!(self);
        return if self == "1" || self == "true" {
            Ok(true)
        } else if self == "0" || self == "false" {
            Ok(false)
        } else {
            Err(anyhow::anyhow!(format!("incorrect cell data: {}",self)))
        };
    }
}
parse!(String);

macro_rules! parse_vector2 {
    ($ty:ty) => {
        impl Parse<$ty> for String {
            fn parse(&self) -> anyhow::Result<$ty> {
                default!(self);
                let vector2 = self.split(",").map(ToString::to_string).collect::<Vec<String>>();
                return if vector2.len() != 2 {
                    Err(anyhow::anyhow!(format!("incorrect cell data: {}",self)))
                } else {
                    Ok((vector2[0].parse()?, vector2[1].parse()?))
                };
            }
        }
    };
}

parse_vector2!(Vector2Int);
parse_vector2!(Vector2UInt);
parse_vector2!(Vector2String);
parse_vector2!(Vector2Float);

macro_rules! parse_vector3 {
    ($ty:ty) => {
        impl Parse<$ty> for String {
            fn parse(&self) -> anyhow::Result<$ty> {
                default!(self);
                let vector3 = self.split(",").map(ToString::to_string).collect::<Vec<String>>();
                return if vector3.len() != 3 {
                    Err(anyhow::anyhow!(format!("incorrect cell data: {}",self)))
                } else {
                    Ok((vector3[0].parse()?, vector3[1].parse()?, vector3[2].parse()?))
                };
            }
        }
    };
}

parse_vector3!(Vector3Int);
parse_vector3!(Vector3UInt);
parse_vector3!(Vector3Float);

macro_rules! parse_array {
    ($ty:ty) => {
        impl Parse<$ty> for String {
            fn parse(&self) -> anyhow::Result<$ty> {
                default!(self);
                let str_array = self.split(",").map(ToString::to_string).collect::<Vec<String>>();
                let mut array = vec![];
                for n in str_array {
                    array.push(n.parse()?);
                }
                Ok(array)
            }
        }
    };
}

parse_array!(ArrayInt);
parse_array!(ArrayUInt);

macro_rules! parse_vector_array {
    ($ty:ty) => {
        impl Parse<$ty> for String {
            fn parse(&self) -> anyhow::Result<$ty> {
                default!(self);
                let vector_array = self.split(";").map(ToString::to_string).collect::<Vec<String>>();
                let mut array = vec![];
                for vector in vector_array {
                    array.push(vector.parse()?);
                }
                Ok(array)
            }
        }
    };
}

parse_vector_array!(Vector2ArrayInt);
parse_vector_array!(Vector3ArrayInt);

pub struct LuaWriter;

impl LuaWriter {
    pub fn write(ty: &CellType, data: &String) -> anyhow::Result<String> {
        match ty {
            CellType::UInt => {
                crate::parse!(data,u32).to_lua()
            }
            CellType::Int => {
                crate::parse!(data,i32).to_lua()
            }
            CellType::Long => {
                crate::parse!(data,i64).to_lua()
            }
            CellType::String => {
                crate::parse!(data,String).to_lua()
            }
            CellType::Bool => {
                crate::parse!(data,bool).to_lua()
            }
            CellType::Vector3ArrayInt => {
                crate::parse!(data,Vector3ArrayInt).to_lua()
            }
            CellType::Vector3Int => {
                crate::parse!(data,Vector3Int).to_lua()
            }
            CellType::Vector2Int => {
                crate::parse!(data,Vector2Int).to_lua()
            }
            CellType::Vector3UInt => {
                crate::parse!(data,Vector3UInt).to_lua()
            }
            CellType::Vector2UInt => {
                crate::parse!(data,Vector2UInt).to_lua()
            }
            CellType::Vector2ArrayInt => {
                crate::parse!(data,Vector2ArrayInt).to_lua()
            }
            CellType::ArrayInt => {
                crate::parse!(data,ArrayInt).to_lua()
            }
            CellType::ArrayUInt => {
                crate::parse!(data,ArrayUInt).to_lua()
            }
            CellType::DictionaryStringFloat => {
                unimplemented!("DictionaryStringFloat")
            }
            CellType::DictionaryStringInt => {
                unimplemented!("DictionaryStringInt")
            }
            CellType::Lang => {
                crate::parse!(data,String).to_lua()
            }
            CellType::Float => {
                crate::parse!(data,f32).to_lua()
            }
            CellType::Double => {
                crate::parse!(data,f64).to_lua()
            }
            CellType::Vector2Float => {
                crate::parse!(data,Vector2Float).to_lua()
            }
            CellType::Vector3Float => {
                crate::parse!(data,Vector3Float).to_lua()
            }
            CellType::Vector2String => {
                crate::parse!(data,Vector2String).to_lua()
            }
        }
    }
}