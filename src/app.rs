use chrono::{DateTime, Datelike, Local, Timelike};
use ratatui::layout::Rect;
use crate::font::{measure_text, FontSize};
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleSetting {
    Auto,
    AlwaysOn,
    AlwaysOff,
}

impl ToggleSetting {
    pub fn next(&self) -> Self {
        match self {
            ToggleSetting::Auto => ToggleSetting::AlwaysOn,
            ToggleSetting::AlwaysOn => ToggleSetting::AlwaysOff,
            ToggleSetting::AlwaysOff => ToggleSetting::Auto,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ToggleSetting::Auto => "Auto",
            ToggleSetting::AlwaysOn => "On",
            ToggleSetting::AlwaysOff => "Off",
        }
    }
}

pub struct App {
    pub theme: Theme,
    pub blink_colon: bool,
    pub seconds_mode: ToggleSetting,
    pub date_mode: ToggleSetting,
    pub forced_font: Option<FontSize>, // None = Auto
    pub border_mode: ToggleSetting,
    pub show_help: bool,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            theme: Theme::Cyan,
            blink_colon: true,
            seconds_mode: ToggleSetting::Auto,
            date_mode: ToggleSetting::Auto,
            forced_font: None,
            border_mode: ToggleSetting::Auto,
            show_help: false,
            should_quit: false,
        }
    }
}

/// 描画計画（レスポンシブに計算された各要素の状態）
#[derive(Debug, Clone)]
pub struct RenderPlan {
    pub font_size: FontSize,
    pub time_str: String,
    pub date_str: Option<String>,
    pub show_border: bool,
    pub show_footer: bool,
    #[allow(dead_code)]
    pub clock_width: u16,
    pub clock_height: u16,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_theme(&mut self) {
        self.theme = self.theme.next();
    }

    pub fn toggle_seconds(&mut self) {
        self.seconds_mode = self.seconds_mode.next();
    }

    pub fn toggle_date(&mut self) {
        self.date_mode = self.date_mode.next();
    }

    pub fn toggle_blink(&mut self) {
        self.blink_colon = !self.blink_colon;
    }

