use components::{ActiveTheme, Root, TitleBar};
use editor::editor::Editor;
use gpui::{div, img, px, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window};

pub struct RootView {
  editor: Entity<Editor>,
}

impl RootView {
  pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
    Self {
      editor: cx.new(|cx| Editor::new(window, cx)),
    }
  }
}

impl Render for RootView {
  fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
    let theme = cx.theme().clone();
    let notification_layer = Root::render_notification_layer(window, cx);

    let title_bar = TitleBar::new()
      .child(
        div()
          .flex()
          .gap_1()
          .w_20()
          .child(
            img("brand/icon.svg")
              .size_6()
          )
          .child("Glyph")
      );

    div()
      .h_full()
      .bg(theme.background)
      .text_color(theme.foreground)
      .font_family("Montserrat")
      .child(title_bar)
      .child(
        div()
          .flex()
          .justify_center()
          .h_full()
          .py_16()
          .px_16()
          .child(
            div()
              .max_w(px(1000f32))
              .flex_1()
              .h_full()
              .child(self.editor.clone())
          )
      )
      .child(div().absolute().top_8().children(notification_layer).debug())
  }
}