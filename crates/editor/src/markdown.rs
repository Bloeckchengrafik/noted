mod skipmap;
mod inline;

use crate::markdown::inline::preshape_inline;
use components::ActiveTheme;
use gpui::{App, Pixels, SharedString, Window, WrappedLine};
use smallvec::SmallVec;
use std::ops::Range;

pub(crate) fn intersects(a: &Range<usize>, b: &Range<usize>) -> bool {
  a.start < b.end && a.end > b.start
}

fn shape_line(line: &str, line_range: Range<usize>, wrap_width: Pixels, selection: Range<usize>, window: &mut Window, cx: &mut App) -> SmallVec<[WrappedLine; 1]> {
  let relative_selection = (selection.start as i32 - line_range.start as i32)..(selection.end as i32 - line_range.start as i32);
  let text_style = window.text_style();
  let font_size = text_style.font_size.to_pixels(window.rem_size());
  let pre = preshape_inline(line, &text_style, &(cx.theme().foreground.clone()), relative_selection, window, cx);
  pre.shape(window, wrap_width, font_size)
}

pub(crate) fn shape_markdown(raw: SharedString, wrap_width: Pixels, selection: Range<usize>, window: &mut Window, cx: &mut App) -> SmallVec<[WrappedLine; 1]> {
  let mut index = 0;
  let mut lines = SmallVec::new();

  while index < raw.len() {
    let start = index;
    let end = raw[index..].find('\n').map(|i| index + i).unwrap_or(raw.len());
    let range = start..end;
    let line = &raw[range.clone()];
    let line = if intersects(&range, &selection) {
      shape_line(line, range, wrap_width, selection.clone(), window, cx) // TODO shape with selection support selection
    } else {
      shape_line(line, range, wrap_width, selection.clone(), window, cx)
    };
    lines.extend(line);
    index = end + 1;
  }

  lines
}