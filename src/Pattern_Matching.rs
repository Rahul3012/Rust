enum TrafficLight {
  Red,
  Yellow,
  Green
}

fn main() {
  let light = TrafficLight::Red;

  match light {
    TrafficLight::Red => println!("stop"),
    TrafficLight::Green => println!("go"),
    TrafficLight::Yellow => println!("Slow Down")
  }
}
    
