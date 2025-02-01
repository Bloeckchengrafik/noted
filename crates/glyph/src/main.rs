mod assets;
mod ui;
mod fonts;

use components::{hsl, Root, Theme, ThemeColor, ThemeMode};
use gpui::{AnyView, App, AppContext, Timer, TitlebarOptions, WindowOptions};
use log::info;
use crate::ui::RootView;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEV: bool = cfg!(debug_assertions);

fn init_logging() {
  pretty_env_logger::formatted_builder()
    .format_timestamp(None)
    .filter_level(log::LevelFilter::Info)
    .filter_module("gpui", log::LevelFilter::Warn)
    .filter_module("blade_graphics", log::LevelFilter::Warn)
    .filter_module("naga", log::LevelFilter::Warn)
    .init();
}

fn init_theme(cx: &mut App) {
  let mut theme = Theme::from(ThemeColor::dark());
  theme.mode = ThemeMode::Dark;
  theme.accent = hsl(231.0, 97.0, 72.0);
  theme.title_bar = hsl(248.0, 12.0, 8.0);
  theme.background = hsl(248.0, 12.0, 8.0);

  cx.set_global(theme);
  cx.refresh_windows();
}

fn main() {
  init_logging();

  info!("Welcome to Glyph");

  gpui::Application::new()
    .with_assets(assets::Assets)
    .run(
      |cx| {
        components::init(cx);
        init_theme(cx);
        editor::init(cx);
        fonts::init(cx);

        cx.open_window(
          WindowOptions {
            titlebar: Some(TitlebarOptions {
              title: Some(format!("Glyph Notes {}{}", VERSION, if DEV { "-dev" } else { "" }).into()),
              ..Default::default()
            }),
            ..Default::default()
          },
          |window, cx| cx.new(|cx| {
            window.spawn(cx, |mut wnd| async move {
              Timer::after(std::time::Duration::from_millis(100)).await;
              let _ = wnd.update(|window, cx| {
                settings::init(window, cx);
              });
            }).detach();

            let root = cx.new(|cx| RootView::new(window, cx));
            Root::new(AnyView::from(root), window, cx)
          }),
        )
          .expect("failed to open window");
      });
}
