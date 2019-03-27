use vertex::Vertex;

use rand::Rng;

/// Represents a single equation in an IFS
/// Contains equations parameters and the probability value
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Eqn {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub p: f32,
}

impl Eqn {
    pub fn eval(&self, v: Vertex) -> Vertex {
        Vertex {
            position: [
                self.a * v.position[0] + self.b * v.position[1] + self.e,
                self.c * v.position[0] + self.d * v.position[1] + self.f,
            ],
            hue: v.hue,
        }
    }
}

/// Represents a set of equations with probabilities. The sum value is set to
/// the sum of the probability values for each equation so that the individual
/// Eqn's probabilities need not sum to 1.
#[derive(Clone, Debug)]
pub struct IFS {
    pub eqns: Vec<Eqn>,
    sum: f32,
}

impl IFS {
    pub fn new(e: Vec<Eqn>) -> IFS {
        let mut l = IFS { eqns: e, sum: 0.0 };
        l.update_sum();
        l
    }

    fn choose<R: Rng>(&self, rng: &mut R) -> Eqn {
        let p = rng.gen::<f32>() * self.sum;
        let mut sum = 0.0;
        for &eq in &self.eqns {
            if p - sum < eq.p {
                return eq;
            }
            sum += eq.p;
        }
        unreachable!();
    }

    fn update_sum(&mut self) {
        let sum = self.eqns.iter().map(|l| l.p).sum();
        self.sum = sum;
    }

    pub fn generate(&mut self, v: &mut Vec<Vertex>) {
        use rand::SeedableRng;
        self.update_sum();
        let mut rng = ::rand::rngs::SmallRng::from_seed([0; 16]);
        let hue_increment = 1.0 / v.len() as f32;
        let mut current = Vertex {
            position: [0.0, 0.0],
            hue: 0.0,
        };
        for point in v.iter_mut() {
            current = self.choose(&mut rng).eval(current);
            current.hue += hue_increment;
            *point = current;
        }
    }
}
