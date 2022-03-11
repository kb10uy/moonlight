//! Lua operations.

use async_std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    task::block_on,
};

use anyhow::Result;
use log::{debug, info, warn};
use mlua::prelude::*;

/// Registers hierarchical searcher function.
pub fn register_hierarchical_searcher(lua: &Lua, paths: Vec<String>) -> Result<()> {
    let globals = lua.globals();
    let package: LuaTable = globals.get("package")?;

    package.set("hlpath", paths)?;

    let searchers: LuaTable = package.get("searchers")?;
    searchers.raw_set(
        searchers.len()? + 1,
        lua.create_function(hierarchical_searcher)?,
    )?;
    Ok(())
}

/// Searches Lua modules hierarchically.
fn hierarchical_searcher(
    lua: &Lua,
    module: String,
) -> LuaResult<(LuaValue<'_>, Option<String>)> {
    let hlpath: Vec<String> = {
        let package: LuaTable = lua.globals().get("package")?;
        package.get("hlpath")?
    };

    // Loader searcher cannot be called asynchronously


    block_on(async move {
        let module_path = format!("{}.lua", module.replace(".", "/"));
        let mut target_path = None;
        for module_root in hlpath {
            let mut path = PathBuf::from(module_root);
            path.push(&module_path);

            if Path::is_file(&path).await {
                target_path = Some(path.to_string_lossy().to_string());
                break;
            }
        }

        let script = match &target_path {
            Some(path) => {
                info!("Loading Lua script {:?} for module {}", path, module);
                read_to_string(path).await?
            }
            None => {
                warn!("Lua module {} not found", module);
                let error_text = format!("{} not found in any root", module);
                return Ok((error_text.to_lua(lua)?, None));
            }
        };
        let loader_function = lua.load(&script).into_function()?;
        debug!("Lua module {:?}: {:?}", module, loader_function);

        Ok((LuaValue::Function(loader_function), target_path))
    })
}
