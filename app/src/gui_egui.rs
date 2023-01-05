use conf::Conf;
use core::{
    action::{Action, TimerId, TimerOp},
    timer, Core,
};
use eframe::egui;
use std::{
    iter::once,
    time::{Duration, Instant},
};

pub struct Gui {
    conf: Conf,
    core: Core,
}

impl Gui {
    pub fn run(conf: Conf, core: Core) {
        let this = Self { conf, core };

        let options = eframe::NativeOptions {
            always_on_top: true,
            // decorated: false,
            centered: true,
            initial_window_size: Some(egui::vec2(this.conf.window_width as f32, 240.0)),
            run_and_return: true,
            ..Default::default()
        };
        eframe::run_native(
            &this.conf.title.clone(),
            options,
            Box::new(|_cc| Box::new(this)),
        );
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let actions = self
                .core
                .state()
                .timers
                .iter()
                .enumerate()
                .flat_map(|(i, t)| {
                    ui.push_id(i, |ui| {
                        ui.horizontal(|ui| {
                            let play_button_width = 2.0 * ui.spacing().button_padding.x
                                + ui.fonts()
                                    .glyph_width(&egui::TextStyle::Button.resolve(ui.style()), '▶');

                            let action1 = match t.state() {
                                timer::State::Reset | timer::State::Stopped => {
                                    ui.button("▶").clicked().then_some(Action::TimerAction {
                                        id: TimerId(i),
                                        op: TimerOp::Start,
                                    })
                                }
                                timer::State::Started => egui::Widget::ui(
                                    egui::Button::new("⏸")
                                        .min_size(egui::vec2(play_button_width, 0.0)),
                                    ui,
                                )
                                .clicked()
                                .then_some(Action::TimerAction {
                                    id: TimerId(i),
                                    op: TimerOp::Pause,
                                }),
                                timer::State::Paused => {
                                    ui.button("▶").clicked().then_some(Action::TimerAction {
                                        id: TimerId(i),
                                        op: TimerOp::Resume,
                                    })
                                }
                            };

                            let action2 = ui
                                .add_enabled_ui(t.state() != timer::State::Reset, |ui| {
                                    ui.button("⏹").clicked().then_some(Action::TimerAction {
                                        id: TimerId(i),
                                        op: TimerOp::Stop,
                                    })
                                })
                                .inner;

                            ui.add(
                                egui::ProgressBar::new(t.elapsed_frac(Instant::now()))
                                    .text(t.label()),
                            );

                            action1.into_iter().chain(action2.into_iter())
                        })
                        .inner
                    })
                    .inner
                })
                .chain(once(Action::Tick))
                .collect::<Vec<_>>();

            for action in actions.into_iter() {
                self.core.handle(action);
            }

            let cursor = ui.cursor();
            _frame.set_window_size(egui::vec2(
                self.conf.window_width as f32,
                cursor.top() + ui.spacing().window_margin.bottom,
            ));

            ctx.request_repaint_after(Duration::new(0, 100_000_000));
        });
    }
}
