use ratatui::{Frame, layout::Alignment, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph}};

use crate::app::App;

/// Render the main block
pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!(
                "
Press `Esc`, `Ctrl-C`, `q` to stop running.
                "
            ))
            .block(
                Block::default()
                    .title("JustDeploy - Deployments Made Easy")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
                )
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center),
            frame.area(),
    )
}
