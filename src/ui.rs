use chrono::Local;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{App, RenderPlan};
use crate::font::render_styled_lines;

pub fn render(frame: &mut Frame, app: &App) {
    let now = Local::now();
    let area = frame.area();

    // 画面サイズが0なら何もしない
    if area.width == 0 || area.height == 0 {
        return;
    }

    let plan = app.plan_layout(area, &now);

    // 1. フッター領域とメイン描画領域の分割
    let (main_area, footer_area) = if plan.show_footer {
        let footer = Rect {
            x: area.x,
            y: area.y + area.height.saturating_sub(1),
            width: area.width,
            height: 1,
        };
        let main = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: area.height.saturating_sub(1),
        };
        (main, Some(footer))
    } else {
        (area, None)
    };

    // 2. メイン領域の内側領域の決定（枠線ありなら小さく）
    let inner_area = if plan.show_border {
        let mode_label = if let Some(font) = app.forced_font {
            format!("Size: Manual ({})", font.name())
        } else {
            format!("Size: Auto ({})", plan.font_size.name())
        };

        let title = if main_area.width >= 65 {
            format!(" rust-time │ {} │ {} ", app.theme.name(), mode_label)
        } else if main_area.width >= 40 {
            format!(" rust-time │ {} ", app.theme.name())
        } else if main_area.width >= 20 {
            " rust-time ".to_string()
        } else {
            String::new()
        };

        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(app.theme.border_color()));

        if !title.is_empty() {
            block = block
                .title(Span::styled(
                    title,
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
                ))
                .title_alignment(Alignment::Center);
        }

        let inner = block.inner(main_area);
        frame.render_widget(block, main_area);
        inner
    } else {
        main_area
    };

    // 3. 中央配置で時計と日付を描画
    render_clock_and_date(frame, app, &plan, inner_area);

    // 4. フッターの描画
    if let Some(footer) = footer_area {
        render_footer(frame, app, footer);
    }

    // 5. ヘルプモーダルの描画
    if app.show_help {
        render_help_modal(frame, app, area);
    }
}

/// 時計と日付を画面中央にレンダリングする
fn render_clock_and_date(frame: &mut Frame, app: &App, plan: &RenderPlan, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    // 時計のスタイル付き行を生成
    let clock_lines = render_styled_lines(&plan.time_str, plan.font_size, app.theme);
    let mut all_lines = clock_lines;

    // 日付があれば下に追加
    if let Some(ref date) = plan.date_str {
        // 時計と日付の間に余白行（高さに余裕がある場合）
        let total_needed = plan.clock_height + 2;
        if area.height >= total_needed {
            all_lines.push(Line::raw(""));
        }
        all_lines.push(Line::styled(
            date.as_str(),
            Style::default()
                .fg(app.theme.date_color())
                .add_modifier(Modifier::BOLD),
        ));
    }

    let content_height = all_lines.len() as u16;

    // 垂直方向のパディングを計算して中央揃え
    let pad_top = area.height.saturating_sub(content_height) / 2;
    let actual_h = content_height.min(area.height);

    let render_rect = Rect {
        x: area.x,
        y: area.y + pad_top,
        width: area.width,
        height: actual_h,
    };

    let paragraph = Paragraph::new(all_lines).alignment(Alignment::Center);
    frame.render_widget(paragraph, render_rect);
}

