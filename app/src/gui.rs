use imgui::{sys::ImVec2, ProgressBar};
use imgui_glow_renderer::glow::{self, HasContext};

use conf::Conf;
use core::{
    action::{Action, TimerId, TimerOp},
    app_state::AppState,
    timer,
};
use sdl2::{
    sys::SDL_WindowFlags,
    video::{GLProfile, SwapInterval},
};

pub struct Gui {
    _gl_context: sdl2::video::GLContext,
    event_pump: sdl2::EventPump,
    platform: imgui_sdl2_support::SdlPlatform,
    imgui: imgui::Context,
    window: sdl2::video::Window,
    window_height: f32,
    renderer: imgui_glow_renderer::AutoRenderer,
}

impl Gui {
    pub fn new(conf: &Conf, app_state: &AppState) -> Self {
        let sdl = sdl2::init().unwrap();
        let sdl_video = sdl.video().unwrap();

        let gl_attr = sdl_video.gl_attr();
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(GLProfile::Core);

        let mut window = sdl_video
            .window(&conf.title, conf.window_width, 240)
            .set_window_flags(SDL_WindowFlags::SDL_WINDOW_ALWAYS_ON_TOP as u32)
            // Borderless looks good, but requires extra code
            // to enable the user to move the window.
            // .borderless()
            .allow_highdpi()
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();

        window
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync)
            .unwrap();

        let gl = glow_context(&window);

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);

        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        let window_padding: ImVec2 = imgui.style().window_padding.into();
        let item_spacing: ImVec2 = imgui.style().item_spacing.into();
        let window_height = conf.bar_height as f32 * app_state.timers.len() as f32
            + item_spacing.y * (app_state.timers.len() - 1) as f32
            + window_padding.y * 2.0;

        window
            .set_size(conf.window_width, window_height as u32)
            .unwrap();

        let platform = imgui_sdl2_support::SdlPlatform::init(&mut imgui);
        let renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui).unwrap();
        let event_pump = sdl.event_pump().unwrap();

        Self {
            _gl_context: gl_context,
            event_pump,
            platform,
            imgui,
            window,
            window_height,
            renderer,
        }
    }

    pub fn update(&mut self, conf: &Conf, app_state: &AppState) -> Vec<Action> {
        for event in self.event_pump.poll_iter() {
            self.platform.handle_event(&mut self.imgui, &event);

            if let sdl2::event::Event::Quit { .. } = event {
                return vec![Action::Quit];
            }
        }

        self.platform
            .prepare_frame(&mut self.imgui, &self.window, &self.event_pump);

        let ui = self.imgui.new_frame();

        let actions = ui
            .window(&conf.title)
            .no_decoration()
            .position([0.0, 0.0], imgui::Condition::Always)
            .size(
                [conf.window_width as f32, self.window_height],
                imgui::Condition::Once,
            )
            .build(|| {
                app_state
                    .timers
                    .iter()
                    .enumerate()
                    .flat_map(|(i, t)| {
                        let t_id = ui.push_id_usize(i);

                        ProgressBar::new(t.elapsed_frac())
                            .size([conf.window_width as f32 - 62.0, conf.bar_height as f32])
                            .overlay_text(t.label())
                            .build(ui);

                        ui.same_line();
                        let action1 = match t.state() {
                            timer::State::Stopped => {
                                ui.button(">").then_some(Action::TimerAction {
                                    id: TimerId(i),
                                    op: TimerOp::Start,
                                })
                            }
                            timer::State::Started => {
                                ui.button("_").then_some(Action::TimerAction {
                                    id: TimerId(i),
                                    op: TimerOp::Pause,
                                })
                            }
                            timer::State::Paused => ui.button(">").then_some(Action::TimerAction {
                                id: TimerId(i),
                                op: TimerOp::Start,
                            }),
                        };

                        ui.same_line();
                        let disabled = ui.begin_disabled(t.state() == timer::State::Stopped);
                        let action2 = ui.button("X").then_some(Action::TimerAction {
                            id: TimerId(i),
                            op: TimerOp::Stop,
                        });
                        disabled.end();

                        t_id.end();

                        action1.into_iter().chain(action2.into_iter())
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let draw_data = self.imgui.render();

        unsafe { self.renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        self.renderer.render(draw_data).unwrap();

        self.window.gl_swap_window();

        actions
    }
}

fn glow_context(window: &sdl2::video::Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}
