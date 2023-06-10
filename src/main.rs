mod renderer;
mod gamestate;
mod tetrominos;
use std::{io::stdout, time::{Instant, Duration}, error::Error};
#[macro_use]
extern crate crossterm;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor::{Hide,Show,MoveTo},
    event::{read, poll,Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState},
};
fn main() {
    let clock = Instant::now();
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    execute!(stdout, Hide).unwrap();
    let mut previous: u128 = 0;
    loop {
        //going to top left corner
        let now = clock.elapsed().as_millis();
        if now - previous > 16 {
            match renderer::draw_demo(&mut stdout) {
                Ok(_o) => (),
                Err(_e) => break
            };
            previous = now;
        }
        match poll(Duration::from_millis(1)) {
            //matching the key
            Ok(t) => 
                if t {
                    match read().unwrap() {
                        //i think this speaks for itself
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: KeyModifiers::CONTROL,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE
                        }) => break,
                        _ => (),
                    }
                }
            Err(_e) => ()
        }

    }
    execute!(stdout, Clear(ClearType::All)).unwrap();
    execute!(stdout, MoveTo(0,0)).unwrap();
    disable_raw_mode().unwrap();
    execute!(stdout, Show).unwrap();
}
