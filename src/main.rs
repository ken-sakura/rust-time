mod app;
mod font;
mod theme;
mod ui;

use std::io::{self, stdout, Stdout};
use std::time::Duration;

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use app::App;
use font::FontSize;

/// ターミナルを自動で通常モードに復元するガード構造体
struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, Show);
    }
}

fn main() -> io::Result<()> {
    // パニック時にもターミナルを正常復元するパニックフックを設定
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, Show);
        default_hook(panic_info);
    }));

    // ターミナルの初期化
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    let _guard = TerminalGuard;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<()> {
    // 約50msごとにポーリングして描画更新（コロンの点滅や秒の進みを滑らかにする）
    let tick_rate = Duration::from_millis(50);

    while !app.should_quit {
        terminal.draw(|f| ui::render(f, app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                // キー押下時のみ処理（Releaseイベントは無視）
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Ctrl+C チェック
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    app.should_quit = true;
                    break;
                }

                match key.code {
                    KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    KeyCode::Esc => {
                        if app.show_help {
                            app.show_help = false;
                        } else {
                            app.should_quit = true;
                        }
                    }
                    KeyCode::Char('?') | KeyCode::Char('h') => {
                        app.toggle_help();
                    }
                    KeyCode::Char('c') | KeyCode::Char('t') => {
                        app.next_theme();
                    }
                    KeyCode::Char('s') => {
                        app.toggle_seconds();
                    }
                    KeyCode::Char('d') => {
                        app.toggle_date();
                    }
                    KeyCode::Char('b') => {
                        app.toggle_blink();
                    }
                    KeyCode::Char('w') => {
                        app.toggle_border();
                    }
                    KeyCode::Char('0') | KeyCode::Char('a') => {
                        app.set_font(None);
                    }
                    KeyCode::Char('1') => {
                        app.set_font(Some(FontSize::ExtraLarge));
                    }
                    KeyCode::Char('2') => {
                        app.set_font(Some(FontSize::Large));
                    }
                    KeyCode::Char('3') => {
                        app.set_font(Some(FontSize::Medium));
                    }
                    KeyCode::Char('4') => {
                        app.set_font(Some(FontSize::Small));
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
