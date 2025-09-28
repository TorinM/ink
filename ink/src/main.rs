use std::env;
use std::io::{self, Error, ErrorKind};
use std::{thread, time::Duration};

use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal,

};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

mod file_system;
mod gap_buffer;

fn handle_raw_input(user_in:String) {
    if user_in == "." || user_in.ends_with("/") {
        //file_system::handle_directory::handle();
        println!("Handling a directory!")
    }
    else {
        println!("Not handling directory!");
        // file_system::handle_directory::handle();
    }
}

fn main() -> Result<(), Error> {
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() != 2 {
        println!("Usage: ink [fileName|directoryName]");
        return Err(Error::new(ErrorKind::Other, "oh no!"));
    }
    else {
        let user_in:String = cmd_args[1].clone();

        handle_raw_input(user_in);
    }

    println!("{:?}", cmd_args);
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Ink - A Super Simple Text Editor")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    
    // This is where I block the terminal from exiting, below I can wait for ctrl+c or something
    // so i can exit 
    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
