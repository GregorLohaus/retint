use std::io::{Write, Stdout};
use crate::gamestate::{Board};
use rand::Rng;
use crossterm::{
  QueueableCommand, ExecutableCommand, Result, 
  cursor, 
  terminal::{Clear, ClearType,size},
  style::{Stylize,PrintStyledContent}, 
};

pub fn draw(stdout: &mut Stdout,board:Board ) -> Result<()> {
  draw_demo(stdout)?;
  Ok(())
}

pub fn draw_demo(stdout: &mut Stdout) -> Result<()> {
    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(Clear(ClearType::Purge))?;
    let out:String = String::from("██");
    let xoffset = (size().unwrap().0-10*2)/2;
    let yoffset = (size().unwrap().1-20)/2;
    let mut rng = rand::thread_rng();
    for y in (1..20).step_by(2){
      for x in (1..10).step_by(2) {
          let rand = rng.gen_range(1..10);
          // in this loop we are more efficient by not flushing the buffer. 
          let x_scaled = x*2+xoffset-2;
          // let xout = (x_scaled-xoffset).to_string();
          stdout
            .queue(cursor::MoveTo(x_scaled,y+yoffset))?;
          if rand%2==0 {
            stdout
              .queue(PrintStyledContent( out.clone().magenta()))?;
          } else {
            stdout
            .queue(PrintStyledContent( out.clone().underline_cyan()))?;
          }
      }
    }
    stdout.flush()?;
    Ok(())
  }