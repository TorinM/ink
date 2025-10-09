mod operator;
mod gap_buffer;

use termion::input::TermRead;
use termion::event::Key;
use termion::screen::IntoAlternateScreen;
use termion::raw::IntoRawMode;
use termion::{style, color};
use std::io::{stdout, stdin, Write};

fn write_top_banner<W: Write>(screen: &mut W, file_name: &str, screen_len: u16) {
    let mut s = String::from("Welcome to INK v0.1. Current file: ");
    s.push_str(file_name);
 
    let pad_size = screen_len as usize - s.len();
    let pad: String = " ".repeat(pad_size);
    s.push_str(&pad);

    write!(screen, "{}{}{}{}\n{}", 
        termion::clear::All,
        color::Bg(color::LightWhite),
        color::Fg(color::LightBlack),
        s,
        termion::cursor::Goto(1,2),
    ).unwrap();

    write!(screen, "{}", style::Reset).unwrap();
}

fn write_bottom_banner<W: Write>(screen: &mut W, curr_mode: &operator::OperatorMode, screen_height: u16) {
    write!(
        screen, 
        "{}{}{}Current Mode: {:?}. Available Modes | Operator (Ctrl+O) | Edit (Ctrl+E) | File (Ctrl+F). Use 'q' in Operator Mode to quit.",
        termion::cursor::Goto(1, screen_height-1),
        color::Bg(color::LightWhite),
        color::Fg(color::LightBlack),
        curr_mode
    ).unwrap();

    write!(screen, "{}", style::Reset).unwrap();
    write!(screen, "{}", termion::cursor::Goto(1, 2)).unwrap();
}

fn write_error<W: Write>(screen: &mut W, msg: &str, screen_height: u16, flush_error: bool) {
    if flush_error {
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(1, screen_height),
            termion::clear::CurrentLine
        ).unwrap()
    }
    else {
        write!(
            screen,
            "{}{}{}",
            termion::cursor::Goto(1, screen_height),
            color::Bg(color::Red),
            msg
        ).unwrap();
    }
}

fn main() {
    let mut screen = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();

    let mut curr_mode = operator::OperatorMode::O;

    let (x, y) = termion::terminal_size().unwrap();
    write_top_banner(&mut screen, "test.py", x);
    write_bottom_banner(&mut screen, &curr_mode, y);

    //write!(screen, "{}", style::Reset).unwrap();
    write!(screen, "{}{}{}", termion::cursor::Goto(1, 2), termion::clear::CurrentLine, style::Underline).unwrap();

    screen.flush().unwrap();

    let stdin = stdin();
    let mut gb = gap_buffer::GapBuffer::new(80);
    
    let mut displayed_error = false;
    
    // mainloop, each key stroke is treated as a "frame"
    for k in stdin.keys() {
        let (x, y) = termion::terminal_size().unwrap();
        let max_x = x;
        let max_y = y-1;


        if displayed_error {
            write_error(&mut screen, "", y, true);
            displayed_error = false;
        }

        match k.as_ref().unwrap() {
            Key::Ctrl('c') => {
               println!("Ending capture");
               break
            },
            Key::Ctrl('o') => {
                curr_mode = operator::OperatorMode::O;
            },
            Key::Ctrl('e') => {
                curr_mode = operator::OperatorMode::E;
            },
            Key::Ctrl('f') => {
                curr_mode = operator::OperatorMode::F;
            },
            //Key::Up => pos.update(0, -1),
            //Key::Down => pos.update(0, 1),
            Key::Left => gb.move_cursor(1),
            //Key::Right => pos.update(1, 0),
            Key::Char(c) => gb.insert_data(*c),
            _ => {
                write_error(&mut screen, "Not implemented", y, false);
                displayed_error = true;
            }
        }
        write_bottom_banner(&mut screen, &curr_mode, y);
        screen.flush().unwrap();
    }
    screen.flush().unwrap();
    write!(screen, "{}", termion::cursor::Show).unwrap();

}

