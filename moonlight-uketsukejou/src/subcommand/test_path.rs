//! Subcommand `test-path`
//!
//! Simulates Moonlight encode/decode path.

use crate::{app::TestPathArguments, luautil::register_hierarchical_searcher};

use async_std::fs::read_to_string;
use std::env::current_dir;

use anyhow::{bail, Result};
use log::info;
use mlua::prelude::*;

pub async fn run(args: TestPathArguments) -> Result<()> {
    let curdir = match current_dir()?.to_str() {
        Some(dir) => dir.to_string(),
        None => bail!("Failed to fetch current directory"),
    };

    let lua = Lua::new();
    register_hierarchical_searcher(&lua, vec![curdir])?;

    let script = read_to_string(args.script).await?;
    let module: LuaTable = match lua.load(&script).eval()? {
        LuaValue::Table(t) => t,
        otherwise => bail!("Script returned wrong value: {:?}", otherwise),
    };

    let encode: LuaFunction = module.get("encode")?;
    let decode: LuaFunction = module.get("decode")?;
    let filter: LuaFunction = module.get("filter")?;

    println!("Original values: {:?}", args.values);

    let encoded_pixels = encode.call::<_, Vec<[f64; 3]>>(args.values)?;
    info!("Encoded pixels: {:?}", encoded_pixels);

    let filtered_pixels = filter.call::<_, Vec<[f64; 3]>>(encoded_pixels)?;
    info!("Filtered pixels: {:?}", filtered_pixels);

    let decoded_values = decode.call::<_, Vec<f64>>(filtered_pixels)?;
    println!("Filtered values: {:?}", decoded_values);

    Ok(())
}
