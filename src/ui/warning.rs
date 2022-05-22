use crate::ui::*;

pub fn console_size_warning<B: Backend>(f: &mut Frame<B>) {
    // divide screen; so we can center warning text
    let warning = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // waste the top half of the screen
    f.render_widget(
        Block::default().style(Style::default().bg(Color::Red)),
    warning[0]);

    // waring text
    f.render_widget(
        Paragraph::new("Must: Increase Terminal Size")
            .style(Style::default().bg(Color::Red).fg(Color::Black))
            .alignment(Alignment::Center),
    warning[1]);
}