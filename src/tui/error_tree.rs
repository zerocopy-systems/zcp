use ratatui::{
    prelude::*,
    widgets::*,
};

/// A visual tree to replace raw stack traces
pub fn build_error_tree(error_msg: &str) -> Paragraph<'static> {
    let text = vec![
        Line::from(vec![Span::styled("⚡ CRITICAL SYSTEM FAULT ⚡", Style::default().fg(Color::Red).bold())]),
        Line::from(""),
        Line::from(vec![Span::styled("├── [Source]", Style::default().fg(Color::DarkGray)), Span::raw(" zero_copy_core::engine")]),
        Line::from(vec![Span::styled("│", Style::default().fg(Color::DarkGray))]),
        Line::from(vec![Span::styled("├── [Context]", Style::default().fg(Color::DarkGray)), Span::raw(" Validation Phase")]),
        Line::from(vec![Span::raw("│    ├── Operation: SignPayload")]),
        Line::from(vec![Span::raw("│    └── Constraint: Latency < 42µs")]),
        Line::from(vec![Span::styled("│", Style::default().fg(Color::DarkGray))]),
        Line::from(vec![Span::styled("└── [Details]", Style::default().fg(Color::DarkGray)), Span::styled(format!(" {}", error_msg), Style::default().fg(Color::White))]),
    ];

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Double).border_style(Style::default().fg(Color::Red)))
        .alignment(Alignment::Left)
}
