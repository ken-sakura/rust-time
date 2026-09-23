use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use crate::theme::Theme;

/// フォントサイズ定義
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSize {
    ExtraLarge, // 7x5 ブロック
    Large,      // 5x3 ブロック
    Medium,     // 3x3 ハーフブロック
    Small,      // 1行テキスト
}

impl FontSize {
    pub fn name(&self) -> &'static str {
        match self {
            FontSize::ExtraLarge => "Extra Large (7x5)",
            FontSize::Large => "Large (5x3)",
            FontSize::Medium => "Medium (3x3)",
            FontSize::Small => "Small (1-line)",
        }
    }

    #[allow(dead_code)]
    pub fn height(&self) -> u16 {
        match self {
            FontSize::ExtraLarge => 7,
            FontSize::Large => 5,
            FontSize::Medium => 3,
            FontSize::Small => 1,
        }
    }
}

// -------------------------------------------------------------
// Extra Large (7x5)
// -------------------------------------------------------------
const XL_0: [&str; 7] = [
    "█████",
    "█   █",
    "█   █",
    "█   █",
    "█   █",
    "█   █",
    "█████",
];
const XL_1: [&str; 7] = [
    "  ██ ",
    " ███ ",
    "   █ ",
    "   █ ",
    "   █ ",
    "   █ ",
    "█████",
];
const XL_2: [&str; 7] = [
    "█████",
    "    █",
    "    █",
    "█████",
    "█    ",
    "█    ",
    "█████",
];
const XL_3: [&str; 7] = [
    "█████",
    "    █",
    "    █",
    "█████",
    "    █",
    "    █",
    "█████",
];
const XL_4: [&str; 7] = [
    "█   █",
    "█   █",
    "█   █",
    "█████",
    "    █",
    "    █",
    "    █",
];
const XL_5: [&str; 7] = [
    "█████",
    "█    ",
    "█    ",
    "█████",
    "    █",
    "    █",
    "█████",
];
const XL_6: [&str; 7] = [
    "█████",
    "█    ",
    "█    ",
    "█████",
    "█   █",
    "█   █",
    "█████",
];
const XL_7: [&str; 7] = [
    "█████",
    "    █",
    "    █",
    "    █",
    "    █",
    "    █",
    "    █",
];
const XL_8: [&str; 7] = [
    "█████",
    "█   █",
    "█   █",
    "█████",
    "█   █",
    "█   █",
    "█████",
];
const XL_9: [&str; 7] = [
    "█████",
    "█   █",
    "█   █",
    "█████",
    "    █",
    "    █",
    "█████",
];
const XL_COLON: [&str; 7] = [
    "  ",
    "██",
    "██",
    "  ",
    "██",
    "██",
    "  ",
];
const XL_SPACE: [&str; 7] = [
    "  ",
    "  ",
    "  ",
    "  ",
    "  ",
    "  ",
    "  ",
];

// -------------------------------------------------------------
// Large (5x3)
// -------------------------------------------------------------
const L_0: [&str; 5] = [
    "███",
    "█ █",
    "█ █",
    "█ █",
    "███",
];
const L_1: [&str; 5] = [
    " ██",
    "  █",
    "  █",
    "  █",
    "███",
];
const L_2: [&str; 5] = [
    "███",
    "  █",
    "███",
    "█  ",
    "███",
];
const L_3: [&str; 5] = [
    "███",
    "  █",
    "███",
    "  █",
    "███",
];
const L_4: [&str; 5] = [
    "█ █",
    "█ █",
    "███",
    "  █",
    "  █",
];
const L_5: [&str; 5] = [
    "███",
    "█  ",
    "███",
    "  █",
    "███",
];
const L_6: [&str; 5] = [
    "███",
    "█  ",
    "███",
    "█ █",
    "███",
];
const L_7: [&str; 5] = [
    "███",
    "  █",
    "  █",
    "  █",
    "  █",
];
const L_8: [&str; 5] = [
    "███",
    "█ █",
    "███",
    "█ █",
    "███",
];
const L_9: [&str; 5] = [
    "███",
    "█ █",
    "███",
    "  █",
    "███",
];
const L_COLON: [&str; 5] = [
    " ",
    "█",
    " ",
    "█",
    " ",
];
const L_SPACE: [&str; 5] = [
    "  ",
    "  ",
    "  ",
    "  ",
    "  ",
];

