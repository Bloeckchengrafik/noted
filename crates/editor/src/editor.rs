use crate::actions;
use components::{Sizable, StyleSized};
use gpui::{div, point, px, App, AppContext, Bounds, Context, CursorStyle, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement, Pixels, Point, Rems, Render, ScrollHandle, SharedString, Size, Styled, UTF16Selection, ViewInputHandler, Window, WrappedLine};
use std::ops::Range;
use smallvec::SmallVec;
use unicode_segmentation::UnicodeSegmentation;
use crate::cursor::Cursor;
use crate::element::EditorElement;

const LINE_HEIGHT: Rems = Rems(1.5);

pub enum EditorEvent {
  TextChanged(SharedString),
  PressEnter,
  Focus,
  Blur,
}

pub struct Editor {
  pub(crate) focus_handle: FocusHandle,
  pub(crate) text: SharedString,
  pub(crate) selected_range: Range<usize>,
  pub(crate) size: components::Size,
  pub(crate) scroll_handle: ScrollHandle,
  pub(super) last_bounds: Option<Bounds<Pixels>>,
  pub(super) last_selected_range: Option<Range<usize>>,
  pub(super) last_cursor_offset: Option<usize>,
  pub(super) selection_reversed: bool,
  pub(super) last_layout: Option<SmallVec<[WrappedLine; 1]>>,
  pub(super) last_line_height: Pixels,
  pub(super) input_bounds: Bounds<Pixels>,
  pub(super) scroll_size: Size<Pixels>,
  pub(super) marked_range: Option<Range<usize>>,
  pub(super) preferred_x_offset: Option<Pixels>,
  pub(super) selected_word_range: Option<Range<usize>>,
  pub(super) is_selecting: bool,
  pub(super) cursor: Entity<Cursor>,
}

impl EventEmitter<EditorEvent> for Editor {}

impl Editor {
  pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
    let focus_handle = cx.focus_handle();
    // Auto-acquire focus when the editor is created
    focus_handle.focus(window);
    let editor = Self {
      focus_handle,
      text: include_str!("../../../example.md").into(),
      selected_range: 0..0,
      size: components::Size::Medium,
      scroll_handle: ScrollHandle::default(),
      last_bounds: None,
      last_selected_range: None,
      last_cursor_offset: None,
      selection_reversed: false,
      last_layout: None,
      last_line_height: px(20.0),
      input_bounds: Bounds::default(),
      scroll_size: Size::default(),
      marked_range: None,
      preferred_x_offset: None,
      selected_word_range: None,
      is_selecting: false,
      cursor: cx.new(|_| Cursor::new()),
    };

    // Redraw the cursor when it blinks
    cx.observe(&editor.cursor, |_, _, cx| cx.notify()).detach();
    cx.observe_window_activation(window, |editor, window, cx| {
      if window.is_window_active() {
        let focus_handle = editor.focus_handle.clone();
        if focus_handle.is_focused(window) {
          editor.cursor.update(cx, |blink_cursor, cx| {
            blink_cursor.start(cx);
          });
        }
      }
    })
      .detach();

