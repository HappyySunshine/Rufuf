use std::{ any::Any, borrow::{Borrow, BorrowMut}, cell::RefCell, fmt::Display, fs::{self, DirEntry, ReadDir}, hash::Hash, io::{Stdout, Write}, num::Wrapping, ops::{Add, Deref, Index, IndexMut}, rc::Rc, sync::{Arc, Mutex}, u16, vec};

use anyhow::{anyhow, bail, Result};
use crossterm::{cursor::EnableBlinking, execute, style::{ SetBackgroundColor, SetForegroundColor}, ExecutableCommand};
use mlua::Lua;
use ratatui::{backend::CrosstermBackend, buffer::Buffer, layout::{self, Constraint, Layout, Rect}, style::{Color, Stylize}, text::Line, widgets::{ Block, BorderType, Borders, List, ListState, Paragraph, StatefulWidget}, Frame, Terminal};
pub mod events;
pub mod fs_entries;
pub mod display;

use fs_entries::*;

use file_logger::*;

use crate::tools::{clamp, clamp_simple};

use self::{display::{DisplayFsList, DisplayPanel, DisplayPanelsLayout, PanelType}, events::{CursorMov, Event}};

#[derive(Debug, Clone)]
pub struct  RufufLine{
   pub line: Line<'static>,
    pub depth: usize,
}

struct Index_Pos{
    index: usize,
    pos: usize
}
impl Index_Pos{
    pub fn new(index:usize, pos:usize)-> Self{
        return Index_Pos{index, pos}
        
    }
}

enum  Actions {
    Click(Index_Pos ),
    Redraw,
    None,
    
}

pub fn run(mut term: Terminal<CrosstermBackend<Stdout>>, config : Lua)-> Result<()>{
    let root_dir = fs::read_dir(".")?;
    let fs_list = FsListLayout::new(root_dir, 0)?;
    let fs_list_display = Rc::new(RefCell::new(DisplayFsList::new(fs_list, &config)));
    let mut layout = RufufLayout::new(PanelType::FsList(fs_list_display));
    let mut display_panels: DisplayPanelsLayout = DisplayPanelsLayout{panels: vec!(), current_focused: 0};
    layout.add(PanelType::ShowData);
    term.draw(|frame|{ui(frame, &layout, &mut display_panels, &config)})?;
    term.show_cursor()?;
    term.set_cursor(1, 1)?;

    loop{
        let event = events::get_event()?;
        let result = handle_event(event, &mut term,  &layout, &mut display_panels, &config);
        // display_panels.get_focused();
        // display_panels = DisplayPanelsLayout::empty();
        if result.is_err(){
            break
        }
        match result.unwrap(){
            Actions::Click(index_pos) => {
            },
            Actions::Redraw => {
                term.draw(|frame|{ui(frame, &layout, &mut display_panels, &config)})?;
                term.show_cursor()?;
            },
            Actions::None => {},
        }
    
    }
    Ok(())
}

// impl std::ops::Add<i16> for usize{
// }
//
//
struct Wrap<T>
where T: Add{
    num: T
}

impl Add<i16> for Wrap<usize>{
    type Output = usize;

    fn add(self, other: i16) -> Self::Output {
        let mut result;
        result = self.num as i16+ other;
        if result < 0{
            result = 0;
        }
        return result as usize
    }
}

fn handle_event<'a,'b,'c>(event: Event,  term: &mut Terminal<CrosstermBackend<Stdout>>, layout: &'a RufufLayout, display_panels: &'b mut DisplayPanelsLayout, config: &Lua) -> 
Result<Actions> 
where  'a: 'c, 'a :'b, 'b: 'c{
    match event{
        events::Event::Close => {bail!("errorrrr");},
        events::Event::MoveCursor(direction) =>{
            // let mut panel  = display_panels.get_focused();
            let mut panel = display_panels.get_focused_mut();
            match &panel.panel{
                PanelType::FsList(fs_list) => {
                     // use std::num::Wrapping;

                    // execute!(term, );
                    

                    let list = fs_list.try_borrow()?;
                    let lines = list.reference.to_lines(config);
                    let y1 = Wrap{ num: panel.cursor.y};
                    let y2 =  direction.y;
                    let new_y = y1 + y2;

                    let cursor_y = clamp_simple(new_y as u16, panel.area.y +1 , panel.area.y  + panel.area.height  -2);
                    panel.cursor.y = cursor_y as usize;
                    
                    // drop(panel);
                    term.set_cursor(1, cursor_y as u16);
                    

                    

                },
                PanelType::ShowData => todo!(),
            }

        },
        events::Event::ChangePanel(direction)=>{
            {
                let result = display_panels.get_panel(direction);
                if result.is_none(){
                   return Ok(Actions::None)
                }
                display_panels.current_focused = result.unwrap();
                let new_area = display_panels.get_focused().area;
                let cursor = (new_area.x, new_area.y);
                let new_pos = clamp(cursor, new_area, true);
                term.set_cursor(new_pos.0, new_pos.1)?;

            }

        },
        events::Event::Click=>{
            file_log!("CLICK");
            let panel = &display_panels.get_focused().panel;
            // *display_panels = DisplayPanelsLayout::empty();
            if let PanelType::FsList(list) = panel{
            let mut mut_list = list.try_borrow_mut()?;
                if mut_list.reference.click_at(3).is_none(){
                    return Ok(Actions::None);
                }
                mut_list.data = mut_list.reference.to_lines(&config);
                drop(mut_list);
            }
            return Ok(Actions::Redraw)
        },
   
   
        events::Event::ShowData =>{

        },
        events::Event::Continue =>{}
        }
    Ok(Actions::None)
}


