use crate::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Tabs},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        1 => draw_second_tab(f, app, chunks[1]),
        2 => draw_third_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
//     let chunks = Layout::default()
//         .constraints(
//             [
//                 Constraint::Length(9),
//                 Constraint::Min(8),
//                 Constraint::Length(7),
//             ]
//             .as_ref(),
//         )
//         .split(area);
//     draw_gauges(f, app, chunks[0]);
//     draw_panes(f, app, chunks[1]);
//     draw_text(f, chunks[2]);
    draw_panes(f, app, area);
}

// fn draw_gauges<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
// where
//     B: Backend,
// {
//     let chunks = Layout::default()
//         .constraints(
//             [
//                 Constraint::Length(2),
//                 Constraint::Length(3),
//                 Constraint::Length(1),
//             ]
//             .as_ref(),
//         )
//         .margin(1)
//         .split(area);
//     let block = Block::default().borders(Borders::ALL).title("Graphs");
//     f.render_widget(block, area);
//
//     let label = format!("{:.2}%", app.progress * 100.0);
//     let gauge = Gauge::default()
//         .block(Block::default().title("Gauge:"))
//         .gauge_style(
//             Style::default()
//                 .fg(Color::Magenta)
//                 .bg(Color::Black)
//                 .add_modifier(Modifier::ITALIC | Modifier::BOLD),
//         )
//         .label(label)
//         .ratio(app.progress);
//     f.render_widget(gauge, chunks[0]);
//
//     let sparkline = Sparkline::default()
//         .block(Block::default().title("Sparkline:"))
//         .style(Style::default().fg(Color::Green))
//         .data(&app.sparkline.points)
//         .bar_set(if app.enhanced_graphics {
//             symbols::bar::NINE_LEVELS
//         } else {
//             symbols::bar::THREE_LEVELS
//         });
//     f.render_widget(sparkline, chunks[1]);
//
//     let line_gauge = LineGauge::default()
//         .block(Block::default().title("LineGauge:"))
//         .gauge_style(Style::default().fg(Color::Magenta))
//         .line_set(if app.enhanced_graphics {
//             symbols::line::THICK
//         } else {
//             symbols::line::NORMAL
//         })
//         .ratio(app.progress);
//     f.render_widget(line_gauge, chunks[2]);
// }

fn draw_panes<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(25), Constraint::Percentage(25)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    let subchunks = Layout::default()
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .direction(Direction::Vertical)
        .split(chunks[0]);

    // Draw word frontier -- TODO: Highlight words based on how known they are
    {
        // It seems dumb to be creating a new Vec here each render.
        let word_frontier_list_item_v: Vec<ListItem> =
            app.word_frontier
                .items
                .iter()
                .map(|word_frontier_member| {
//                     ListItem::new(vec![Spans::from(Span::raw(&word_frontier_member.text))])
                    ListItem::new(vec![Spans::from(Span::raw(format!("{} : {}", word_frontier_member.text, word_frontier_member.unknown_word_count)))])
//                     ListItem::new(vec![Spans::from(Span::raw(format!("{} : {} : (sentences_rowid: {})", word_frontier_member.text, word_frontier_member.unknown_word_count, word_frontier_member.sentences_rowid)))])
                })
                .collect();
        let word_frontier_size = word_frontier_list_item_v.len();
        let word_frontier_list = List::new(word_frontier_list_item_v)
            .block(Block::default().borders(Borders::ALL).title(format!(" Word Frontier ({} Sentences) ", word_frontier_size)))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("-> ");
        f.render_stateful_widget(word_frontier_list, subchunks[0], &mut app.word_frontier.state);
    }

    // Draw translations -- TODO: Highlight words based on how known they are
    {
        // It seems dumb to be creating a new Vec here each render.
        let translation_list_item_v: Vec<ListItem> =
            app.translations
                .items
                .iter()
                .map(|translation_with_text| {
                    ListItem::new(vec![Spans::from(Span::raw(&translation_with_text.reference_lang_sentence_text))])
                })
                .collect();
        let translation_count = translation_list_item_v.len();
        let translation_list = List::new(translation_list_item_v)
            .block(Block::default().borders(Borders::ALL).title(format!(" Possible Translations ({}) ", translation_count)))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("-> ");
        f.render_stateful_widget(translation_list, subchunks[1], &mut app.translations.state);
    }

    // Draw sentence memberships -- TODO: Highlight words based on how known they are
    {
        // It seems dumb to be creating a new Vec here each render.
        let sentence_membership_list_item_v: Vec<ListItem> =
            app.sentence_memberships
                .items
                .iter()
                .map(|sentence_membership_with_text_etc| {
                    ListItem::new(vec![Spans::from(Span::raw(
                        format!(
                            "{} : {} : {}",
                            sentence_membership_with_text_etc.word_text,
                            sentence_membership_with_text_etc.word_freq,
                            if sentence_membership_with_text_etc.word_is_known { "Known" } else { "Unknown" },
                        )
                    ))])
                })
                .collect();
        let sentence_membership_count = sentence_membership_list_item_v.len();
        let sentence_membership_list = List::new(sentence_membership_list_item_v)
            .block(Block::default().borders(Borders::ALL).title(format!(" Sentence Words ({}) ", sentence_membership_count)))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("-> ");
        f.render_stateful_widget(sentence_membership_list, chunks[1], &mut app.sentence_memberships.state);
    }

    // Draw known words
    {
        // It seems dumb to be creating a new Vec here each render.
        let known_word_list_item_v: Vec<ListItem> =
            app.known_words
                .items
                .iter()
                .map(|known_word_with_text| {
                    ListItem::new(vec![Spans::from(Span::raw(&known_word_with_text.word_text))])
                })
                .collect();
        let known_word_count = known_word_list_item_v.len();
        let known_word_list = List::new(known_word_list_item_v)
            .block(Block::default().borders(Borders::ALL).title(format!(" Known Words ({}) ", known_word_count)))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("-> ");
        f.render_stateful_widget(known_word_list, chunks[2], &mut app.known_words.state);
    }
}

