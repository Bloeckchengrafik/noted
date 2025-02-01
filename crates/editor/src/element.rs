use components::ActiveTheme;
use crate::editor::Editor;
use gpui::{fill, point, px, relative, size, App, Bounds, Corners, Element, ElementId, ElementInputHandler, Entity, GlobalElementId, IntoElement, LayoutId, MouseButton, MouseMoveEvent, PaintQuad, Path, Pixels, Point, Style, TextRun, Window, WrappedLine};
use smallvec::SmallVec;

const RIGHT_MARGIN: Pixels = px(5.);
const BOTTOM_MARGIN: Pixels = px(20.);


pub(crate) struct EditorElement {
  editor: Entity<Editor>,
}

impl EditorElement {
  pub(crate) fn new(editor: Entity<Editor>) -> Self {
    Self { editor }
  }

  fn paint_mouse_listeners(&mut self, window: &mut Window, _: &mut App) {
    window.on_mouse_event({
      let editor = self.editor.clone();

      move |event: &MouseMoveEvent, _, window, cx| {
        if event.pressed_button == Some(MouseButton::Left) {
          editor.update(cx, |input, cx| {
            input.on_drag_move(event, window, cx);
          });
        }
      }
    });
  }

  fn layout_cursor(
    &self,
    lines: &[WrappedLine],
    line_height: Pixels,
    bounds: &mut Bounds<Pixels>,
    window: &mut Window,
    cx: &mut App,
  ) -> (Option<PaintQuad>, Point<Pixels>) {
    let editor = self.editor.read(cx);
    let selected_range = &editor.selected_range;
    let cursor_offset = editor.cursor_offset();
    let mut scroll_offset = editor.scroll_handle.offset();
    let mut cursor = None;

    // The cursor corresponds to the current cursor position in the text no only the line.
    let mut cursor_pos = None;
    let mut cursor_start = None;
    let mut cursor_end = None;

    let mut prev_lines_offset = 0;
    let mut offset_y = px(0.);
    for line in lines.iter() {
      // break loop if all cursor positions are found
      if cursor_pos.is_some() && cursor_start.is_some() && cursor_end.is_some() {
        break;
      }

      let line_origin = point(px(0.), offset_y);
      if cursor_pos.is_none() {
        let offset = cursor_offset.saturating_sub(prev_lines_offset);
        if let Some(pos) = line.position_for_index(offset, line_height) {
          cursor_pos = Some(line_origin + pos);
        }
      }
      if cursor_start.is_none() {
        let offset = selected_range.start.saturating_sub(prev_lines_offset);
        if let Some(pos) = line.position_for_index(offset, line_height) {
          cursor_start = Some(line_origin + pos);
        }
      }
      if cursor_end.is_none() {
        let offset = selected_range.end.saturating_sub(prev_lines_offset);
        if let Some(pos) = line.position_for_index(offset, line_height) {
          cursor_end = Some(line_origin + pos);
        }
      }

      offset_y += line.size(line_height).height;
      // +1 for skip the last `\n`
      prev_lines_offset += line.len() + 1;
    }

    if let (Some(cursor_pos), Some(cursor_start), Some(cursor_end)) =
      (cursor_pos, cursor_start, cursor_end)
    {
      let cursor_moved = editor.last_cursor_offset != Some(cursor_offset);
      let selection_changed = editor.last_selected_range != Some(selected_range.clone());

      if cursor_moved || selection_changed {
        scroll_offset.x =
          if scroll_offset.x + cursor_pos.x > (bounds.size.width - RIGHT_MARGIN) {
            // cursor is out of right
            bounds.size.width - RIGHT_MARGIN - cursor_pos.x
          } else if scroll_offset.x + cursor_pos.x < px(0.) {
            // cursor is out of left
            scroll_offset.x - cursor_pos.x
          } else {
            scroll_offset.x
          };
        scroll_offset.y =
          if scroll_offset.y + cursor_pos.y > (bounds.size.height - BOTTOM_MARGIN) {
            // cursor is out of bottom
            bounds.size.height - BOTTOM_MARGIN - cursor_pos.y
          } else if scroll_offset.y + cursor_pos.y < px(0.) {
            // cursor is out of top
            scroll_offset.y - cursor_pos.y
          } else {
            scroll_offset.y
          };

        if editor.selection_reversed {
          if scroll_offset.x + cursor_start.x < px(0.) {
            // selection start is out of left
            scroll_offset.x = -cursor_start.x;
          }
          if scroll_offset.y + cursor_start.y < px(0.) {
            // selection start is out of top
            scroll_offset.y = -cursor_start.y;
          }
        } else {
          if scroll_offset.x + cursor_end.x <= px(0.) {
            // selection end is out of left
            scroll_offset.x = -cursor_end.x;
          }
          if scroll_offset.y + cursor_end.y <= px(0.) {
            // selection end is out of top
            scroll_offset.y = -cursor_end.y;
          }
        }
      }

      bounds.origin = bounds.origin + scroll_offset;

      if editor.show_cursor(window, cx) {
        // cursor blink
        let cursor_height =
          window.text_style().font_size.to_pixels(window.rem_size()) + px(2.);
        cursor = Some(fill(
          Bounds::new(
            point(
              bounds.left() + cursor_pos.x,
              bounds.top() + cursor_pos.y + ((line_height - cursor_height) / 2.),
            ),
            size(px(1.), cursor_height),
          ),
          cx.theme().caret,
        ))
      };
    }

    (cursor, scroll_offset)
  }


