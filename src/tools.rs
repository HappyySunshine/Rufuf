
use ratatui::layout::Rect;



pub fn clamp(pos: (u16,u16), rect: Rect, bordered: bool)-> (u16,u16){
    let mut new_pos  = pos;
    let mut offset = (0,0);
    if bordered{
        offset = (1,1);
    }
    if pos.0 < rect.x + offset.0{
        new_pos.0 = rect.x + offset.0;
    } 
    else if pos.0 > rect.x + rect.width - offset.0 -1{
        new_pos.0 = rect.x + rect.width - offset.0 -1 ;
    }
    if pos.1 < rect.y + offset.1 {
        new_pos.1 = rect.y + offset.1;
    }
    else if pos.1 > rect.y + rect.height - offset.1 -1{
        new_pos.1 = rect.y + rect.height - offset.1 -1; 
    }
    return new_pos;

}

pub fn clamp_simple(value: u16, start: u16, end:u16)-> u16{
    if value < start{
        return start
    }
    else if value > end{
        return end
    }
    return value
}
pub fn center(rect: Rect, term_size: Rect)-> Rect{
    let x = (term_size.width - rect.width)/2;
    let y = (term_size.height - rect.height)/2;
    let width = rect.width;
    let height = rect.height;
    return Rect{x,y,width,height}
}

pub fn divide(frame: Rect, ratio: usize) -> Rect{
    let r = 100.0/ ratio as f32;
    let width = (frame.width as f32 /r) as u16;
    let height = (frame.height as f32 / r) as u16;
    let x = frame.x;
    let y = frame.y;
    let new_rect = Rect{x,y,width,height};
    return new_rect;
}
