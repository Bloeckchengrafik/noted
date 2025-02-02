use crate::markdown::skipmap::SkipMap;
use gpui::{px, App, Hsla, Pixels, SharedString, StrikethroughStyle, TextRun, TextStyle, UnderlineStyle, Window, WrappedLine};
use std::ops::Range;
use smallvec::SmallVec;

pub(crate) struct PreshapedInlineText {
  reduced_text: SharedString,
  skip_map: SkipMap,
  runs: Vec<TextRun>,
}

impl PreshapedInlineText {
  pub(crate) fn shape(&self, window: &mut Window, wrap_width: Pixels, font_size: Pixels) -> SmallVec<[WrappedLine; 1]> {
    window
      .text_system()
      .shape_text(self.reduced_text.clone(), font_size, &self.runs, Some(wrap_width), None)
      .unwrap()
  }
}

struct PreshapingEngine {
  runs: Vec<TextRun>,
  style: TextStyle,
  color: Hsla,
  skip_map: SkipMap,
  run_start: usize,
  is_bold: bool,
  is_italic: bool,
  is_strikethrough: bool,
  is_underline: bool,
  is_code: bool,
}

impl PreshapingEngine {
  fn new(text_style: TextStyle, text_color: Hsla) -> Self {
    Self {
      runs: vec![],
      skip_map: SkipMap::new(),
      style: text_style,
      color: text_color,
      run_start: 0,
      is_bold: false,
      is_italic: false,
      is_strikethrough: false,
      is_underline: false,
      is_code: false,
    }
  }

  fn flush_run(&mut self, end: usize) {
    let mut font = self.style.font();

    if self.is_bold {
      font = font.bold();
    }

    if self.is_italic {
      font = font.italic();
    }

    let run = TextRun {
      len: end - self.run_start,
      font,
      color: self.color.clone(),
      background_color: None,
      underline: if self.is_underline {
        Some(UnderlineStyle {
          thickness: px(1.0),
          color: Some(self.color.clone()),
          wavy: false,
        })
      } else { None },
      strikethrough: if self.is_strikethrough {
        Some(StrikethroughStyle {
          thickness: px(1.0),
          color: Some(self.color.clone()),
        })
      } else { None },
    };
    self.runs.push(run);
    self.run_start = end;
  }

  fn toggle_bold(&mut self, end: usize) {
    self.flush_run(end);
    self.is_bold = !self.is_bold;
  }
  fn toggle_italic(&mut self, end: usize) {
    self.flush_run(end);
    self.is_italic = !self.is_italic;
  }
  fn toggle_strikethrough(&mut self, end: usize) {
    self.flush_run(end);
    self.is_strikethrough = !self.is_strikethrough;
  }
  fn toggle_underline(&mut self, end: usize) {
    self.flush_run(end);
    self.is_underline = !self.is_underline;
  }
  fn toggle_code(&mut self, end: usize) {
    self.flush_run(end);
    self.is_code = !self.is_code;
  }
}

pub(crate) fn preshape_inline(line: &str, text_style: &TextStyle, text_color: &Hsla, _sel: Range<i32>, _: &mut Window, _: &mut App) -> PreshapedInlineText {
  let mut engine = PreshapingEngine::new(text_style.clone(), text_color.clone());
  let mut iter = line.char_indices().peekable();
  let mut reduced_text = String::new();
  while let Some((index, char)) = iter.next() {
    match char {
      '_' => {
        if let Some((_, next)) = iter.peek() {
          if *next == '_' {
            engine.toggle_underline(index);
            iter.next();
          } else {
            engine.toggle_italic(index);
          }
        }
      }
      '*' => {
        if let Some((_, next)) = iter.peek() {
          if *next == '*' {
            engine.toggle_bold(index);
            iter.next();
          } else {
            engine.toggle_italic(index);
          }
        }
      }
      '~' => {
        engine.toggle_strikethrough(index);
      }
      '`' => {
        engine.toggle_code(index);
      }
      _ => reduced_text.push(char)
    }
  }

  engine.flush_run(line.len());

  let PreshapingEngine { runs, skip_map, .. } = engine;

  PreshapedInlineText {
    runs,
    skip_map,
    reduced_text: line.to_string().into(),
  }
}