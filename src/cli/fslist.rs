use std::{borrow::{Borrow, BorrowMut}, cell::Ref, env::current_dir, fs::{self, DirEntry, File, FileType, ReadDir}, io::Read, path::{Path, PathBuf}};

use anyhow::{anyhow, bail, Result};
use mlua::{Lua, Table};
use ratatui::{ layout::Rect, style::{Color, Style, Styled, Stylize}, text::Span, widgets::Block};
use swift_vec::vector::Vec2;

use crate::{misc::rgb::{to_rgb, Rgb}, tools};

use super::{buffer::{Buffer, ReadFileWidget, WidgetType}, panel::{DisplayPanel, Panel}, widgets::show_image::ShowImageWidget};

use file_logger::*;

#[derive( Debug)]
struct DisplayEntry{
    pub entry: DirEntry,
    pub span: Span<'static>,
    pub rect: Rect,
    pub selected: bool,
    // pub fg: Color
}
impl DisplayEntry{
    pub fn new(entry: DirEntry, span: Span<'static>, rect: Rect, selected: bool) -> Self{
        return Self{entry, span, rect, selected};
    }

    pub fn render(&self)-> Span<'static>{
        if self.selected{
            return self.span.clone().on_white();
        }
        else{
            return self.span.clone();
        }
    }


}
pub struct FsList{
    pub entries: Vec<DirEntry>,
    pub cwd : PathBuf,
    options: FsListOptions,
    display_entries: DisplayEntries
}

// #[derive(PartialEq)]
enum DisplayEntries{
    SingleLine(SingleLine),
    MultiLine(MultiLineStruct),
    Unitialized
}
impl DisplayEntries{
    pub fn is_unitialized(&self)-> bool{
        match self{
            DisplayEntries::SingleLine(_) => false,
            DisplayEntries::MultiLine(_) => false,
            DisplayEntries::Unitialized => true,
        }

    }

    pub fn get_selected(&self)-> &DisplayEntry{
        match self{
            DisplayEntries::SingleLine(line) => {
               let entry = line.vec.get(line.selected).unwrap();
                return entry
            },
            DisplayEntries::MultiLine(multi_line) => {
                let entry = multi_line.vec.get(multi_line.selected.y()).unwrap().get(multi_line.selected.x()).unwrap();
                return entry
            },
            DisplayEntries::Unitialized => panic!("a"),
        }
    }
}
// #[derive(PartialEq)]
struct SingleLine{
    pub vec: Vec<DisplayEntry>,
    pub selected: usize
}
// #[derive(PartialEq)]
struct MultiLineStruct{
    pub vec : Vec<Vec<DisplayEntry>>,
    pub selected: Vec2<usize>,
}

impl SingleLine{
    pub fn new_default()-> Self{
        return Self{vec: vec!(), selected: 0}
    }
}

impl MultiLineStruct{
    pub fn new_default()-> Self{
        return Self{vec: vec!(vec!()), selected: Vec2(0,0)}
    }
}
struct Bindagen{
    pub entry: DirEntry,
    pub span: Span<'static>
}



struct FsListOptions{
    pub single_line: bool,
    pub show_file_size: bool,
    pub space_size: u16,
}
impl FsListOptions{

    pub fn new_default()-> Self{
        let single_line = false;
        let show_file_size = false;
        let space_size = 4;
        return Self{single_line, show_file_size, space_size}

    }

}

impl FsList{
    fn get_entries(at: &PathBuf) -> Vec<DirEntry>{
        let dir = fs::read_dir(at);
        let mut entries = vec!();
        for entry in dir{
            for entry in entry.into_iter(){
                if let Ok(entry) = entry{
                    entries.push(entry);
                }
            }
        }
        return entries;
    }

    pub fn sort(&mut self){
        let mut dirs = vec!();
        let mut files = vec!();
        loop{
            let entry = self.entries.pop();
            if entry.is_none(){ break;};
            let entry = entry.unwrap();
            if let Ok(file_type) = entry.file_type(){
                if file_type.is_dir(){
                    dirs.push(entry);
                   }
                else{
                    files.push(entry);
                }
            }
        }
        dirs.sort_by(|a, b| a.file_name().to_str().unwrap().to_lowercase().cmp(&b.file_name().to_str().unwrap().to_lowercase()));
        files.sort_by(|a, b| a.file_name().to_str().unwrap().to_lowercase().cmp(&b.file_name().to_str().unwrap().to_lowercase()));
        dirs.append(&mut files);
        self.entries = dirs; 
    }

