use std::{cell::RefCell, rc::Rc};

use mlua::Lua;
use ratatui::layout::Rect;

use super::{events::CursorMov, Cords, FsListLayout, RufufLine};


pub struct DisplayPanelsLayout{
    pub panels: Vec<DisplayPanel>,
    pub current_focused: usize

}
#[derive(Debug, Clone)]
pub struct DisplayPanel{
    pub panel : PanelType,
    // data: PanelData,
    pub area : Rect,
    pub cursor: Cords,
}

#[derive(Debug, Clone)]
pub enum  PanelType {
    FsList (Rc<RefCell<DisplayFsList>>),
    ShowData,
}
impl DisplayPanel{
    pub fn new(panel: &PanelType, area: Rect, offset: Cords)-> DisplayPanel{
        return DisplayPanel{
            panel: panel.clone(),
            area,
            cursor: offset
        }
    }
}
#[derive(Debug)]
pub struct DisplayFsList{
    pub data: Vec<RufufLine>,
    pub reference : FsListLayout,
}

impl DisplayFsList{
    pub fn new(fs_list: FsListLayout, config: &Lua)->Self{
        let data = fs_list.to_lines(config);
        return DisplayFsList{
            data, 
            reference: fs_list,
        }
    }
}

impl<'a> DisplayPanelsLayout{
    pub fn get_panel<'b>(&self, dir:CursorMov)-> Option<usize>
    where {
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

    pub fn get_focused(&self)-> &DisplayPanel{
        return self.panels.get(self.current_focused).unwrap();
    }
    pub fn get_focused_mut<'b>(&'a mut self)-> &'b mut DisplayPanel
    where 'a: 'b{
        return self.panels.get_mut(self.current_focused).unwrap();
    }

    fn empty() -> DisplayPanelsLayout {
        return DisplayPanelsLayout{panels: vec!(), current_focused: 0}
    }
}
