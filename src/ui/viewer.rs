use crate::ui::*;

use std::{os::unix::prelude::MetadataExt, fs, default};
use pretty_bytes::converter::convert;

// draws the middle section where we see the actual log updating
pub fn viewer<B: Backend>(
    f: &mut Frame<B>,
    app: &App,
    chunks: &Vec<Rect>,
) {     
    // colors
    let border_color = Color::Rgb(050, 200, 200);
    let title_color  = Color::Rgb(204, 204, 255);
    let log_color    = Color::Rgb(210, 250, 250);

    // easier to read variables
    let group = &app.groups[app.index];
    let filename = &group.locations[group.index];
    let file_size = get_size(filename).unwrap_or("0 MB".to_string());

    // text
    let log_text = read_log(&filename, f.size().height).unwrap_or_else(|err| err.to_string());
    let title_text = Spans::from(vec![
        Span::styled("( ",      Style::default().fg(border_color)),
        Span::styled(filename,  Style::default().fg(title_color)),
        Span::styled("  ",      Style::default().fg(border_color)),
        Span::styled(file_size, Style::default().fg(title_color)),
        Span::styled(" )",      Style::default().fg(border_color)),
    ]);


    let block = Block::default()
        .title(title_text)
        .borders(Borders::ALL)
        .title_alignment(Alignment::Right)
        .border_style(Style::default().fg(border_color));

    let complete = Paragraph::new(log_text)
        .block(block)
        .style(Style::default().fg(log_color))
        .wrap(Wrap { trim: true });

    f.render_widget(complete, chunks[1]);
}

// human readable size of file
fn get_size(file: &String) -> io::Result<String> {
    let meta = fs::metadata(file)?;
    Ok(convert(meta.size() as f64))
}