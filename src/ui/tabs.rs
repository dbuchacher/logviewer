use crate::ui::*;

pub fn tabs<B: Backend>(f: &mut Frame<B>, app: &App, chunks: &Vec<Rect>,) {

    // colors
    let border    = Color::Rgb(050, 200, 200);
    let title_rgb = Color::Rgb(204, 204, 255);
    let active    = Color::Rgb(255, 255, 255);
    let inactive  = Color::Rgb(085, 085, 085);

    // log names for chooser
    let titles = app.names.iter().map(|t| {Spans::from(vec![
        Span::styled(*t, Style::default().fg(inactive))
    ])}).collect();

    // title `Logs`
    let title_txt = Spans::from(vec![
        Span::styled("( ",   Style::default().fg(border)),
        Span::styled("Logs", Style::default().fg(title_rgb)),
        Span::styled(" )",   Style::default().fg(border)),
    ]);

    let tabs_block = Block::default()
        .border_style(Style::default().fg(border))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .title(title_txt);

    let tabs_complete = Tabs::new(titles)
        .block(tabs_block)
        .select(app.index)
        .highlight_style(Style::default().fg(active))
        .divider(Span::styled("|", Style::default().fg(border)));

    f.render_widget(tabs_complete, chunks[0]);
}