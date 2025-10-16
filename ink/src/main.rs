mod operator;
mod gap_buffer;
mod file_handler;

use std::io::{SeekFrom, Seek, Read};
use std::fs::OpenOptions;
use termion::input::TermRead;
use termion::event::Key;
use termion::screen::IntoAlternateScreen;
use termion::raw::IntoRawMode;
use termion::{style, color};
use std::io::{stdout, stdin, Write};
use termion::color::Color;

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

fn write_bottom_banner<W: Write>(screen: &mut W, curr_mode: &operator::OperatorMode, screen_height: u16, buffer: &gap_buffer::GapBuffer) {
    write!(screen, "{}{}", termion::cursor::Goto(1, screen_height-1), termion::clear::CurrentLine).unwrap();
    
    write!(
        screen, 
        "{}{}{}Current Mode: {:?}. Switch with (Ctrl+[O,E,F]. Buffer Status: {}",
        termion::cursor::Goto(1, screen_height-1),
        color::Bg(color::LightWhite),
        color::Fg(color::LightBlack),
        curr_mode,
        buffer.get_diagnostics()
    ).unwrap();

    write!(screen, "{}", style::Reset).unwrap();
    write!(screen, "{}", termion::cursor::Goto(1, 2)).unwrap();
}

fn write_status<W: Write>(screen: &mut W, msg: String, screen_height: u16, flush_status: bool, is_error: bool) {
    if flush_status {
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(1, screen_height),
            termion::clear::CurrentLine
        ).unwrap()
    }
    else {
        let status_color: &dyn Color = if is_error {
            &color::LightRed
        } else {
            &color::LightGreen
        };

        write!(
            screen,
            "{}{}{}{}",
            termion::cursor::Goto(1, screen_height),
            color::Bg(status_color),
            color::Fg(color::Black),
            msg
        ).unwrap();
    }
    write!(screen, "{}", style::Reset).unwrap();
}

fn write_buffer<W: Write>(screen: &mut W, buffer: &gap_buffer::GapBuffer, curr_line: u16) {
    write!(screen, "{}{}", termion::cursor::Goto(1, curr_line), termion::clear::CurrentLine).unwrap();

    write!(
        screen,
        "{}",
        buffer
    ).unwrap();
}

fn main() {
    let mut screen = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();

    let file_name = "test.py";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    let mut file_data_buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_data_buf).unwrap();

    let mut curr_mode = operator::OperatorMode::O;
    let mut gb = gap_buffer::GapBuffer::from_data(file_data_buf);

    let (x, y) = termion::terminal_size().unwrap();
    write_top_banner(&mut screen, "test.py", x);
    write_bottom_banner(&mut screen, &curr_mode, y, &gb);

    write!(screen, "{}{}", termion::cursor::Goto(1, 2), termion::clear::CurrentLine).unwrap();

    screen.flush().unwrap();

    let stdin = stdin();

    let curr_line: u16 = 2;
    write_buffer(&mut screen, &gb, curr_line);

    let mut displayed_status = false;
    
    // mainloop, each key stroke is treated as a "frame"
    for k in stdin.keys() {
        let (_x, y) = termion::terminal_size().unwrap();

        if displayed_status {
            write_status(&mut screen, "".to_string(), y, true, true);
            displayed_status = false;
        }

        match k.as_ref().unwrap() {
            Key::Ctrl('c') => break,
            Key::Ctrl('o') => curr_mode = operator::OperatorMode::O,
            Key::Ctrl('e') => curr_mode = operator::OperatorMode::E,
            //Key::Up => pos.update(0, -1),
            //Key::Down => pos.update(0, 1),
            //Key::Left => gb.move_cursor_by(-1),
            //Key::Right => gb.move_cursor_by(1),
            //Key::Char('\n') => {},//gb.move_cursor(1),
            Key::Esc => curr_mode = operator::OperatorMode::O,
            Key::Backspace => {
                if matches!(curr_mode, operator::OperatorMode::E) {
                    gb.delete_data();
                }
            },
            Key::Char(c) => {
                match &curr_mode {
                    operator::OperatorMode::O => {
                        match c {
                            'q' => break,
                            's' => {
                                match file_handler::overwrite_file(&mut file, &gb.get_data()) {
                                    Ok(_) => write_status(&mut screen, "Saved File!".to_string(), y, false, false),
                                    Err(e) => write_status(&mut screen, format!("Failed to save file with error: {:?}", e), y, false, true)
                                }
                                displayed_status = true;
                            },
                            _ => {
                                write_status(&mut screen, format!("Key in 'O' mode not implemented: {:?}", c), y, false, true);
                                displayed_status = true;
                            }
                        }
                    },
                    operator::OperatorMode::E => {
                        gb.insert_data(*c);

                        if *c == '\n' {
                            gb.insert_data('\r');
                        }
                    }
                }
            },
            c => {
                write_status(&mut screen, format!("Key not implemented: {:?}", c), y, false, true);
                displayed_status = true;
            }
        };

        write_bottom_banner(&mut screen, &curr_mode, y, &gb);

        write_buffer(&mut screen, &gb, curr_line);

        screen.flush().unwrap();
    }
    screen.flush().unwrap();
    write!(screen, "{}", termion::cursor::Show).unwrap();
}

