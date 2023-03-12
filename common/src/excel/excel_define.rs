use anyhow::anyhow;

use crate::excel::convert::{LuaWriter, ToLua};
use crate::excel::excel_define::KeyType::{All, AllKey, Client, ClientKey, Server, ServerKey};

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
    pub fn server_side() -> Vec<KeyType> {
        vec![AllKey, All, Server, ServerKey]
    }

    pub fn client_side() -> Vec<KeyType> {
        vec![AllKey, All, Client, ClientKey]
    }

    pub fn server_key() -> Vec<KeyType> {
        vec![AllKey, ServerKey]
    }

    pub fn client_key() -> Vec<KeyType> {
        vec![AllKey, ClientKey]
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
    pub commit_id: String,
    pub create_mills: u128,
    pub data: Vec<GameConfig>,
}

impl ToLua for GameConfig {
    type Output = String;

    fn to_lua(&self) -> anyhow::Result<Self::Output> {
        // todo key all
        let (key_index, _) = self.key_type.iter().enumerate().find(|(_, key)| { **key == AllKey || **key == ServerKey || **key == ClientKey || **key == All }).ok_or(anyhow!(format!("{} allkey|serverkey|clientkey not found",self.name)))?;
        let mut key_to_index = Vec::with_capacity(self.data.len());
        let mut formatted_rows = vec![];
        for (i, row_data) in self.data.iter().enumerate() {
            let mut formatted_one_cell = vec![];
            for (i, (cell_data, ty)) in row_data.iter().zip(&self.cell_type).enumerate() {
                let formatted_cell_data = LuaWriter::write(ty, cell_data)?;
                formatted_one_cell.push(format!("[{}] = {}", i + 1, formatted_cell_data));
                if i == key_index {
                    key_to_index.push((formatted_cell_data, i + 1));
                }
            }
            let formatted_one_row = format!("    [{}] = {{ {} }}", i + 1, formatted_one_cell.join(", "));
            formatted_rows.push(formatted_one_row);
        }
        let formatted_config = format!("{}", formatted_rows.join(",\n"));
        let mut formatted_id_to_index = vec![];
        for (id, index) in key_to_index {
            formatted_id_to_index.push(format!("[{}] = {}", id, index));
        }
        let formatted_id_to_index = format!("{}", formatted_id_to_index.join(", "));
        let mut formatted_key_to_index = vec![];
        for (i, n) in self.cell_name.iter().enumerate() {
            formatted_key_to_index.push(format!("['{}'] = {}", n, i + 1));
        }
        let formatted_key_to_index = format!("{}", formatted_key_to_index.join(", "));
        Ok(format!(r#"
local data = {{
{}
}}

local s_name = '{}'

local s_id = {{ {} }}

local s_key = {{ {} }}

{}
        "#, formatted_config, self.name, formatted_id_to_index, formatted_key_to_index, lua_meta_table()).trim().to_string())
    }
}

fn lua_meta_table() -> String {
    r#"
local meta = {
    __index = function(_, key)
        if type(key) == "string" then
            local i = s_id[key]
            local v = rawget(data, i)
            return v
        elseif type(key) == "number" then
            local v = rawget(data, key)
            return v
        else
            return nil
        end
    end,
    __newindex = function()
        error("Attempt to modify read-only table")
    end,
    __pairs = function(_, _)
        local function iter(tbl, key)
            local i, v = next(tbl, key)
            if i then
                v = tbl[i]
            end
            return i, v
        end
        return iter, data, nil
    end,
    __ipairs = function(_)
        local function iter(tbl, i)
            i = i + 1
            local v = tbl[i]
            if nil ~= v then
                return i, v
            end
        end
        return iter, data, 0
    end
}

local config = { name = s_name }

setmetatable(config, meta)
do
    local data_meta = {
        __index = function(table, key)
            if s_key[key] then
                return rawget(table, s_key[key]);
            else
                return nil
            end
        end,
        __newindex = function()
            error("Attempt to modify read-only table")
        end
    }
    for _, v in ipairs(data) do
        setmetatable(v, data_meta)
    end
end

return config
    "#.trim().to_string()
}