// fn draw_text<B>(f: &mut Frame<B>, area: Rect)
// where
//     B: Backend,
// {
//     let text = vec![
//         Spans::from("This is a paragraph with several lines. You can change style your text the way you want"),
//         Spans::from(""),
//         Spans::from(vec![
//             Span::from("For example: "),
//             Span::styled("under", Style::default().fg(Color::Red)),
//             Span::raw(" "),
//             Span::styled("the", Style::default().fg(Color::Green)),
//             Span::raw(" "),
//             Span::styled("rainbow", Style::default().fg(Color::Blue)),
//             Span::raw("."),
//         ]),
//         Spans::from(vec![
//             Span::raw("Oh and if you didn't "),
//             Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
//             Span::raw(" you can "),
//             Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
//             Span::raw(" "),
//             Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
//             Span::raw(" your "),
//             Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
//             Span::raw(".")
//         ]),
//         Spans::from(
//             "One more thing is that it should display unicode characters: 10€"
//         ),
//     ];
//     let block = Block::default().borders(Borders::ALL).title(Span::styled(
//         "Footer",
//         Style::default()
//             .fg(Color::Magenta)
//             .add_modifier(Modifier::BOLD),
//     ));
//     let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
//     f.render_widget(paragraph, area);
// }

fn draw_second_tab<B>(_f: &mut Frame<B>, _app: &mut App, _area: Rect)
where
    B: Backend,
{
//     let chunks = Layout::default()
//         .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
//         .direction(Direction::Horizontal)
//         .split(area);
//     let up_style = Style::default().fg(Color::Green);
//     let failure_style = Style::default()
//         .fg(Color::Red)
//         .add_modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT);
//     let rows = app.servers.iter().map(|s| {
//         let style = if s.status == "Up" {
//             up_style
//         } else {
//             failure_style
//         };
//         Row::new(vec![s.name, s.location, s.status]).style(style)
//     });
//     let table = Table::new(rows)
//         .header(
//             Row::new(vec!["Server", "Location", "Status"])
//                 .style(Style::default().fg(Color::Yellow))
//                 .bottom_margin(1),
//         )
//         .block(Block::default().title("Servers").borders(Borders::ALL))
//         .widths(&[
//             Constraint::Length(15),
//             Constraint::Length(15),
//             Constraint::Length(10),
//         ]);
//     f.render_widget(table, chunks[0]);
//
//     let map = Canvas::default()
//         .block(Block::default().title("World").borders(Borders::ALL))
//         .paint(|ctx| {
//             ctx.draw(&Map {
//                 color: Color::White,
//                 resolution: MapResolution::High,
//             });
//             ctx.layer();
//             ctx.draw(&Rectangle {
//                 x: 0.0,
//                 y: 30.0,
//                 width: 10.0,
//                 height: 10.0,
//                 color: Color::Yellow,
//             });
//             for (i, s1) in app.servers.iter().enumerate() {
//                 for s2 in &app.servers[i + 1..] {
//                     ctx.draw(&Line {
//                         x1: s1.coords.1,
//                         y1: s1.coords.0,
//                         y2: s2.coords.0,
//                         x2: s2.coords.1,
//                         color: Color::Yellow,
//                     });
//                 }
//             }
//             for server in &app.servers {
//                 let color = if server.status == "Up" {
//                     Color::Green
//                 } else {
//                     Color::Red
//                 };
//                 ctx.print(server.coords.1, server.coords.0, "X", color);
//             }
//         })
//         .marker(if app.enhanced_graphics {
//             symbols::Marker::Braille
//         } else {
//             symbols::Marker::Dot
//         })
//         .x_bounds([-180.0, 180.0])
//         .y_bounds([-90.0, 90.0]);
//     f.render_widget(map, chunks[1]);
}

fn draw_third_tab<B>(_f: &mut Frame<B>, _app: &mut App, _area: Rect)
where
    B: Backend,
{
//     let chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
//         .split(area);
//     let colors = [
//         Color::Reset,
//         Color::Black,
//         Color::Red,
//         Color::Green,
//         Color::Yellow,
//         Color::Blue,
//         Color::Magenta,
//         Color::Cyan,
//         Color::Gray,
//         Color::DarkGray,
//         Color::LightRed,
//         Color::LightGreen,
//         Color::LightYellow,
//         Color::LightBlue,
//         Color::LightMagenta,
//         Color::LightCyan,
//         Color::White,
//     ];
//     let items: Vec<Row> = colors
//         .iter()
//         .map(|c| {
//             let cells = vec![
//                 Cell::from(Span::raw(format!("{:?}: ", c))),
//                 Cell::from(Span::styled("Foreground", Style::default().fg(*c))),
//                 Cell::from(Span::styled("Background", Style::default().bg(*c))),
//             ];
//             Row::new(cells)
//         })
//         .collect();
//     let table = Table::new(items)
//         .block(Block::default().title("Colors").borders(Borders::ALL))
//         .widths(&[
//             Constraint::Ratio(1, 3),
//             Constraint::Ratio(1, 3),
//             Constraint::Ratio(1, 3),
//         ]);
//     f.render_widget(table, chunks[0]);
}
