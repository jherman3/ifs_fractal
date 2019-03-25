use ifs::{Eqn, IFS};

use imgui::{ImGui, ImGuiCond, Ui};

/// State controllable by GUI
///
/// Internally represents the IFS as an IFS struct but with all values scaled
/// by 100 because ImGui slider_float's can't have a step specified yet
#[derive(Debug, Clone)]
pub struct State {
    pub sys: IFS,
    pub num_points: f32,
    pub fps: f32,
}

impl Default for State {
    fn default() -> State {
        State {
            sys: IFS::new(vec![
                Eqn {
                    a: 85.0,
                    b: 4.0,
                    c: -4.0,
                    d: 85.0,
                    e: 0.0,
                    f: 160.0,
                    p: 85.0,
                },
                Eqn {
                    a: 0.0,
                    b: 0.0,
                    c: 0.0,
                    d: 16.0,
                    e: 0.0,
                    f: 0.0,
                    p: 1.0,
                },
                Eqn {
                    a: 20.0,
                    b: -26.0,
                    c: 23.0,
                    d: 22.0,
                    e: 0.0,
                    f: 160.0,
                    p: 7.0,
                },
                Eqn {
                    a: -15.0,
                    b: 28.0,
                    c: 26.0,
                    d: 24.0,
                    e: 0.0,
                    f: 44.0,
                    p: 7.0,
                },
            ]),
            num_points: 1_000_000.0,
            fps: 0.0,
        }
    }
}

impl State {
    pub fn get_sys(&self) -> IFS {
        IFS::new(self.sys.eqns.iter().map(|e| norm(*e)).collect())
    }
}

fn norm(e: Eqn) -> Eqn {
    Eqn {
        a: e.a / 100.0,
        b: e.b / 100.0,
        c: e.c / 100.0,
        d: e.d / 100.0,
        e: e.e / 100.0,
        f: e.f / 100.0,
        ..e
    }
}

/// Helper to draw a Eqn's sliders
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
    ui.slider_float(im_str!("{}p", id), &mut eqn.p, 1.0, 100.0)
        .display_format(im_str!("%.0f"))
        .build();
}

/// Main GUI draw function
pub fn draw_gui<'a>(ui: &Ui<'a>, state: &mut State) {
    ui.window(im_str!("Equation Parameters"))
        .size((300.0, 500.0), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.text(im_str!("x = a * x + b * y + e"));
            ui.text(im_str!("y = c * x + d * y + f"));
            ui.slider_float(
                im_str!("NumPoints"),
                &mut state.num_points,
                0.0,
                10_000_000.0,
            )
            .power(10.0)
            .display_format(im_str!("a: %.0f"))
            .build();
            ui.text(im_str!("FPS: {:.1}", state.fps));
            ui.separator();
            if ui.small_button(im_str!("Add Equation")) {
                state.sys.eqns.push(Eqn::default());
            }
            let mut del = None;
            for (i, mut eq) in state.sys.eqns.iter_mut().enumerate() {
                if ui.collapsing_header(im_str!("Eqn {}", i)).build() {
                    ui_eqn(ui, &mut eq, i);
                    if ui.small_button(im_str!("Delete##eqn{}", i)) {
                        del = Some(i);
                        break;
                    }
                }
            }
            if let Some(i) = del {
                state.sys.eqns.remove(i);
            }
        });
}

/// Keeps track of mouse position for ImGui
/// Coordinates are in LogicalPosition so we don't have to deal with hidpi
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct MouseState {
    pub pos: (f32, f32),
    pub pressed: (bool, bool, bool),
    pub wheel: f32,
}

impl MouseState {
    /// Sets ImGui's mouse state to match the MouseState struct
    pub fn update_imgui(&mut self, imgui: &mut ImGui) {
        imgui.set_mouse_pos(self.pos.0, self.pos.1);
        imgui.set_mouse_down([self.pressed.0, self.pressed.1, self.pressed.2, false, false]);
        imgui.set_mouse_wheel(self.wheel);
        self.wheel = 0.0;
    }
}
