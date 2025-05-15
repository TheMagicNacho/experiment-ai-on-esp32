# Question
Could I get an AI model on an ESP 32?

# Notes
Memory on an ESP32: 160 Kb - DRAM
- https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-guides/memory-types.html


Memory on an ARM Cortex M: 4 Gb
- https://developer.arm.com/documentation/dui0552/a/the-cortex-m3-processor/memory-model

Use the Candle Tutorial for MNIST
- https://huggingface.github.io/candle/guide/hello_world.html
- https://www.kaggle.com/datasets/hojjatk/mnist-dataset

## Implementation Approach
- Use candles to create a simple NN for MNIST.
    - I hand to write a custom memmap2 implementation to bypass the dependency.
    - The microcontroller does not have a file system so we really can't use any of those functions. Writting the memmap2 allows us to compile the app without using the functionality.

### First Attempt: Compute at run time
- Fuzz the inputs with random data on the tensors.

- Flash the firmware to the device.
     - NOTICE: the opt-level is set to "z" aggressive.

- This will work but crashes when it tries to allocate memeory because of the size of the tensor.

### Second Attempt: Pre Compute the model
- Precompute the models using the `gen_model.py` script
- Load the models in during compile time.
- Flash the device
- Still crashes

## Mathmatical Analysis
- Create tensors of width 784 (Images are 28px x 28px), thn fill with random numbers to simulate a dataset.

- Assuming 100 nurons, the tensor's dimensions are (100, 784).

- Since the ESP uses 32 bit assume 4 bytes per value

- Estimated Size of each Tensor: 313 Kb
    - 100 * 784 * 4

- A single NN layer exceeds the memory space of the ESP32

# Summary / Recomendation
A useful tensor (meaning a tensor which intends to provide any value towards a NN), cannot fit on an ESP 32.
By today's standards, 100 nurons is fairly small. 

There is research regarding [1bit weights](https://arxiv.org/abs/2402.17764) . This research is very theoretical right now, but could potentially mean a 1:3 reduction in size for models. 
This compression could yield a 104kb tensor, which could potentially fit on the ESP, but it's usefuless is questionable.

Given the current technology, NN at the edge could be achieved on an ARM Cortex. However, more experimentation is required.

Development of the 1bit parameter models could advance the probability of AI on an ESP 32. This assumes, however, speed is not a concern. 
The NN firmware takes up the majority of the space on the ESP device, leving very little resources for operational computations.

