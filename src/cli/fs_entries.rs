use std::fs::{DirEntry, ReadDir};

use anyhow::{bail, Result};
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
   pub line : Line<'static>,
   pub nodes: Option<Vec<FsEntry>>
}
#[derive(Debug)]
pub struct FsListLayout{
    pub nodes: Vec<FsEntry>,
    pub state : ListState,
    // pub cursor: (usize, usize)
}

impl FsEntry{
    pub fn to_line(&self, ident_level:usize)-> Line<'static>{
        let spaces = " ".repeat(ident_level * 4);
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
impl StatefulWidget for FsListLayout{
    type State = ListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let a = self.nodes.into_iter().map(|entry|{
            entry.line
        }).collect::<Vec<_>>();
        let list = List::new(a);
        list.render(area, buf, &mut self.state);
    }

}
impl FsListLayout{
    pub fn new(root_dir: ReadDir, depth : u8)-> Result<Self>{
        let entries = calculate_entries(root_dir, depth)?;
        Ok(FsListLayout{nodes: entries, state: ListState::default() })
    }

    pub fn to_list(&self)-> List<'static>{
        let a = self.nodes.iter().map(|node|{
            node.line.clone()
        }).collect::<Vec<_>>();
        let list = List::new(a);
        return list
    }

    // fn to_vec(&mut self)-> Vec<&mut FsEntry>{
    // }

    pub fn click_at(&mut self, index: u16)-> Result<List>{
        let at = self.state.offset() +index as usize;
        let mut buffer = vec!();
        let mut i = 0;
        for entry in self.nodes.iter_mut(){
            let mut indent=0;
            if i == at{
                if entry.entry_type == EntryType::File{
                    bail!("bbanana")
                }
                entry.open = !entry.open;
            }
            buffer.push(entry.to_line(indent));
            i+=1;
            if entry.open{
                // let Some(other_entries) = entry.nodes;
                match entry.nodes.as_mut(){
                    Some(other_entries)=>{
                        indent +=1;
                        for other_entry in other_entries.iter_mut(){
                            if i == at{
                                if other_entry.entry_type == EntryType::File{
                                    bail!("bbanana")
                                }
                                other_entry.open = !other_entry.open;
                            }
                            buffer.push(other_entry.to_line(indent));
                            i+=1;
                        }
                    },
                    None=>{}
                }
            }
        }
        Ok(List::new(buffer))
    }
}
