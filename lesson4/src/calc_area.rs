pub trait AreaCalc {
    fn get_area(&self) -> f32 ;
}

pub struct Round {
    pub radius: f32,
}

impl AreaCalc for Round {
    fn get_area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

pub fn calculate_area<T: AreaCalc>(instance: T) -> f32 {
    instance.get_area()
}