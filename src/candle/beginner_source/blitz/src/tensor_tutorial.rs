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
use candle_core::Tensor;
use kits::candle_kit::dbgts;

pub fn tensor_tutorial() {
    // #####################################################################
    // Tensor Initialization
    // ~~~~~~~~~~~~~~~~~~~~~

    // Tensors can be initialized in various ways. Take a look at the following examples:
    //
    // **Directly from data**
    //
    // Tensors can be created directly from data. The data type is automatically inferred.

    let device = kits::candle_kit::get_device_from_current_env().unwrap();

    // new tensor from array with inherited shape
    let arr_2d = [[1_u32, 2], [3, 4]];
    dbgts!(Tensor::new(&arr_2d, &device).unwrap());

    // new tensor from iterables (vec, array..)
    let iter_vec = (0..60).collect::<Vec<u32>>();
    dbgts!(Tensor::from_iter(iter_vec, &device).unwrap());

    let iter_arr: [u32; 60] = std::array::from_fn(|i| i as u32);
    dbgts!(Tensor::from_iter(iter_arr, &device).unwrap());

    // new tensor from vec with specified shape
    let data = (0..60).collect::<Vec<u32>>();
    let shape = (3, 4, 5);
    let tensor_3d = dbgts!(Tensor::from_vec(data, shape, &device).unwrap());

    // ##############################################################
    //  **From another tensor:**
    //
    //  The new tensor retains the properties (shape, datatype) of the argument tensor, unless explicitly overridden.
    dbgts!(Tensor::ones_like(&tensor_3d).unwrap());
    dbgts!(Tensor::rand_like(
        &tensor_3d.to_dtype(candle_core::DType::F16).unwrap(),
        -1.0,
        1.0
    )
    .unwrap()); // error if no conversion to float

    // #####################################################################
    //  **With random or constant values:**
    //
    //  ``shape`` is a tuple of tensor dimensions. In the functions below, it determines the dimensionality of the output tensor.
    let shape = [3, 4, 5];
    dbgts!(Tensor::rand(-1.0, 1.0, &shape, &device).unwrap());
    dbgts!(Tensor::ones(&shape, candle_core::DType::F16, &device).unwrap());
    dbgts!(Tensor::zeros(&shape, candle_core::DType::F16, &device).unwrap());
}
