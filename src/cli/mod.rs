use std::{ any::Any, borrow::{Borrow, BorrowMut}, fs::{self, DirEntry, ReadDir}, hash::Hash, io::Stdout, ops::{Index, IndexMut}, sync::Mutex, u16, vec};

use anyhow::{anyhow, bail, Result};
use crossterm::{cursor::EnableBlinking, ExecutableCommand};
use mlua::Lua;
use ratatui::{backend::CrosstermBackend, buffer::Buffer, layout::{self, Constraint, Layout, Rect}, style::Stylize, text::Line, widgets::{ Block, BorderType, Borders, List, ListState, Paragraph, StatefulWidget}, Frame, Terminal};
pub mod events;
pub mod fs_entries;

use fs_entries::*;

use file_logger::*;

use crate::tools::clamp;

use self::events::{CursorMov, Event};

struct DisplayPanelsLayout<'a>{
    panels: Vec<DisplayPanel<'a>>,
    current_focused: usize

}

impl<'a> DisplayPanelsLayout<'a>{
    pub fn get_panel<'b>(&'a self, dir:CursorMov)-> Option<usize>
    where 'a: 'b{
        let mut buffer= vec!();
        let focused = self.get_focused();
        let mut index: usize = 0;
        for panel in self.panels.iter(){

            
            if dir.x == 1{
                if panel.area.x >= focused.area.x + focused.area.width{
                    buffer.push((panel, index));
                }
            }
            if dir.x == -1{
                if panel.area.x <=focused.area.x {
                    buffer.push((panel, index));
                }
            }

            if dir.y == 1{
                if panel.area.y >= focused.area.x +focused.area.height{
                    buffer.push((panel, index));
                }
            }

            if dir.y == -1 {
                if panel.area.y <= focused.area.y{
                    buffer.push((panel, index));
                }
            }
             index +=1;

        }
        if buffer.len() == 0{
            return None
            
        }
        
        // self.current_focused  = buffer.get(0).unwrap().1;
        let a = buffer[0].1;
        return Some(a);
        // return Some(buffer.get(0).unwrap().0);
        
        
        // return None


    }

    pub fn get_focused(&'a self)-> &DisplayPanel{
        return self.panels.get(self.current_focused).unwrap();
    }
}
//
enum Directions{
    Right,
    Down,
    None,
}

