use ifs::{Eqn, IFS};

#[derive(Debug, Clone)]
pub struct State(pub IFS);

impl Default for State {
    fn default() -> State {
        State(IFS::new(vec![
            Eqn {a: 85.0, b: 4.0, c: -4.0, d: 85.0, e: 0.0, f: 160.0, p: 85.0 },
            Eqn {a: 0.0, b: 0.0, c: 0.0, d: 16.0, e: 0.0, f: 0.0, p: 1.0 },
            Eqn {a: 20.0, b: -26.0, c: 23.0, d: 22.0, e: 0.0, f: 160.0, p: 7.0 },
            Eqn {a: -15.0, b: 28.0, c: 26.0, d: 24.0, e: 0.0, f: 44.0, p: 7.0 },
        ]))
    }
}

impl State {
    pub fn get_sys(&self) -> IFS {
        IFS::new(self.0.eqns.iter().map(|e| norm(*e)).collect())
    }
}

fn norm(e: Eqn) -> Eqn {
    Eqn { a: e.a / 100.0, b: e.b / 100.0, c: e.c / 100.0,
          d: e.d / 100.0, e: e.e / 100.0, f: e.f / 100.0, .. e}
}
