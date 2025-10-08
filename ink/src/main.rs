use crate::gap_buffer::GapBuffer;

mod gap_buffer;


fn main() {
    let mut gb = GapBuffer::new(10);
    println!("{:?}", gb);

    gb.insert_data('H');
    println!("{:?}", gb);
    gb.insert_data('L');
    println!("{:?}", gb);
    gb.insert_data('L');
    println!("{:?}", gb);
    gb.insert_data('O');
    println!("{:?}", gb);

    gb.move_cursor(1);
    println!("{:?}", gb);
    
    gb.insert_data('E');
    println!("{:?}", gb);

    gb.move_cursor(5);
    println!("{:?}", gb);
    
    gb.insert_data('W');
    println!("{:?}", gb);
    
    gb.insert_data('O');
    println!("{:?}", gb);

    gb.insert_data('R');
    println!("{:?}", gb);
    
    gb.insert_data('L');
    println!("{:?}", gb);

    gb.insert_data('D');
    println!("{:?}", gb);

    println!("{}", gb);
}






// use std::io::{stdout, stdin, Write};
// use std::{env, thread, time};

// use termion::input::TermRead;
// use termion::event::Key;
// use termion::screen::IntoAlternateScreen;
// use termion::raw::IntoRawMode;
// use termion::{style, color};

// #[derive(Debug)]
// enum OperatorMode {
//     O,         // Operator
//     E,         // Edit
//     F          // File
// }

// struct CursorPos {
//     x: u16,
//     max_x: u16,
//     y: u16,
//     max_y: u16
// }
// impl CursorPos {
//     fn update(&mut self, move_x: i8, move_y: i8) {
//         let new_x = self.x as i8 + move_x;
//         let new_y = self.y as i8 + move_y;

//         if (new_x >= 0) && (new_x < self.max_x as i8) {
//             self.x = new_x as u16;
//         }

//         if (new_y >= 0) && (new_y < self.max_y as i8) {
//             self.y = new_y as u16;
//         }
//     }
// }

// fn insertchar(lines: &mut Vec<Vec<char>>, val: &char, pos: &CursorPos) {
//    lines[pos.y as usize][pos.x as usize] = val.clone(); 
// }

// fn writelines<W: Write>(screen: &mut W, lines: &Vec<Vec<char>>, pos: &CursorPos) {

//     write!(screen, "{}", termion::cursor::Goto(1, 2)).unwrap(); // return to base to write buffer

//     let mut curr_line = 3;
//     for line in lines {
//         let line_str: String = line.into_iter().collect();
//         write!(screen, "{}{}", termion::cursor::Goto(1, curr_line), line_str).unwrap();

//         curr_line += 1;

//     }
// }

// fn set_banner(file_name: &str, screen_len: u16) -> Result<String, &'static str> {
//     let mut s = String::from("Welcome to INK v0.1. Current file: ");
//     s.push_str(file_name);
    
//     let pad_size = screen_len as usize - s.len();
//     let pad: String = " ".repeat(pad_size);
//     s.push_str(&pad);

//     Ok(s)
// }

// fn write_bottom_banner<W: Write>(screen: &mut W, curr_mode: &OperatorMode) {
//     let (_x, y) = termion::terminal_size().unwrap();
//     write!(screen, "{}{}Current Mode: {:?}. Available Modes | Operator (Ctrl+O) | Edit (Ctrl+E) | File (Ctrl+F)", termion::cursor::Goto(1, y), color::Bg(color::Red), curr_mode).unwrap();
//     write!(screen, "{}", style::Reset).unwrap();
//     write!(screen, "{}", termion::cursor::Goto(1, 2)).unwrap();
// }

// fn main() {
//     let mut screen = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();
//     let (x, y) = termion::terminal_size().unwrap();
    
//     let x_boundary = x - 3;
//     let y_boundary = y - 5;
//     let mut lines = vec![vec!['#'; x_boundary.into()]; y_boundary.into()];
    
//     let mut curr_mode = OperatorMode::O;
    
//     let banner_str = set_banner("test.py", x).unwrap();
//     write!(screen, "{}{}{}\n{}{}", 
//         termion::clear::All,
//         color::Bg(color::Red),
//         banner_str,
//         termion::cursor::Goto(1,2),
//         termion::cursor::Hide
//     ).unwrap();

