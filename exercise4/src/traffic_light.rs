#[derive(Debug)]
enum TrafficLight{
     Red, 
     Green, 
     Yellow,
}

pub trait LightTime {
     fn light(&self) -> i32; 
}

impl LightTime for TrafficLight {
     fn light(&self) -> i32 {
          match self {
               TrafficLight::Red => 20,
               TrafficLight::Green => 25,
               TrafficLight::Yellow => 5,
          }
          
     }
}

pub fn problem1(){
     let l = TrafficLight::Yellow;
     let light_time = l.light();
     println!("{:?} lighting for {}s", l, light_time );
}