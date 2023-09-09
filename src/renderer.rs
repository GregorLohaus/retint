use crate::gamestate::State;
use crossterm::{
    cursor,
    style::{Color, PrintStyledContent, Stylize},
    terminal::{size, Clear, ClearType},
    ExecutableCommand, QueueableCommand, Result,
};
use rand::Rng;
use std::io::{Stdout, Write};

pub fn draw(stdout: &mut Stdout, state: &State) -> Result<()> {
    draw(stdout, state);
    Ok(())
}

pub fn draw_state(stdout: &mut Stdout, state: &State) -> Result<()> {
    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(Clear(ClearType::Purge))?;
    let out = String::from("█").repeat(state.scalex);
    // let out: String = String::from("██");
    let xoffset = (size().unwrap().0 - 10 * 2) / 2;
    let yoffset = (size().unwrap().1 - 20) / 2;
    for (yindex, row) in state.board.iter().enumerate() {
        for (xindex, block) in row.iter().enumerate() {
            let x_scaled = xindex * state.scalex + usize::try_from(xoffset).unwrap() - 2;
            stdout.queue(PrintStyledContent(out.clone().magenta()))?;
        }
    }
    Ok(())
}

pub fn draw_blink_demo(stdout: &mut Stdout) -> Result<()> {
    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(Clear(ClearType::Purge))?;
    let out: String = String::from("██");
    let xoffset = (size().unwrap().0 - 10 * 2) / 2;
    let yoffset = (size().unwrap().1 - 20) / 2;
    let mut rng = rand::thread_rng();
    for y in (1..20).step_by(2) {
        for x in (1..10).step_by(2) {
            let rand = rng.gen_range(1..10);
            // in this loop we are more efficient by not flushing the buffer.
            let x_scaled = x * 2 + xoffset - 2;
            // let xout = (x_scaled-xoffset).to_string();
            stdout.queue(cursor::MoveTo(x_scaled, y + yoffset))?;
            if rand % 2 == 0 {
                stdout.queue(PrintStyledContent(out.clone().magenta()))?;
            } else {
                stdout.queue(PrintStyledContent(out.clone().underline_cyan()))?;
            }
        }
    }
    stdout.flush()?;
    Ok(())
}
