pub struct PetriNetStage {
    tokens: Vec<i32>,
}

impl PetriNetStage {
    pub fn new(v: Vec<i32>) -> Option<PetriNetStage> {
        for i in &v {
            if *i < 0 {
                return None;
            }
        }
        Some(PetriNetStage {
            tokens: v,
        })
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct PetriNetTransition {
    coeffs: Vec<i32>,
}

impl PetriNetTransition {
    pub fn new(coeffs: Vec<i32>) -> PetriNetTransition {
        PetriNetTransition { coeffs }
    }

    pub fn id(size: usize) -> PetriNetTransition {
        PetriNetTransition { coeffs: vec![0; size] }
    }

    pub fn compose(a: PetriNetTransition, b: PetriNetTransition) -> Option<PetriNetTransition> {
        if !a.coeffs.iter().zip(b.coeffs.iter()).all(|(a, b)| b >= &0 || a + b >= 0) {
            None
        } else {
                Some(PetriNetTransition {
                coeffs: a.coeffs
                    .into_iter()
                    .zip(b.coeffs.into_iter())
                    .map(|(a, b)| a + b)
                    .collect()
            })
        }
    }

    pub fn is_inverse(a: &PetriNetTransition, b: &PetriNetTransition) -> bool {
        a.coeffs
            .iter()
            .zip(b.coeffs.iter())
            .all(|(a, b)| a == b)
    }

    pub fn check_duality(&self, colors: &Vec<Color>) -> bool {
        self.coeffs
            .iter()
            .zip(colors.iter())
            .fold((0, 0), |(red, blue), (curr, col)| {
                match col {
                    &Color::None => (red, blue),
                    &Color::Red => (red + (*curr), blue),
                    &Color::Blue => (red, blue + (*curr)),
                    &Color::Purple => (red + (*curr), blue + (*curr)),
                }
            }) == (0, 0)
    }
}

pub enum Color {
    None,
    Red,
    Blue,
    Purple,
}

pub struct PetriNet {
    size: usize,
    transitions: Vec<PetriNetTransition>,
}

impl PetriNet {
    pub fn new(size: usize, transitions: Vec<PetriNetTransition>) -> PetriNet {
        PetriNet { size, transitions }
    }

    pub fn check_reversibility(&self, reverse: Vec<Vec<PetriNetTransition>>) -> bool {
        for i in reverse.clone().concat() {
            if let None = self.transitions.iter().find(|&r| r == &i) {
                return false;
            }
        }

        let compositions = reverse
            .into_iter()
            .map(|x| {
                x
                    .into_iter()
                    .fold(Some(PetriNetTransition::id(self.size)), |a, b| {
                        a.map(|a| PetriNetTransition::compose(a, b))?
                    })
            })
            .collect::<Option<Vec<PetriNetTransition>>>();
        
        if let Some(compositions) = compositions {
            for i in 0..self.size {
                if !PetriNetTransition::is_inverse(&self.transitions[i], &compositions[i]) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn check_duality(&self, colors: &Vec<Color>) -> bool {
        self.transitions.iter().all(|x| x.check_duality(colors))
    }
}