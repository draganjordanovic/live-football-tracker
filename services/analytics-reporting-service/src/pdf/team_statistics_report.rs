use chrono::Local;
use genpdf::{
    elements::{Break, FrameCellDecorator, Paragraph, TableLayout},
    style::{Color, Style},
    Alignment, Document, Element as _,
};

use crate::models::team_statistics::TeamStatisticsResponse;

pub fn build_team_statistics_pdf(
    data: &TeamStatisticsResponse,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)?;
    let mut doc = Document::new(font_family);

    doc.set_title(format!("Team Statistics Report - {}", data.team_name));
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.2);

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(genpdf::Margins::trbl(14, 12, 14, 12));

    let team_name = data.team_name.clone();
    let competition_name = data.competition_name.clone();

    decorator.set_header(move |page| {
        Paragraph::new(format!(
            "{} — {} — Team Statistics Report — Page {}",
            team_name, competition_name, page
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

    let label_style = Style::new().bold().with_font_size(11);
    let value_style = Style::new()
        .bold()
        .with_font_size(12)
        .with_color(Color::Rgb(0, 76, 153));

    let footer_style = Style::new()
        .italic()
        .with_font_size(10)
        .with_color(Color::Rgb(110, 110, 110));

    // HERO
    doc.push(
        Paragraph::new(data.team_name.clone())
            .aligned(Alignment::Center)
            .styled(title_style),
    );

    doc.push(
        Paragraph::new(format!(
            "{} • Position #{} • Matchday {}",
            data.competition_name,
            data.position,
            data.current_matchday
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string())
        ))
        .aligned(Alignment::Center)
        .styled(subtitle_style),
    );

    doc.push(Break::new(1));

    // QUICK HIGHLIGHTS
    doc.push(Paragraph::new("Quick Highlights").styled(section_title_style));

    let mut highlights = TableLayout::new(vec![2, 2, 2, 2]);
    highlights.set_cell_decorator(FrameCellDecorator::new(true, true, false));

    highlights
        .row()
        .element(Paragraph::new("Played").styled(label_style))
        .element(Paragraph::new("Points").styled(label_style))
        .element(Paragraph::new("Goal Diff").styled(label_style))
        .element(Paragraph::new("Avg Pts / Match").styled(label_style))
        .push()?;

    highlights
        .row()
        .element(Paragraph::new(data.played_games.to_string()).styled(value_style))
        .element(Paragraph::new(data.points.to_string()).styled(value_style))
        .element(Paragraph::new(data.goal_difference.to_string()).styled(value_style))
        .element(
            Paragraph::new(format!("{:.2}", data.average_points_per_match)).styled(value_style),
        )
        .push()?;

    doc.push(highlights);
    doc.push(Break::new(1));

    // PERFORMANCE
    doc.push(Paragraph::new("Performance").styled(section_title_style));
    let mut performance = TableLayout::new(vec![3, 1]);
    performance.set_cell_decorator(FrameCellDecorator::new(true, true, false));

    add_row(&mut performance, "Played Games", data.played_games.to_string(), label_style, value_style)?;
    add_row(&mut performance, "Wins", data.won.to_string(), label_style, value_style)?;
    add_row(&mut performance, "Draws", data.draw.to_string(), label_style, value_style)?;
    add_row(&mut performance, "Losses", data.lost.to_string(), label_style, value_style)?;
    add_row(&mut performance, "Points", data.points.to_string(), label_style, value_style)?;
    add_row(
        &mut performance,
        "Average Points per Match",
        format!("{:.2}", data.average_points_per_match),
        label_style,
        value_style,
    )?;

    doc.push(performance);
    doc.push(Break::new(1));

    // GOALS
    doc.push(Paragraph::new("Goals").styled(section_title_style));
    let mut goals = TableLayout::new(vec![3, 1]);
    goals.set_cell_decorator(FrameCellDecorator::new(true, true, false));

    add_row(&mut goals, "Goals For", data.goals_for.to_string(), label_style, value_style)?;
    add_row(&mut goals, "Goals Against", data.goals_against.to_string(), label_style, value_style)?;
    add_row(&mut goals, "Goal Difference", data.goal_difference.to_string(), label_style, value_style)?;
    add_row(
        &mut goals,
        "Average Goals For per Match",
        format!("{:.2}", data.average_goals_for_per_match),
        label_style,
        value_style,
    )?;
    add_row(
        &mut goals,
        "Average Goals Against per Match",
        format!("{:.2}", data.average_goals_against_per_match),
        label_style,
        value_style,
    )?;
    add_row(
        &mut goals,
        "Average Goal Difference per Match",
        format!("{:.2}", data.average_goal_difference_per_match),
        label_style,
        value_style,
    )?;

    doc.push(goals);
    doc.push(Break::new(1));

    // RATES
    doc.push(Paragraph::new("Rates").styled(section_title_style));
    let mut rates = TableLayout::new(vec![3, 1]);
    rates.set_cell_decorator(FrameCellDecorator::new(true, true, false));

    add_row(
        &mut rates,
        "Win Rate",
        format!("{:.1}%", data.win_rate * 100.0),
        label_style,
        value_style,
    )?;
    add_row(
        &mut rates,
        "Draw Rate",
        format!("{:.1}%", data.draw_rate * 100.0),
        label_style,
        value_style,
    )?;
    add_row(
        &mut rates,
        "Loss Rate",
        format!("{:.1}%", data.loss_rate * 100.0),
        label_style,
        value_style,
    )?;

    doc.push(rates);

    doc.push(Break::new(2));

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

fn add_row(
    table: &mut TableLayout,
    label: &str,
    value: String,
    label_style: Style,
    value_style: Style,
) -> Result<(), Box<dyn std::error::Error>> {
    table
        .row()
        .element(Paragraph::new(label).styled(label_style))
        .element(Paragraph::new(value).styled(value_style))
        .push()?;
    Ok(())
}