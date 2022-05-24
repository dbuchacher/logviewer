use log::*;

fn main() -> Result<(), Box<dyn Error>> {

    let mut app = App::new();
    // let mut app = App::new(env_var);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(666);
    
    let mut terminal = setup_terminal().unwrap_or_else(|err| {
        eprintln!("\nError: {}\n", err);
        std::process::exit(1);
    });

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // mouse and keyboard events
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => app.next_log(),
                    KeyCode::Left => app.previous_log(),
                    KeyCode::Up => app.next_group(),
                    KeyCode::Down => app.previous_group(),
                    _ => {}
                }
            }
        }
        
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    
    restore_terminal(terminal)?;
    
    Ok(())
}

// if let Event::Mouse(key) = event::read()? {
//     match key.kind {
//         MouseEventKind::Down(MouseButton::Middle) => execute!(io::stdout(), DisableMouseCapture)?,
//         _ => {}
//     }
// }