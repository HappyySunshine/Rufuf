use std::{default, fs::File, io::{Read, Write}, path::{Path, PathBuf}, sync::{Arc, Mutex}};



use anyhow::Result;
use generic_array::{arr, typenum::array, ArrayLength};
use mlua::Lua;
use ratatui::{style::{Color, Stylize}, text::{Line, Span, Text}, widgets::{Block, Clear, List, Widget}};
use tokio::io::BufWriter;
use file_logger::*;

use super::{fslist::FsList, panel::Panel, widgets::show_image::ShowImageWidget};



// #[derive(Clone)]
pub enum WidgetType{
    FsList(FsList),
    ReadFileWidget(ReadFileWidget),
    ShowImage(ShowImageWidget),
}



#[derive(Default)]
pub struct WidgetState{
    pub boxed: bool,
}

pub struct ReadFileWidget{
    pub buffer: String,
    pub cwd: PathBuf,
    // pub state: WidgetState
}

impl ReadFileWidget{
    pub fn new(buffer: String, cwd: PathBuf)-> Self{
        return Self{buffer, cwd}
    
    }
}

pub trait MyWidget{
    fn draw_to_frame(&self, frame: &mut ratatui::Frame,panel: &Panel,config: &Lua) -> Result<()>;
}

impl MyWidget for ReadFileWidget{
    fn draw_to_frame(&self, frame: &mut ratatui::Frame,panel: &Panel,config: &Lua) -> Result<()> {
        let mut rect = panel.rect;
        if panel.state.boxed{
            let title = Span::from(self.cwd.to_str().unwrap_or("")).fg(Color::Blue);
            let widget = Block::bordered().title(title);
            rect = widget.inner(rect);
            frame.render_widget(widget, panel.rect);
        }
        let text = self.buffer.to_string();

        frame.render_widget(Clear, rect);
        frame.render_widget(Text::from(text), rect);
        Ok(())
    }
}

// impl Widget for ReadFileWidget{
//     fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
//     where
//         Self: Sized {
//         Text::from(self.buffer.to_string())
//         todo!()
//     }
// }
pub struct Buffers{
    pub buffers: Vec<Arc<Mutex<WidgetType>>>,
}

impl Buffers{
    pub fn add(&mut self, buffer: WidgetType){
        let buf = Arc::new(Mutex::new(buffer));
        self.buffers.push(buf);
    }
    
    pub fn new()->Self{
        let buffers = vec!();
        return Self{buffers}
    }
}



pub struct Buffer{
    pub buf: [u8; 2000],
    read: usize,
}


impl Write for Buffer{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {

        // self.content.
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // BufWriter::new()
        todo!()
    }
}

// impl<N> Sized for Buffer<N>{}
impl ToString for Buffer{
    fn to_string(&self) -> String {
        let mut str = String::new();
        let read = self.read;
        file_dbg!(read);
        for utf8 in self.buf[..self.read].utf8_chunks(){
            for ch in utf8.valid().chars(){
                str.push(ch);

            }
            for invalid in utf8.invalid(){
                str.push(' ');
            }
        }
        return str;
    }
}



impl Buffer{
    // type LEN: ArrayLength;
    pub fn with_capacity(size: usize)-> Buffer{
        let buf = [0; 2000];
        let read = 0;
        return Self{buf,read}
    }

    pub fn read_file(&mut self, mut file: File)-> Result<()>{
        let bytes = file.read(&mut self.buf)?;
        self.read = bytes-7;

        Ok(())
    }
}

