use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame, accent_color: Color) {
    //----------[ helpers ]----------//
    fn calculate_centered_rect(r: Rect, x: u16, y: u16, frame: &mut Frame) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(frame.size().height / 2 - y / 2),
                Constraint::Length(y),
                Constraint::Length(frame.size().height / 2 - y / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(frame.size().width / 2 - x / 2),
                Constraint::Length(x),
                Constraint::Length(frame.size().width / 2 - x / 2),
            ])
            .split(popup_layout[1])[1]
    }
    //-------------------------------//

    //----------[ layouts ]----------//
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .split(frame.size());

    let centered = calculate_centered_rect(main_layout[0], 30, 22, frame);

    let playfield_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Length(24), Constraint::Length(6)])
        .split(centered);

    let side_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(72), Constraint::Percentage(28)])
        .split(playfield_split[1]);

    //-------------------------------//

    //----------[ render widgets ]----------//
    {
        // Render the main interface.
        frame.render_widget(
            Paragraph::new("")
                .block(
                    Block::bordered()
                        .title("┤ TETRS ├")
                        .title_alignment(Alignment::Center)
                        .border_type(BorderType::Plain),
                )
                .style(Style::default().fg(accent_color))
                .centered(),
            main_layout[0],
        );

        //----------[ Game Area ]----------//
        {
            // Render the playfield.
            frame.render_widget(
                Paragraph::new(app.playfield_string().to_string())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Plain)
                            .border_style(Style::default().fg(accent_color)), // Set border color to Yellow
                    )
                    .style(Style::default().fg(Color::White)), // Set content color to white
                playfield_split[0],
            );
            //----------[ Side Bar]----------//
            {
                //preview
                frame.render_widget(
                    Paragraph::new(String::new())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Plain)
                                .border_style(Style::default().fg(accent_color)), // Set border color to Yellow
                        )
                        .style(Style::default().fg(Color::White))
                        .centered(),
                    side_layout[0],
                );
                //swap
                frame.render_widget(
                    Paragraph::new(app.tetromino_string(app.swap_tetromino).to_string())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Plain)
                                .border_style(Style::default().fg(accent_color)), // Set border color to Yellow
                        )
                        .style(Style::default().fg(Color::White))
                        .centered(),
                    side_layout[1],
                );
            }
            //-------------------------------//
        }
        //---------------------------------//
    }
    //--------------------------------------//
}
