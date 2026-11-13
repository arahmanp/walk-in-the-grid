use std::{io, time::Duration};

use crossterm::{
    ExecutableCommand,
    cursor,
    event::{self, Event, KeyCode},
    style::{self, Stylize},
    terminal::{self, ClearType}
};

const HEIGHT: u16 = 15;
const WIDTH: u16 = 31;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    io::stdout().execute(terminal::Clear(ClearType::All))?;

    let mut x = 0_u16;
    let mut y = 0_u16;

    print_grid(x, y)?;

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('w') => {
                        if x > 0 {
                            x -= 1;
                        }
                    },
                    KeyCode::Char('a') => {
                        if y > 1 {
                            y -= 2;
                        }
                    },
                    KeyCode::Char('s') => {
                        if x < HEIGHT {
                            x += 1;
                        }
                    },
                    KeyCode::Char('d') => {
                        if y < WIDTH - 1 {
                            y += 2;
                        }
                    },
                    _ => {}
                }

                print_grid(x, y)?;
            }
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}

fn print_grid(x: u16, y: u16) -> io::Result<()> {
    io::stdout().execute(cursor::MoveTo(0, 0))?;

    for i in 0..=HEIGHT {
        for j in 0..=WIDTH {
            if i == x && (j == y || j == y + 1) {
                io::stdout().execute(style::PrintStyledContent(" ".on_red()))?;
            } else {
                io::stdout().execute(style::PrintStyledContent(" ".on_dark_green()))?;
            }
        }

        println!("\r");
    }

    Ok(())
}
