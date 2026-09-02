use ratatui::{Frame, layout::{Alignment, Constraint, Layout}, style::{Color, Style, Stylize}, symbols::line, text::Line, widgets::{Block, BorderType, Borders, Paragraph}};

use crate::app::App;

/// Render the main block
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
    ]);

    let [title_area, body_area] = frame.area().layout(&layout);

    let title = Line::from("JustDeploy - Deployments made easy!").centered().bold();
    
    // rendering widgets
    frame.render_widget(title, title_area);
}
