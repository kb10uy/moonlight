use std::io::Cursor;

use anyhow::Result;
use async_std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    task::block_on,
};
use maenami::{Error as MaenamiError, Parser};

async fn read_blender_obj(filename: impl AsRef<Path>) -> Result<()> {
    let mut parser = Parser::new(|filename, parent: &PathBuf| {
        let target_filename = parent.join(filename);
        let mtl_source = match block_on(read_to_string(target_filename)) {
            Ok(s) => s,
            Err(e) => return Err(MaenamiError::IoError(e)),
        };
        Ok(Cursor::new(mtl_source))
    });

    let filename = filename.as_ref();
    let obj_source = read_to_string(filename)
        .await
        .map_err(|e| MaenamiError::IoError(e))?;
    let wobj = parser.parse(
        Cursor::new(obj_source),
        filename.parent().expect("Not found").into(),
    )?;

    todo!();
}
