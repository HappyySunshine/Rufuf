use std::{ cell::RefCell, ops::DerefMut, rc::Rc, sync::Arc};

use anyhow::Result;
use crossterm::{cursor};
use mlua::Lua;
use num_vector::NumVector;
use ratatui::{layout::{Margin, Rect}, style::Stylize, text::{Line, Span, Text}, widgets::{Block, Cell, Clear, Paragraph}};
use swift_vec::{ vector::Vec2};
use tokio::sync::Mutex;

use crate::tools;

use super::{buffer::{MyWidget, WidgetType}, tabs::RufLayout};


pub struct Panel{
   pub buffer: Arc<Mutex<WidgetType>>,
   pub cursor: CursorPos,
    pub state : PanelState,
    pub focus : bool,
    pub rect: Rect,
    pub overlay: Option<Rc<RefCell<Panel>>>
}

pub struct PanelState{
    pub boxed : bool,
}

impl PanelState{
    pub fn new(boxed: bool)->Self {
        Self{boxed}
    }
}

pub struct  CursorPos{
    x: u16,
    y:u16,
}
impl CursorPos{
}

impl CursorPos{
    pub fn new(x: u16, y:u16)-> Self{
        return Self{x,y}
    }
}

impl<'a> Panel{
    pub fn new(buffer: Arc<Mutex<WidgetType>>, focus: bool, rect: Rect)-> Self{
        let cursor = CursorPos::new(0, 0);
        let state = PanelState::new(true);
        

        Self{buffer, cursor, state, focus, rect, overlay: None }
}
}

pub struct DisplayPanel{
    pub panel: Rc<RefCell<Panel>>,
    pub rect: Rect,
}


impl Panel{
    pub fn to_display(&self, frame: &mut ratatui::prelude::Frame<'_>, config: &Lua){
        let mut buffer = self.buffer.try_lock().unwrap();
        match buffer.deref_mut(){
            WidgetType::FsList(list) => {
                let _ = list.update_display(&self, config);
                let _ = list.draw_to_frame(frame, self, config);
            },
            WidgetType::ReadFileWidget(buffer) => {
                let _ = buffer.draw_to_frame(frame, self, config);
                }
            WidgetType::ShowImage(image_widget) => {
                image_widget.draw_to_frame(frame, self, config);
            },
            };
    }

    pub fn move_cursor(&self, frame: &mut ratatui::prelude::Frame<'_>, dir: Vec2<i16>, config: &Lua)-> Result<()>{
        let mut buffer = self.buffer.try_lock().unwrap();
        match buffer.deref_mut(){
            WidgetType::FsList(list) => {
                list.move_selected(frame, dir, config)?;
                if let Some(panel) = &self.overlay{
                    let panel = panel.borrow();
                    // let a = panel.buffer.try_lock().unwrap();
                    *panel.buffer.try_lock().unwrap() = list.get_widget(panel.rect, config)?;
                }
            },
            WidgetType::ReadFileWidget(_) => todo!(),
            WidgetType::ShowImage(_) => todo!(),
        };
        Ok(())
        
    }

    pub fn select(&self,  config: &Lua)-> Result<()>{
        let mut buffer = self.buffer.try_lock().unwrap();
        match buffer.deref_mut(){
            WidgetType::FsList(list) => {
                let fs_list = list.select(self, config)?;
                *list = fs_list;
            },
            WidgetType::ReadFileWidget(_) => todo!(),
            WidgetType::ShowImage(_) => todo!(),
        };
        Ok(())
    }
    pub fn toggle_viewer(&mut self, layout: &mut RufLayout, config: &Lua)-> Result<()>{
        let mut buffer = self.buffer.try_lock().unwrap();
        if self.overlay.is_some(){
            let pointer = self.overlay.as_ref().unwrap().as_ptr();
            self.overlay = None;
            for index in 0..layout.panels.len(){
                if layout.panels.get(index).unwrap().as_ptr() == pointer{
                    layout.panels.remove(index);
                    return Ok(())
                }
            }
            return Ok(());
        }
        match buffer.deref_mut(){
            WidgetType::FsList(list) => {
                let mut rect = tools::divide(self.rect, 60);
                let widget = list.get_widget(rect, config)?;
                let mut rect = tools::center(rect, self.rect) ;
                rect.y +=5;
                let panel = Panel::new(Arc::new(Mutex::new(widget)), false, rect );
                let panel_ref = Rc::new(RefCell::new(panel));
                self.overlay = Some(panel_ref.clone());
                layout.panels.push(panel_ref);
            },
            WidgetType::ReadFileWidget(_) => todo!(),
            WidgetType::ShowImage(_) => todo!(),
        };
        Ok(())
    }

    pub fn switch_panel(&self, frame: &mut ratatui::prelude::Frame<'_>, config: &Lua){
        
    }
}
// pub struct PanelState{
// }

// impl PanelState