    editor
  }
  //#region utf16-Utilities
  fn next_boundary(&self, offset: usize) -> usize {
    self.text
      .grapheme_indices(true)
      .find_map(|(idx, _)| (idx > offset).then_some(idx))
      .unwrap_or(self.text.len())
  }

  fn prev_boundary(&self, offset: usize) -> usize {
    if offset == 0 {
      return 0;
    }

    self.text
      .grapheme_indices(true)
      .rev()
      .find_map(|(idx, _)| (idx < offset).then_some(idx))
      .unwrap_or(self.text.len())
  }

  fn offset_from_utf16(&self, offset: usize) -> usize {
    let mut utf8_offset = 0;
    let mut utf16_count = 0;

    for ch in self.text.chars() {
      if utf16_count >= offset {
        break;
      }
      utf16_count += ch.len_utf16();
      utf8_offset += ch.len_utf8();
    }

    utf8_offset
  }

  fn offset_to_utf16(&self, offset: usize) -> usize {
    let mut utf16_offset = 0;
    let mut utf8_count = 0;

    for ch in self.text.chars() {
      if utf8_count >= offset {
        break;
      }
      utf8_count += ch.len_utf8();
      utf16_offset += ch.len_utf16();
    }

    utf16_offset
  }

  fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
    self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
  }

  fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
    self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
  }
  //#endregion

  //#region Actions
  fn right(&mut self, _: &actions::Right, window: &mut Window, cx: &mut Context<Self>) {
    if self.selected_range.is_empty() {
      self.move_to(self.next_boundary(self.selected_range.end), window, cx);
    } else {
      self.move_to(self.selected_range.end, window, cx)
    }
  }

  fn left(&mut self, _: &actions::Left, window: &mut Window, cx: &mut Context<Self>) {
    if self.selected_range.is_empty() {
      self.move_to(self.prev_boundary(self.selected_range.end), window, cx);
    } else {
      self.move_to(self.selected_range.start, window, cx)
    }
  }

  fn up(&mut self, _: &actions::Up, window: &mut Window, cx: &mut Context<Self>) {
    self.pause_blink_cursor(cx);
    self.move_vertical(-1, window, cx);
  }

  fn down(&mut self, _: &actions::Down, window: &mut Window, cx: &mut Context<Self>) {
    self.pause_blink_cursor(cx);
    self.move_vertical(1, window, cx);
  }

  fn select_right(&mut self, _: &actions::SelectRight, window: &mut Window, cx: &mut Context<Self>) {
    self.select_to(self.next_boundary(self.cursor_offset()), window, cx);
  }

  fn select_left(&mut self, _: &actions::SelectLeft, window: &mut Window, cx: &mut Context<Self>) {
    self.select_to(self.prev_boundary(self.cursor_offset()), window, cx);
  }

  fn select_up(&mut self, _: &actions::SelectUp, window: &mut Window, cx: &mut Context<Self>) {
    let offset = self.start_of_line(window, cx).saturating_sub(1);
    self.select_to(offset, window, cx);
  }

  fn select_down(&mut self, _: &actions::SelectDown, window: &mut Window, cx: &mut Context<Self>) {
    let offset = (self.end_of_line(window, cx) + 1).min(self.text.len());
    self.select_to(self.next_boundary(offset), window, cx);
  }

  fn enter(&mut self, _: &actions::Enter, window: &mut Window, cx: &mut Context<Self>) {
    let is_eof = self.selected_range.end == self.text.len();
    self.replace_text_in_range(None, "\n", window, cx);

    // Move cursor to the start of the next line
    let mut new_offset = self.next_boundary(self.cursor_offset()) - 1;
    if is_eof {
      new_offset += 1;
    }
    self.move_to(new_offset, window, cx);
  }

  pub(crate) fn on_drag_move(&mut self, event: &MouseMoveEvent, window: &mut Window, cx: &mut Context<Editor>) {
    if self.text.is_empty() {
      return;
    }

    if self.last_layout.is_none() {
      return;
    }

    if !self.focus_handle.is_focused(window) {
      return;
    }

    if !self.is_selecting {
      return;
    }

    let offset = self.index_for_mouse_position(event.position, window, cx);
    self.select_to(offset, window, cx);
    self.pause_blink_cursor(cx);
  }

  fn line_origin_with_y_offset(&self, y_offset: &mut Pixels, line: &WrappedLine, line_height: Pixels) -> Point<Pixels> {
    let p = point(px(0.), *y_offset);
    let height = line_height + line.wrap_boundaries.len() as f32 * line_height;
    *y_offset = *y_offset + height;
    p
  }

  fn index_for_mouse_position(&self, position: Point<Pixels>, _window: &Window, _cx: &App) -> usize {
    // If the text is empty, always return 0
    if self.text.is_empty() {
      return 0;
    }

    let (Some(bounds), Some(lines)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
    else {
      return 0;
    };

    let line_height = self.last_line_height;

    let inner_position = position - bounds.origin;

    let mut index = 0;
    let mut y_offset = px(0.);

    for line in lines.iter() {
      let line_origin = self.line_origin_with_y_offset(&mut y_offset, &line, line_height);
      let pos = inner_position - line_origin;

      let index_result = line.index_for_position(pos, line_height);
      if let Ok(v) = index_result {
        index += v;
        break;
      } else if let Ok(_) = line.index_for_position(point(px(0.), pos.y), line_height) {
        // Click in the this line but not in the text, move cursor to the end of the line.
        // The fallback index is saved in Err from `index_for_position` method.
        index += index_result.unwrap_err();
        break;
      } else if line.len() == 0 {
        // empty line
        let line_bounds = Bounds {
          origin: line_origin,
          size: gpui::size(bounds.size.width, line_height),
        };
        let pos = inner_position;
        if line_bounds.contains(&pos) {
          break;
        }
      } else {
        index += line.len();
      }

      // add 1 for \n
      index += 1;
    }

    if index > self.text.len() {
      self.text.len()
    } else {
      index
    }
  }

  fn on_mouse_down(&mut self, event: &MouseDownEvent, window: &mut Window, cx: &mut Context<Self>) {
    self.is_selecting = true;
    let offset = self.index_for_mouse_position(event.position, window, cx);
    // Double click to select word
    if event.button == MouseButton::Left && event.click_count == 2 {
      self.select_word(offset, window, cx);
      return;
    }

    if event.modifiers.shift {
      self.select_to(offset, window, cx);
    } else {
      self.move_to(offset, window, cx)
    }
  }

  fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _cx: &mut Context<Self>) {
    self.is_selecting = false;
    self.selected_word_range = None;
  }
  //#endregion

  pub(super) fn cursor_offset(&self) -> usize {
    if self.selection_reversed {
      self.selected_range.start
    } else {
      self.selected_range.end
    }
  }

  fn pause_blink_cursor(&mut self, cx: &mut Context<Self>) {
    self.cursor.update(cx, |cursor, cx| cursor.pause(cx));
  }

  pub(crate) fn show_cursor(&self, _: &Window, cx: &App) -> bool {
    self.cursor.read(cx).visible()
  }

  fn move_to(&mut self, offset: usize, _: &mut Window, cx: &mut Context<Self>) {
    self.selected_range = offset..offset;
    self.pause_blink_cursor(cx);
    self.update_preferred_x_offset(cx);
    cx.notify();
  }

  fn select_word(&mut self, offset: usize, window: &mut Window, cx: &mut Context<Editor>) {
    fn is_word(c: char) -> bool {
      c.is_alphanumeric() || matches!(c, '_')
    }

    let mut start = self.offset_to_utf16(offset);
    let mut end = start;
    let prev_text = self
      .text_for_range(0..start, &mut None, window, cx)
      .unwrap_or_default();
    let next_text = self
      .text_for_range(end..self.text.len(), &mut None, window, cx)
      .unwrap_or_default();

    let prev_chars = prev_text.chars().rev().peekable();
    let next_chars = next_text.chars().peekable();

    for (_, c) in prev_chars.enumerate() {
      if !is_word(c) {
        break;
      }

      start -= c.len_utf16();
    }

    for (_, c) in next_chars.enumerate() {
      if !is_word(c) {
        break;
      }

      end += c.len_utf16();
    }

    self.selected_range = self.range_from_utf16(&(start..end));
    self.selected_word_range = Some(self.selected_range.clone());
    cx.notify()
  }

  fn select_to(&mut self, offset: usize, _: &mut Window, cx: &mut Context<Self>) {
    if self.selection_reversed {
      self.selected_range.start = offset
    } else {
      self.selected_range.end = offset
    };

    if self.selected_range.end < self.selected_range.start {
      self.selection_reversed = !self.selection_reversed;
      self.selected_range = self.selected_range.end..self.selected_range.start;
    }

    // Ensure keep word selected range
    if let Some(word_range) = self.selected_word_range.as_ref() {
      if self.selected_range.start > word_range.start {
        self.selected_range.start = word_range.start;
      }
      if self.selected_range.end < word_range.end {
        self.selected_range.end = word_range.end;
      }
    }
    if self.selected_range.is_empty() {
      self.update_preferred_x_offset(cx);
    }
    cx.notify()
  }

  fn update_preferred_x_offset(&mut self, _: &mut Context<Editor>) {
    if let (Some(lines), Some(bounds)) = (&self.last_layout, &self.last_bounds) {
      let offset = self.cursor_offset();
      let line_height = self.last_line_height;

      // Find which line and sub-line the cursor is on and its position
      let (_line_index, _sub_line_index, cursor_pos) =
        self.line_and_position_for_offset(offset, lines, line_height);

      if let Some(pos) = cursor_pos {
        // Adjust by scroll offset
        let scroll_offset = bounds.origin;
        self.preferred_x_offset = Some(pos.x + scroll_offset.x);
      }
    }
  }

  fn move_vertical(&mut self, direction: i32, window: &mut Window, cx: &mut Context<Editor>) {
    let (Some(lines), Some(bounds)) = (&self.last_layout, &self.last_bounds) else { return; };

    let offset = self.cursor_offset();
    let line_height = self.last_line_height;
    let (current_line_index, current_sub_line, current_pos) =
      self.line_and_position_for_offset(offset, lines, line_height);

    let Some(current_pos) = current_pos else {
      return;
    };

    let current_x = self
      .preferred_x_offset
      .unwrap_or_else(|| current_pos.x + bounds.origin.x);

    let mut new_line_index = current_line_index;
    let mut new_sub_line = current_sub_line as i32;

    new_sub_line += direction;

    // Handle moving above the first line
    if direction == -1 && new_line_index == 0 && new_sub_line < 0 {
      // Move cursor to the beginning of the text
      self.move_to(0, window, cx);
      return;
    }

    if new_sub_line < 0 {
      if new_line_index > 0 {
        new_line_index -= 1;
        new_sub_line = lines[new_line_index].wrap_boundaries.len() as i32;
      } else {
        new_sub_line = 0;
      }
    } else {
      let max_sub_line = lines[new_line_index].wrap_boundaries.len() as i32;
      if new_sub_line > max_sub_line {
        if new_line_index < lines.len() - 1 {
          new_line_index += 1;
          new_sub_line = 0;
        } else {
          new_sub_line = max_sub_line;
        }
      }
    }

    // If after adjustment, still at the same position, do not proceed
    if new_line_index == current_line_index && new_sub_line == current_sub_line {
      return;
    }

    let target_line: &WrappedLine = &lines[new_line_index];
    let line_x = current_x - bounds.origin.x;
    let target_sub_line = new_sub_line as usize;

    let approx_pos = point(line_x, px(target_sub_line as f32 * line_height.0));
    let index_res = target_line.index_for_position(approx_pos, line_height);

    let new_local_index = match index_res {
      Ok(i) => i + 1,
      Err(i) => i,
    };

    let mut prev_lines_offset = 0;
    for (i, l) in lines.iter().enumerate() {
      if i == new_line_index {
        break;
      }
      prev_lines_offset += l.len() + 1;
    }

    let new_offset = (prev_lines_offset + new_local_index).min(self.text.len());
    self.selected_range = new_offset..new_offset;
    self.pause_blink_cursor(cx);
    cx.notify();
  }

  fn start_of_line(&mut self, window: &mut Window, cx: &mut Context<Self>) -> usize {
    let offset = self.prev_boundary(self.cursor_offset());
    let line = self
      .text_for_range(self.range_to_utf16(&(0..offset + 1)), &mut None, window, cx)
      .unwrap_or_default()
      .rfind('\n')
      .map(|i| i + 1)
      .unwrap_or(0);
    line
  }

  /// Get end of line
  fn end_of_line(&mut self, window: &mut Window, cx: &mut Context<Self>) -> usize {
    let offset = self.next_boundary(self.cursor_offset());
    // ignore if offset is "\n"
    if self
      .text_for_range(
        self.range_to_utf16(&(offset - 1..offset)),
        &mut None,
        window,
        cx,
      )
      .unwrap_or_default()
      .eq("\n")
    {
      return offset;
    }

    let line = self
      .text_for_range(
        self.range_to_utf16(&(offset..self.text.len())),
        &mut None,
        window,
        cx,
      )
      .unwrap_or_default()
      .find('\n')
      .map(|i| i + offset)
      .unwrap_or(self.text.len());
    line
  }

  fn line_and_position_for_offset(&self, offset: usize, lines: &[WrappedLine], line_height: Pixels) -> (usize, i32, Option<Point<Pixels>>) {
    let mut prev_lines_offset = 0;
    let mut y_offset = px(0.);
    for (line_index, line) in lines.iter().enumerate() {
      let local_offset = offset.saturating_sub(prev_lines_offset);
      if let Some(pos) = line.position_for_index(local_offset, line_height) {
        let sub_line_index = (pos.y.0 / line_height.0) as usize;
        let adjusted_pos = point(pos.x, pos.y + y_offset);
        return (line_index, sub_line_index as i32, Some(adjusted_pos));
      }

      y_offset += line.size(line_height).height;
      prev_lines_offset += line.len() + 1;
    }
    (0, 0, None)
  }
}

