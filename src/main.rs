#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

use std::sync::Mutex;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

use luminance_sdl2::GL33Surface;

use luminance::context::GraphicsContext as _;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;

use imgui::im_str;

pub mod lua_api;
pub mod graphics;

lazy_static! {
    static ref pipeline_state: Mutex<PipelineState> = Mutex::new(PipelineState::default());
}

fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::max())
        .init();

    debug!("Hello, world!");

    let lua = lua_api::init_lua();
    lua_api::load_code(&lua, "print(\"hello from lua!\")").exec().expect("Failed to run lua code!");

    lua_api::load_code(&lua, include_str!("test.lua")).exec().expect("Failed to load lua code!");
    lua_api::call_rock_func(&lua, "load", 0).expect("Failed to call `rock.load`");

    //TODO: Error handling
    let mut surface = GL33Surface::build_with(|video| {
        let gl_attr = video.gl_attr();
        let mut builder = video.window("Rock", 1280, 720);
        builder
    }).expect("Failed to open window!");
    let video = surface.sdl().video().expect("Failed to acquire video system!");
    let swap_interval = sdl2::video::SwapInterval::Immediate;
    video.gl_set_swap_interval(swap_interval).expect("Failed to set window swap interval!");

    //IMGUI initialization
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &surface.window());
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

    let mut start_t = Instant::now();
    let mut deltatime = 0.0; //In seconds
    let bg_color = [0.25, 0.25, 0.25, 1.0];

    {
        let mut state = crate::pipeline_state.lock().expect("Unable to acquire lock!");
        (*state).clear_color = bg_color;
    }

    //Temporary hardcoded program
    let mut program = graphics::g2d::get_default_program(&mut surface);

    let mut event_pump = surface.sdl().event_pump().expect("Failed to create event pump!");
    'running: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) { continue; }

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        //Call game update
        lua_api::call_rock_func(&lua, "update", deltatime).expect("Failed to call `rock.update`");

        //Prepare IMGUI
        imgui_sdl2.prepare_frame(imgui.io_mut(), &surface.window(), &event_pump.mouse_state());
        let ui = imgui.frame();
        let perf_window = imgui::Window::new(im_str!("Performance"))
                    .position([5.0, 5.0], imgui::Condition::Appearing)
                    .size([180.0, 80.0], imgui::Condition::Appearing)
                    .resizable(true)
                    .title_bar(true);
        perf_window.build(&ui, || {
            ui.text(im_str!("FPS: {:.2} ({:.1}ms)", 1.0 / deltatime, deltatime * 1000.0));
        });
        let debug_window = imgui::Window::new(im_str!("Debug"))
                    .position([5.0, 90.0], imgui::Condition::Appearing)
                    .size([180.0, 80.0], imgui::Condition::Appearing)
                    .resizable(true)
                    .title_bar(true);
        debug_window.build(&ui, || {
            ui.text("Nothing here yet :)");
        });

        //Call game draw
        lua_api::call_rock_func(&lua, "draw", 0).expect("Failed to call `rock.draw`");

        //Render application
        let back_buffer = surface.back_buffer().expect("Failed to get backbuffer!");

        let render = surface.new_pipeline_gate().pipeline(
            &back_buffer,
            &pipeline_state.lock().unwrap(),
            |_pipeline, mut shd_gate| {
                shd_gate.shade(&mut program, |_, _, mut rdr_gate| {
                    rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                        Ok(())
                    })
                })
            },
        ).assume();

        if !render.is_ok() {
            error!("Renderer ran into unknown error!");
            break 'running;
        }

        //Render IMGUI
        imgui_sdl2.prepare_render(&ui, &surface.window());
        renderer.render(ui);

        deltatime = start_t.elapsed().as_secs_f32();
        start_t = Instant::now();
        // warn!("dt: {}", deltatime);
        // surface.window().set_title(&format!("fps: {:.2}", 1.0 / deltatime));
        imgui.io_mut().delta_time = deltatime;
        surface.window().gl_swap_window();
    }
}
