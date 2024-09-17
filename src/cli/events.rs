use anyhow::{anyhow, Result};
use crossterm::event::{self, Event};
use futures::{FutureExt, StreamExt};
use swift_vec::vector::Vec2;
use tokio::{select, sync::mpsc::{self, UnboundedReceiver, UnboundedSender}, time::Interval};


pub struct EventHandler{
    pub rx: UnboundedReceiver<Actions>
}




pub enum Actions{
    Quit,
    Tick,
    Select,
    Redraw,
    ToggleViewer,
    Move(Vec2<i16>),
}


fn handle_event(tx: &UnboundedSender<Actions>, event: Event )-> Result<()>{
      match event {
        Event::Key(key) => {
          if key.kind == event::KeyEventKind::Press {
                if  key.code == event::KeyCode::Char('q'){
                    tx.send(Actions::Quit)?;
                }
                if  key.code == event::KeyCode::Char('h'){
                    tx.send(Actions::Move(Vec2(-1,0)))?;
                }
                if  key.code == event::KeyCode::Char('l'){
                    tx.send(Actions::Move(Vec2(1,0)))?;
                }
                if  key.code == event::KeyCode::Char('j'){
                    tx.send(Actions::Move(Vec2(0,1)))?;
                }
                if  key.code == event::KeyCode::Char('k'){
                    tx.send(Actions::Move(Vec2(0,-1)))?;
                }
                if  key.code == event::KeyCode::Enter{
                    tx.send(Actions::Select)?;
                }
                if  key.code == event::KeyCode::Char('v'){
                    tx.send(Actions::ToggleViewer)?;
                }
          }
        },
        Event::FocusGained => todo!(),
        Event::FocusLost => todo!(),
        Event::Mouse(_) => {},
        Event::Paste(_) => todo!(),
        Event::Resize(_, _) => {tx.send(Actions::Redraw).unwrap()},
        // _ => {},
      }
    Ok(())

}
impl EventHandler{
    pub fn spawn()-> Self{
        let tick_rate = std::time::Duration::from_millis(250);
        let (tx, rx) = mpsc::unbounded_channel::<Actions>();
        let _tx = tx.clone();
        let _ = tokio::spawn(async move {
            let mut interval = tokio::time::interval(tick_rate);
            let mut reader = crossterm::event::EventStream::new();
            loop {
                let delay = interval.tick();
                let crossterm_event = reader.next().fuse();
                select! {
                    maybe_event = crossterm_event =>{
                        match maybe_event {
                          Some(Ok(event)) => {
                                handle_event(&_tx, event).unwrap()
                            },
                            Some(Err(_))=>{},
                            None=>{}
                        }

                    }
                    _ = delay =>{
                            tx.send(Actions::Tick).unwrap();
                        }
                }
            }
        });   
        return Self{rx}
    }

    pub fn new()-> Self{
        return Self::spawn();
    }
    pub async fn next(&mut self) -> Result<Actions> {
        let event = self.rx.recv().await.ok_or(anyhow!("event handler closed unexpectedlyt"))?;
        Ok(event)
  }
}
