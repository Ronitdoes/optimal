//! GPUI 0.2.2 root view. API shape from docs.rs 0.2.2 (see
//! docs/DEPENDENCY-PINS.md §3). `TitlebarOptions` fields are intentionally
//! avoided: `WindowOptions::default()` is the compile-safe first spike.

use gpui::{App, AppContext, Application, Context, IntoElement, ParentElement, Render, Window, WindowOptions, div};

struct Root {
    title: String,
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child(self.title.clone())
    }
}

/// Open the single GPUI window and run the event loop. Call once, after Win32
/// DPI awareness and single-instance checks have succeeded in `runner`.
pub fn run_ui() {
    let app = Application::new();
    app.run(|cx: &mut App| {
        let options = WindowOptions {
            app_id: Some("com.optimal.desktop".into()),
            ..Default::default()
        };
        cx.open_window(options, |_window, cx| cx.new(|_ctx| Root { title: "Optimal O0".into() }))
            .expect("open main window");
    });
}
