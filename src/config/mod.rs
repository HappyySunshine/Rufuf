// use crate::HexColorString;


use std::{collections::HashMap, fs::{self, DirEntry, File, ReadDir}, io::Read, path::{Path, PathBuf}};
use anyhow::{bail, Result};
use mlua::Lua;
// use hlua::{AnyHashableLuaValue, AnyLuaValue, Lua};
// use serde::{Deserialize, Serialize};

// #[derive(Debug)]
// pub struct Config{
    // pub colors: Colors
// }


// impl Config{
// }

use file_logger::*;

pub fn load_default(lua: &mut Lua )-> Result<()>{
    let file  = PathBuf::from("./src/config/default/colors.lua");
    lua.load(file).exec()?;

    Ok(())
}