  fn layout_selections(
    &self,
    lines: &[WrappedLine],
    line_height: Pixels,
    bounds: &mut Bounds<Pixels>,
    _: &mut Window,
    cx: &mut App,
  ) -> Option<Path<Pixels>> {
    let editor = self.editor.read(cx);
    let selected_range = &editor.selected_range;
    if selected_range.is_empty() {
      return None;
    }

    let (start_ix, end_ix) = if selected_range.start < selected_range.end {
      (selected_range.start, selected_range.end)
    } else {
      (selected_range.end, selected_range.start)
    };

    let mut prev_lines_offset = 0;
    let mut line_corners = vec![];

    let mut offset_y = px(0.);
    for line in lines.iter() {
      let line_size = line.size(line_height);
      let line_wrap_width = line_size.width;

      let line_origin = point(px(0.), offset_y);

      let line_cursor_start =
        line.position_for_index(start_ix.saturating_sub(prev_lines_offset), line_height);
      let line_cursor_end =
        line.position_for_index(end_ix.saturating_sub(prev_lines_offset), line_height);

      if line_cursor_start.is_some() || line_cursor_end.is_some() {
        let start = line_cursor_start
          .unwrap_or_else(|| line.position_for_index(0, line_height).unwrap());

        let end = line_cursor_end
          .unwrap_or_else(|| line.position_for_index(line.len(), line_height).unwrap());

        // Split the selection into multiple items
        let wrapped_lines =
          (end.y / line_height).ceil() as usize - (start.y / line_height).ceil() as usize;

        let mut end_x = end.x;
        if wrapped_lines > 0 {
          end_x = line_wrap_width;
        }

        line_corners.push(Corners {
          top_left: line_origin + point(start.x, start.y),
          top_right: line_origin + point(end_x, start.y),
          bottom_left: line_origin + point(start.x, start.y + line_height),
          bottom_right: line_origin + point(end_x, start.y + line_height),
        });

        // wrapped lines
        for i in 1..=wrapped_lines {
          let start = point(px(0.), start.y + i as f32 * line_height);
          let mut end = point(end.x, end.y + i as f32 * line_height);
          if i < wrapped_lines {
            end.x = line_size.width;
          }

          line_corners.push(Corners {
            top_left: line_origin + point(start.x, start.y),
            top_right: line_origin + point(end.x, start.y),
            bottom_left: line_origin + point(start.x, start.y + line_height),
            bottom_right: line_origin + point(end.x, start.y + line_height),
          });
        }
      }

      if line_cursor_start.is_some() && line_cursor_end.is_some() {
        break;
      }

      offset_y += line_size.height;
      // +1 for skip the last `\n`
      prev_lines_offset += line.len() + 1;
    }

    let mut points = vec![];
    if line_corners.is_empty() {
      return None;
    }

    // Fix corners to make sure the left to right direction
    for corners in &mut line_corners {
      if corners.top_left.x > corners.top_right.x {
        std::mem::swap(&mut corners.top_left, &mut corners.top_right);
        std::mem::swap(&mut corners.bottom_left, &mut corners.bottom_right);
      }
    }

    for corners in &line_corners {
      points.push(corners.top_right);
      points.push(corners.bottom_right);
      points.push(corners.bottom_left);
    }

    let mut rev_line_corners = line_corners.iter().rev().peekable();
    while let Some(corners) = rev_line_corners.next() {
      points.push(corners.top_left);
      if let Some(next) = rev_line_corners.peek() {
        if next.top_left.x > corners.top_left.x {
          points.push(point(next.top_left.x, corners.top_left.y));
        }
      }
    }

    // print_points_as_svg_path(&line_corners, &points);

    let first_p = *points.get(0).unwrap();
    let mut builder = gpui::PathBuilder::fill();
    builder.move_to(bounds.origin + first_p);
    for p in points.iter().skip(1) {
      builder.line_to(bounds.origin + *p);
    }

    builder.build().ok()
  }
}

