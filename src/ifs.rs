use vertex::Vertex;

#[derive(Copy, Clone, Debug)]
pub struct Eqn {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32
}

impl Eqn {
    pub fn eval(&self, v: Vertex) -> Vertex {
        Vertex { position: [self.a * v.position[0] + self.b * v.position[1] + self.e,
                            self.c * v.position[0] + self.d * v.position[1] + self.f],
                hue: v.hue }
    }
}

#[derive(Clone, Debug)]
pub struct IFS {
    pub eqns: Vec<(Eqn, f32)>
}

impl IFS {
    pub fn choose(&self) -> Eqn {
        let p = ::rand::random::<f32>();
        let mut sum = 0.0;
        for &(eq, prob) in &self.eqns {
            if p - sum < prob {
                return eq;
            }
            sum += prob;
        }
        unreachable!();
    }
}
