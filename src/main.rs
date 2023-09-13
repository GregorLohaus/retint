mod gamestate;
mod renderer;
mod tetrominos;
use std::{
    env,
    error::Error,
    io::{stdout, Write},
    time::{Duration, Instant},
};
#[macro_use]
extern crate crossterm;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
fn main() {
    let args: Vec<String> = env::args().collect();
    let clock = Instant::now();
    let mut stdout = stdout();
    let mut state = gamestate::create_state();
    enable_raw_mode().unwrap();
    execute!(stdout, Hide).unwrap();
    let mut previous: u128 = 0;
    // renderer::draw(&mut stdout, &state);
    loop {
        //going to top left corner
        let now = clock.elapsed().as_millis();
        if now - previous > 3000 {
            match renderer::draw(&mut stdout, &state) {
                Ok(_o) => stdout.flush(),
                Err(_e) => break,
            };
            previous = now;
        }
        match poll(Duration::from_millis(1)) {
            //matching the key
            Ok(t) => {
                if t {
                    match read().unwrap() {
                        //i think this speaks for itself
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: KeyModifiers::CONTROL,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        }) => break,
                        _ => (),
                    }
                }
            }
            Err(_e) => (),
        }
    }
    // execute!(stdout, Clear(ClearType::All)).unwrap();
    // execute!(stdout, MoveTo(0, 0)).unwrap();
    // disable_raw_mode().unwrap();
    // execute!(stdout, Show).unwrap();
}
