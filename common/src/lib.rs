use tracing_subscriber::fmt::time::LocalTime;

pub mod excel;
pub mod lua_helper;

pub fn init_logger(max_level: tracing::Level) -> anyhow::Result<()> {
    let format = tracing_subscriber::fmt::format()
        .with_timer(LocalTime::rfc_3339())
        .compact();
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .event_format(format)
        .with_max_level(max_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use mlua::Lua;
}