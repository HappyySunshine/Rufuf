use std::{fs::{DirEntry, ReadDir}, str::FromStr, sync::Arc};

use anyhow::{bail, Result};
use crossterm::style::Color;
use mlua::{Lua, Table};
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::Line, widgets::{List, ListState, StatefulWidget}};

use super::calculate_entries;



#[derive(Debug, Clone)]
pub struct  Cords{
    pub x:usize,
    pub y: usize,
}
#[derive(PartialEq)]
#[derive(Debug)]
pub enum EntryType{
    File,
    Directory,
}

#[derive(Debug)]
pub struct FsEntry{
    pub entry_type: EntryType,
    pub open: bool,
    pub entry: DirEntry,
    pub children_count: usize,
    // pub depth: usize,
   // pub line : Line<'static>,
   // pub nodes: Option<Vec<FsEntry>>
}


#[derive(Debug)]
pub struct FsListLayout{
    pub nodes: Vec<FsEntry>,
    pub state : ListState,
    // pub cursor: (usize, usize)
}

impl FsEntry{
    pub fn to_line(&self, config: &Lua)-> Line<'static>{
        let colors: Table = config.globals().get("colors").unwrap();
        let white: String = colors.get("white").unwrap();
        let blue: String = colors.get("white").unwrap();
        let spaces = " ".repeat(self.depth  * 4);
        let name = self.entry.file_name().into_string().unwrap();
        if self.entry_type == EntryType::File{
            let line =Line::from(format!("{spaces}{name}")).white(); 
            return line
        }
        else{
            return Line::from(format!("{spaces}{name}")).blue()

        }
    }

}


// impl StatefulWidget for FsListLayout{
//     type State = ListState;
//
//     fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
//         // let a = self.nodes.into_iter().map(|entry|{
//         //     entry.line
//         // }).collect::<Vec<_>>();
//         let list = self.to_list(config, ident)
//         let list = List::new(a);
//         list.render(area, buf, &mut self.state);
//     }
//
// }
//
//


impl FsListLayout{
    pub fn new(root_dir: ReadDir, depth : usize)-> Result<Self>{
        let entries = calculate_entries(root_dir, depth)?;
        Ok(FsListLayout{nodes: entries, state: ListState::default() })
    }

    pub fn to_line_vec(&self, config: &Lua) -> Vec<Line<'static>>{
        // let config = Arc::new(config);
        let mut buffer = vec!();
        for entry in self.nodes{
            buffer.push(entry.to_line(config));
            
        }
        return buffer;
    }


    pub fn to_list(&self, config: &Lua)-> List<'static>{
        let vec = self.to_line_vec(config);
        let list = List::new(vec);
        return list;
    }
    //
    //
    pub fn click_at(&mut self, index: u16)-> Option<&FsEntry>{
        let at = self.state.offset() +index as usize;
        let entry = self.nodes.get(at);
        match entry{
            Some(entry)=>{
                return Some(entry)
            }
            None=>{
                return None
            }
        }
   
    }
}
