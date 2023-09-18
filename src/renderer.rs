use std::fmt::format;
use crate::gamestate::State;
use memory_stats::memory_stats;
use crossterm::{
    cursor,
    style::{Color, Print, PrintStyledContent, Stylize},
    terminal::{size, Clear, ClearType},
    ExecutableCommand, QueueableCommand, Result,
};
use rand::Rng;
use std::io::{Stdout, Write};
use crossterm::cursor::MoveTo;

pub fn draw(stdout: &mut Stdout, state: &State) -> Result<()> {
    draw_state(stdout, state)?;
    Ok(())
}

pub fn draw_state(stdout: &mut Stdout, state: &State) -> Result<()> {
    // stdout.queue(Clear(ClearType::All))?;
    // stdout.queue(Clear(ClearType::Purge))?;
    let mut fps_formated = String::from("FPS: ");
    let mut fps_string = state.fps.to_string();
    fps_formated.push_str(fps_string.as_mut_str());
    stdout.queue(MoveTo(0, 0))?;
    stdout.write(fps_formated.as_bytes())?;
    if let Some(usage) = memory_stats() {
        stdout.queue(MoveTo(0, 1))?;
        stdout.write(format!("MEM: {}",usage.physical_mem / 1000000).as_bytes())?;
        stdout.queue(MoveTo(0, 2))?;
        stdout.write(format!("MEMV: {}",usage.virtual_mem / 1000000).as_bytes())?;
    }
    let mut out = String::from("▇").repeat(state.scale_x - 1);
    out.push_str("▉");
    // let out: String = String::from("██");
    let xoffset = (size().unwrap().0 - 10 * 2) / 2;
    let yoffset = (size().unwrap().1 - 20) / 2;
    // dbg!(state.board);
    // panic!();
    for (yindex, row) in state.board.iter().enumerate() {
        for (xindex, block) in row.iter().enumerate() {
            // dbg!(block);
            let x_scaled = xindex * state.scale_x + usize::try_from(xoffset).unwrap();
            stdout.queue(cursor::MoveTo(
                x_scaled.try_into().unwrap(),
                u16::try_from(yindex).unwrap() + yoffset,
            ))?;
            stdout.queue(PrintStyledContent(
                out.clone().with(block.color).underline(Color::Black),
            ))?;
        }
        stdout.queue(Print(yindex));
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
