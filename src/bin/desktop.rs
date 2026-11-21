#[cfg(windows)]
mod windows {
    use anyhow::{Result, ensure};
    use gpui::{
        App, Application, Bounds, Context, IntoElement, Window, WindowBounds, WindowOptions, div,
        prelude::*, px, rgb, size,
    };
    use optimal::protocol::{Command, DEMO_STEPS, EventKind, VERSION, connect};
    use std::{cell::Cell, rc::Rc, time::Duration};

    #[derive(Default)]
    struct Demo {
        sequence: u64,
        progress: u8,
        clicks: u64,
        in_flight_renders: u64,
        status: &'static str,
    }

    impl Render for Demo {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            if self.status == "Running" {
                self.in_flight_renders += 1;
            }
            div()
                .size_full()
                .flex()
                .flex_col()
                .gap_4()
                .p_6()
                .bg(rgb(0x15201f))
                .text_color(rgb(0xf2f4eb))
                .child(div().text_xl().child("Optimal · Foundation"))
                .child(format!(
                    "{} · Step {} of {}",
                    self.status, self.progress, DEMO_STEPS
                ))
                .child(format!("Last event sequence {}", self.sequence))
                .child(
                    div()
                        .id("render-check")
                        .cursor_pointer()
                        .rounded_md()
                        .px_3()
                        .py_2()
                        .bg(rgb(0x293b36))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.clicks += 1;
                            cx.notify();
                        }))
                        .child(format!("Check responsiveness · {} clicks", self.clicks)),
                )
        }
    }

    pub fn run() -> Result<()> {
        let smoke = std::env::args().any(|arg| arg == "--smoke");
        let runtime = tokio::runtime::Runtime::new()?;
        let (commands, mut events) = connect(runtime.handle());
        let verified = Rc::new(Cell::new(false));
        let window_opened = Rc::new(Cell::new(false));
        let result = verified.clone();
        let opened = window_opened.clone();
        Application::new().run(move |cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(500.), px(280.)), cx);
            let window = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| Demo {
                        status: "Waiting for Tokio",
                        ..Demo::default()
                    })
                },
            );
            let view = match window.and_then(|window| window.entity(cx)) {
                Ok(view) => view,
                Err(error) => {
                    eprintln!("Opening the GPUI window failed: {error:#}");
                    cx.quit();
                    return;
                }
            };
            opened.set(true);
            cx.on_window_closed(|cx| cx.quit()).detach();
            cx.activate(true);
            cx.spawn(async move |cx| {
                if commands.send(Command::start_demo(1)).is_err() {
                    eprintln!("The Tokio command receiver closed.");
                    cx.update(|cx| cx.quit()).ok();
                    return;
                }
                while let Some(event) = events.recv().await {
                    let update = view.update(cx, |demo, cx| {
                        if event.version != VERSION
                            || event.run_id != 1
                            || event.sequence != demo.sequence + 1
                        {
                            demo.status = "Invalid event";
                            cx.notify();
                            return false;
                        }
                        demo.sequence = event.sequence;
                        match event.kind {
                            EventKind::Started => demo.status = "Running",
                            EventKind::Progress { step } => demo.progress = step,
                            EventKind::Completed => {
                                demo.status = "Complete";
                                result.set(
                                    demo.in_flight_renders >= 2
                                        && demo.progress == DEMO_STEPS
                                        && demo.sequence == u64::from(DEMO_STEPS) + 2,
                                );
                            }
                            EventKind::Rejected { .. } => demo.status = "Rejected",
                        }
                        cx.notify();
                        true
                    });
                    if !matches!(update, Ok(true)) {
                        break;
                    }
                }
                if smoke {
                    cx.update(|cx| cx.quit()).ok();
                }
            })
            .detach();

            if smoke {
                let timer = cx.background_executor().timer(Duration::from_secs(8));
                cx.spawn(async move |cx| {
                    timer.await;
                    cx.update(|cx| cx.quit()).ok();
                })
                .detach();
            }
        });
        ensure!(window_opened.get(), "the GPUI window failed to open");
        if smoke {
            ensure!(
                verified.get(),
                "GPUI did not render multiple times during the Tokio run"
            );
            println!("PASS: GPUI rendered multiple times while receiving ordered Tokio events.");
        }
        Ok(())
    }
}

#[cfg(windows)]
fn main() -> anyhow::Result<()> {
    windows::run()
}

#[cfg(not(windows))]
fn main() -> anyhow::Result<()> {
    anyhow::bail!("The Phase 0 desktop probe requires Windows.")
}
