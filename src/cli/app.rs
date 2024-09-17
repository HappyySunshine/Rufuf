use std::{env::current_dir, io::Stdout, sync::Arc};

use anyhow::Result;
use mlua::Lua;
use ratatui::{prelude::CrosstermBackend, widgets::Clear, Terminal};
use tokio::sync::Mutex;

use super::{buffer::WidgetType, events::Actions, fslist::FsList, panel::Panel, tabs::RufLayout};

pub struct App{
    pub should_quit: bool,
    pub lua: Lua,
    term : Terminal<CrosstermBackend<Stdout>>,
    layout: RufLayout,
}




impl App{
    pub fn new(mut term: Terminal<CrosstermBackend<Stdout>>, lua: Lua)-> Result<Self>{
        let fs_list_buffer = Arc::new(Mutex::new(WidgetType::FsList(FsList::new(current_dir()?)?)));
        let panel = Panel::new(fs_list_buffer.clone(), true, term.get_frame().area());
        let layout = RufLayout::new(panel);
        let mut app = Self{should_quit: false,lua, term,layout};
        app.redraw();
        Ok(app)
    }

    pub fn redraw(&mut self){
        self.term.clear();
        
        let _ = self.term.draw(|frame| { 
            ui(frame, &self.layout, &self.lua);
        });
        // self.term.flush();
    }

    pub fn update(&mut self, action: Actions)-> Result<()>{
        match action{
            Actions::Quit => {self.should_quit = true;},
            Actions::Tick => {},
            Actions::Redraw=>{self.redraw()},
            Actions::Move(pos) => {
                let panel = self.layout.get_current_focus();
                let mut frame = self.term.get_frame();
                if panel.borrow().move_cursor(&mut frame, pos, &self.lua).is_ok(){
                   self.redraw();
                }
            },
            Actions::Select=>{
                let panel= self.layout.get_current_focus();
                if panel.borrow().select(&self.lua).is_ok(){
                    self.redraw();
                }
            }
            Actions::ToggleViewer => {
                let panel = self.layout.get_current_focus();
                if panel.borrow_mut().toggle_viewer(&mut self.layout, &self.lua).is_ok(){
                    self.redraw();
                }
            },
        }
        Ok(())
        
    }
}

fn ui(frame: &mut ratatui::prelude::Frame<'_>, layout: &RufLayout, config: &Lua) {
    // frame.render_widget(Clear, frame.area());
    layout.update_panels_size(frame.area());
    let display_panels = &layout.panels;
    for display in display_panels{
        display.borrow().to_display(frame, config);
    }
}
