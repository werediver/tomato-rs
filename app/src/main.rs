use std::fs::read_to_string;
use std::io;

use glow::HasContext;
use imgui::sys::ImVec2;
use imgui::Condition;
use imgui::Context;
use imgui::ProgressBar;
use imgui_glow_renderer::glow;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use sdl2::video::SwapInterval;
use sdl2::{
    event::Event,
    video::{GLProfile, Window},
};

use conf::Conf;

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn load_conf() -> io::Result<Conf> {
    let data = read_to_string("tomato.toml")?;
    let conf = Conf::from_str(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(conf)
}

fn main() {
    let conf = load_conf().unwrap();

    let sdl = sdl2::init().unwrap();
    let sdl_video = sdl.video().unwrap();

    /* Hint SDL to initialize an OpenGL 3.3 core profile context */
    let gl_attr = sdl_video.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    let mut window = sdl_video
        .window(&conf.title, conf.window_width, 240)
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

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let window_padding: ImVec2 = imgui.style().window_padding.into();
    let item_spacing: ImVec2 = imgui.style().item_spacing.into();
    let window_height = conf.bar_height as f32 * conf.timers.len() as f32
        + item_spacing.y * (conf.timers.len() - 1) as f32
        + window_padding.y * 2.0;

    window
        .set_size(conf.window_width, window_height as u32)
        .unwrap();

    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            platform.handle_event(&mut imgui, &event);

            if let Event::Quit { .. } = event {
                break 'main;
            }
        }

        platform.prepare_frame(&mut imgui, &window, &event_pump);

        let ui = imgui.new_frame();

        ui.window(&conf.title)
            .no_decoration()
            .position([0.0, 0.0], Condition::Always)
            .size(
                [conf.window_width as f32, window_height],
                Condition::FirstUseEver,
            )
            .build(|| {
                for t in &conf.timers {
                    ProgressBar::new(t.duration as f32 / 30.0)
                        .size([-1.0, conf.bar_height as f32])
                        .overlay_text(&t.name)
                        .build(&ui);
                }
            });

        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();
    }
}
