use chrono::Local;
use genpdf::{
    elements::{Break, FrameCellDecorator, Paragraph, TableLayout},
    style::{Color, Style},
    Alignment, Document, Element as _,
};

use crate::models::standings::CompetitionStandingResponse;

pub fn build_standings_pdf(
    data: &CompetitionStandingResponse,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)?;
    let mut doc = Document::new(font_family);

    doc.set_title(format!("{} Standings Report", data.competition.name));
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.2);

    // Page decorator: margins + header
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(genpdf::Margins::trbl(14, 12, 14, 12));
    let competition_name = data.competition.name.clone();
    let competition_code = data.competition.code.clone();
    decorator.set_header(move |page| {
        Paragraph::new(format!(
            "{} ({}) — Standings Report — Page {}",
            competition_name, competition_code, page
        ))
        .aligned(Alignment::Center)
        .styled(
            Style::new()
                .bold()
                .with_font_size(10)
                .with_color(Color::Rgb(90, 90, 90)),
        )
    });
    doc.set_page_decorator(decorator);

    let title_style = Style::new()
        .bold()
        .with_font_size(22)
        .with_color(Color::Rgb(22, 22, 22));

    let subtitle_style = Style::new()
        .with_font_size(12)
        .with_color(Color::Rgb(90, 90, 90));

    let section_title_style = Style::new()
        .bold()
        .with_font_size(14)
        .with_color(Color::Rgb(20, 20, 20));

    let body_style = Style::new().with_font_size(11);

    let table_header_style = Style::new()
        .bold()
        .with_font_size(10)
        .with_color(Color::Rgb(15, 15, 15));

    let normal_row_style = Style::new().with_font_size(10);

    let muted_row_style = Style::new()
        .with_font_size(10)
        .with_color(Color::Rgb(70, 70, 70));

    let top_team_style = Style::new()
        .bold()
        .with_font_size(10)
        .with_color(Color::Rgb(0, 76, 153));

    let points_style = Style::new()
        .bold()
        .with_font_size(10)
        .with_color(Color::Rgb(0, 0, 0));

    let footer_style = Style::new()
        .italic()
        .with_font_size(10)
        .with_color(Color::Rgb(110, 110, 110));

    // Hero header
    doc.push(
        Paragraph::new(data.competition.name.clone())
            .aligned(Alignment::Center)
            .styled(title_style),
    );

    doc.push(
        Paragraph::new("League Standings Report")
            .aligned(Alignment::Center)
            .styled(Style::new().bold().with_font_size(16)),
    );

    doc.push(
        Paragraph::new(format!(
            "Competition code: {}   •   Current matchday: {}",
            data.competition.code,
            data.season
                .current_matchday
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string())
        ))
        .aligned(Alignment::Center)
        .styled(subtitle_style),
    );

    doc.push(Break::new(1));

    // Intro
    doc.push(Paragraph::new("Summary").styled(section_title_style));
    doc.push(
        Paragraph::new(format!(
            "This report presents the current standings for {}. \
The table below shows position, matches played, wins, draws, losses, goals for, goals against, goal difference and total points.",
            data.competition.name
        ))
        .styled(body_style),
    );

    doc.push(Break::new(1));

    // Top 3 block
    if let Some(group) = data.standings.first() {
        let top_three: Vec<_> = group.table.iter().take(3).collect();

        if !top_three.is_empty() {
            doc.push(Paragraph::new("Top 3 Teams").styled(section_title_style));

            for team in top_three {
                doc.push(
                    Paragraph::new(format!(
                        "{}. {}  —  {} pts  •  GD {}",
                        team.position, team.team_name, team.points, team.goal_difference
                    ))
                    .styled(
                        Style::new()
                            .bold()
                            .with_font_size(11)
                            .with_color(Color::Rgb(0, 76, 153)),
                    ),
                );
            }

            doc.push(Break::new(1));
        }
    }

    // Standings tables
    for standing_group in &data.standings {
        doc.push(
            Paragraph::new(format!("Standing Type: {}", standing_group.standing_type))
                .styled(section_title_style),
        );
        doc.push(Break::new(1));

        let mut table = TableLayout::new(vec![1, 5, 1, 1, 1, 1, 1, 1, 1, 1]);
        table.set_cell_decorator(FrameCellDecorator::new(true, true, false));

        table
            .row()
            .element(Paragraph::new("#").styled(table_header_style))
            .element(Paragraph::new("Team").styled(table_header_style))
            .element(Paragraph::new("P").styled(table_header_style))
            .element(Paragraph::new("W").styled(table_header_style))
            .element(Paragraph::new("D").styled(table_header_style))
            .element(Paragraph::new("L").styled(table_header_style))
            .element(Paragraph::new("GF").styled(table_header_style))
            .element(Paragraph::new("GA").styled(table_header_style))
            .element(Paragraph::new("GD").styled(table_header_style))
            .element(Paragraph::new("Pts").styled(table_header_style))
            .push()?;

        for (index, row) in standing_group.table.iter().enumerate() {
            let base_style = if index % 2 == 0 {
                normal_row_style
            } else {
                muted_row_style
            };

            let row_style = if row.position <= 3 {
                top_team_style
            } else {
                base_style
            };

            table
                .row()
                .element(Paragraph::new(row.position.to_string()).styled(row_style))
                .element(Paragraph::new(truncate(&row.team_short_name, 24)).styled(row_style))
                .element(Paragraph::new(row.played_games.to_string()).styled(row_style))
                .element(Paragraph::new(row.won.to_string()).styled(row_style))
                .element(Paragraph::new(row.draw.to_string()).styled(row_style))
                .element(Paragraph::new(row.lost.to_string()).styled(row_style))
                .element(Paragraph::new(row.goals_for.to_string()).styled(row_style))
                .element(Paragraph::new(row.goals_against.to_string()).styled(row_style))
                .element(Paragraph::new(row.goal_difference.to_string()).styled(row_style))
                .element(Paragraph::new(row.points.to_string()).styled(points_style))
                .push()?;
        }

        doc.push(table);
        doc.push(Break::new(2));
    }

    // Footer
    let generated_at = Local::now().format("%d.%m.%Y %H:%M").to_string();

    doc.push(
        Paragraph::new("Generated by Analytics & Reporting Service")
            .aligned(Alignment::Center)
            .styled(footer_style),
    );
    doc.push(
        Paragraph::new(format!("Generated at: {}", generated_at))
            .aligned(Alignment::Right)
            .styled(footer_style),
    );

    let mut bytes = Vec::new();
    doc.render(&mut bytes)?;
    Ok(bytes)
}

fn truncate(value: &str, max_len: usize) -> String {
    if value.chars().count() <= max_len {
        value.to_string()
    } else {
        let mut result = value.chars().take(max_len - 3).collect::<String>();
        result.push_str("...");
        result
    }
}