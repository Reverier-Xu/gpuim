#![cfg_attr(target_family = "wasm", no_main)]

use gpuim::{
    App, Bounds, Context, FontWeight, Render, SharedString, Window, WindowBounds, WindowOptions,
    div, prelude::*, px, rgb, size,
};
use gpuim_platform::application;

struct Counter {
    count: i32,
    label: SharedString,
}

impl Counter {
    fn increment(&mut self, _: &gpuim::ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.count += 1;
        self.label = format!("Count: {}", self.count).into();
        cx.notify();
    }

    fn decrement(&mut self, _: &gpuim::ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.count -= 1;
        self.label = format!("Count: {}", self.count).into();
        cx.notify();
    }

    fn reset(&mut self, _: &gpuim::ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.count = 0;
        self.label = "Count: 0".into();
        cx.notify();
    }
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count_color = match self.count {
            n if n > 0 => rgb(0x22AA22),
            n if n < 0 => rgb(0xCC2222),
            _ => rgb(0x333333),
        };

        div()
            .flex()
            .flex_col()
            .size(px(320.0))
            .bg(rgb(0xF8F9FA))
            .justify_center()
            .items_center()
            .gap_4()
            .p_6()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded_xl()
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x555555))
                    .child("Counter"),
            )
            .child(
                div()
                    .text_3xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(count_color)
                    .child(self.label.clone()),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(
                        div()
                            .id("decrement")
                            .px_4()
                            .py_2()
                            .bg(rgb(0xCC2222))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .text_sm()
                            .cursor_pointer()
                            .active(|s| s.opacity(0.8))
                            .hover(|s| s.opacity(0.9))
                            .on_click(cx.listener(Self::decrement))
                            .child("-"),
                    )
                    .child(
                        div()
                            .id("reset")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x888888))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .text_sm()
                            .cursor_pointer()
                            .active(|s| s.opacity(0.8))
                            .hover(|s| s.opacity(0.9))
                            .on_click(cx.listener(Self::reset))
                            .child("Reset"),
                    )
                    .child(
                        div()
                            .id("increment")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x22AA22))
                            .text_color(rgb(0xFFFFFF))
                            .rounded_md()
                            .text_sm()
                            .cursor_pointer()
                            .active(|s| s.opacity(0.8))
                            .hover(|s| s.opacity(0.9))
                            .on_click(cx.listener(Self::increment))
                            .child("+"),
                    ),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(320.), px(280.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| Counter {
                    count: 0,
                    label: "Count: 0".into(),
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
