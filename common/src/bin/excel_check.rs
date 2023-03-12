use clap::Parser;

use common::excel::checker::{Checker, LuaChecker};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct CheckArg {
    #[clap(long, short, default_value = "common/lua/rust_entry.lua")]
    path: String,
}

fn main() -> anyhow::Result<()> {
    let arg = CheckArg::parse();
    let checker = LuaChecker;
    checker.check(arg.path)?;
    Ok(())
}