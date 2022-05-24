

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

// groups are a collection of log files;  lets users organize simular types of logs
pub struct Group {
    pub name: String,
    pub locations: Vec<String>,
    pub filenames: Vec<String>,
    pub index: usize,
}

impl Group {
    pub fn new() -> Vec<Group> {
        // enviroment variable from the operationg system that points to where the config file is located
        let env = std::env::var("LOGVIEWER").unwrap_or_else(|err| {
            eprintln!("\nError: {}\nTry setting `LOGVIEWER` environment variable.\n", err);
            std::process::exit(1);
        });

        // open config file
        let config_file = std::fs::read_to_string(env).unwrap_or_else(|err| {
            eprintln!("\nError: {}\nDoes the config file exist?\n", err);
            std::process::exit(1);
        });

        let mut groups = Vec::new(); // name, locations, filenames, index

        for line in config_file.lines() {
            groups.push(process_data(&line));
        }

        fn process_data(line: &str) -> Group {
                                                         // example text:     `group_name=/var/log/aaa;/var/log/bbb`
            let s = line.split_once('=').unwrap();       // `group_name`      `/var/log/aaa;/var/log/bbb`
            let mut locations: Vec<String> = Vec::new(); // `/var/log/aaa`    `/var/log/bbb`
            let mut filenames: Vec<String> = Vec::new(); // `aaa`             `bbb`
        
            for path in s.1.split(':').collect::<Vec<&str>>() {
                locations.push(String::from(path));
            }

            for filename in &locations {
                if let Some(last) = filename.split('/').last() {
                    filenames.push(String::from(last));
                }
            }
            
            Group { 
                name: s.0.to_string(),
                locations: locations,
                filenames: filenames,
                index: 0,
            }
        }

        groups
    }
}
pub struct App {
    pub groups: Vec<Group>,
    pub index: usize,
}

impl App {
    pub fn new() -> App {
        App {
            groups: Group::new(),
            index: 0,
        }
    }

    pub fn next_group(&mut self) {
        self.index = (self.index + 1) % self.groups.len();
    }

    pub fn previous_group(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.groups.len() - 1;
        }
    }

    pub fn next_log(&mut self) {
        self.groups[self.index].index = (self.groups[self.index].index + 1) % self.groups[self.index].filenames.len();
    }

    pub fn previous_log(&mut self) {
        if self.groups[self.index].index > 0 {
            self.groups[self.index].index -= 1;
        } else {
            self.groups[self.index].index = self.groups[self.index].filenames.len() - 1;
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

// returns the tail text of a log file based on the height of the console screen
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

// boiler plate code for using tui/crossterm
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    Ok(Terminal::new(backend)?)
}

// boiler plate code for using tui/crossterm
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