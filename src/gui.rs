use ifs::{Eqn, IFS};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct State {
    pub e1: Eqn,
    pub p1: f32,
    pub e2: Eqn,
    pub p2: f32,
    pub e3: Eqn,
    pub p3: f32,
    pub e4: Eqn,
    pub p4: f32,
}

fn norm(e: Eqn) -> Eqn {
    Eqn { a: e.a / 100.0, b: e.b / 100.0, c: e.c / 100.0,
          d: e.d / 100.0, e: e.e / 100.0, f: e.f / 100.0}
}

impl State {
    pub fn get_sys(&self) -> IFS {
        IFS::new(vec![
            (norm(self.e1), self.p1),
            (norm(self.e2), self.p2),
            (norm(self.e3), self.p3),
            (norm(self.e4), self.p4)
        ])
    }
}