
/*
实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束.
*/

use std::f64::consts::PI;

pub trait Area { 
    fn calculate_area(&self) -> f64; 
}

fn get_area<T:Area>(polygon:&T) -> f64{
    return polygon.calculate_area();
}



struct Circle { 
    radius: f64 
}

impl Area for Circle { 
    fn calculate_area(&self) -> f64 {
        return PI * self.radius * self.radius;
    } 
}

struct Square { 
    side: f64 
}

impl Area for Square { 
    fn calculate_area(&self) -> f64 {
        return self.side * self.side;
    } 
}



pub fn problem3(){
    let c = Circle{radius: 1.0};
    let s = Square{side: 2.0};
    println!("Circle Area: {}", get_area(&c));
    println!("Square Area: {}", get_area(&s));
}