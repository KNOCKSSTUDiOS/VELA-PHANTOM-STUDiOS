use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::{io, time::Duration};

use prompt_engine::PromptCore;
use timeline_engine::TimelineState;
use render_engine::RenderCore;

struct App {
    prompt_core: PromptCore,
    timeline_core: TimelineState,
    render_core: RenderCore,
    input_text: String,
    selected_tab: usize,
    selected_palette: usize,
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App {
        prompt_core: PromptCore::load_local(),
        timeline_core: TimelineState::default(),
        render_core: RenderCore::default(),
        input_text: String::new(),
        selected_tab: 0,
        selected_palette: 3,
    };

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Tab => app.selected_tab = (app.selected_tab + 1) % 5,
                    KeyCode::BackTab => app.selected_tab = (app.selected_tab + 4) % 5,
                    KeyCode::F(1) => app.selected_palette = 0,
                    KeyCode::F(2) => app.selected_palette = 1,
                    KeyCode::F(3) => app.selected_palette = 2,
                    KeyCode::F(4) => app.selected_palette = 3,
                    KeyCode::F(5) => app.selected_palette = 4,
                    KeyCode::Enter => {
                        if app.selected_tab == 0 && !app.input_text.is_empty() {
                            let prompt = app.input_text.clone();
                            app.prompt_core.submit_prompt(&prompt);
                            app.input_text.clear();
                        } else if app.selected_tab == 2 {
                            app.timeline_core.step();
                        } else if app.selected_tab == 3 {
                            app.render_core.trigger_2d();
                        }
                    }
                    KeyCode::Char(c) => {
                        if app.selected_tab == 0 {
                            app.input_text.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if app.selected_tab == 0 {
                            app.input_text.pop();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    let accent_color = match app.selected_palette {
        0 => Color::LightRed,
        1 => Color::Magenta,
        2 => Color::Cyan,
        3 => Color::White,
        4 => Color::Yellow,
        _ => Color::White,
    };

    let tabs_text = vec![
        Span::styled(" [1:Dashboard] ", if app.selected_tab == 0 { Style::default().fg(accent_color).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) }),
        Span::styled(" [2:Gallery] ", if app.selected_tab == 1 { Style::default().fg(accent_color).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) }),
        Span::styled(" [3:Timeline] ", if app.selected_tab == 2 { Style::default().fg(accent_color).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) }),
        Span::styled(" [4:Render] ", if app.selected_tab == 3 { Style::default().fg(accent_color).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) }),
        Span::styled(" [5:Audio] ", if app.selected_tab == 4 { Style::default().fg(accent_color).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) }),
    ];
    let top_bar = Paragraph::new(Line::from(tabs_text))
        .block(Block::default().borders(Borders::ALL).title("⚡ VELA PHANTOM STUDiO [KNOCKSSTUDiOS]"));
    f.render_widget(top_bar, chunks[0]);

    match app.selected_tab {
        0 => {
            let inner = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(5)])
                .split(chunks[1]);

            let prompt_box = Paragraph::new(app.input_text.as_str())
                .style(Style::default().fg(accent_color))
                .block(Block::default().borders(Borders::ALL).title("Prompt Input:"));
            f.render_widget(prompt_box, inner[0]);

            let items: Vec<ListItem> = app.prompt_core.get_history().iter().rev()
                .map(|h| ListItem::new(h.as_str()))
                .collect();
            let history = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Execution Feed"));
            f.render_widget(history, inner[1]);
        }
        1 => {
            let text = vec![
                Line::from("📁 [3D MESH] .obj / .fbx — Indexed & Ready"),
                Line::from("📁 [2D RENDER] .png / .jpg — High Res"),
                Line::from("📁 [AUDIO STREAM] .mp3 / .wav — Spatial"),
                Line::from("📁 [VIDEO SEQUENCE] .mp4 — 4K Stream"),
            ];
            f.render_widget(Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Media Gallery")), chunks[1]);
        }
        2 => {
            let text = vec![
                Line::from(format!("Current Frame Marker: {}", app.timeline_core.get_frame())),
                Line::from(""),
                Line::from("Press [ENTER] to Step Forward Frame."),
            ];
            f.render_widget(Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Timeline Scrubber")), chunks[1]);
        }
        3 => {
            let text = vec![
                Line::from(format!("Render Log: {}", app.render_core.get_status())),
                Line::from(""),
                Line::from("Press [ENTER] to trigger 2D Vector Export."),
            ];
            f.render_widget(Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Render Engine")), chunks[1]);
        }
        4 => {
            let text = vec![
                Line::from("Multi-channel audio telemetry locked at 24-bit 96kHz sovereign output."),
            ];
            f.render_widget(Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Spatial Sound Matrix")), chunks[1]);
        }
        _ => {}
    }

    let footer = Paragraph::new(" [TAB]: Switch | [F1-F5]: Palettes | [ENTER]: Action | [ESC]: Exit ")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
