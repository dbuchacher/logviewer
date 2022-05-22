pub use tui::{backend::*, layout::*, style::*, text::*, widgets::*, Frame, Terminal };
pub use std::{ error::Error, time::*, io };
pub use crossterm::{ event::{*, self}, execute, terminal::* };

pub use crate::{*, warning::*, viewer::*, tabs::*, help::* };

pub mod help;
pub mod tabs;
pub mod viewer;
pub mod warning;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // display size warning if needed
    if f.size().height < MIN_HEIGHT {
        console_size_warning(f);
        return
    }

    // seperate console into 3 parts
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
                Constraint::Length(3),                   // chunk[0] = chooser
                Constraint::Length(f.size().height - 4), // chunk[1] = viewer
                Constraint::Length(1),                   // chunk[2] = help
            ].as_ref()
        )
        .split(f.size());
    
    // draw
    tabs(f, app, &chunks);
    viewer(f, app, &chunks);
    help_bar(f, chunks);
}