// -------------------------------------------------------------
// Medium (3x3) - Unicode Half Blocks
// -------------------------------------------------------------
const M_0: [&str; 3] = [
    "█▀█",
    "█ █",
    "▀▀▀",
];
const M_1: [&str; 3] = [
    " █ ",
    " █ ",
    " ▀ ",
];
const M_2: [&str; 3] = [
    "▀▀█",
    "█▀▀",
    "▀▀▀",
];
const M_3: [&str; 3] = [
    "▀▀█",
    " ▀█",
    "▀▀▀",
];
const M_4: [&str; 3] = [
    "█ █",
    "▀▀█",
    "  ▀",
];
const M_5: [&str; 3] = [
    "█▀▀",
    "▀▀█",
    "▀▀▀",
];
const M_6: [&str; 3] = [
    "█▀▀",
    "█▀█",
    "▀▀▀",
];
const M_7: [&str; 3] = [
    "▀▀█",
    "  █",
    "  ▀",
];
const M_8: [&str; 3] = [
    "█▀█",
    "█▀█",
    "▀▀▀",
];
const M_9: [&str; 3] = [
    "█▀█",
    "▀▀█",
    "▀▀▀",
];
const M_COLON: [&str; 3] = [
    "▄",
    " ",
    "▀",
];
const M_SPACE: [&str; 3] = [
    " ",
    " ",
    " ",
];

fn get_xl_glyph(c: char) -> &'static [&'static str; 7] {
    match c {
        '0' => &XL_0,
        '1' => &XL_1,
        '2' => &XL_2,
        '3' => &XL_3,
        '4' => &XL_4,
        '5' => &XL_5,
        '6' => &XL_6,
        '7' => &XL_7,
        '8' => &XL_8,
        '9' => &XL_9,
        ':' => &XL_COLON,
        _ => &XL_SPACE,
    }
}

fn get_l_glyph(c: char) -> &'static [&'static str; 5] {
    match c {
        '0' => &L_0,
        '1' => &L_1,
        '2' => &L_2,
        '3' => &L_3,
        '4' => &L_4,
        '5' => &L_5,
        '6' => &L_6,
        '7' => &L_7,
        '8' => &L_8,
        '9' => &L_9,
        ':' => &L_COLON,
        _ => &L_SPACE,
    }
}

fn get_m_glyph(c: char) -> &'static [&'static str; 3] {
    match c {
        '0' => &M_0,
        '1' => &M_1,
        '2' => &M_2,
        '3' => &M_3,
        '4' => &M_4,
        '5' => &M_5,
        '6' => &M_6,
        '7' => &M_7,
        '8' => &M_8,
        '9' => &M_9,
        ':' => &M_COLON,
        _ => &M_SPACE,
    }
}

/// 指定文字・インデックスにおけるスタイル（色）を決定する
fn char_color(theme: Theme, c: char, char_idx: usize, total_chars: usize) -> Color {
    if theme == Theme::Rainbow {
        if c == ':' {
            Color::DarkGray
        } else if total_chars >= 8 {
            // HH:MM:SS
            match char_idx {
                0 | 1 => Color::LightMagenta,
                3 | 4 => Color::LightCyan,
                6 | 7 => Color::LightYellow,
                _ => Color::White,
            }
        } else {
            // HH:MM
            match char_idx {
                0 | 1 => Color::LightMagenta,
                3 | 4 => Color::LightCyan,
                _ => Color::White,
            }
        }
    } else if c == ':' {
        theme.colon_color()
    } else {
        theme.primary_color()
    }
}

/// 文字列を指定のフォントで複数行文字列にレンダリングする
pub fn render_ascii_time(text: &str, size: FontSize) -> Vec<String> {
    match size {
        FontSize::ExtraLarge => {
            let mut lines = vec![String::new(); 7];
            for (i, c) in text.chars().enumerate() {
                let glyph = get_xl_glyph(c);
                for row in 0..7 {
                    if i > 0 {
                        lines[row].push(' ');
                    }
                    lines[row].push_str(glyph[row]);
                }
            }
            lines
        }
        FontSize::Large => {
            let mut lines = vec![String::new(); 5];
            for (i, c) in text.chars().enumerate() {
                let glyph = get_l_glyph(c);
                for row in 0..5 {
                    if i > 0 {
                        lines[row].push(' ');
                    }
                    lines[row].push_str(glyph[row]);
                }
            }
            lines
        }
        FontSize::Medium => {
            let mut lines = vec![String::new(); 3];
            for (i, c) in text.chars().enumerate() {
                let glyph = get_m_glyph(c);
                for row in 0..3 {
                    if i > 0 {
                        lines[row].push(' ');
                    }
                    lines[row].push_str(glyph[row]);
                }
            }
            lines
        }
        FontSize::Small => {
            vec![text.to_string()]
        }
    }
}

