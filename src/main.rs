#[macro_use]
extern crate glium;

extern crate rand;

mod ifs;
mod vertex;
mod gui;

use gui::State;

use vertex::Vertex;
use ifs::IFS;
use ifs::Eqn;
use std::thread;

use std::time::Instant;

#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

use imgui::{ImGui, Ui, ImGuiCond, ImString};
use imgui_glium_renderer::Renderer;


#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    pressed: (bool, bool, bool),
    wheel: f32,
}

fn ui_eqn<'a>(ui: &Ui<'a>, eqn: &mut Eqn, id: usize) {
    ui.slider_float(im_str!("{}a", id), &mut eqn.a, -100.0, 100.0)
                    .display_format(im_str!("a: %.0f"))
                    .build();
    ui.slider_float(im_str!("{}b", id), &mut eqn.b, -100.0, 100.0)
                .display_format(im_str!("b: %.0f"))
                .build();
    ui.slider_float(im_str!("{}c", id), &mut eqn.c, -100.0, 100.0)
                .display_format(im_str!("c: %.0f"))
                .build();
    ui.slider_float(im_str!("{}d", id), &mut eqn.d, -100.0, 100.0)
                .display_format(im_str!("d: %.0f"))
                .build();
    ui.slider_float(im_str!("{}e", id), &mut eqn.e, -100.0, 100.0)
                .display_format(im_str!("e: %.0f"))
                .build();
    ui.slider_float(im_str!("{}f", id), &mut eqn.f, -100.0, 100.0)
                .display_format(im_str!("f: %.0f"))
                .build();
    ui.slider_float(im_str!("{}p", id), &mut eqn.p, 1.0, 50.0)
                        .display_format(im_str!("%.0f"))
                        .build();
}

fn draw_gui<'a>(ui: &Ui<'a>, state: &mut State) {
    ui.window(im_str!("Equation Parameters"))
        .size((300.0, 500.0), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.text(im_str!("x = a * x + b * y + e"));
            ui.text(im_str!("y = c * x + d * y + f"));
            ui.separator();
            for (i, mut eq) in state.0.eqns.iter_mut().enumerate() {
                if ui.collapsing_header(im_str!("Eqn {}", i)).build() {
                    ui_eqn(ui, &mut eq, i);
                }
            }
        });
}

fn update_mouse(imgui: &mut ImGui, mouse_state: &mut MouseState) {
    let scale = imgui.display_framebuffer_scale();
    imgui.set_mouse_pos(
        mouse_state.pos.0 as f32 / scale.0,
        mouse_state.pos.1 as f32 / scale.1,
    );
    imgui.set_mouse_down(
        &[
            mouse_state.pressed.0,
            mouse_state.pressed.1,
            mouse_state.pressed.2,
            false,
            false,
        ],
    );
    imgui.set_mouse_wheel(mouse_state.wheel / scale.1);
    mouse_state.wheel = 0.0;
}

fn main() {
    const NUM_POINTS: usize = 1_000_000;

    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);
    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let vertex_shader_src = r#"
        #version 330
        in vec2 position;
        in float hue;
        out float v_hue;

        uniform mat3 transform;

        void main() {
            v_hue = hue;
            vec3 pos = vec3(position, 1.0) * transform;
            gl_Position = vec4(pos.xy, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330
        in float v_hue;
        out vec4 color;

        vec3 hsv2rgb(vec3 c) {
            vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
            vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
            return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
        }

        void main() {
            color = vec4(hsv2rgb(vec3(1-v_hue, 0.8, 0.8)), 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut closed = false;
    let mut scale: f32 = 1.0;
    let mut xpos: f32 = 0.0;
    let mut ypos: f32 = 0.0;

    let mut last_frame = Instant::now();
    let mut mouse_state = MouseState::default();
    let mut state = State::default();

    while !closed {
        events_loop.poll_events(|event| {
            use glium::glutin::WindowEvent::*;
            use glium::glutin::ElementState::Pressed;
            use glium::glutin::MouseButton;
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    Closed => closed = true,
                    KeyboardInput { input, device_id: _ } => {
                        if let Some(v) = input.virtual_keycode {
                            match v {
                                glutin::VirtualKeyCode::Up => ypos += 0.05 * scale,
                                glutin::VirtualKeyCode::Down => ypos -= 0.05 * scale,
                                glutin::VirtualKeyCode::Right => xpos += 0.05 * scale,
                                glutin::VirtualKeyCode::Left => xpos -= 0.05 * scale,
                                glutin::VirtualKeyCode::Q => scale *= 1.10,
                                glutin::VirtualKeyCode::Z => scale *= 0.9,
                                _ => ()
                            }
                        }
                    },
                    CursorMoved { position: (x, y), .. } => mouse_state.pos = (x as i32, y as i32),
                    MouseInput { state, button, .. } => {
                        match button {
                            MouseButton::Left => mouse_state.pressed.0 = state == Pressed,
                            MouseButton::Right => mouse_state.pressed.1 = state == Pressed,
                            MouseButton::Middle => mouse_state.pressed.2 = state == Pressed,
                            _ => {}
                        }
                    }
                    _ => ()
                }
                _ => (),
            }
        });

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        update_mouse(&mut imgui, &mut mouse_state);

        let gl_window = display.gl_window();
        let size_points = gl_window.get_inner_size_points().unwrap();
        let size_pixels = gl_window.get_inner_size_pixels().unwrap();

        let ui = imgui.frame(size_points, size_pixels, delta_s);

        let mut target = display.draw();

        let sys = state.get_sys();
        let fract = sys.generate(NUM_POINTS);
        let vertex_buffer = glium::VertexBuffer::new(&display, &fract).unwrap();

        let transform = [[scale, 0.0, -xpos],
                         [0.0, scale, -ypos],
                         [0.0, 0.0, scale]] ;

        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program,
                    &uniform! { transform: transform },
                    &Default::default()).unwrap();
        draw_gui(&ui, &mut state);
        renderer.render(&mut target, ui).expect("Rendering failed");
        target.finish().unwrap();
    }
}