fn ui<'b, 'a, 'c>(frame: &mut Frame, panels_layout: &'a RufufLayout, current_focused: &'b mut DisplayPanelsLayout, config: &Lua) 
where  'a : 'c {
    let mut display_panels = panels_layout.get_display_frames(&frame).unwrap();
    for display_panel in display_panels.iter(){
        match &display_panel.panel{
            PanelType::FsList(list) =>{
                let mut state = ListState::default();
                let mut vec = list.try_borrow().unwrap().data.clone();
                let at = vec.get_mut(0).unwrap();
                
                let line = at.line.clone().bg(Color::Blue);
                // let depth = at.depth;
                // *at = RufufLine{line, depth};
                at.line  = line;
                // vec[0].line =  vec.get_mut(0).unwrap().line.bg(Color::White);
                let fs_list = List::new(vec);

                // let fs_list = list.to_lines(config);
                let list = fs_list.block(Block::bordered().title("list"));
                // display_panel.data = 

                // let list = fs_list.to_list(config).block(Block::bordered().title("list"));
                // let list = list.to_list(config,0).block(Block::bordered().title("list"));
                frame.render_stateful_widget(list, display_panel.area, &mut state);
                
            },
            PanelType::ShowData => {

            },
        }
    }
    current_focused.panels = display_panels;
}







fn read_entries(root_dir: ReadDir, depth: usize)-> Result<Vec<FsEntry>>{
    let mut buffer = vec!();
    for entry in root_dir{
        let entry = entry?;
        let file_type = entry.file_type().unwrap();
        let file_name = entry.file_name();
        if file_type.is_dir(){
            buffer.push(FsEntry{entry, entry_type: EntryType::Directory, open: false,   children_count: 0 });
        }
        else{
            buffer.push(FsEntry{entry, entry_type: EntryType::File, open: false, children_count: 0});
        }
    }
    Ok(buffer)
}



#[derive(Debug, Clone)]
enum PanelData{
    List(List<'static>),
    None,
}


trait Cursor{

}

#[derive(Debug)]
enum RufufDirection{
    Horizontal(Vec<RufufDirection>),
    Vertical(Vec<RufufDirection>),
    Panel(usize)
}
#[derive(Debug)]
struct RufufLayout{
    panels : Vec<PanelType>,
    layout : RufufDirection,
    current_focused_panel : usize,
}

impl Index<usize> for RufufLayout{
    // type Output;
    type Output = PanelType;

    fn index(&self, _index: usize) -> &Self::Output {
        return &self.panels[0]
    }
}

impl IndexMut<usize> for RufufLayout{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.panels[index];
    }
}

impl RufufLayout{ 
    pub fn get_display_frames(&self, frame: &Frame) -> Result<Vec<DisplayPanel>>{
        let mut buffer = vec!();
        self.idk(&self.layout, frame.size(), &mut buffer).unwrap();
        file_dbg!(buffer);
        Ok(buffer)
    }
    fn idk<'a, 'b, 'c>(&'a self, dir: &'c RufufDirection, area:Rect, buffer: &'c mut Vec<DisplayPanel>)-> Result<()>
    where 'a: 'b, 'a: 'c{
        match dir{
            RufufDirection::Vertical(direction) =>{
                let len = direction.len();
                if len == 0{
                    // return
                    // anyhow!("lenght of 0 error");
                    bail!("lenght is 0 of vertical thing")
                }
                let mut start = area.y;
                for dir in direction{
                    let fraction = area.height/(len as u16);
                    let new_area = Rect::new(area.x, start, area.width, fraction);
                    start+= fraction;
                    file_log!(level="TEST", "vertical");
                    file_dbg!(area);
                    self.idk(dir, new_area, buffer);
                }
            }
            RufufDirection::Horizontal(direction)=>{
                let len = direction.len();
                if len == 0{
                    // panic!("qaaaaa");
                    bail!("lenght is 0 of horizontal thing")
                    // return
                }
                // let constrain = Constraint::P 
                let mut start = area.x;
                for dir in direction{

                    let fraction = (area.width/(len as u16)) as u16 ;
                    let new_area = Rect::new(start, area.y, fraction, area.height);
                    start+= fraction;
                    file_dbg!(area);
                     self.idk(dir, new_area, buffer);
                }
            }
            RufufDirection::Panel(number) =>{
                let reference = self.panels.get(number.clone()).unwrap();
                let display_panel =  DisplayPanel::new(reference, area, Cords{x:0,y:0});
                buffer.push(display_panel); 
            }
        }
        Ok(())

    }

    fn add(&mut self, panel: PanelType){
        self.panels.push(panel);
        let index = self.panels.len() -1 ;
        // self.layout = RufufDirection::Vertical(vec!());
        let lay = self.layout.borrow_mut();
        match lay{
            RufufDirection::Horizontal(_) => todo!(),
            RufufDirection::Vertical(direction) => {
                direction.push(RufufDirection::Panel(0));

            },
            RufufDirection::Panel(_) => todo!(),
        }

    }

    fn new(panel: PanelType)-> Self{
        let panels = vec!(panel);
        // let panels = vec!(DisplayPanel::new(panel));
        let layout = RufufDirection::Vertical(vec!(RufufDirection::Panel(0)));
        RufufLayout{ panels, layout, current_focused_panel: 0}
        // return RufufLayout { panels: vec!(vec!(panel)) }
    }

    fn draw_to_frame(&self, frame: &mut Frame){
    }
}


