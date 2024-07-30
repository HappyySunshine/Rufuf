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
    // let dir = fs::ReadDir(Path::new("./src/config/default"))?;
    let file  = PathBuf::from("./src/config/default/colors.lua");
    // let file = File::open(Path::new("./src/config/default/colors.lua"))?;
    file_dbg!(file);
    // let bytes = file.bytes();
    lua.load(file).exec()?;
    // lua.execute_from_reader(file)?;


    Ok(())
}

