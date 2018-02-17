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
    pub eqns: Vec<(Eqn, f32)>,
    sum: f32
}

impl IFS {
    pub fn new(e: Vec<(Eqn, f32)>) -> IFS {
        let mut l = IFS { eqns: e, sum: 0.0 };
        l.update();
        l
    }

    pub fn choose(&self) -> Eqn {
        let p = ::rand::random::<f32>() * self.sum;
        let mut sum = 0.0;
        for &(eq, prob) in &self.eqns {
            if p - sum < prob {
                return eq;
            }
            sum += prob;
        }
        unreachable!();
    }

    pub fn update(&mut self) {
        let sum = self.eqns.iter().map(|l| l.1).sum();
        self.sum = sum;
    }
}
