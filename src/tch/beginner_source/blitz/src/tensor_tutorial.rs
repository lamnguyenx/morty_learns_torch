#![allow(unused_must_use)]

/*
Tensors
========

Tensors are a specialized data structure that are very similar to arrays
and matrices. In PyTorch, we use tensors to encode the inputs and
outputs of a model, as well as the model’s parameters.

Tensors are similar to NumPy’s ndarrays, except that tensors can run on
GPUs or other specialized hardware to accelerate computing. If you’re familiar with ndarrays, you’ll
be right at home with the Tensor API. If not, follow along in this quick
API walkthrough.

*/
use local_utils::dbgts;
use tch::Tensor;

pub fn tensor_tutorial() {
    // #####################################################################
    // Tensor Initialization
    // ~~~~~~~~~~~~~~~~~~~~~

    // Tensors can be initialized in various ways. Take a look at the following examples:
    //
    // **Directly from data**
    //
    // Tensors can be created directly from data. The data type is automatically inferred.

    let device = utils_tch::get_device_from_current_env().unwrap();
    dbg!(device);

    // new tensor from array with inherited shape
    // N/A

    // new tensor from iterables (vec, array..)
    let iter_vec = (-30..30).collect::<Vec<i16>>();
    dbgts!(Tensor::from_slice(&iter_vec).to(device));

    let iter_arr: [i16; 60] = std::array::from_fn(|i| (i as i16 - 30));
    dbgts!(Tensor::from_slice(&iter_arr).to(device));

    // new tensor from vec with specified shape
    let data = (-30..30).collect::<Vec<i16>>();
    let shape = [3, 4, 5];
    dbgts!(Tensor::from_slice(&data).reshape(&shape).to(device))
}
