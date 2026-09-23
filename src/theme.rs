use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Cyan,
    Green,
    Yellow,
    Magenta,
    Blue,
    Red,
    White,
    Rainbow,
}

impl Theme {
    pub const ALL: [Theme; 8] = [
        Theme::Cyan,
        Theme::Green,
        Theme::Yellow,
        Theme::Magenta,
        Theme::Blue,
        Theme::Red,
        Theme::White,
        Theme::Rainbow,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Theme::Cyan => "Cyber Cyan",
            Theme::Green => "Matrix Green",
            Theme::Yellow => "Amber Gold",
            Theme::Magenta => "Neon Magenta",
            Theme::Blue => "Cool Blue",
            Theme::Red => "Vivid Red",
            Theme::White => "Pure White",
            Theme::Rainbow => "Rainbow Spectrum",
        }
    }

    pub fn primary_color(&self) -> Color {
        match self {
            Theme::Cyan => Color::Cyan,
            Theme::Green => Color::Green,
            Theme::Yellow => Color::Yellow,
            Theme::Magenta => Color::Magenta,
            Theme::Blue => Color::LightBlue,
            Theme::Red => Color::LightRed,
            Theme::White => Color::White,
            Theme::Rainbow => Color::Cyan,
        }
    }

    pub fn colon_color(&self) -> Color {
        match self {
            Theme::Cyan => Color::LightCyan,
            Theme::Green => Color::LightGreen,
            Theme::Yellow => Color::LightYellow,
            Theme::Magenta => Color::LightMagenta,
            Theme::Blue => Color::Cyan,
            Theme::Red => Color::Yellow,
            Theme::White => Color::Gray,
            Theme::Rainbow => Color::White,
        }
    }

    pub fn date_color(&self) -> Color {
        match self {
            Theme::Cyan => Color::LightBlue,
            Theme::Green => Color::LightGreen,
            Theme::Yellow => Color::LightYellow,
            Theme::Magenta => Color::LightMagenta,
            Theme::Blue => Color::Cyan,
            Theme::Red => Color::Yellow,
            Theme::White => Color::Gray,
            Theme::Rainbow => Color::LightCyan,
        }
    }

    pub fn border_color(&self) -> Color {
        Color::DarkGray
    }

    pub fn next(&self) -> Self {
        let idx = Self::ALL.iter().position(|t| t == self).unwrap_or(0);
        Self::ALL[(idx + 1) % Self::ALL.len()]
    }
}
