use std::{ any::Any, borrow::{Borrow, BorrowMut}, fs::{self, DirEntry, ReadDir}, hash::Hash, io::Stdout, ops::{Deref, Index, IndexMut}, sync::{Arc, Mutex}, u16, vec};

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

    fn empty() -> DisplayPanelsLayout<'static> {
        return DisplayPanelsLayout{panels: vec!(), current_focused: 0}
    }
}
//
// enum Directions{
//     Right,
//     Down,
//     None,
// }

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
    None,
    
}

pub fn run(mut term: Terminal<CrosstermBackend<Stdout>>, config : Lua)-> Result<()>{
    let root_dir = fs::read_dir(".")?;
    let mut list_layout = FsListLayout::new(root_dir, 1)?;
    let mut layout = RufufLayout::new(PanelType::FsList(list_layout));
    let mut display_panels: DisplayPanelsLayout = DisplayPanelsLayout{panels: vec!(), current_focused: 0};
    layout.add(PanelType::ShowData);
    // let conf = config.borrow();

    term.draw(|frame|{ui(frame, &layout, &mut display_panels, &config)})?;
    term.show_cursor()?;
    term.set_cursor(1, 1)?;

    loop{
        // let a = Arc::new(config);
        let event = events::get_event()?;
        let result = handle_event(event, &mut term,  &layout, &mut display_panels);
        if result.is_err(){
            break
        }
        match result.unwrap(){
            Actions::Click(index_pos) => {
                display_panels = DisplayPanelsLayout::empty();
                let panel = layout.panels.get_mut(index_pos.index).unwrap();
                if let PanelType::FsList(list) = panel{
                    let fs_list = list.to_fs_list(&config);

                    // file_dbg!(lista);
                    term.draw(|frame|{ui(frame, &layout, &mut display_panels, &config)})?;

                }
            },
            Actions::None => {},
        }
    
    }
    Ok(())
}

fn handle_event<'a,'b,'c>(event: Event,  term: &mut Terminal<CrosstermBackend<Stdout>>, layout: &'a RufufLayout, display_panels: &'b mut DisplayPanelsLayout<'c>) -> 
Result<Actions> 
where  'a: 'c{
    match event{
        events::Event::Close => {bail!("errorrrr");},
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
            return Ok(Actions::Click(Index_Pos::new(0,3)))
            // let display_panel = display_panels.get_focused();
            // let panel = display_panel.panel.borrow_mut();
            // let panel = layout.panels.get_mut(3).unwrap();
            // if let PanelType::FsList( list) = panel{
                // list.click_at(3);

            // }
        },
   
   
        events::Event::ShowData =>{

        },
        events::Event::Continue =>{}
        }
    Ok(Actions::None)
}

// fn ui2(frame: &mut Frame, list: List,  state: &mut ListState){
//     let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
//     let areas = layout.areas::<2>(frame.size());
//     let main_area =  Rect::new(1, 1,  areas[0].width -2, areas[0].height -1);
//     let widget = Block::bordered().title("../rust/rufuf/");
//     frame.render_widget(widget, areas[0]);
//     frame.render_stateful_widget(list, main_area, state)
// }

fn ui<'b, 'a, 'c>(frame: &mut Frame, panels_layout: &'a RufufLayout, current_focused: &'b mut DisplayPanelsLayout<'c>, config: &Lua) 
where  'a : 'c {
    let display_panels = panels_layout.get_display_frames(&frame).unwrap();
    for display_panel in display_panels.iter(){
        match display_panel.panel{
            PanelType::FsList(list) =>{
                let mut state = ListState::default();
                let fs_list = list.to_fs_list(config);
                let list = fs_list.to_list(config).block(Block::bordered().title("list"));
                // let list = list.to_list(config,0).block(Block::bordered().title("list"));
                frame.render_stateful_widget(list, display_panel.area, &mut state);
            },
            PanelType::ShowData => {

            },
        }
    }
    current_focused.panels = display_panels;
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

fn calculate_entries(root_dir: ReadDir, depth: usize)-> Result<Vec<FsEntry>>{
    let mut buffer = vec!();
    // let dir = fs::read_dir(root_dir.path())?;
    for entry in root_dir{
        let entry = entry?;
        let file_type = entry.file_type().unwrap();
        let file_name = entry.file_name();
        let name = file_name.to_str().unwrap();
        if file_type.is_dir(){
            let children_count = 0;

            // buffer.push(FsEntry{entry, entry_type: EntryType::File, open: false,  depth, children_count });
            let dir = fs::read_dir(entry.path())?;
            let mut nodes = calculate_entries(dir, depth+ 1)?;
            buffer.push(FsEntry{entry, entry_type: EntryType::File, open: false,  depth, children_count });
            buffer.push(FsEntry{entry, entry_type: EntryType::File, open: false,  depth });
        }
        else{
            buffer.push(FsEntry{entry, entry_type: EntryType::File, open: false,  depth, nodes:None});
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


