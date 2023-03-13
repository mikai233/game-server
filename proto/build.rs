fn main() -> anyhow::Result<()> {
    prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
    Ok(())
}