pub fn run(mut term: Terminal<CrosstermBackend<Stdout>>, lua : Lua)-> Result<()>{
    let root_dir = fs::read_dir(".")?;
    let mut list_layout = FsListLayout::new(root_dir, 1)?;
    let mut layout = RufufLayout::new(PanelType::FsList(list_layout));
    layout.add(PanelType::ShowData);
    // let mut display_panels : Vec<DisplayPanel> = vec!();
    // let mut current_focused: Option<DisplayPanel> = None;
    let mut display_panels: DisplayPanelsLayout = DisplayPanelsLayout{panels: vec!(), current_focused: 0};
    // let directions = vec!(Directions::None);

    term.draw(|frame|{ui(frame, &mut layout, &mut display_panels)})?;
    term.show_cursor()?;
    term.set_cursor(1, 1)?;
    // loop{

    // }
    loop{
        let event = events::get_event()?;
        let result = handle_event(event, &mut term, &mut layout, &mut display_panels);
        if result.is_err(){
            break
        }
        match event{
            events::Event::Close => {break;},
            events::Event::MoveCursor(direction) =>{
                let cursor_pos = term.get_cursor()?;
                let new_pos = direction.add_to_cursor(cursor_pos);
                // layout.current_focused_panel
                // let area = current_focused.as_ref().unwrap().area;
                let area = display_panels.get_focused().area;
                let new_pos = clamp(new_pos, area, true);
                term.set_cursor(new_pos.0 , new_pos.1)?;
    
            },
            events::Event::ChangePanel(direction)=>{
                {
                    let result = display_panels.get_panel(direction);
                    if result.is_none(){
                        continue
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
                // let display_panel = display_panels.get_focused();
                // let panel = display_panel.panel.borrow_mut();
                let panel = layout.panels.get_mut(3).unwrap();
                if let PanelType::FsList( list) = panel{
                    list.click_at(3);

                }
                

                // list_layout.click_at(3);
                // let cursor_pos = term.get_cursor()?;
                // let result = list_layout.click_at(cursor_pos.1 - 1);
                // match result{
                //     Ok(list)=> {
                //         // let mut state = list_layout.state.clone();
                //         let mut state = ListState::default();
                //         term.draw(|frame|{ui2(frame, list, &mut state)})?; term.show_cursor()?;
                //         term.set_cursor(cursor_pos.0, cursor_pos.1)?;
                //         file_log!("enter pressed");
                //         },
                //     Err(_) => {}
                // }
            },
            events::Event::ShowData =>{
    
            },
            events::Event::Continue =>{}
        }
    }
    Ok(())
}

fn handle_event(event: Event  term: &mut Terminal<CrosstermBackend<Stdout>>, layout: &mut RufufLayout, display_panels: &mut DisplayPanelsLayout<'static>) -> Result<()> {
    Ok(())
}

fn ui2(frame: &mut Frame, list: List,  state: &mut ListState){
    let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let areas = layout.areas::<2>(frame.size());
    let main_area =  Rect::new(1, 1,  areas[0].width -2, areas[0].height -1);
    let widget = Block::bordered().title("../rust/rufuf/");
    frame.render_widget(widget, areas[0]);
    frame.render_stateful_widget(list, main_area, state)
}

fn ui<'b, 'a, 'c>(frame: &mut Frame, panels_layout: &'a mut RufufLayout, mut current_focused: &'b mut DisplayPanelsLayout<'c>) 
where 'a: 'b, 'a: 'c{
    let mut display_panels = panels_layout.get_display_frames(&frame).unwrap();
    for display_panel in display_panels.iter(){
        match display_panel.panel{
            PanelType::FsList(list) =>{
                let mut state = ListState::default();
                let list = list.to_list().block(Block::bordered().title("list"));
                // frame.render_widget(widget, area)
                frame.render_stateful_widget(list, display_panel.area, &mut state);
                // file_log!("just pritned the list??");
                let a = display_panel.area;
                // file_dbg!(a);

            } 

            ,
            PanelType::ShowData => {

            },
        }
    }
    current_focused.panels = display_panels;
    // *current_focused = Some(display_panels.pop().unwrap());
    // panels_layout.draw_to_frame(frame);
    // let layout = calculate_layout(frame.size());
    // let main_layout = layout[0];
    // let mut state = ListState::default();
    // let widget = Block::bordered().title("../rust/rufuf/");
    // let main_area =  Rect::new(1, 1, main_layout.width -2,main_layout.height -1);
    // frame.render_widget(widget, main_layout);
    // let list = list_layout.to_list();
    // frame.render_stateful_widget(list, main_area, &mut state);
}





fn get_fs_list()-> Result<List<'static>>{
    let root_dir = fs::read_dir(".")?;
    let mut vec_buffer = Vec::new();
    
    for entry in root_dir {
        let entry = entry?;
        let file_type = entry.file_type().unwrap();
        let file_name = entry.file_name();
        let name = file_name.to_str().unwrap();
        if file_type.is_dir(){
            let line = Line::from(format!("{}",name)).blue();
            vec_buffer.push(line);
        }
        else{
            let line = Line::from(format!("{}",name)).white();
            vec_buffer.push(line);
        }
    }
    let list = List::new(vec_buffer).block(Block::new().borders(Borders::NONE));
    Ok(list)
}



fn calculate_layout(area: Rect)-> Vec<Rect>{
    let main_layout = Layout::horizontal([Constraint::Percentage(50) , Constraint::Min(0)]);
    // let [title_area, main_area] = main_layout.areas(area);
    let areas = main_layout.areas::<2>(area);
    // title_area
    areas.into()
    // main_area
     // let block_layout = Layout::vertical([Constraint::Max(4); 9]);

}

