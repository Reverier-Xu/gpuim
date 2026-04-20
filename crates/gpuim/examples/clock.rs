#![cfg_attr(target_family = "wasm", no_main)]

use gpuim::{
    App, Bounds, Context, FontWeight, Render, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use gpuim_platform::application;

struct Clock {
    hours: i32,
    minutes: i32,
    seconds: i32,
}

impl Clock {
    fn new() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let total_secs = now.as_secs() % 86400;
        Self {
            hours: (total_secs / 3600 % 12) as i32,
            minutes: (total_secs / 60 % 60) as i32,
            seconds: (total_secs % 60) as i32,
        }
    }
}

impl Render for Clock {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let hour_rad = ((self.hours % 12) as f32 * 30.0 + self.minutes as f32 * 0.5).to_radians();

        let center = 110.0;

        div()
            .flex()
            .flex_col()
            .size(px(340.0))
            .bg(rgb(0x1A1A2E))
            .justify_center()
            .items_center()
            .gap_4()
            .shadow_2xl()
            .rounded_2xl()
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xE0E0E0))
                    .child("Clock (static snapshot)"),
            )
            .child(
                div()
                    .size(px(220.0))
                    .rounded_full()
                    .border_2()
                    .border_color(rgb(0x444466))
                    .bg(rgb(0x16213E))
                    .relative()
                    .child(
                        div()
                            .absolute()
                            .top(px(center - 52.0))
                            .left(px(center - 2.0))
                            .w(px(4.0))
                            .h(px(50.0))
                            .bg(rgb(0xFFFFFF))
                            .rounded_full(),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(center - 71.5))
                            .left(px(center - 1.5))
                            .w(px(3.0))
                            .h(px(70.0))
                            .bg(rgb(0xCCCCFF))
                            .rounded_full(),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(center - 86.0))
                            .left(px(center - 1.0))
                            .w(px(1.5))
                            .h(px(85.0))
                            .bg(rgb(0xFF4444))
                            .rounded_full(),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(105.0))
                            .left(px(105.0))
                            .size(px(10.0))
                            .rounded_full()
                            .bg(rgb(0xFFFFFF)),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(hour_rad.sin() as f32 * 50.0 + 105.0))
                            .left(px(hour_rad.cos() as f32 * 50.0 + 105.0))
                            .size(px(10.0))
                            .rounded_full()
                            .bg(rgb(0x888888)),
                    ),
            )
            .child(
                div()
                    .text_lg()
                    .text_color(rgb(0x8888AA))
                    .child(format!(
                        "{:02}:{:02}:{:02}",
                        self.hours, self.minutes, self.seconds
                    )),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(340.), px(380.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Clock::new()),
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