/// 最下部のステータス＆キーガイド描画
fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let size_label = match app.forced_font {
        None => "Auto",
        Some(crate::font::FontSize::ExtraLarge) => "XL",
        Some(crate::font::FontSize::Large) => "L",
        Some(crate::font::FontSize::Medium) => "M",
        Some(crate::font::FontSize::Small) => "S",
    };

    let spans = if area.width >= 75 {
        vec![
            Span::styled("[q]", Style::default().fg(Color::Yellow)),
            Span::raw("Quit "),
            Span::styled("[c]", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Theme:{} ", app.theme.name())),
            Span::styled("[s]", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Sec:{} ", app.seconds_mode.label())),
            Span::styled("[d]", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Date:{} ", app.date_mode.label())),
            Span::styled("[b]", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Blink:{} ", if app.blink_colon { "On" } else { "Off" })),
            Span::styled("[1-4,0]", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Size:{size_label} ")),
            Span::styled("[?]", Style::default().fg(Color::Yellow)),
            Span::raw("Help"),
        ]
    } else if area.width >= 50 {
        vec![
            Span::styled("[q]", Style::default().fg(Color::Yellow)),
            Span::raw("Quit "),
            Span::styled("[c]", Style::default().fg(Color::Yellow)),
            Span::raw("Color "),
            Span::styled("[s]", Style::default().fg(Color::Yellow)),
            Span::raw("Sec "),
            Span::styled("[d]", Style::default().fg(Color::Yellow)),
            Span::raw("Date "),
            Span::styled("[b]", Style::default().fg(Color::Yellow)),
            Span::raw("Blink "),
            Span::styled("[1-4]", Style::default().fg(Color::Yellow)),
            Span::raw("Size "),
            Span::styled("[?]", Style::default().fg(Color::Yellow)),
            Span::raw("Help"),
        ]
    } else {
        vec![
            Span::styled("[q]", Style::default().fg(Color::Yellow)),
            Span::raw("Quit "),
            Span::styled("[c]", Style::default().fg(Color::Yellow)),
            Span::raw("Color "),
            Span::styled("[?]", Style::default().fg(Color::Yellow)),
            Span::raw("Help"),
        ]
    };

    let footer_line = Line::from(spans);
    let paragraph = Paragraph::new(footer_line)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(paragraph, area);
}

/// ヘルプモーダルポップアップ描画
fn render_help_modal(frame: &mut Frame, app: &App, area: Rect) {
    let width = 54.min(area.width.saturating_sub(4));
    let height = 18.min(area.height.saturating_sub(2));

    if width < 20 || height < 6 {
        return; // 小さすぎる画面ではモーダルを描画しない
    }

    let popup_rect = Rect {
        x: area.x + (area.width.saturating_sub(width)) / 2,
        y: area.y + (area.height.saturating_sub(height)) / 2,
        width,
        height,
    };

    frame.render_widget(Clear, popup_rect);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(app.theme.primary_color()))
        .title(" Controls & Shortcuts ")
        .title_alignment(Alignment::Center);

    let help_lines = vec![
        Line::raw(""),
        Line::from(vec![
            Span::styled("  q / Esc / Ctrl+C  ", Style::default().fg(Color::Yellow)),
            Span::raw("Quit application"),
        ]),
        Line::from(vec![
            Span::styled("  c / t             ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Next color theme ({})", app.theme.name())),
        ]),
        Line::from(vec![
            Span::styled("  s                 ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Toggle seconds ({})", app.seconds_mode.label())),
        ]),
        Line::from(vec![
            Span::styled("  d                 ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Toggle date ({})", app.date_mode.label())),
        ]),
        Line::from(vec![
            Span::styled("  b                 ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Toggle blinking colon ({})", if app.blink_colon { "On" } else { "Off" })),
        ]),
        Line::from(vec![
            Span::styled("  w                 ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Toggle border window ({})", app.border_mode.label())),
        ]),
        Line::from(vec![
            Span::styled("  0 / a             ", Style::default().fg(Color::Yellow)),
            Span::raw("Auto responsive size mode"),
        ]),
        Line::from(vec![
            Span::styled("  1 / 2 / 3 / 4     ", Style::default().fg(Color::Yellow)),
            Span::raw("Force size (XL / Large / Med / Small)"),
        ]),
        Line::from(vec![
            Span::styled("  ? / h             ", Style::default().fg(Color::Yellow)),
            Span::raw("Toggle this help popup"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("  Press [?] or [Esc] to close", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let paragraph = Paragraph::new(help_lines).block(block);
    frame.render_widget(paragraph, popup_rect);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn print_buffer(buffer: &ratatui::buffer::Buffer) {
        let area = buffer.area;
        for y in area.top()..area.bottom() {
            let mut line = String::new();
            for x in area.left()..area.right() {
                line.push_str(buffer[(x, y)].symbol());
            }
            println!("{line}");
        }
    }

    #[test]
    fn test_render_large() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = App::new();

        terminal.draw(|f| render(f, &app)).unwrap();
        let buffer = terminal.backend().buffer();
        println!("--- 80x24 Render ---");
        print_buffer(buffer);
    }

    #[test]
    fn test_render_medium() {
        let backend = TestBackend::new(40, 12);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = App::new();

        terminal.draw(|f| render(f, &app)).unwrap();
        let buffer = terminal.backend().buffer();
        println!("--- 40x12 Render ---");
        print_buffer(buffer);
    }

    #[test]
    fn test_render_narrow() {
        let backend = TestBackend::new(25, 6);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = App::new();

        terminal.draw(|f| render(f, &app)).unwrap();
        let buffer = terminal.backend().buffer();
        println!("--- 25x6 Render ---");
        print_buffer(buffer);
    }

    #[test]
    fn test_render_tiny() {
        let backend = TestBackend::new(10, 2);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = App::new();

        terminal.draw(|f| render(f, &app)).unwrap();
        let buffer = terminal.backend().buffer();
        println!("--- 10x2 Render ---");
        print_buffer(buffer);
    }

    #[test]
    fn test_render_zero_size_no_panic() {
        let backend = TestBackend::new(0, 0);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = App::new();

        // ゼロサイズでもpanicしない
        terminal.draw(|f| render(f, &app)).unwrap();
    }
}
