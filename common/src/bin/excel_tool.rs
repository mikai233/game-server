use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::ops::Not;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context};
use calamine::{DataType, open_workbook, Reader, Xlsx};
use clap::Parser;
use lz4::EncoderBuilder;
use tracing::{error, info, warn};
use walkdir::WalkDir;

use common::excel::checker::{CellChecker, Checker};
use common::excel::convert::ToLua;
use common::excel::excel_define::{CellType, GameConfig, GameConfigs, KeyType};
use common::init_logger;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct ExcelArgs {
    #[clap(long, short)]
    input_path: String,
    #[clap(long, short)]
    bytes: bool,
    #[clap(long, short)]
    lua: bool,
    #[clap(long, short, default_value_t = get_output_path().expect("failed to get default output path, try to specify manually"))]
    output_path: String,
    #[clap(long, short, value_parser = clap::value_parser ! (u32).range(0..12), default_value_t = 4)]
    compress_level: u32,
    #[clap(long, value_parser = clap::value_parser ! (tracing::Level), default_value = "info")]
    log_level: tracing::Level,
    #[clap(long)]
    client: bool,
}

fn get_output_path() -> anyhow::Result<String> {
    let current_dir = env::current_dir()?.into_os_string().into_string().map_err(|_| { anyhow!("failed to convert os string to string") })?;
    Ok(format!("{}/lua/generated_excel", current_dir))
}


fn main() -> anyhow::Result<()> {
    let args = ExcelArgs::parse();
    init_logger(args.log_level).context("failed to init logger")?;

    let excel_dir = WalkDir::new(&args.input_path);
    let mut all_excel_path = vec![];
    for dir in excel_dir {
        let dir = dir?;
        if dir.file_type().is_file() {
            all_excel_path.push(dir.into_path());
        }
    }

    let mut game_configs = GameConfigs {
        data: Vec::with_capacity(all_excel_path.len()),
        create_mills: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
        ..Default::default()
    };
    for path in all_excel_path {
        if let Some(ext) = path.extension() {
            match ext.to_os_string().into_string() {
                Ok(ext) => {
                    if ext == "xlsx" {
                        if let Some(config) = read_game_config(path, &args)? {
                            game_configs.data.push(config);
                        }
                    } else {
                        warn!("ignore files that are not of type xlsx: {}", path.display());
                    }
                }
                Err(error) => {
                    error!("failed to convert osString: {:?}", error);
                }
            };
        } else {
            warn!("ignore files without extensions: {}", path.display());
        }
    }
    check_data_type(&game_configs)?;
    write_to_bytes(&game_configs, &args)?;
    generate_lua(&game_configs, &args)?;
    Ok(())
}

fn read_game_config(path: PathBuf, arg: &ExcelArgs) -> anyhow::Result<Option<GameConfig>> {
    let display_path = path.display().to_string();
    info!("read: {}", display_path);
    let mut workbook: Xlsx<_> = open_workbook(path).context(format!("open excel: {} failed", display_path))?;
    let (sheet_name, data) = &workbook.worksheets()[0];
    let mut cell_name = vec![];
    let mut cell_type = vec![];
    let mut key_type = vec![];
    let mut excel_data = vec![];
    for (i, row) in data.rows().enumerate() {
        let mut row_data = vec![];
        for data_type in row.iter() {
            match i {
                0 => {
                    match data_type {
                        DataType::String(data) => {
                            cell_name.push(data.trim().to_string());
                        }
                        other => {
                            return Err(anyhow!(format!("excel string expected, got: {}",other)));
                        }
                    }
                }
                1 => {
                    match data_type {
                        DataType::String(data) => {
                            cell_type.push(CellType::from_str(data.trim())?);
                        }
                        other => {
                            return Err(anyhow!(format!("excel string expected, got: {}",other)));
                        }
                    }
                }
                2 => {
                    match data_type {
                        DataType::String(data) => {
                            key_type.push(KeyType::from_str(data.trim()).context(format!("convert string {} to enum KeyType error", data))?);
                        }
                        other => {
                            return Err(anyhow!(format!("excel string expected, got: {}",other)));
                        }
                    }
                }
                3 | 4 => {}
                _ => {
                    row_data.push(data_type.to_string().trim().to_string());
                }
            }
        }
        if i >= 5 {
            excel_data.push(row_data);
        }
    }
    let config = GameConfig::builder()
        .name(sheet_name.clone())
        .cell_name(cell_name)
        .data(excel_data)
        .cell_type(cell_type)
        .key_type(key_type)
        .build();
    let final_config = if !arg.client {
        drop_client_data(config)
    } else {
        config
    };
    return if !final_config.key_type.is_empty() {
        Ok(Some(final_config))
    } else {
        Ok(None)
    };
}

