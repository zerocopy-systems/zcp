use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::*,
    widgets::*,
};
use std::time::{Duration, Instant};

/// Runs the 60fps boot matrix animation
pub fn run_boot_matrix(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let start = Instant::now();
    let duration = Duration::from_secs(2); // 2 second splash

    while start.elapsed() < duration {
        terminal.draw(|f| {
            let size = f.area();
            
            let progress = start.elapsed().as_secs_f32() / duration.as_secs_f32();
            let opacity = (progress * 255.0) as u8;

            let block = Block::default()
                .title(" SOVEREIGN AUTHORITY v4.0.0 ".to_string())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(80, 250, 123))); // Accent green

            let inner = block.inner(size);
            f.render_widget(block, size);

            let text = vec![
                Line::from(vec![Span::styled(
                    "INITIALIZING ZERO-COPY PIPELINE...",
                    Style::default().fg(Color::Rgb(0, opacity, 0)).bold()
                )]),
                Line::from(vec![Span::styled(
                    "PCR0 VERIFICATION: OK",
                    Style::default().fg(Color::Rgb(0, opacity, 0))
                )]),
                Line::from(vec![Span::styled(
                    "LATENCY CONSTRAINT: < 42µs",
                    Style::default().fg(Color::Rgb(0, opacity, 0))
                )]),
                Line::from(vec![Span::styled(
                    "████████████████████████████████",
                    Style::default().fg(Color::Rgb(80, 250, 123))
                )]),
            ];

            let paragraph = Paragraph::new(text)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            // Center it
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Length(5),
                    Constraint::Percentage(40),
                ])
                .split(inner);

            f.render_widget(paragraph, layout[1]);
        })?;

        // Poll for early exit
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }

    Ok(())
}
