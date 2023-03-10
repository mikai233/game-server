use std::env;

use anyhow::Context;
use mlua::chunk;
use tracing::trace;

use crate::excel::convert::*;
use crate::excel::excel_define::CellType;
use crate::init_logger;
use crate::lua_helper::RustUtil;

#[macro_export]
macro_rules! parse {
    ($data:expr,$ty:ty) => {
        {
            let data: $ty = $data.parse()?;
            data
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
                let parsed = parse!(data,u32);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Int => {
                let parsed = parse!(data,i32);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Long => {
                let parsed = parse!(data,i64);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::String => {
                let parsed = parse!(data,String);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Bool => {
                let parsed = parse!(data,bool);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Vector3ArrayInt => {
                let parsed = parse!(data,Vector3ArrayInt);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Vector3Int => {
                let parsed = parse!(data,Vector3Int);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Vector2Int => {
                let parsed = parse!(data,Vector2Int);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Vector3UInt => {
                let parsed = parse!(data,Vector3UInt);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Vector2UInt => {
                let parsed = parse!(data,Vector2UInt);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Vector2ArrayInt => {
                let parsed = parse!(data,Vector2ArrayInt);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::ArrayInt => {
                let parsed = parse!(data,ArrayInt);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::ArrayUInt => {
                let parsed = parse!(data,ArrayUInt);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::DictionaryStringFloat => {
                unimplemented!("DictionaryStringFloat")
            }
            CellType::DictionaryStringInt => {
                unimplemented!("DictionaryStringInt")
            }
            CellType::Lang => {
                let parsed = parse!(data,String);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Float => {
                let parsed = parse!(data,f32);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Double => {
                let parsed = parse!(data,f64);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Vector2Float => {
                let parsed = parse!(data,Vector2Float);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Vector3Float => {
                let parsed = parse!(data,Vector3Float);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
            CellType::Vector2String => {
                let parsed = parse!(data,Vector2String);
                trace!("key: {}, parse data: {:?} to {:?}",ty,data,parsed);
            }
        }
        Ok(())
    }
}

pub struct LuaChecker;

impl Checker for LuaChecker {
    type Input = String;
    type Output = ();

    fn check(&self, input: Self::Input) -> anyhow::Result<Self::Output> {
        init_logger(tracing::Level::INFO).context("failed to init logger")?;
        let lua = mlua::Lua::new();
        let rust_util = lua.create_proxy::<RustUtil>()?;
        lua.globals().set("RustUtil", rust_util)?;
        println!("{}",input);
        lua.load(chunk!{
            package.path = "common/?.lua"
        }).exec()?;
        let current_dir = env::current_dir()?;
        let init_path = current_dir.join(input);
        let init = std::fs::read_to_string(init_path)?;
        lua.load(&init).exec()?;
        Ok(())
    }
}