pub(crate) struct PrepaintState {
  lines: SmallVec<[WrappedLine; 1]>,
  cursor: Option<PaintQuad>,
  cursor_scroll_offset: Point<Pixels>,
  selection_path: Option<Path<Pixels>>,
  bounds: Bounds<Pixels>,
}

impl IntoElement for EditorElement {
  type Element = Self;

  fn into_element(self) -> Self::Element {
    self
  }
}

impl Element for EditorElement {
  type RequestLayoutState = ();
  type PrepaintState = PrepaintState;

  fn id(&self) -> Option<ElementId> {
    None
  }

  fn request_layout(&mut self, _: Option<&GlobalElementId>, window: &mut Window, cx: &mut App) -> (LayoutId, Self::RequestLayoutState) {
    let mut style = Style::default();
    style.size.width = relative(1f32).into();
    style.size.height = relative(1f32).into();

    (window.request_layout(style, [], cx), ())
  }

  fn prepaint(&mut self, _: Option<&GlobalElementId>, bounds: Bounds<Pixels>, _: &mut Self::RequestLayoutState, window: &mut Window, cx: &mut App) -> Self::PrepaintState {
    let multi_line = true;
    let line_height = window.line_height();
    let editor = self.editor.read(cx);
    let display_text = editor.text.clone();
    let style = window.text_style();
    let mut bounds = bounds;
    let text_color = cx.theme().foreground;

    let run = TextRun {
      len: display_text.len(),
      font: style.font(),
      color: text_color,
      background_color: None,
      underline: None,
      strikethrough: None,
    };

    let runs = vec![run];

    let font_size = style.font_size.to_pixels(window.rem_size());
    let wrap_width = if multi_line {
      Some(bounds.size.width - RIGHT_MARGIN)
    } else {
      None
    };

    let lines = window
      .text_system()
      .shape_text(display_text, font_size, &runs, wrap_width, None)
      .unwrap();

    // Calculate the scroll offset to keep the cursor in view
    let (cursor, cursor_scroll_offset) =
      self.layout_cursor(&lines, line_height, &mut bounds, window, cx);

    let selection_path = self.layout_selections(&lines, line_height, &mut bounds, window, cx);

    PrepaintState {
      bounds,
      lines,
      cursor,
      cursor_scroll_offset,
      selection_path,
    }
  }

  fn paint(&mut self, _: Option<&GlobalElementId>, input_bounds: Bounds<Pixels>, _: &mut Self::RequestLayoutState, prepaint: &mut Self::PrepaintState, window: &mut Window, cx: &mut App) {
    let focus_handle = self.editor.read(cx).focus_handle.clone();
    let focused = focus_handle.is_focused(window);
    let bounds = prepaint.bounds;
    let selected_range = self.editor.read(cx).selected_range.clone();

    window.handle_input(
      &focus_handle,
      ElementInputHandler::new(bounds, self.editor.clone()),
      cx
    );

    if let Some(path) = prepaint.selection_path.take() {
      window.paint_path(path, cx.theme().selection);
    }

    // Paint multi line text
    let line_height = window.line_height();
    let origin = bounds.origin;

    let mut offset_y = px(0.);
    for line in prepaint.lines.iter() {
      let p = point(origin.x, origin.y + offset_y);
      _ = line.paint(p, line_height, window, cx);
      offset_y += line.size(line_height).height;
    }

    if focused {
      if let Some(cursor) = prepaint.cursor.take() {
        window.paint_quad(cursor);
      }
    }

    let width = prepaint
      .lines
      .iter()
      .map(|l| l.width())
      .max()
      .unwrap_or_default();
    let height = prepaint
      .lines
      .iter()
      .map(|l| l.size(line_height).height.0)
      .sum::<f32>();

    let scroll_size = size(width, px(height));

    self.editor.update(cx, |input, _cx| {
      input.last_layout = Some(prepaint.lines.clone());
      input.last_bounds = Some(bounds);
      input.last_cursor_offset = Some(input.cursor_offset());
      input.last_line_height = line_height;
      input.input_bounds = input_bounds;
      input.last_selected_range = Some(selected_range);
      input
        .scroll_handle
        .set_offset(prepaint.cursor_scroll_offset);
      input.scroll_size = scroll_size;
    });

    self.paint_mouse_listeners(window, cx);
  }
}

