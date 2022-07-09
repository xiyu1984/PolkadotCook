mod traffic_lights;
use traffic_lights::Duration;

mod make_sum;
mod calc_area;

fn main() {
    let traffic_light = traffic_lights::TrafficLight::Green;
    println!("Light Green for {} seconds.", traffic_light.get_duration());

    println!("{:?}", make_sum::checked_sum(&[1, 2, 3, 4]));
    println!("{:?}", make_sum::checked_sum(&[1, 2, u32::MAX, 3]));

    let my_round = calc_area::Round{};
    println!("The area is: {}", calc_area::calculate_area(my_round));
}
