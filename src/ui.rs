use ratatui::{
	prelude::*,
	widgets::{
		Block,
		BorderType,
		Borders,
		Paragraph,
	},
};

use crate::{
	app::App,
	clap::clap_parse,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame, accent_color: Color, border_type: BorderType) {
	let binding = clap_parse();

	let control_buttons: bool = *binding.get_one("ControlButtons").unwrap();

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

	let centered = calculate_centered_rect(
		main_layout[0],
		30,
		if control_buttons { 32 } else { 24 },
		frame,
	);

	let button_split = Layout::default()
		.direction(Direction::Vertical)
		.constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
		.split(if control_buttons {
			centered
		} else {
			main_layout[0]
		});

	let playfield_split = Layout::default()
		.direction(Direction::Horizontal)
		.constraints(vec![Constraint::Length(22), Constraint::Length(6)])
		.split(if control_buttons {
			button_split[0]
		} else {
			centered
		});

	let side_layout = Layout::default()
		.direction(Direction::Vertical)
		.constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
		.split(playfield_split[1]);

	let button_columns = Layout::default()
		.direction(Direction::Horizontal)
		.constraints(vec![
			Constraint::Length(6),
			Constraint::Length(1),
			Constraint::Length(6),
			Constraint::Length(9),
			Constraint::Length(6),
		])
		.split(button_split[1]);

	let button_rows_one = Layout::default()
		.direction(Direction::Vertical)
		.constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
		.split(button_columns[0]);

	let button_rows_two = Layout::default()
		.direction(Direction::Vertical)
		.constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
		.split(button_columns[2]);

	let button_rows_three = Layout::default()
		.direction(Direction::Vertical)
		.constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
		.split(button_columns[4]);

	//-------------------------------//

	//----------[ render widgets ]----------//
	{
		// Render the main interface.
		frame.render_widget(
			Paragraph::new(control_buttons.to_string())
				.block(
					Block::bordered()
						.title("┤ TETRS ├")
						.title_alignment(Alignment::Center)
						.border_type(border_type),
				)
				.style(Style::default().fg(accent_color))
				.centered(),
			main_layout[0],
		);

		//----------[ Game Area ]----------//
		{
			// Render the playfield.
			frame.render_widget(
				Paragraph::new(app.playfield_string())
					.block(
						Block::default()
							.borders(Borders::ALL)
							.border_type(border_type)
							.border_style(Style::default().fg(accent_color)), /* Set border color to
						                                                   * Yellow */
					)
					.style(Style::default().fg(Color::White)), // Set content color to white
				playfield_split[0],
			);
			//----------[ Side Bar]----------//
			{
				//preview
				frame.render_widget(
					Paragraph::new(app.tetromino_queue_string())
						.block(
							Block::default()
								.borders(Borders::ALL)
								.border_type(border_type)
								.border_style(Style::default().fg(accent_color)), /* Set border
							                                                   * color to Yellow */
						)
						.style(Style::default().fg(Color::White))
						.centered(),
					side_layout[0],
				);
				//swap
				frame.render_widget(
					Paragraph::new(app.tetromino_string(app.swap_tetromino))
						.block(
							Block::default()
								.borders(Borders::ALL)
								.border_type(border_type)
								.border_style(Style::default().fg(accent_color)), /* Set border
							                                                   * color to Yellow */
						)
						.style(Style::default().fg(Color::White))
						.centered(),
					side_layout[1],
				);
			}
			//-------------------------------//
		}
		//---------------------------------//
		//----------[ Control Buttons ]----------//
		{
			if control_buttons {
				frame.render_widget(
					Paragraph::new("")
						.block(Block::bordered().border_type(border_type))
						.style(Style::default().fg(accent_color))
						.centered(),
					button_rows_one[0],
				);
				frame.render_widget(
					Paragraph::new("")
						.block(Block::bordered().border_type(border_type))
						.style(Style::default().fg(accent_color))
						.centered(),
					button_rows_one[1],
				);
				frame.render_widget(
					Paragraph::new("")
						.block(Block::bordered().border_type(border_type))
						.style(Style::default().fg(accent_color))
						.centered(),
					button_rows_two[0],
				);
				frame.render_widget(
					Paragraph::new("")
						.block(Block::bordered().border_type(border_type))
						.style(Style::default().fg(accent_color))
						.centered(),
					button_rows_two[1],
				);
				frame.render_widget(
					Paragraph::new("")
						.block(Block::bordered().border_type(border_type))
						.style(Style::default().fg(accent_color))
						.centered(),
					button_rows_three[0],
				);
				frame.render_widget(
					Paragraph::new("")
						.block(Block::bordered().border_type(border_type))
						.style(Style::default().fg(accent_color))
						.centered(),
					button_rows_three[1],
				);
			}
		}
		//---------------------------------------//
	}
	//--------------------------------------//
}
