#![cfg_attr(target_family = "wasm", no_main)]

use gpuim::{
    App, Bounds, Context, FontWeight, Render, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use gpuim_platform::application;

#[derive(Clone)]
struct TodoItem {
    text: SharedString,
    done: bool,
}

struct TodoList {
    items: Vec<TodoItem>,
    input_text: SharedString,
}

impl TodoList {
    fn add_item(&mut self) {
        let text = self.input_text.trim().to_string();
        if !text.is_empty() {
            self.items.push(TodoItem {
                text: text.into(),
                done: false,
            });
            self.input_text = "".into();
        }
    }

    fn toggle_item(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(item) = self.items.get_mut(index) {
            item.done = !item.done;
            cx.notify();
        }
    }

    fn remove_item(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.items.len() {
            self.items.remove(index);
            cx.notify();
        }
    }
}

impl Render for TodoList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let remaining = self.items.iter().filter(|i| !i.done).count();
        let total = self.items.len();

        div()
            .flex()
            .flex_col()
            .size(px(420.0))
            .bg(rgb(0xFFFFFF))
            .shadow_xl()
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded_xl()
            .overflow_hidden()
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_3()
                    .bg(rgb(0x3377CC))
                    .text_color(rgb(0xFFFFFF))
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .child("Todo List"),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .w_full()
                    .px_4()
                    .py_2()
                    .child(
                        div()
                            .flex_1()
                            .px_3()
                            .py_2()
                            .bg(rgb(0xFFFFFF))
                            .border_1()
                            .border_color(rgb(0xCCCCCC))
                            .rounded_md()
                            .text_sm()
                            .child(if self.input_text.is_empty() {
                                SharedString::from("Add a new task...")
                            } else {
                                self.input_text.clone()
                            }),
                    )
                    .child(
                        div()
                            .id("add-task")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x3377CC))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.9))
                            .active(|s| s.opacity(0.8))
                            .on_click(cx.listener(|this, _: &gpuim::ClickEvent, _, cx| {
                                this.add_item();
                                cx.notify();
                            }))
                            .child("Add"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .px_4()
                    .py_2()
                    .bg(rgb(0xF0F0F0))
                    .border_b_1()
                    .border_color(rgb(0xDDDDDD))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child(format!("{remaining}/{total} remaining")),
                    )
                    .child(
                        div()
                            .id("clear-completed")
                            .px_3()
                            .py_1()
                            .bg(rgb(0xCC4444))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .text_xs()
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.9))
                            .active(|s| s.opacity(0.8))
                            .on_click(cx.listener(|this, _: &gpuim::ClickEvent, _, cx| {
                                this.items.retain(|i| !i.done);
                                cx.notify();
                            }))
                            .child("Clear done"),
                    ),
            )
            .child({
                let mut container = div().flex_1().flex().flex_col().overflow_hidden();
                for (i, item) in self.items.iter().enumerate() {
                    let text_color = if item.done {
                        rgb(0xAAAAAA)
                    } else {
                        rgb(0x333333)
                    };

                    let row = div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .w_full()
                        .px_4()
                        .py_3()
                        .bg(if i % 2 == 0 {
                            rgb(0xFAFAFA)
                        } else {
                            rgb(0xFFFFFF)
                        })
                        .border_b_1()
                        .border_color(rgb(0xEEEEEE))
                        .child(
                            div()
                                .id(format!("toggle-{i}"))
                                .size(px(20.0))
                                .rounded_full()
                                .border_1()
                                .border_color(if item.done {
                                    rgb(0x33AA44)
                                } else {
                                    rgb(0xBBBBBB)
                                })
                                .bg(if item.done {
                                    rgb(0x33AA44)
                                } else {
                                    rgb(0xFFFFFF)
                                })
                                .cursor_pointer()
                                .on_click(cx.listener(move |this, _: &gpuim::ClickEvent, _, cx| {
                                    this.toggle_item(i, cx);
                                })),
                        )
                        .child(
                            div()
                                .flex_1()
                                .text_sm()
                                .text_color(text_color)
                                .child(item.text.clone()),
                        )
                        .child(
                            div()
                                .id(format!("remove-{i}"))
                                .text_color(rgb(0xCC4444))
                                .text_sm()
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.7))
                                .on_click(cx.listener(move |this, _: &gpuim::ClickEvent, _, cx| {
                                    this.remove_item(i, cx);
                                }))
                                .child("x"),
                        );

                    let row = if item.done { row.line_through() } else { row };
                    container = container.child(row);
                }
                container
            })
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(420.), px(500.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| TodoList {
                    items: vec![
                        TodoItem {
                            text: "Learn GPUI basics".into(),
                            done: true,
                        },
                        TodoItem {
                            text: "Build a todo app".into(),
                            done: false,
                        },
                        TodoItem {
                            text: "Add more features".into(),
                            done: false,
                        },
                    ],
                    input_text: "".into(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpuim_platform::web_init();
    run_example();
}
