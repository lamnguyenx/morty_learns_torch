use candle_core::{Device, Tensor};

fn main() {
    println!("-----------------------------------------------");
    let x = Tensor::new(&[1.0, 2.0], &Device::Cpu).unwrap();
    dbg!(x);
}
