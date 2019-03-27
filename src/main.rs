#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate rand;

mod gui;
mod ifs;
mod vertex;

fn main() {
    use glium::{glutin, Surface};
    use gui::{draw_gui, MouseState, State};
    use imgui::ImGui;
    use imgui_glium_renderer::Renderer;
    use std::collections::VecDeque;
    use std::time::Instant;
    use vertex::Vertex;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display =
        glium::Display::new(window, context, &events_loop).expect("Failed to initialize display");

    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);
    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let vertex_shader_src = include_str!("shaders/ifs.vert");

    let fragment_shader_src = include_str!("shaders/ifs.frag");

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .expect("Failed to build program");

    let mut closed = false;
    let mut scale: f32 = 1.0;
    let mut xpos: f32 = 0.0;
    let mut ypos: f32 = 0.0;

    let mut last_frame = Instant::now();
    let mut mouse_state = MouseState::default();
    let mut state = State::default();
    let mut num_points = state.num_points as usize;
    let mut fpsbuf = VecDeque::new();
    let mut fract: Vec<Vertex> = vec![Vertex::default(); num_points];

    while !closed {
        events_loop.poll_events(|event| {
            use glium::glutin::dpi::LogicalPosition;
            use glium::glutin::ElementState::Pressed;
            use glium::glutin::WindowEvent::*;
            use glium::glutin::{MouseButton, MouseScrollDelta, TouchPhase};
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    CloseRequested => closed = true,
                    KeyboardInput {
                        input,
                        ..
                    } => {
                        if let Some(v) = input.virtual_keycode {
                            match v {
                                // Divide by scale so moving feels uniform
                                glutin::VirtualKeyCode::Up => ypos += 0.05 / scale,
                                glutin::VirtualKeyCode::Down => ypos -= 0.05 / scale,
                                glutin::VirtualKeyCode::Right => xpos += 0.05 / scale,
                                glutin::VirtualKeyCode::Left => xpos -= 0.05 / scale,
                                glutin::VirtualKeyCode::Q => scale *= 1.10,
                                glutin::VirtualKeyCode::Z => scale *= 0.9,
                                _ => (),
                            }
                        }
                    }
                    CursorMoved {
                        position: LogicalPosition { x, y },
                        ..
                    } => mouse_state.pos = (x as f32, y as f32),
                    MouseInput { state, button, .. } => match button {
                        MouseButton::Left => mouse_state.pressed.0 = state == Pressed,
                        MouseButton::Right => mouse_state.pressed.1 = state == Pressed,
                        MouseButton::Middle => mouse_state.pressed.2 = state == Pressed,
                        _ => {}
                    },
                    MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        phase: TouchPhase::Moved,
                        ..
                    } => mouse_state.wheel = y,
                    MouseWheel {
                        delta: MouseScrollDelta::PixelDelta(pos),
                        phase: TouchPhase::Moved,
                        ..
                    } => mouse_state.wheel = pos.y as f32,
                    _ => (),
                }
            }
        });

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        if delta_s < 1.0 / 60.0 {
            std::thread::sleep(std::time::Duration::from_millis(
                (1000.0 * (1.0 / 60.0)) as u64,
            ));
        }

        fpsbuf.push_back(1.0 / delta_s);
        if fpsbuf.len() > 10 {
            fpsbuf.pop_front();
        }
        state.fps = fpsbuf.iter().sum::<f32>() / fpsbuf.len() as f32;

        mouse_state.update_imgui(&mut imgui);

        // Generate fractal
        if num_points != state.num_points as usize {
            num_points = state.num_points as usize;
            fract = vec![Vertex::default(); num_points];
        }
        let mut sys = state.get_sys();
        sys.generate(&mut fract);
        let vertex_buffer = glium::VertexBuffer::new(&display, &fract).expect("vertex buffer");
        // Translate/scale matrix
        let transform = [
            [scale, 0.0, -xpos * scale],
            [0.0, scale, -ypos * scale],
            [0.0, 0.0, scale],
        ];

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform! { transform: transform },
                &Default::default(),
            )
            .expect("Fractal draw failed");

        // Draw GUI
        let gl_window = display.gl_window();
        let size_pixels = gl_window.get_inner_size().unwrap();
        let hidpi = gl_window.get_hidpi_factor();
        let frame_size = imgui::FrameSize::new(size_pixels.width, size_pixels.height, hidpi);
        let ui = imgui.frame(frame_size, delta_s);
        draw_gui(&ui, &mut state);
        renderer.render(&mut target, ui).expect("Rendering failed");

        target.finish().unwrap();
    }
}
