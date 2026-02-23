use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub fn run_live_monitor(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    let mut data: Vec<u64> = vec![0; 100];

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(size);

            // Header
            let header = Paragraph::new(Line::from(vec![
                Span::styled(" zcp monitor ", Style::default().fg(Color::White).bold()),
                Span::raw(" | "),
                Span::styled("LIVE TESTNET", Style::default().fg(Color::Green)),
            ]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );
            f.render_widget(header, chunks[0]);

            // Body
            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(chunks[1]);

            // Sparkline Panel
            let sparkline = Sparkline::default()
                .block(
                    Block::default()
                        .title(" Latency Jitter (µs) ")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .data(&data)
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(sparkline, body_chunks[0]);

            // Order Book Panel
            let bids = [
                Line::from(vec![
                    Span::styled("99,990.50", Style::default().fg(Color::Green)),
                    Span::raw("  1.5 BTC"),
                ]),
                Line::from(vec![
                    Span::styled("99,990.00", Style::default().fg(Color::Green)),
                    Span::raw("  0.8 BTC"),
                ]),
                Line::from(vec![
                    Span::styled("99,985.25", Style::default().fg(Color::Green)),
                    Span::raw("  3.2 BTC"),
                ]),
            ];
            let asks = [
                Line::from(vec![
                    Span::styled("100,005.10", Style::default().fg(Color::Red)),
                    Span::raw(" 12.0 BTC"),
                ]),
                Line::from(vec![
                    Span::styled("100,002.00", Style::default().fg(Color::Red)),
                    Span::raw("  2.5 BTC"),
                ]),
                Line::from(vec![
                    Span::styled("100,000.50", Style::default().fg(Color::Red)),
                    Span::raw("  0.5 BTC"),
                ]),
            ];

            let ob_text = vec![
                Line::from("ASKS"),
                asks[0].clone(),
                asks[1].clone(),
                asks[2].clone(),
                Line::from("---"),
                bids[0].clone(),
                bids[1].clone(),
                bids[2].clone(),
                Line::from("BIDS"),
            ];

            let ob = Paragraph::new(ob_text)
                .block(
                    Block::default()
                        .title(" L2 Order Book ")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .alignment(Alignment::Right);
            f.render_widget(ob, body_chunks[1]);

            // Footer
            let footer = Paragraph::new(
                "Press 'q' or 'Esc' to quit | PnL: +$1,204.50 | Target: Binance Futures",
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );
            f.render_widget(footer, chunks[2]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Update sparkline data
            data.remove(0);
            let sys_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            let random_val = (sys_time % 100) as u64;
            data.push(random_val);
            last_tick = Instant::now();
        }
    }
}
