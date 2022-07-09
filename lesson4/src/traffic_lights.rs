pub enum TrafficLight {
    Green,
    Yellow,
    Red,
}

pub trait Duration {
    fn get_duration(&self)-> u32;
}

impl Duration for TrafficLight {
    fn get_duration(&self) -> u32 {
        match self {
            TrafficLight::Green => 60,
            TrafficLight::Red => 45,
            TrafficLight::Yellow => 3,
        }
    }
}