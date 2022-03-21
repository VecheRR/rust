#[allow(overflowing_literals)]
#[warn(unused_variables)]

fn main() {
    let a = 65.4321_f32;

    let _b = a as u32;
    let c = (-1i8) as u8;
    
    println!("(-1) in u8: {}", c);

}