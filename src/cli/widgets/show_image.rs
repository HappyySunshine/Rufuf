use std::path::PathBuf;

use anyhow::Result;
use image::ImageReader;
use ratatui::widgets::canvas::{Canvas, Shape};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol, StatefulImage};

use crate::cli::buffer::{MyWidget, WidgetState};

pub struct ShowImageWidget{
    pub image: Box<dyn StatefulProtocol>,
    pub img_path: PathBuf,
    // pub state: WidgetState
}

impl ShowImageWidget{
    pub fn new(img_path: PathBuf)->Result<Self>{
        let mut picker = Picker::from_termios().unwrap();
        picker.guess_protocol();
        let dyn_img = ImageReader::open(&img_path)?.decode()?;
        let image = picker.new_resize_protocol(dyn_img);
        return Ok(Self{image,img_path})
    }
    pub fn draw_to_frame(&mut self, frame: &mut ratatui::Frame,panel: &crate::cli::panel::Panel,config: &mlua::Lua) -> anyhow::Result<()> {
        let image = StatefulImage::new(None);

        frame.render_stateful_widget(image, panel.rect, &mut self.image);
        
        // let img = ImageReader::open(&self.img).unwrap().decode().unwrap();
        // let canvas = Canvas::default().paint(|ctx|{
            // ctx.draw(canvas::)
        // });
        Ok(())
    }
}
// impl MyWidget for ShowImageWidget{
//     fn draw_to_frame(&self, frame: &mut ratatui::Frame,panel: &crate::cli::panel::Panel,config: &mlua::Lua) -> anyhow::Result<()> {
//         let image = StatefulImage::new(None);
//
//         frame.render_stateful_widget(image, panel.rect, &mut self.image);
//        
//         // let img = ImageReader::open(&self.img).unwrap().decode().unwrap();
//         // let canvas = Canvas::default().paint(|ctx|{
//             // ctx.draw(canvas::)
//         // });
//         Ok(())
//     }
// }
//
