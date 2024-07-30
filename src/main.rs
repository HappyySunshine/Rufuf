use std::{any, io::{self, stdin, stdout}, ops::Deref};

use crossterm::{
    cursor::{self, SetCursorStyle}, event::{self, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
};
use file_logger::logger::LoggerConfig;
use mlua::Lua;
use ratatui::{prelude::*, widgets::*};
// use termion::{cursor, event::Key, input::{Keys, TermRead}, raw::IntoRawMode};
use rufuf::{cli, config};
// use ratatui::backend::

use file_logger::*;
fn main() -> anyhow::Result<()> {
    LoggerConfig::new().file("log_file")?.commit();
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut lua = Lua::new();
    config::load_default(&mut lua)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let result = cli::run(terminal,lua);
    match result{
        Ok(_) => {},
        Err(e)=> {file_dbg!(e)},
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

