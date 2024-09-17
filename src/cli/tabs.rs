use std::{ cell::RefCell, clone, rc::Rc, sync::{Arc, Mutex}};

use ratatui::layout::Rect;

use crate::tools::{center, divide};

use super::panel::{DisplayPanel, Panel};


pub enum Directions{
    Horizontal(Vec<Boundry>),
    Vertical(Vec<Boundry>),
    Panel(Rc<RefCell<Panel>>)
}

pub struct Boundry{
    pub ratio : usize,
    pub directions: Directions,
}

impl Boundry{
    fn new(ratio: usize, directions: Directions) -> Self{
        Self{ratio, directions}
    }
}

pub struct RufLayout{
     boundries : Boundry,
     pub panels: Vec<Rc<RefCell<Panel>>>
}


impl RufLayout{
    pub fn new(panel : Panel)-> Self{
        let panel_ref = Rc::new(RefCell::new(panel));
        let boundries = Boundry::new(100, Directions::Panel(panel_ref.clone()));
        let panels = vec!(panel_ref);
        Self{boundries, panels}
    }

    pub fn update_panels_size(&self, frame: Rect) {
        let rect = divide(frame, self.boundries.ratio);
        // let mut display_panels = vec!();
        match &self.boundries.directions{
            Directions::Horizontal(_) => todo!(),
            Directions::Vertical(_) => todo!(),
            Directions::Panel(panel) => {
                let rect = center(rect, frame);
                panel.borrow_mut().rect= rect;
            },
        }
        // return display_panels;
    }

    pub fn get_panels(&self)-> Vec<Rc<RefCell<Panel>>>{
        todo!()
        // return self.panels;
        // let mut vec = vec!();
        // match &self.boundries.directions{
        //     Directions::Horizontal(_) => todo!(),
        //     Directions::Vertical(_) => todo!(),
        //     Directions::Panel(panel) => {
        //         vec.push(panel.clone());
        //     },
        //
        // }
        // return vec;

    }

    pub fn get_current_focus(&self )-> Rc<RefCell<Panel>>{
        for panel in self.panels.iter(){
            if panel.try_borrow().unwrap().focus{
                return panel.clone();
            }
        }
        panic!()
        // match &self.boundries.directions{
        //     Directions::Horizontal(_) => todo!(),
        //     Directions::Vertical(_) => todo!(),
        //     Directions::Panel(panel) => {
        //         return panel.clone();
        //     }
        // }

    }
}

