extern crate indoc;

use indoc::indoc;
use itertools::izip;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{block::Title, Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_main_area(f: &mut Frame, content_chunk: &[Rect], app: &mut App) {
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from("Welcome!"))
        .style(Style::default().bg(app.background_color));

    let logo = Paragraph::new(logo()).block(content_block);

    f.render_widget(logo, content_chunk[1]);
}

fn logo() -> String {
    let spoify = indoc! {"
███████╗██████╗  ██████╗ ██╗███████╗██╗   ██╗
██╔════╝██╔══██╗██╔═══██╗██║██╔════╝╚██╗ ██╔╝
███████╗██████╔╝██║   ██║██║█████╗   ╚████╔╝ 
╚════██║██╔═══╝ ██║   ██║██║██╔══╝    ╚██╔╝  
███████║██║     ╚██████╔╝██║██║        ██║   
╚══════╝╚═╝      ╚═════╝ ╚═╝╚═╝        ╚═╝   

    "};

    izip!(spoify.lines())
        .map(|spoify| format!("{spoify:5}"))
        .collect::<Vec<_>>()
        .join("\n")
}
