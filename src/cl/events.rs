use std::{i16, num::Wrapping, u16};

use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyModifiers};
// use ratatui::layout::Direction;


pub enum Event {
    Close,
    Continue,
    MoveCursor(CursorMov),
    Click,
    ShowData,
    ChangePanel(CursorMov)
}

pub struct CursorMov{
    pub x : i16,
    pub y : i16,
}

impl CursorMov{
    fn new(x: i16,y : i16)-> Self{
        return CursorMov{x,y}
    }
   pub fn add_to_cursor(&self, cursor: (u16, u16))->(u16, u16){
        let mut x = self.x + cursor.0 as i16;
        let mut y = self.y + cursor.1 as i16;
        if x<0{
            x = 0;
        }
        if y <0{
            y=0;
        }
        return (x as u16, y as u16);
    }
}


use file_logger::*;
pub fn get_event()-> Result<Event>{
    let mod1 = KeyModifiers::CONTROL;
    if event::poll(std::time::Duration::from_millis(50))? {
        if let crossterm::event::Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press{
                if key.modifiers.contains(KeyModifiers::CONTROL){
                    if key.code == KeyCode::Right{
                        return Ok(Event::ShowData)
                    }
                }
                if key.code == KeyCode::Char('q') {

                    return Ok(Event::Close);
                }
                if key.code == KeyCode::Char('j') {
                    if key.modifiers.contains(mod1){
                        file_log!(level="IDK", "going down");
                        return Ok(Event::ChangePanel(CursorMov::new(0,1)));

                    }
                    return Ok(Event::MoveCursor(CursorMov::new(0,1)));
                }
                if  key.code == KeyCode::Char('k') {
                    if key.modifiers.contains(mod1){
                        file_log!(level="IDK", "going up");
                        return Ok(Event::ChangePanel(CursorMov::new(0, -1)));
                    }
                    return Ok(Event::MoveCursor(CursorMov::new(0,-1)));
                }
                if key.code == KeyCode::Char('l') {
                    if key.modifiers.contains(mod1){
                        return Ok(Event::ChangePanel(CursorMov::new(1,0)));
                    }
                    return Ok(Event::MoveCursor(CursorMov::new(1,0)));
                }
                if key.code == KeyCode::Char('h') {
                    if key.modifiers.contains(mod1){
                        return Ok(Event::ChangePanel(CursorMov::new(-1,0)));
                    }
                    return Ok(Event::MoveCursor(CursorMov::new(-1,0)));
                }
                if  key.code == KeyCode::Enter {
                    return Ok(Event::Click);
                }
            }
        }
    }
    // Direction::Vertical()
    Ok(Event::Continue)
}
