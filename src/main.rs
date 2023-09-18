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
use gamestate::{Action, State};
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
    enable_raw_mode().unwrap();
    let args: Vec<String> = env::args().collect();
    let clock = Instant::now();
    let mut current_frame_count_second = Instant::now();
    let mut stdout = stdout();
    execute!(stdout, Hide).unwrap();
    let mut state = State::new();
    let mut previous_frame_time: u128 = 0;
    let mut previous_mutation_time: u128 = 0;
    let mut frame_count_for_second: u64 = 0;
    let mut current_fps: u64 = 0;
    stdout.queue(Clear(ClearType::All));
    loop {
        //going to top left corner
        let now = clock.elapsed().as_micros();
        mutate(
            &mut state,
            Duration::from_micros((now - previous_mutation_time).try_into().unwrap()),
        );
        previous_mutation_time = now;
        if now - previous_frame_time > 16666 {
            match renderer::draw(&mut stdout, &state) {
                Ok(_o) => match stdout.flush() {
                    Ok(_o) => (),
                    Err(_e) => (),
                },
                Err(_e) => break,
            };
            previous_frame_time = now;
            frame_count_for_second += 1;
            if current_frame_count_second.elapsed().as_secs() > 1 {
                current_frame_count_second = Instant::now();
                current_fps = frame_count_for_second;
                state.fps = current_fps;
                frame_count_for_second = 0;
            }
        }
        match poll(Duration::from_micros(0)) {
            //matching the key
            Ok(t) => {
                if t {
                    match read().unwrap() {
                        //i think this speaks for itself
                        key_press!('q') => break,
                        key_press!(' ') => state.event_queue.push(Action::HardDrop),
                        key_press!('o') => state.event_queue.push(Action::RotateR),
                        key_press!('k') => state.event_queue.push(Action::RotateL),
                        key_press!('p') => state.event_queue.push(Action::Flip),
                        key_press!('w') => state.event_queue.push(Action::Hold),
                        key_press!('a') => state.event_queue.push(Action::MoveLeftA),
                        key_release!('a') => state.event_queue.push(Action::MoveLeftD),
                        key_press!('s') => state.event_queue.push(Action::SoftDropA),
                        key_release!('s') => state.event_queue.push(Action::SoftDropD),
                        key_press!('d') => state.event_queue.push(Action::MoveRightA),
                        key_release!('d') => state.event_queue.push(Action::MoverRightD),

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