impl Focusable for Editor {
  fn focus_handle(&self, _: &App) -> FocusHandle {
    self.focus_handle.clone()
  }
}

impl Sizable for Editor {
  fn with_size(mut self, size: impl Into<components::Size>) -> Self {
    self.size = size.into();
    self
  }
}

impl ViewInputHandler for Editor {
  fn text_for_range(&mut self, range_utf16: Range<usize>, adjusted_range: &mut Option<Range<usize>>, _: &mut Window, _: &mut Context<Self>) -> Option<String> {
    let range = self.range_from_utf16(&range_utf16);
    adjusted_range.replace(self.range_to_utf16(&range));
    Some(self.text[range].to_string())
  }

  fn selected_text_range(&mut self, _: bool, _: &mut Window, _: &mut Context<Self>) -> Option<UTF16Selection> {
    Some(UTF16Selection {
      range: self.range_to_utf16(&self.selected_range),
      reversed: false,
    })
  }

  fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
    self.marked_range
      .as_ref()
      .map(|range| self.range_to_utf16(range))
  }

  fn unmark_text(&mut self, _: &mut Window, _: &mut Context<Self>) {
    self.marked_range = None;
  }

  fn replace_text_in_range(&mut self, range_utf16: Option<Range<usize>>, new_text: &str, _: &mut Window, cx: &mut Context<Self>) {
    let range = range_utf16
      .as_ref()
      .map(|range_utf16| self.range_from_utf16(range_utf16))
      .or(self.marked_range.clone())
      .unwrap_or(self.selected_range.clone());

    let pending_text: SharedString =
      (self.text[0..range.start].to_owned() + new_text + &self.text[range.end..]).into();

    // self.push_history(&range, new_text, window, cx); TODO
    self.text = pending_text;
    self.selected_range = range.start + new_text.len()..range.start + new_text.len();
    self.marked_range.take();
    self.update_preferred_x_offset(cx);
    cx.emit(EditorEvent::TextChanged(self.text.clone()));
    cx.notify();
  }

  fn replace_and_mark_text_in_range(&mut self, range_utf16: Option<Range<usize>>, new_text: &str, new_selected_range_utf16: Option<Range<usize>>, _: &mut Window, cx: &mut Context<Self>) {
    let range = range_utf16
      .as_ref()
      .map(|range_utf16| self.range_from_utf16(range_utf16))
      .or(self.marked_range.clone())
      .unwrap_or(self.selected_range.clone());
    let pending_text: SharedString =
      (self.text[0..range.start].to_owned() + new_text + &self.text[range.end..]).into();

    // self.push_history(&range, new_text, window, cx);
    self.text = pending_text;
    self.marked_range = Some(range.start..range.start + new_text.len());
    self.selected_range = new_selected_range_utf16
      .as_ref()
      .map(|range_utf16| self.range_from_utf16(range_utf16))
      .map(|new_range| new_range.start + range.start..new_range.end + range.end)
      .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());
    cx.emit(EditorEvent::TextChanged(self.text.clone()));
    cx.notify();
  }

  fn bounds_for_range(&mut self, range_utf16: Range<usize>, element_bounds: Bounds<Pixels>, _: &mut Window, _: &mut Context<Self>) -> Option<Bounds<Pixels>> {
    let line_height = self.last_line_height;
    let lines = self.last_layout.as_ref()?;
    let range = self.range_from_utf16(&range_utf16);

    let mut start_origin = None;
    let mut end_origin = None;
    let mut y_offset = px(0.);
    let mut index_offset = 0;

    for line in lines.iter() {
      if let Some(p) = line.position_for_index(range.start - index_offset, line_height) {
        start_origin = Some(p + point(px(0.), y_offset));
      }
      if let Some(p) = line.position_for_index(range.end - index_offset, line_height) {
        end_origin = Some(p + point(px(0.), y_offset));
      }

      y_offset += line.size(line_height).height;
      if start_origin.is_some() && end_origin.is_some() {
        break;
      }

      index_offset += line.len();
    }

    Some(Bounds::from_corners(
      element_bounds.origin + start_origin.unwrap_or_default(),
      element_bounds.origin + end_origin.unwrap_or_default(),
    ))
  }
}

impl Render for Editor {
  fn render(&mut self, _: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
    div()
      .flex()
      .flex_col()
      .id("editor")
      .key_context(actions::CONTEXT)
      .track_focus(&self.focus_handle)
      .cursor(CursorStyle::IBeam)
      .on_action(cx.listener(Self::right))
      .on_action(cx.listener(Self::left))
      .on_action(cx.listener(Self::up))
      .on_action(cx.listener(Self::down))
      .on_action(cx.listener(Self::select_right))
      .on_action(cx.listener(Self::select_left))
      .on_action(cx.listener(Self::select_up))
      .on_action(cx.listener(Self::select_down))
      .on_action(cx.listener(Self::enter))
      .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
      .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
      .size_full()
      .line_height(LINE_HEIGHT)
      .input_py(self.size)
      .items_center()
      .child(
        div()
          .id("editor-content")
          .flex_grow()
          .overflow_x_hidden()
          .child(EditorElement::new(cx.model().clone())),
      )
  }
}
