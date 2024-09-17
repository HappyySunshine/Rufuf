use std::{ fs::{self, DirEntry, ReadDir}, ops::Range, str::FromStr, sync::Arc};



use anyhow::{bail, Result};
use crossterm::style::Color;
// use file_logger::{file_dbg, file_log};
use file_logger::*;
use mlua::{Lua, Table};
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::Line, widgets::{List, ListItem, ListState, StatefulWidget}};

use super::{read_entries, RufufLayout, RufufLine};



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


impl Into<ListItem<'_>> for RufufLine{
    fn into(self) -> ListItem<'static> {
        return ListItem::new(self.line);
    }
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
    pub fn to_line(&self, config: &Lua,  depth: usize)-> RufufLine{
        let colors: Table = config.globals().get("colors").unwrap();
        let white: String = colors.get("white").unwrap();
        let blue: String = colors.get("white").unwrap();
        let spaces = " ".repeat(depth  * 4);
        let name = self.entry.file_name().into_string().unwrap();
        if self.entry_type == EntryType::File{
            let line =Line::from(format!("{spaces}{name}")).white(); 
            return RufufLine{line, depth}
        }
        else{
            let line = Line::from(format!("{spaces}{name}")).blue();
            return RufufLine{line,depth}

        }
    }

}



impl FsListLayout{
    pub fn new(root_dir: ReadDir, depth : usize)-> Result<Self>{
        let entries = read_entries(root_dir, depth)?;
        Ok(FsListLayout{nodes: entries, state: ListState::default() })
    }

    pub fn to_line_vec(&self, config: &Lua, mut depth: usize, mut index: usize, mut amount: usize) -> Vec<RufufLine>{
        // let config = Arc::new(config);
        let mut buffer = vec!();
        loop{
            if amount == 0 && depth !=0{
                return buffer;
            }

            let next = self.nodes.get(index);
            if next.is_none(){
                return buffer
            }
            let entry = next.unwrap();

            let line = entry.to_line(config, depth);
            buffer.push(line);
            // if entry.entry_type == EntryType::File{
            index+=1;
            if amount!=0{
                amount-=1;
            }

            if entry.entry_type == EntryType::File{
                continue;
            }
            // if !entry.open{
                // index+= entry.children_count;
                // continue
            // }
            if entry.children_count!=0 && entry.open{
                let mut lines = self.to_line_vec(config, depth+1, index, entry.children_count);
                buffer.append(&mut lines);
                index+=entry.children_count;
            }
        }
    }

    pub fn to_lines(&self, config: &Lua)->Vec<RufufLine>{
        return self.to_line_vec(config, 0,0,0);

    }
        // return buffer;


    // pub fn to_list(&self, config: &Lua)-> List<'static>{
    //     let vec = self.to_line_vec(config);
    //     let list = List::new(vec);
    //     return list;
    // }
    //
    //
    pub fn click_at(&mut self, index: u16)-> Option<()>{
        let at = self.state.offset() +index as usize;
        let entry = self.nodes.get(at);
        match entry{
            Some(entry)=>{
                // drop(entry);
                self.toggle(at);
                return Some(())
            }
            None=>{
                return None
            }
        }
   
    }

    pub fn toggle(&mut self, mut index: usize){
        let entry = self.nodes.get_mut(index).unwrap();
        if entry.entry_type == EntryType::Directory{
            if entry.open == false{
                entry.open = !entry.open;
                let dir = fs::read_dir(entry.entry.path()).unwrap();
                let entries = read_entries(dir, 0).unwrap();
                entry.children_count = entries.len();
                for new_entry in entries{
                    index+=1;
                    self.nodes.insert(index, new_entry);
                }
                // self.nodes.insert(index, entries);
            }
            else if entry.open{
                entry.open = !entry.open;
                let size = entry.children_count;
                self.nodes.drain(index+1..=index+size);
        }
    } 
}
}
