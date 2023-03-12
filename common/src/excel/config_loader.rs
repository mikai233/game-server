#[cfg(test)]
mod test {
    use std::env;

    use anyhow::Context;

    use crate::init_logger;
    use crate::lua_helper::RustUtil;

    #[test]
    fn load_cfg() -> anyhow::Result<()> {
        init_logger(tracing::Level::INFO).context("failed to init logger")?;
        let lua = unsafe { mlua::Lua::unsafe_new() };
        let rust_util = lua.create_proxy::<RustUtil>()?;
        lua.globals().set("RustUtil", rust_util)?;
        let current_dir = env::current_dir()?;
        let init_path = current_dir.join("lua/rust_entry.lua");
        let init = std::fs::read_to_string(init_path)?;
        lua.load(&init).exec()?;
        Ok(())
    }
}