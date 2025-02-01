use gpui::App;

pub(crate) fn init(app: &mut App) {
  app.text_system().add_fonts(vec![
    app
      .asset_source()
      .load("fonts/montserrat/regular.ttf")
      .unwrap()
      .unwrap(),
    app
      .asset_source()
      .load("fonts/montserrat/bold.ttf")
      .unwrap()
      .unwrap(),
    app
      .asset_source()
      .load("fonts/montserrat/italic.ttf")
      .unwrap()
      .unwrap()
  ]).unwrap();
}