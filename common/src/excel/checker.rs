use anyhow::anyhow;

use crate::excel::excel_define::{ArrayInt, ArrayUInt, CellType, DictionaryStringFloat, DictionaryStringInt, Vector2ArrayInt, Vector2Float, Vector2Int, Vector2String, Vector2UInt, Vector3ArrayInt, Vector3Float, Vector3Int, Vector3UInt};

macro_rules! check {
    ($data:expr,$ty:ty) => {
        if !$data.is_empty() {
            $data.parse::<$ty>()?;
        }
    };
}
pub trait Checker {
    type Input;
    type Output;
    fn check(&self, input: Self::Input) -> anyhow::Result<Self::Output>;
}

pub struct CellChecker;

impl Checker for CellChecker {
    type Input = (CellType, String);
    type Output = ();

    fn check(&self, input: Self::Input) -> anyhow::Result<Self::Output> {
        let (ty, data) = input;
        match ty {
            CellType::UInt => {
                check!(data,u32);
            }
            CellType::Int => {
                check!(data,i32);
            }
            CellType::Long => {
                check!(data,i64);
            }
            CellType::String => {
                check!(data,String);
            }
            CellType::Bool => {
                if data != "1" && data != "0" {
                    return Err(anyhow!(format!("illegal bool data: {}",data)));
                }
            }
            CellType::Vector3ArrayInt => {
                check!(data,Vector3ArrayInt);
            }
            CellType::Vector3Int => {
                check!(data,Vector3Int);
            }
            CellType::Vector2Int => {
                check!(data,Vector2Int);
            }
            CellType::Vector3UInt => {
                check!(data,Vector3UInt);
            }
            CellType::Vector2UInt => {
                check!(data,Vector2UInt);
            }
            CellType::Vector2ArrayInt => {
                check!(data,Vector2ArrayInt);
            }
            CellType::ArrayInt => {
                check!(data,ArrayInt);
            }
            CellType::ArrayUInt => {
                check!(data,ArrayUInt);
            }
            CellType::DictionaryStringFloat => {
                check!(data,DictionaryStringFloat);
            }
            CellType::DictionaryStringInt => {
                check!(data,DictionaryStringInt);
            }
            CellType::Lang => {
                check!(data,String);
            }
            CellType::Float => {
                check!(data,f32);
            }
            CellType::Double => {
                check!(data,f64);
            }
            CellType::Vector2Float => {
                check!(data,Vector2Float);
            }
            CellType::Vector3Float => {
                check!(data,Vector3Float);
            }
            CellType::Vector2String => {
                check!(data,Vector2String);
            }
        }
        Ok(())
    }
}

