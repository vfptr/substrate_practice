#[derive(Debug)]
//A traffic singal enum, with a field as duration of the lights.
pub enum Signal {
    Green(usize),
    Yellow(usize),
    Red(usize),
}

//trait for geting the duration of the traffic lights
pub trait LightsDuration {
    fn duration(&self) -> usize;
}

impl LightsDuration for Signal {
    fn duration(&self) -> usize {
        match self {
            Signal::Red(v) => *v, 
            Signal::Yellow(v) => *v,
            Signal::Green(v) => *v
        }
    }
}
