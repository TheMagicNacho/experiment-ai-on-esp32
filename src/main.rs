// Expose our custom mmap module to replace memmap2
pub mod mmap;

// Trick the candle library into using our mmap module instead of memmap2
#[allow(unused_imports)]
use crate::mmap as memmap2;

use candle_core::{Device, Result, Tensor};


const LAYER1_WEIGHT: &[u8] = include_bytes!("../model/layer1_weight.bin");
const LAYER1_BIAS: &[u8] = include_bytes!("../model/layer1_bias.bin");
const LAYER2_WEIGHT: &[u8] = include_bytes!("../model/layer2_weight.bin");
const LAYER2_BIAS: &[u8] = include_bytes!("../model/layer2_bias.bin");

#[derive(Debug)]
struct Model {
    first: Linear,
    second: Linear,
}

impl Model {
    fn forward(&self, image: &Tensor) -> Result<Tensor> {
        let x = self.first.forward(image)?;
        let x = x.relu()?;
        self.second.forward(&x)
    }
}

#[derive(Debug)]
struct Linear {
    weight: Tensor,
    bias: Tensor,
}
impl Linear {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x = x.matmul(&self.weight)?;
        x.broadcast_add(&self.bias)
    }
}


fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("TMN: Initializing ESP32...");


    // Use Device::new_cuda(0)?; to use the GPU.
    // Use Device::Cpu; to use the CPU.
    // let device = Device::cuda_if_available(0)?;
    let device = Device::Cpu;
    log::info!("TMN: Using device: {:?}", device);

    // Creating a dummy model
    // let weight = Tensor::randn(0f32, 1.0, (784, 100), &device)?;
    // let bias = Tensor::randn(0f32, 1.0, (100,), &device)?;
    // let first = Linear { weight, bias };
    // let weight = Tensor::randn(0f32, 1.0, (100, 10), &device)?;
    // let bias = Tensor::randn(0f32, 1.0, (10,), &device)?;
    // let second = Linear { weight, bias };

    let first = Linear {
        weight: Tensor::from_slice(LAYER1_WEIGHT, (784, 100), &device)?,
        bias: Tensor::from_slice(LAYER1_BIAS, (100,), &device)?,
    };

    let second = Linear {
        weight: Tensor::from_slice(LAYER2_WEIGHT, (100, 10), &device)?,
        bias: Tensor::from_slice(LAYER2_BIAS, (10,), &device)?,
    };

    let model = Model { first, second };
    log::info!("Model: {:?}", model);

    let dummy_image = Tensor::randn(0f32, 1.0, (1, 784), &device)?;

    // Inference on the model
    let digit = model.forward(&dummy_image)?;
    log::info!("Digit {:?}", digit);

    Ok(())
}
