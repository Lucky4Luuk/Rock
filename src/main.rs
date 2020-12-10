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

use glam::*;

pub mod lua_api;
pub mod graphics;
pub mod math;

use lua_api::LuaApi;
use graphics::{ShaderProgram, Camera, CameraMode};
use math::Transform;

static mut ROCK: Option<Rock> = None;

pub struct Rock {
    pub pipeline_state: PipelineState,
    pub surface: GL33Surface,
    pub lua: LuaApi,
    pub imgui: imgui::Context,
    pub imgui_sdl2: imgui_sdl2::ImguiSdl2,
    pub renderer: imgui_opengl_renderer::Renderer,

    //Runtime variables
    pub default_program: ShaderProgram,
    pub cur_program: ShaderProgram,
    pub camera: Camera,
}

impl Rock {
    pub fn new() -> Self {
        let lua = lua_api::init_lua();
        lua_api::load_code(&lua, "print(\"hello from lua!\")").exec().expect("Failed to run lua code!");

        lua_api::load_code(&lua, include_str!("test.lua")).exec().expect("Failed to load lua code!");

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
        let imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &surface.window());
        let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

        //Default shader program. 2nd program is because program doesn't implement `Clone`
        let program = graphics::get_default_program(&mut surface);
        let program2 = graphics::get_default_program(&mut surface);

        let cam_pos = Vec3::new(0.0,0.0,-2.0);
        let cam_rot = Quat::from_rotation_ypr(0.0, 0.0, 0.0);
        let cam_scale = Vec3::new(1.0, 1.0, 1.0); //Useless but needed
        let cam_transform = Transform::new(cam_pos, cam_rot, cam_scale);

        let camera = Camera::new(CameraMode::Perspective, cam_transform, 60.0 / 180.0 * 3.14);

        Rock {
            pipeline_state: PipelineState::default(),
            surface: surface,
            lua: lua,
            imgui: imgui,
            imgui_sdl2: imgui_sdl2,
            renderer: renderer,

            default_program: program,
            cur_program: program2,
            camera: camera,
        }
    }

    pub fn get_render_state(&self) -> PipelineState {
        self.pipeline_state.clone()
            .enable_clear_color(false)
            .enable_clear_depth(false)
    }
}

fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::max())
        .init();

    debug!("Hello, world!");

    unsafe {
        ROCK = Some(Rock::new());
    }

    let mut start_t = Instant::now();
    let mut deltatime = 0.0; //In seconds
    let bg_color = [0.25, 0.25, 0.25, 1.0];

    // let triangle = unsafe { graphics::g2d::primitives::create_triangle(&mut ROCK.as_mut().unwrap().surface) };

    unsafe { if let Some(ref mut rock) = ROCK { rock.pipeline_state = rock.pipeline_state.clone().set_clear_color(bg_color); } }

    //Call program load
    unsafe { lua_api::call_rock_func(&ROCK.as_ref().unwrap().lua, "load", 0).expect("Failed to call `rock.load`"); }

    let mut event_pump = unsafe { ROCK.as_mut().unwrap().surface.sdl().event_pump().expect("Failed to create event pump!") };
    'running: loop {
        for event in event_pump.poll_iter() {
            unsafe {
                ROCK.as_mut().unwrap().imgui_sdl2.handle_event(&mut ROCK.as_mut().unwrap().imgui, &event);
                if ROCK.as_mut().unwrap().imgui_sdl2.ignore_event(&event) { continue; }
            }

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        //Call game update
        unsafe { lua_api::call_rock_func(&ROCK.as_ref().unwrap().lua, "update", deltatime).expect("Failed to call `rock.update`"); }

        //Prepare IMGUI
        unsafe { ROCK.as_mut().unwrap().imgui_sdl2.prepare_frame(ROCK.as_mut().unwrap().imgui.io_mut(), &ROCK.as_mut().unwrap().surface.window(), &event_pump.mouse_state()); }
        let ui = unsafe { ROCK.as_mut().unwrap().imgui.frame() };
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

        //Get back buffer
        let back_buffer = unsafe { ROCK.as_mut().unwrap().surface.back_buffer().expect("Failed to get backbuffer!") };

        //Clear screen with correct color
        let render = unsafe { ROCK.as_mut().unwrap().surface.new_pipeline_gate().pipeline(
            &back_buffer,
            &ROCK.as_mut().unwrap().pipeline_state,
            |_, _| Ok(()),
        ).assume() };

        if !render.is_ok() {
            error!("Renderer ran into unknown error!");
            break 'running;
        }

        //Call game draw
        unsafe { lua_api::call_rock_func(&ROCK.as_ref().unwrap().lua, "draw", 0).expect("Failed to call `rock.draw`"); }

        //Render IMGUI
        unsafe {
            ROCK.as_mut().unwrap().imgui_sdl2.prepare_render(&ui, &ROCK.as_mut().unwrap().surface.window());
            ROCK.as_ref().unwrap().renderer.render(ui);
        }

        deltatime = start_t.elapsed().as_secs_f32();
        start_t = Instant::now();
        // warn!("dt: {}", deltatime);
        // surface.window().set_title(&format!("fps: {:.2}", 1.0 / deltatime));
        unsafe {
            ROCK.as_mut().unwrap().imgui.io_mut().delta_time = deltatime;
            ROCK.as_mut().unwrap().surface.window().gl_swap_window();
        }
    }
}
