#![cfg_attr(target_family = "wasm", no_main)]

use gpuim::{
    App, Bounds, Context, FontWeight, Render, SharedString, Window, WindowBounds, WindowOptions,
    div, prelude::*, px, rgb, size,
};
use gpuim_platform::application;

struct FormField {
    value: SharedString,
}

struct Form {
    name: FormField,
    email: FormField,
    message: FormField,
    submitted: bool,
}

impl Form {
    fn render_field(label: &'static str, value: &SharedString) -> impl IntoElement {
        let display_value: SharedString = if value.is_empty() {
            format!("Enter your {label}...").into()
        } else {
            value.clone()
        };
        let text_color = if value.is_empty() {
            rgb(0xAAAAAA)
        } else {
            rgb(0x333333)
        };

        div()
            .flex()
            .flex_col()
            .gap_1()
            .w_full()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x555555))
                    .child(SharedString::from(label)),
            )
            .child(
                div()
                    .w_full()
                    .px_3()
                    .py_2()
                    .bg(rgb(0xFFFFFF))
                    .border_1()
                    .border_color(rgb(0xCCCCCC))
                    .rounded_md()
                    .text_sm()
                    .text_color(text_color)
                    .child(display_value),
            )
    }

    fn submit(&mut self, cx: &mut Context<Self>) {
        self.submitted = true;
        cx.notify();
    }

    fn reset(&mut self, cx: &mut Context<Self>) {
        self.name.value = "".into();
        self.email.value = "".into();
        self.message.value = "".into();
        self.submitted = false;
        cx.notify();
    }
}

impl Render for Form {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content: gpuim::AnyElement = if self.submitted {
            div()
                .flex()
                .flex_col()
                .gap_4()
                .items_center()
                .child(
                    div()
                        .size(px(48.0))
                        .rounded_full()
                        .bg(rgb(0x33AA44))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgb(0xFFFFFF))
                        .text_xl()
                        .child("OK"),
                )
                .child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x333333))
                        .child("Form Submitted!"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x666666))
                        .child(format!("Name: {}", self.name.value)),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x666666))
                        .child(format!("Email: {}", self.email.value)),
                )
                .child(
                    div()
                        .id("back")
                        .px_4()
                        .py_2()
                        .bg(rgb(0x3377CC))
                        .text_color(rgb(0xFFFFFF))
                        .rounded_md()
                        .text_sm()
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.9))
                        .active(|s| s.opacity(0.8))
                        .on_click(cx.listener(|this, _: &gpuim::ClickEvent, _, cx| {
                            this.reset(cx);
                        }))
                        .child("Back to form"),
                )
                .into_any()
        } else {
            div()
                .flex()
                .flex_col()
                .gap_4()
                .w_full()
                .child(Self::render_field("Name", &self.name.value))
                .child(Self::render_field("Email", &self.email.value))
                .child(Self::render_field("Message", &self.message.value))
                .child(
                    div()
                        .flex()
                        .gap_3()
                        .pt_2()
                        .child(
                            div()
                                .id("submit")
                                .flex_1()
                                .px_4()
                                .py_2()
                                .bg(rgb(0x33AA44))
                                .text_color(rgb(0xFFFFFF))
                                .rounded_md()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_center()
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .active(|s| s.opacity(0.8))
                                .on_click(cx.listener(|this, _: &gpuim::ClickEvent, _, cx| {
                                    this.submit(cx);
                                }))
                                .child("Submit"),
                        )
                        .child(
                            div()
                                .id("reset")
                                .flex_1()
                                .px_4()
                                .py_2()
                                .bg(rgb(0xCCCCCC))
                                .text_color(rgb(0x555555))
                                .rounded_md()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_center()
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .active(|s| s.opacity(0.8))
                                .on_click(cx.listener(|this, _: &gpuim::ClickEvent, _, cx| {
                                    this.reset(cx);
                                }))
                                .child("Reset"),
                        ),
                )
                .into_any()
        };

        div()
            .flex()
            .flex_col()
            .size(px(380.0))
            .bg(rgb(0xF8F9FA))
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
                    .child("Contact Form"),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .child(content),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(380.), px(420.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| Form {
                    name: FormField { value: "".into() },
                    email: FormField { value: "".into() },
                    message: FormField { value: "".into() },
                    submitted: false,
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
