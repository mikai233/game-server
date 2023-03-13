use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    let proto_path = Path::new("src/proto");
    println!("cargo:rerun-if-changed={}", proto_path.display());
    let all_protos = all_protos(proto_path)?;
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]");
    config.compile_protos(&all_protos, &[proto_path])?;
    Ok(())
}

fn all_protos(proto_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut result = vec![];
    for dir in walkdir::WalkDir::new(proto_path) {
        let dir = dir?;
        let dir_path = dir.path();
        if dir_path.is_file() && dir_path.extension().and_then(|t| { Some(t == "proto") }).unwrap_or(false) {
            result.push(dir_path.into());
        } else if dir_path != proto_path {
            let child_result = all_protos(dir_path)?;
            result.extend(child_result);
        }
    }
    Ok(result)
}