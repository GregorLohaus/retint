mod gamestate;
mod mutator;
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
    ExecutableCommand, QueueableCommand,
};
use gamestate::State;
use mutator::mutate;
macro_rules! key_press {
    ($c:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        })
    };
}

macro_rules! key_release {
    ($c:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Release,
            state: KeyEventState::NONE,
        })
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let clock = Instant::now();
    let mut stdout = stdout();
    let mut state = State::new();
    enable_raw_mode().unwrap();
    execute!(stdout, Hide).unwrap();
    let mut previous: u128 = 0;
    stdout.queue(Clear(ClearType::All));
    loop {
        //going to top left corner
        let now = clock.elapsed().as_millis();
        mutate(
            &mut state,
            Duration::from_millis((now - previous).try_into().unwrap()),
        );
        if now - previous > 16 {
            match renderer::draw(&mut stdout, &state) {
                Ok(_o) => match stdout.flush() {
                    Ok(_o) => (),
                    Err(_e) => (),
                },
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
                        key_press!('q') => break,
                        key_press!(' ') => state.eventqueue.push(gamestate::Action::HardDrop),
                        key_press!('o') => state.eventqueue.push(gamestate::Action::RotateR),
                        key_press!('k') => state.eventqueue.push(gamestate::Action::RotateL),
                        key_press!('p') => state.eventqueue.push(gamestate::Action::Flip),
                        key_press!('w') => state.eventqueue.push(gamestate::Action::Hold),
                        key_press!('a') => state.eventqueue.push(gamestate::Action::MoveLeftA),
                        key_release!('a') => state.eventqueue.push(gamestate::Action::MoveLeftD),
                        key_press!('s') => state.eventqueue.push(gamestate::Action::SoftDropA),
                        key_release!('s') => state.eventqueue.push(gamestate::Action::SoftDropD),
                        key_press!('d') => state.eventqueue.push(gamestate::Action::MoveRightA),
                        key_release!('d') => state.eventqueue.push(gamestate::Action::MoverRightD),

                        _ => (),
                    }
                }
            }
            Err(_e) => (),
        }
    }
    // execute!(stdout, Clear(ClearType::All)).unwrap();
    // execute!(stdout, MoveTo(0, 0)).unwrap();
    disable_raw_mode().unwrap();
    execute!(stdout, Show).unwrap();
}