    pub fn toggle_border(&mut self) {
        self.border_mode = self.border_mode.next();
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn set_font(&mut self, font: Option<FontSize>) {
        self.forced_font = font;
    }

    /// 画面サイズと現在時刻から最適なレイアウトを算出する
    pub fn plan_layout(&self, area: Rect, now: &DateTime<Local>) -> RenderPlan {
        let raw_w = area.width;
        let raw_h = area.height;

        // 1. 枠線の判定
        let show_border = match self.border_mode {
            ToggleSetting::AlwaysOn => true,
            ToggleSetting::AlwaysOff => false,
            ToggleSetting::Auto => raw_w >= 36 && raw_h >= 9,
        };

        // 2. フッター（操作案内）の判定
        let show_footer = raw_h >= 14 && raw_w >= 40 && !self.show_help;

        // 内側の利用可能領域
        let mut avail_w = raw_w;
        let mut avail_h = raw_h;
        if show_border {
            avail_w = avail_w.saturating_sub(2);
            avail_h = avail_h.saturating_sub(2);
        }
        if show_footer {
            avail_h = avail_h.saturating_sub(1);
        }

        // 時刻文字列のフォーマット準備
        let hour = now.hour();
        let min = now.minute();
        let sec = now.second();

        // コロンの点滅処理（blink_colonが有効で後半500msなら空白に置換）
        let is_blink_off = self.blink_colon && (now.timestamp_subsec_millis() >= 500);
        let colon = if is_blink_off { " " } else { ":" };

        let time_full = format!("{hour:02}{colon}{min:02}{colon}{sec:02}");
        let time_short = format!("{hour:02}{colon}{min:02}");

        // 3. フォントと秒の決定
        let (font_size, with_seconds) = self.resolve_font_and_seconds(avail_w, avail_h);

        let time_str = if with_seconds {
            time_full
        } else {
            time_short
        };

        let (clock_width, clock_height) = measure_text(&time_str, font_size);

        // 4. 日付の決定
        let date_str = self.resolve_date(avail_w, avail_h, clock_height, now);

        RenderPlan {
            font_size,
            time_str,
            date_str,
            show_border,
            show_footer,
            clock_width,
            clock_height,
        }
    }

    /// 利用可能幅と高さから最適なフォントと秒表示を決定
    fn resolve_font_and_seconds(&self, avail_w: u16, avail_h: u16) -> (FontSize, bool) {
        // 秒の強制フラグ
        let allow_sec = self.seconds_mode != ToggleSetting::AlwaysOff;
        let force_sec = self.seconds_mode == ToggleSetting::AlwaysOn;

        // ユーザーが特定フォントを指定している場合
        if let Some(font) = self.forced_font {
            let (w_full, h) = measure_text("00:00:00", font);
            let (w_short, _) = measure_text("00:00", font);

            if avail_h >= h {
                if allow_sec && avail_w >= w_full {
                    return (font, true);
                }
                if (!force_sec || avail_w < w_full) && avail_w >= w_short {
                    return (font, false);
                }
            }
            // 指定フォントが入らない場合は自動判定へフォールバック
        }

        // 自動レスポンシブ判定
        // 優先度: 大フォント秒あり -> 中フォント秒あり -> 秒なしフォント -> 小型
        let candidates = [
            (FontSize::ExtraLarge, true),
            (FontSize::Large, true),
            (FontSize::ExtraLarge, false),
            (FontSize::Large, false),
            (FontSize::Medium, true),
            (FontSize::Medium, false),
            (FontSize::Small, true),
            (FontSize::Small, false),
        ];

        for (font, sec) in candidates {
            // 秒の個別設定を尊重
            if sec && !allow_sec {
                continue;
            }
            if !sec && force_sec {
                continue;
            }

            let text = if sec { "00:00:00" } else { "00:00" };
            let (w, h) = measure_text(text, font);

            if avail_w >= w && avail_h >= h {
                return (font, sec);
            }
        }

        // 最小フォールバック: 画面が極小でもSmallで表示
        if force_sec || (allow_sec && avail_w >= 8) {
            (FontSize::Small, true)
        } else {
            (FontSize::Small, false)
        }
    }

    /// 日付文字列の決定
    fn resolve_date(
        &self,
        avail_w: u16,
        avail_h: u16,
        clock_height: u16,
        now: &DateTime<Local>,
    ) -> Option<String> {
        if self.date_mode == ToggleSetting::AlwaysOff {
            return None;
        }

        // 時計の下に日付を配置するための余裕があるか？
        // 最低でも時計の高さ + 2行（間隔1行 + 日付1行）が必要
        if self.date_mode == ToggleSetting::Auto && avail_h < clock_height + 2 {
            return None;
        }

        let weekday = match now.weekday() {
            chrono::Weekday::Mon => "Mon",
            chrono::Weekday::Tue => "Tue",
            chrono::Weekday::Wed => "Wed",
            chrono::Weekday::Thu => "Thu",
            chrono::Weekday::Fri => "Fri",
            chrono::Weekday::Sat => "Sat",
            chrono::Weekday::Sun => "Sun",
        };

        let full_weekday = match now.weekday() {
            chrono::Weekday::Mon => "Monday",
            chrono::Weekday::Tue => "Tuesday",
            chrono::Weekday::Wed => "Wednesday",
            chrono::Weekday::Thu => "Thursday",
            chrono::Weekday::Fri => "Friday",
            chrono::Weekday::Sat => "Saturday",
            chrono::Weekday::Sun => "Sunday",
        };

        let full_date = format!(
            "{:04}-{:02}-{:02} ({})",
            now.year(),
            now.month(),
            now.day(),
            full_weekday
        );
        let short_date = format!(
            "{:04}-{:02}-{:02} {}",
            now.year(),
            now.month(),
            now.day(),
            weekday
        );
        let ymd_date = format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day());
        let md_date = format!("{:02}/{:02}", now.month(), now.day());

        if avail_w >= full_date.chars().count() as u16 + 2 {
            Some(full_date)
        } else if avail_w >= short_date.chars().count() as u16 + 2 {
            Some(short_date)
        } else if avail_w >= ymd_date.chars().count() as u16 + 2 {
            Some(ymd_date)
        } else if avail_w >= md_date.chars().count() as u16 + 2 {
            Some(md_date)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn fixed_time() -> DateTime<Local> {
        Local.with_ymd_and_hms(2026, 9, 23, 14, 30, 45).unwrap()
    }

    #[test]
    fn test_large_screen_auto_layout() {
        let app = App::new();
        let now = fixed_time();
        // 80x24 full terminal
        let plan = app.plan_layout(Rect::new(0, 0, 80, 24), &now);

        assert_eq!(plan.font_size, FontSize::ExtraLarge);
        assert!(plan.show_border);
        assert!(plan.show_footer);
        assert!(plan.date_str.is_some());
        assert_eq!(plan.time_str.len(), 8); // 14:30:45 (or 14 30 45)
    }

    #[test]
    fn test_medium_screen_auto_layout() {
        let app = App::new();
        let now = fixed_time();
        // 40x12 pane
        let plan = app.plan_layout(Rect::new(0, 0, 40, 12), &now);

        assert_eq!(plan.font_size, FontSize::Large);
        assert!(plan.show_border);
        assert_eq!(plan.time_str.len(), 8); // 14:30:45
    }

    #[test]
    fn test_narrow_small_screen_auto_layout() {
        let app = App::new();
        let now = fixed_time();
        // 25x6 narrow pane: Large (no seconds: 17x5) fits, border is disabled
        let plan = app.plan_layout(Rect::new(0, 0, 25, 6), &now);

        assert!(!plan.show_border);
        assert_eq!(plan.font_size, FontSize::Large);
        assert_eq!(plan.time_str.len(), 5); // 14:30
    }

    #[test]
    fn test_compact_height_auto_layout() {
        let app = App::new();
        let now = fixed_time();
        // 25x4 low-height pane: Large (height 5) does not fit, Medium (height 3) fits
        let plan = app.plan_layout(Rect::new(0, 0, 25, 4), &now);

        assert!(!plan.show_border);
        assert_eq!(plan.font_size, FontSize::Medium);
        assert_eq!(plan.time_str.len(), 5); // 14:30
    }

    #[test]
    fn test_tiny_pane_auto_layout() {
        let app = App::new();
        let now = fixed_time();
        // 10x2 tiny status pane
        let plan = app.plan_layout(Rect::new(0, 0, 10, 2), &now);

        assert_eq!(plan.font_size, FontSize::Small);
        assert!(!plan.show_border);
        assert!(!plan.show_footer);
    }

    #[test]
    fn test_forced_font() {
        let mut app = App::new();
        let now = fixed_time();
        app.set_font(Some(FontSize::Medium));

        let plan = app.plan_layout(Rect::new(0, 0, 80, 24), &now);
        assert_eq!(plan.font_size, FontSize::Medium);
    }

    #[test]
    fn test_theme_and_toggle_cycles() {
        let mut app = App::new();
        let init_theme = app.theme;
        app.next_theme();
        assert_ne!(app.theme, init_theme);

        assert_eq!(app.seconds_mode, ToggleSetting::Auto);
        app.toggle_seconds();
        assert_eq!(app.seconds_mode, ToggleSetting::AlwaysOn);
        app.toggle_seconds();
        assert_eq!(app.seconds_mode, ToggleSetting::AlwaysOff);
        app.toggle_seconds();
        assert_eq!(app.seconds_mode, ToggleSetting::Auto);
    }
}