//     let mut pos = CursorPos {
//         x: 1,
//         max_x: x_boundary, 
//         y: 2,
//         max_y: y_boundary
//     };

//     write!(screen, "{}", style::Reset).unwrap();
//     writelines(&mut screen, &lines, &pos);

//     write_bottom_banner(&mut screen, &curr_mode);
//     {
//         let stdin = stdin();

//         write!(screen, "{}{}{}", termion::cursor::Goto(pos.x, pos.y), termion::clear::CurrentLine, style::Underline).unwrap();

//         screen.flush().unwrap();
//         for k in stdin.keys() {
//             match k.as_ref().unwrap() {
//                 Key::Ctrl('c') => {
//                     println!("Ending key capture with key");
//                     break
//                 },
//                 Key::Ctrl('o') => {
//                     curr_mode = OperatorMode::O;
//                     write_bottom_banner(&mut screen, &curr_mode)
//                 },
//                 Key::Ctrl('e') => {
//                     curr_mode = OperatorMode::E;
//                     write_bottom_banner(&mut screen, &curr_mode)
//                 },
//                 Key::Ctrl('f') => {
//                     curr_mode = OperatorMode::F;
//                     write_bottom_banner(&mut screen, &curr_mode)
//                 },
//                 Key::Char(c) => {
//                     insertchar(&mut lines, &c, &pos);
//                     pos.update(1, 0)
//                 },
//                 Key::Up => pos.update(0, -1),
//                 Key::Down => pos.update(0, 1),
//                 Key::Left => pos.update(-1, 0),
//                 Key::Right => pos.update(1, 0),
//                 _ => todo!()
//             };

//             writelines(&mut screen, &lines, &pos);
//             screen.flush().unwrap();
//         }
//         screen.flush().unwrap();
//     }
    
//     write!(screen, "{}", termion::cursor::Show).unwrap();

//     println!("Phew! We are back.");
// }

// //use std::env;
// //use std::io::{self, Error, ErrorKind};
// //use std::{thread, time::Duration};
// //
// //use tui::{
// //    backend::CrosstermBackend,
// //    widgets::{Widget, Block, Borders},
// //    layout::{Layout, Constraint, Direction},
// //    Terminal,
// //
// //};
// //
// //use crossterm::{
// //    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
// //    execute,
// //    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
// //};
// //
// //mod file_system;
// //mod gap_buffer;
// //
// //fn handle_raw_input(user_in:String) {
// //    if user_in == "." || user_in.ends_with("/") {
// //        //file_system::handle_directory::handle();
// //        println!("Handling a directory!")
// //    }
// //    else {
// //        println!("Not handling directory!");
// //        // file_system::handle_directory::handle();
// //    }
// //}
// //
// //fn main() -> Result<(), Error> {
// //    let cmd_args: Vec<String> = env::args().collect();
// //
// //    if cmd_args.len() != 2 {
// //        println!("Usage: ink [fileName|directoryName]");
// //        return Err(Error::new(ErrorKind::Other, "oh no!"));
// //    }
// //    else {
// //        let user_in:String = cmd_args[1].clone();
// //
// //        handle_raw_input(user_in);
// //    }
// //
// //    println!("{:?}", cmd_args);
// //    
// //    enable_raw_mode()?;
// //    let mut stdout = io::stdout();
// //    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
// //    let backend = CrosstermBackend::new(stdout);
// //    let mut terminal = Terminal::new(backend)?;
// //
// //    terminal.draw(|f| {
// //        let size = f.size();
// //        let block = Block::default()
// //            .title("Ink - A Super Simple Text Editor")
// //            .borders(Borders::ALL);
// //        f.render_widget(block, size);
// //    })?;
// //    
// //    // This is where I block the terminal from exiting, below I can wait for ctrl+c or something
// //    // so i can exit 
// //    thread::sleep(Duration::from_millis(5000));
// //
// //    // restore terminal
// //    disable_raw_mode()?;
// //    execute!(
// //        terminal.backend_mut(),
// //        LeaveAlternateScreen,
// //        DisableMouseCapture
// //    )?;
// //    terminal.show_cursor()?;
// //    Ok(())
// //}
