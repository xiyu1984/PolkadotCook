pub trait AreaCalc {
    fn get_area(&self) -> u64 ;
}

pub struct Round {

}

impl AreaCalc for Round {
    fn get_area(&self) -> u64 {
        128
    }
}

pub fn calculate_area<T: AreaCalc>(instance: T) -> u64 {
    instance.get_area()
}