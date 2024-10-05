use candle_core::{Device, Tensor};

pub fn tensor_tutorial() {
    println!("-----------------------------------------------");
    let x = Tensor::new(&[1.0, 2.0], &Device::Cpu).unwrap();
    dbg!(x);
}
