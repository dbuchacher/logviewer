mod ui;

pub use tui::{backend::*, layout::*, style::*, text::*, widgets::*, Frame, Terminal };
use std::io::Stdout;
pub use std::{ error::Error, time::*, io };
pub use crossterm::{ event::{*, self}, execute, terminal::* };

pub use crate::ui::*;
pub use crate::ui::help;
pub use crate::ui::tabs;
pub use crate::ui::viewer;
pub use crate::ui::warning;

const MIN_HEIGHT: u16 = 4; // minimun size
const VIEWER_CLUTTER: u16 = 6; // 6 lines used for tabs and help

pub struct App<'a> {
    pub paths: Vec<&'a str>,
    pub names: Vec<&'a str>,
    pub index: usize,
}

impl<'a> App<'a> {
    pub fn new(paths: Vec<&'a str>, names: Vec<&'a str>) -> App<'a> {
        App {
            paths: paths,
            names: names,
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.names.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.names.len() - 1;
        }
    }
}

pub fn log_paths(s: &str) -> Vec<&str> {
    s.split(':').collect()
}

pub fn log_names(s: &str) -> Vec<&str> {
    let mut log_names: Vec<&str> = Vec::new();
    let log_paths: Vec<&str> = s.split(':').collect();

    for filename in log_paths {
        if let Some(last) = filename.split('/').last() { 
            log_names.push(last);
        }
    }

    log_names
}

pub fn read_log(filename: &str, height: u16) -> Result<String, Box<dyn Error>> { 
    let mut proper_height = String::new(); // manipulate this 
    let mut log = String::new();           // return this

    let file = std::fs::read_to_string(filename)?;
    let mut rev_lines = file.lines().rev().into_iter();

    // get the same amount of lines as the console height
    for _i in 0..height - VIEWER_CLUTTER {
        proper_height.push_str(rev_lines.next().unwrap_or("Error"));
        proper_height.push_str("\n");
    }
    // read the text so it isn't backwards
    for line in proper_height.lines().rev().into_iter() {
        log.push_str(line);
        log.push_str("\n");
    }

    Ok(log)
}

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    Ok(Terminal::new(backend)?)
}

pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}