   pub fn new(path: PathBuf) -> Result<Self>{
        let cwd = path;
        let entries = FsList::get_entries(&cwd);
        let options = FsListOptions::new_default();
        let display_entries = DisplayEntries::Unitialized;
        let mut fs_list = Self{entries,  cwd, options, display_entries};
        fs_list.sort();
        
        Ok(fs_list)
    }

    pub fn to_spans(&mut self, config: &mlua::prelude::Lua) -> Vec<Bindagen> {
        let mut vec = vec![];
        for entry in self.entries.drain(..){
            if let Ok(file_type) = entry.file_type(){
                let color;
                let mut string = String::from("");
                if file_type.is_dir(){
                    color = get_color("folder", config);
                    let folder_icon= '\u{ea83}';
                    string.push(folder_icon);
                }
                else{
                    color = get_color("file", config);
                    let file_icon= '\u{ea7b}';
                    string.push(file_icon);
                    }
                if let Some(name) = entry.file_name().to_str(){
                    let name = name.to_string();
                    // let folder_icon= '\u{ea83}';
                    // name.push(folder_icon);
                    string.push(' ');
                    string.push_str(name.as_str());
                    let rgb = Color::Rgb( color.r, color.g, color.b);
                    let span = Span::raw(string).fg(rgb.clone());
                    // let fg = rgb;
                    let a = Bindagen{span, entry };
                    vec.push(a);
                }
            }
        }
        return vec;
    }


    pub fn update_display<'a,'b>(&'a mut self, panel: &Panel, config: &'b Lua) -> Result<()>
    where 'a: 'b{
        if !self.display_entries.is_unitialized(){
            return Ok(())
        }
        // if self.display_entries == DisplayEntries::MultiLine{
            // return Ok(())
        // }
        let mut at = Vec2(0,0);
        let mut rect = panel.rect.clone();
        // let mut rect = panel.rect.clone();
        if panel.state.boxed{
            let widget = Block::bordered();
            rect = widget.inner(rect);
            // frame.render_widget(widget, rect);
            // rect = rect2;
          }
        let mut display_entries;
        if self.options.single_line{
            // panic!();
            display_entries = DisplayEntries::SingleLine(SingleLine::new_default());
        }else{
             display_entries = DisplayEntries::MultiLine(MultiLineStruct::new_default());
        }
        let mut spans = self.to_spans(config);
        let mut index = 0;
        let mut i = 0;
        let mut j = 0;
        for span in spans.drain(0..){
            // let a = display_entries.borrow_mut();
            match display_entries.borrow_mut(){
                DisplayEntries::SingleLine(line) => {
                    let selected={
                        index ==0
                        // line.selected == index
                    };
                    let new_rect= Rect::new(rect.x, rect.y+ at.y(), rect.width, 1);
                    let entry = DisplayEntry::new(span.entry, span.span, new_rect, selected);
                    line.vec.push(entry);
                    at+= Vec2(0,1);
                },
                DisplayEntries::MultiLine(multi_line) => {
                    let selected= index ==0;
                    let space_size = {
                        if index==0{
                            0
                        }
                        else{
                            self.options.space_size
                        }
                    };
                    let text_width: u16 = span.span.width().try_into()?;
                    let mut is_new_line = false;
                    let new_x = at.x() + space_size;
                    let limit_x =  text_width + new_x;
                    if limit_x > rect.width{
                        at = Vec2(0, at.y()+1);
                        is_new_line = true;
                        i+=1;
                        j=0;
                    }else{
                        at.0 = new_x;
                        j+=1
                    }
                    let new_rect = Rect::new(rect.x + at.x(), rect.y + at.y(), text_width, 1);
                    at.0 = at.x()+text_width; 
                    let entry = DisplayEntry::new(span.entry, span.span, new_rect, selected);
                    if is_new_line{
                        multi_line.vec.push(vec!(entry));
                    }else{
                        let i = multi_line.vec.len() -1;
                        multi_line.vec.get_mut(i).unwrap().push(entry);
                    }
                },
                DisplayEntries::Unitialized => {},
            }
            index+=1;
        }
        self.display_entries = display_entries;
        Ok(())
    }

