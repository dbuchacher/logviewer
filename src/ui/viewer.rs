use crate::ui::*;

// draws the middle section where we see the actual log updating
pub fn viewer<B: Backend>(
    f: &mut Frame<B>,
    app: &App,
    chunks: &Vec<Rect>,
) { 
    let filename = app.paths[app.index].to_string();
    
    // colors
    let border_color = Color::Rgb(050, 200, 200);
    let title_color  = Color::Rgb(204, 204, 255);
    let log_color    = Color::Rgb(210, 250, 250);

    // text
    let log_text = read_log(&filename, f.size().height).unwrap_or_else(|err| err.to_string());
    let title_text = Spans::from(vec![
        Span::styled("( ",     Style::default().fg(border_color)),
        Span::styled(filename, Style::default().fg(title_color)),
        Span::styled(" )",     Style::default().fg(border_color)),
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