/// スタイル付きの Ratatui Line リストにレンダリングする
pub fn render_styled_lines<'a>(
    text: &str,
    size: FontSize,
    theme: Theme,
) -> Vec<Line<'a>> {
    let char_count = text.chars().count();
    match size {
        FontSize::ExtraLarge => {
            let mut row_spans: Vec<Vec<Span<'a>>> = (0..7).map(|_| Vec::new()).collect();
            for (i, c) in text.chars().enumerate() {
                let color = char_color(theme, c, i, char_count);
                let style = Style::default().fg(color);
                let glyph = get_xl_glyph(c);
                for row in 0..7 {
                    if i > 0 {
                        row_spans[row].push(Span::raw(" "));
                    }
                    row_spans[row].push(Span::styled(glyph[row].to_string(), style));
                }
            }
            row_spans.into_iter().map(Line::from).collect()
        }
        FontSize::Large => {
            let mut row_spans: Vec<Vec<Span<'a>>> = (0..5).map(|_| Vec::new()).collect();
            for (i, c) in text.chars().enumerate() {
                let color = char_color(theme, c, i, char_count);
                let style = Style::default().fg(color);
                let glyph = get_l_glyph(c);
                for row in 0..5 {
                    if i > 0 {
                        row_spans[row].push(Span::raw(" "));
                    }
                    row_spans[row].push(Span::styled(glyph[row].to_string(), style));
                }
            }
            row_spans.into_iter().map(Line::from).collect()
        }
        FontSize::Medium => {
            let mut row_spans: Vec<Vec<Span<'a>>> = (0..3).map(|_| Vec::new()).collect();
            for (i, c) in text.chars().enumerate() {
                let color = char_color(theme, c, i, char_count);
                let style = Style::default().fg(color);
                let glyph = get_m_glyph(c);
                for row in 0..3 {
                    if i > 0 {
                        row_spans[row].push(Span::raw(" "));
                    }
                    row_spans[row].push(Span::styled(glyph[row].to_string(), style));
                }
            }
            row_spans.into_iter().map(Line::from).collect()
        }
        FontSize::Small => {
            let mut spans = Vec::new();
            for (i, c) in text.chars().enumerate() {
                let color = char_color(theme, c, i, char_count);
                let style = Style::default().fg(color);
                spans.push(Span::styled(c.to_string(), style));
            }
            vec![Line::from(spans)]
        }
    }
}

/// 指定文字列を各フォントで描画した際の (横幅, 縦幅) を算出する
pub fn measure_text(text: &str, size: FontSize) -> (u16, u16) {
    let lines = render_ascii_time(text, size);
    let height = lines.len() as u16;
    let width = lines
        .first()
        .map(|line| line.chars().count() as u16)
        .unwrap_or(0);
    (width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizes() {
        let (w_xl, h_xl) = measure_text("12:34:56", FontSize::ExtraLarge);
        assert_eq!(h_xl, 7);
        println!("XL 12:34:56 size: {w_xl}x{h_xl}");

        let (w_xl_short, _) = measure_text("12:34", FontSize::ExtraLarge);
        println!("XL 12:34 size: {}x{}", w_xl_short, 7);

        let (w_l, h_l) = measure_text("12:34:56", FontSize::Large);
        assert_eq!(h_l, 5);
        println!("L 12:34:56 size: {w_l}x{h_l}");

        let (w_l_short, _) = measure_text("12:34", FontSize::Large);
        println!("L 12:34 size: {}x{}", w_l_short, 5);

        let (w_m, h_m) = measure_text("12:34:56", FontSize::Medium);
        assert_eq!(h_m, 3);
        println!("M 12:34:56 size: {w_m}x{h_m}");

        let (w_m_short, _) = measure_text("12:34", FontSize::Medium);
        println!("M 12:34 size: {}x{}", w_m_short, 3);
    }
}
