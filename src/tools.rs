
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
