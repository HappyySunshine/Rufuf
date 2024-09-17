use std::{io::Stdout, sync::{Arc, Mutex}};

use anyhow::Result;
use app::App;
use events::EventHandler;
use mlua::Lua;
use ratatui::{backend::CrosstermBackend, Terminal};

use self::{buffer::{WidgetType, Buffers}, fslist::FsList, panel::Panel, tabs::RufLayout};

mod fslist;
mod buffer;
mod panel;
mod tabs;
mod events;
mod app;

mod widgets;

pub async fn run(term: Terminal<CrosstermBackend<Stdout>>, lua: Lua) -> Result<()>{
    let mut app = App::new(term, lua)?;
    let mut event_handler  = EventHandler::new();
    loop{
        let action = event_handler.next().await?;
        app.update(action)?;
        if app.should_quit{
            break;
        }
    }
    Ok(())
}