    pub(crate) fn draw_to_frame(&self, frame: &mut ratatui::Frame, panel: &Panel, config: &Lua) -> Result<()> {
        if panel.state.boxed{
            let title = Span::from(self.cwd.to_str().unwrap_or("")).fg(Color::Blue);

            let widget = Block::bordered().title(title);
            frame.render_widget(widget, panel.rect);
        }
        match self.display_entries.borrow(){
            DisplayEntries::SingleLine(line) => {
                for entry in line.vec.iter(){
                    frame.render_widget(entry.render(), entry.rect);

                }
            },
            DisplayEntries::MultiLine(multi_line) => {
                multi_line.vec.iter().for_each(|line|{ line.iter().for_each(|entry|{frame.render_widget(entry.render(), entry.rect)})});
            },
            DisplayEntries::Unitialized => todo!(),
        }
        

        Ok(())
    }



    pub(crate) fn move_selected(&mut self, frame: &mut ratatui::Frame, dir: Vec2<i16>, config: &Lua) -> Result<()> {
        match &mut self.display_entries{
            DisplayEntries::SingleLine(line) => {
                let i = (line.selected as i16 + dir.y()) as usize;
                let other = line.vec.get_mut(i).ok_or(anyhow!(""))?;
                other.selected = true;
                let back = line.selected;
                line.selected = i;
                let entry = line.vec.get_mut(back).ok_or(anyhow!(""))?;
                entry.selected = false;

            },
            DisplayEntries::MultiLine(multi_line) => {
                let back = multi_line.selected.clone();
                
                let i1 = (multi_line.selected.y() as i16 + dir.y()) as usize;
                let j1 = (multi_line.selected.x() as i16 + dir.x()) as usize;
                file_dbg!(i1, j1);
                let other = multi_line.vec.get_mut(i1).ok_or(anyhow!(""))?.get_mut(j1).ok_or(anyhow!(""))?;
                multi_line.selected = Vec2(j1,i1);
                other.selected = true;
                file_log!("other.seletcted true");
                let vec = &multi_line.vec.len();
                file_dbg!(vec);
                let entry = multi_line.vec.get_mut(back.y()).ok_or(anyhow!(""))?.get_mut(back.x()).ok_or(anyhow!(""))?;
                entry.selected = false;
                frame.render_widget(entry.render(), entry.rect)
                
            },
            DisplayEntries::Unitialized => todo!(),
        }
        Ok(())
    }

    pub(crate) fn select(&self, panel: &Panel, config: &Lua) -> Result<FsList> {
        let selected = self.display_entries.get_selected();
        if selected.entry.file_type()?.is_dir(){
            let mut new_list = FsList::new(selected.entry.path())?;
            new_list.update_display(panel, config);
            return Ok(new_list);
        }
        bail!("file")
    }

    pub(crate) fn get_focus(&self,  config: &Lua) -> &DisplayEntry {
        return self.display_entries.get_selected()
    }

    pub(crate) fn get_widget(&self, rect: Rect, config: &Lua) -> Result<WidgetType> {
        let selected = self.display_entries.get_selected();
        if selected.entry.file_type()?.is_dir(){
            return Ok(WidgetType::FsList(FsList::new(selected.entry.path())?));
        }
        let path = selected.entry.path();
        if let Some(extension) = path.extension(){
            if extension == "png" || extension == "jpg"{
                let img = ShowImageWidget::new(path)?;
                return Ok(WidgetType::ShowImage(img));
            }
        }
        let mut file = File::open(selected.entry.path())?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        let widget = ReadFileWidget::new(string, path);
        return Ok(WidgetType::ReadFileWidget(widget));
    }
}

fn get_i_j(multi_line: &[Vec<DisplayEntry>], selected: usize) -> Result<(usize, usize)> {
    let mut index = 0;
    for i in multi_line.iter().enumerate(){
        for j in multi_line.iter().enumerate(){
            if index == selected{
                return Ok((i.0,j.0));
            }
            index+=1;

        }
    }
    bail!("b");
    // anyhow!("banana");

}



fn get_color(arg: &str, config: &Lua) -> Rgb {
    let colors : Table = config.globals().get("colors").unwrap();
    let tree : Table = config.globals().get("tree").unwrap();
    let folder_color: String = tree.get(arg).unwrap_or("orange".into());
    let color_html : String = colors.get(folder_color).unwrap();
    let color = to_rgb(&color_html);
    // let color = Rgb{r: 255, g: 0, b:0};
    return color
}

fn insert_alphabetical(entry: DirEntry, vec: &mut Vec<DirEntry>) {
    if vec.len() == 0{
        vec.push(entry);
        return
    }
    let name = entry.file_name();
    for entry in vec{
        let entry_name = entry.file_name();

    }

}

pub struct DisplayRufEntry{
    name: String,
}

enum EntryType{
    File,
    Dir,
}