fn calculate_entries(root_dir: ReadDir, depth: u8)-> Result<Vec<FsEntry>>{
    let mut buffer = vec!();
    // let dir = fs::read_dir(root_dir.path())?;
    for entry in root_dir{
        let entry = entry?;
        let file_type = entry.file_type().unwrap();
        let file_name = entry.file_name();
        let name = file_name.to_str().unwrap();
        if file_type.is_dir(){
            if depth == 0{
               let line = Line::from(format!("{}",name)).blue();
                
               buffer.push(FsEntry{entry, entry_type: EntryType::Directory, line,  open: false, nodes: None});
            }
            else{
                let line = Line::from(format!("{}",name)).blue();
                let dir = fs::read_dir(entry.path())?;
                let other_nodes = calculate_entries(dir, depth-1)?;
                buffer.push(FsEntry{entry, entry_type: EntryType::Directory, open: false, line, nodes: Some(other_nodes)});
            }
        }
        else{
            let line = Line::from(format!("{}",name)).white();
            buffer.push(FsEntry{entry, entry_type: EntryType::File, open: false, line, nodes: None});
        }
    }

    
    Ok(buffer)
}

#[derive(Debug)]
enum  PanelType {
    FsList(FsListLayout),
    ShowData,
}

#[derive(Debug, Clone)]
struct DisplayPanel<'a>{
    panel : &'a PanelType,
    area : Rect,
    offset: Cords,
}

impl<'a> DisplayPanel<'a>{
    pub fn new(panel: &'a PanelType, area: Rect, offset: Cords)-> DisplayPanel{
        return DisplayPanel{
            panel,
            area,
            offset
        }



    }
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
        // let dir = self.layout;
        let mut buffer = vec!();
        self.idk(&self.layout, frame.size(), &mut buffer).unwrap();
        
        file_dbg!(buffer);
        // return Ok(buffer);
        // anyhow!("aa")
        // bail!("aaa")
        Ok(buffer)





    }
    fn idk<'a, 'b, 'c>(&'a self, dir: &'c RufufDirection, area:Rect, buffer: &'c mut Vec<DisplayPanel<'b>>)-> Result<()>
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
                // frame.size()
                

            }
            RufufDirection::Panel(number) =>{
                let reference = self.panels.get(number.clone()).unwrap();
                let display_panel =  DisplayPanel::new(reference, area, Cords{x:0,y:0});
                buffer.push(display_panel); 
                // let a = number.clone();
                // let panel = self.panels.get(a).unwrap();
                // match panel{
                //     PanelType::FsList(list) => {
                //         let mut state = ListState::default();
                //         let list = list.to_list();
                //         frame.render_stateful_widget(list, area, &mut state);
                //     },
                //     PanelType::ShowData => todo!(),
                // }

                

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
        // let mut layout = Layout::vertical([]);
        // layout.constraints
        // let mut rows = vec!();
        // let layout = Layout::default();
        // let layout = &self.layout;
        // self.idk(layout, frame.size(), frame);
        

        // let constraint = Constraint::Horizontal(Constraint::Percentage(50), Constraint::Percentage(50));
    //     let areas = layout.direction(layout::Direction::Horizontal).constraints([
    //         Constraint::Percentage(50),
    //         Constraint::Percentage(50)
    //     ]).split(frame.size());
    //     (100/3) as usize 
    //     frame.render_widget(widget, areas[0]);
    //
    //         // let vec = vec![Constraint::Percentage((100/len) as u16); len];
    //         // let layout = Layout::horizontal(vec);
    //         // rows.push(constraint);
    //     // layout.direction(layout::Direction::Vertical).constraints(rows);
    //     // let a = Layout::horizontal([]);
    //
    //     // let b = Layout::vertical(a);
    //     // let mut layout = Layout::vertical(rows);
    //     &mut self[1][0];
    // }
    }
}


