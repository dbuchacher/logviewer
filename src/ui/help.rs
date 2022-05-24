use crate::ui::*;

pub fn help_bar<B: Backend>(f: &mut Frame<B>, chunks: Vec<Rect>) {

    // colors
    let light_bg = Color::Rgb(127, 255, 255);
    let light_fg = Color::Rgb(000, 000, 000);
    let dark_bg  = Color::Rgb(015, 085, 085);
    let dark_fg  = Color::Rgb(210, 250, 250);
    let bar_bg   = Color::Rgb(020, 095, 095);

    // make code more readable
    let light = Style::default().bg(light_bg).fg(light_fg);
    let dark  = Style::default().bg(dark_bg) .fg(dark_fg);

    // text
    let text = Spans::from(vec![
        Span::styled("  ", dark), Span::styled(" Q ",       light), Span::styled(" Quit  ",               dark), Span::raw("  "),
        Span::styled("  ", dark), Span::styled(" F ",       light), Span::styled(" Find  ",               dark), Span::raw("  "),
        Span::styled("  ", dark), Span::styled(" ↑ ↓ ← → ", light), Span::styled(" Switch Group/Log  ", dark), Span::raw("  "),
        Span::styled("  ", dark), Span::styled(" ^D ",      light), Span::styled(" Clear Log ",           dark),
    ]);
    let clear = Spans::from(vec![
        Span::styled("  ", dark), Span::styled(" Confirm Delete? ", dark),
        Span::styled("Y", light), Span::styled("es  ",  dark),
        Span::styled("N", light), Span::styled("o  ",   dark), Span::raw("  ")
    ]);

    // bar background
    let full_bar = Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(Style::default().bg(bar_bg));

    f.render_widget(full_bar, chunks[2]);
}