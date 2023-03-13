use std::path::Path;

use mlua::{ExternalError, UserDataMethods};
use mlua::prelude::LuaUserData;

use stardust_derive::{lua_function, lua_helper};

pub struct RustUtil;

#[lua_helper]
impl RustUtil {
    #[lua_function]
    fn list_files(path: String, recursive: Option<bool>, ext_filter: Option<String>) -> mlua::Result<Vec<String>> {
        fn _list_files(path: &Path, recursive: bool, filter: Option<String>) -> anyhow::Result<Vec<String>> {
            let mut dir_entry = vec![];
            for dir in walkdir::WalkDir::new(path) {
                let dir = dir?;
                if recursive && dir.file_type().is_dir() && dir.path() != path {
                    let child_entry = _list_files(dir.path(), recursive, filter.clone())?;
                    dir_entry.extend(child_entry);
                } else if dir.file_type().is_file() {
                    if let Some(filter) = filter.clone() {
                        if let Some(ext) = dir.path().extension() {
                            if let Some(ext) = ext.to_os_string().into_string().ok() {
                                if ext == filter {
                                    dir_entry.push(dir.path().to_string_lossy().replace("\\", "/"));
                                }
                            }
                        }
                    } else {
                        dir_entry.push(dir.path().to_string_lossy().replace("\\", "/"));
                    }
                }
            }
            Ok(dir_entry)
        }
        let path = Path::new(&path);
        let files = _list_files(path, recursive.unwrap_or_default(), ext_filter).map_err(|e| { e.to_lua_err() })?;
        Ok(files)
    }

    #[lua_function]
    fn strip_suffix(string: String, p: String) -> mlua::Result<Option<String>> {
        Ok(string.strip_suffix(&p).map(|t| { t.to_string() }))
    }

    #[lua_function]
    fn strip_prefix(string: String, p: String) -> mlua::Result<Option<String>> {
        Ok(string.strip_prefix(&p).map(|t| { t.to_string() }))
    }
}