fn check_data_type(config: &GameConfigs) -> anyhow::Result<()> {
    let mut errors = vec![];
    let cell_checker = CellChecker;
    for config in &config.data {
        for row in &config.data {
            for (data, ty) in row.iter().zip(&config.cell_type) {
                if let Some(error) = cell_checker.check((ty.clone(), data.clone())).err() {
                    errors.push(error);
                }
            }
        }
    }
    if errors.is_empty().not() {
        for error in errors {
            error!("{}",error);
        }
        return Err(anyhow!("excel data check failed"));
    }
    Ok(())
}

fn drop_client_data(config: GameConfig) -> GameConfig {
    let config_builder = GameConfig::builder().name(config.name);
    let mut server_key = HashMap::new();
    let all_server_side = KeyType::server_side();
    for (i, k) in config.key_type.iter().enumerate() {
        if all_server_side.contains(&k) {
            server_key.insert(i, k.clone());
        }
    }
    let mut server_cell_name = vec![];
    for (i, n) in config.cell_name.into_iter().enumerate() {
        if let Some(_) = server_key.get(&i) {
            server_cell_name.push(n);
        }
    }
    let config_builder = config_builder.cell_name(server_cell_name);

    let mut server_key_type = vec![];
    for (i, k) in config.key_type.into_iter().enumerate() {
        if let Some(_) = server_key.get(&i) {
            server_key_type.push(k);
        }
    }
    let config_builder = config_builder.key_type(server_key_type);

    let mut server_cell_type = vec![];
    for (i, k) in config.cell_type.into_iter().enumerate() {
        if let Some(_) = server_key.get(&i) {
            server_cell_type.push(k);
        }
    }
    let config_builder = config_builder.cell_type(server_cell_type);

    let mut server_cell_data = vec![];
    for row in config.data {
        let mut data = vec![];
        for (i, d) in row.into_iter().enumerate() {
            if let Some(_) = server_key.get(&i) {
                data.push(d);
            }
        }
        server_cell_data.push(data);
    }
    let config_builder = config_builder.data(server_cell_data);

    config_builder.build()
}

fn write_to_bytes(game_configs: &GameConfigs, args: &ExcelArgs) -> anyhow::Result<()> {
    if args.bytes {
        let encoded: Vec<u8> = bincode::serialize(&game_configs).context("failed to serialize GameConfigs")?;
        info!("encoded: {}",encoded.len());
        let path = PathBuf::from(args.output_path.clone());
        std::fs::create_dir_all(&path).context("failed to create dir")?;
        let path = path.join("config.bytes");
        let path_display = path.display().to_string();
        let mut encoder = EncoderBuilder::new().level(args.compress_level).build(File::create(path).context(format!("failed to create file: {}", path_display))?).context("failed to create EncoderBuilder")?;
        encoder.write(&encoded)?;
        info!("config.bytes write to: {}",path_display);
    }
    Ok(())
}

fn generate_lua(game_configs: &GameConfigs, args: &ExcelArgs) -> anyhow::Result<()> {
    if args.lua {
        let path = PathBuf::from(args.output_path.clone()).join("lua");
        if path.exists() {
            std::fs::remove_dir_all(&path).context("failed to remove dir")?;
        }
        std::fs::create_dir_all(&path).context("failed to create dir")?;
        for game_config in &game_configs.data {
            let path = path.join(format!("{}.lua", game_config.name));
            let lua_code = game_config.to_lua()?;
            let mut file = std::fs::File::create(path)?;
            file.write(lua_code.as_bytes()).context("failed to write lua config")?;
        }
        info!("generated lua write to: {}",path.display());
    }
    Ok(())
}