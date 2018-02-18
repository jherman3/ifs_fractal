use vertex::Vertex;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Eqn {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub p: f32
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
    pub eqns: Vec<Eqn>,
    sum: f32
}

impl IFS {
    pub fn new(e: Vec<Eqn>) -> IFS {
        let mut l = IFS { eqns: e, sum: 0.0 };
        l.update();
        l
    }

    pub fn choose(&self) -> Eqn {
        let p = ::rand::random::<f32>() * self.sum;
        let mut sum = 0.0;
        for &eq in &self.eqns {
            if p - sum < eq.p {
                return eq;
            }
            sum += eq.p;
        }
        unreachable!();
    }

    pub fn update(&mut self) {
        let sum = self.eqns.iter().map(|l| l.p).sum();
        self.sum = sum;
    }

    pub fn generate(&self, n: usize) -> Vec<Vertex> {
        let mut fract: Vec<Vertex> = Vec::new();
        let mut last = Vertex {position: [0.0, 0.0], hue: fract.len() as f32 / n as f32};
        fract.push(last);
        for _ in 0..n {
            last = self.choose().eval(last);
            last.hue = fract.len() as f32 / n as f32;
            fract.push(last);
        }
        fract
    }
}
