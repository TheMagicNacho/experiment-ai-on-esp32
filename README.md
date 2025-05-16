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
By today's standards, 100 neurons is fairly small. 

There is research regarding [1bit weights](https://arxiv.org/abs/2402.17764) . This research is very theoretical right now, but could potentially mean a 1:3 reduction in size for models. 
This compression could yield a 104kb tensor, which could potentially fit on the ESP, but it's usefulness is questionable.

Given the current technology, NN at the edge could be achieved on an ARM Cortex. However, more experimentation is required.

Development of the 1bit parameter models could advance the probability of AI on an ESP 32. This assumes, however, speed is not a concern. 
The NN firmware takes up the majority of the space on the ESP device, leving very little resources for operational computations.

## FOLLOW UP
- I was able to get the fake model with 50 neurons to run on the ESP32.
- The question is, is this enough to operationalize a real model?
- My assumption is no, but we know that the dimensions of the tensor are 50 x 784.
- Output:
- 
```bash
I (31) boot: ESP-IDF v5.2.3 2nd stage bootloader
I (31) boot: compile time May 14 2025 22:50:23
I (31) boot: Multicore bootloader
I (35) boot: chip revision: v3.0
I (39) boot.esp32: SPI Speed      : 40MHz
I (44) boot.esp32: SPI Mode       : DIO
I (48) boot.esp32: SPI Flash Size : 4MB
I (53) boot: Enabling RNG early entropy source...
I (58) boot: Partition Table:
I (62) boot: ## Label            Usage          Type ST Offset   Length
I (69) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (77) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (84) boot:  2 factory          factory app      00 00 00010000 00100000
I (92) boot: End of partition table
I (96) esp_image: segment 0: paddr=00010020 vaddr=3f400020 size=1c0e8h (114920) map
I (144) esp_image: segment 1: paddr=0002c110 vaddr=3ffb0000 size=02158h (  8536) load
I (147) esp_image: segment 2: paddr=0002e270 vaddr=40080000 size=01da8h (  7592) load
I (152) esp_image: segment 3: paddr=00030020 vaddr=400d0020 size=9311ch (602396) map
I (364) esp_image: segment 4: paddr=000c3144 vaddr=40081da8 size=09a60h ( 39520) load
I (386) boot: Loaded app from partition at offset 0x10000
I (386) boot: Disabling RNG early entropy source...
I (397) cpu_start: Multicore app
I (406) cpu_start: Pro cpu start user code
I (406) cpu_start: cpu freq: 160000000 Hz
I (406) cpu_start: Application information:
I (409) cpu_start: Project name:     libespidf
I (415) cpu_start: App version:      1
I (419) cpu_start: Compile time:     May 14 2025 22:50:12
I (425) cpu_start: ELF file SHA256:  000000000...
I (430) cpu_start: ESP-IDF:          v5.2.3
I (435) cpu_start: Min chip rev:     v0.0
I (440) cpu_start: Max chip rev:     v3.99 
I (445) cpu_start: Chip rev:         v3.0
I (450) heap_init: Initializing. RAM available for dynamic allocation:
I (457) heap_init: At 3FFAE6E0 len 00001920 (6 KiB): DRAM
I (463) heap_init: At 3FFB2B68 len 0002D498 (181 KiB): DRAM
I (469) heap_init: At 3FFE0440 len 00003AE0 (14 KiB): D/IRAM
I (475) heap_init: At 3FFE4350 len 0001BCB0 (111 KiB): D/IRAM
I (482) heap_init: At 4008B808 len 000147F8 (81 KiB): IRAM
I (489) spi_flash: detected chip: generic
I (493) spi_flash: flash io: dio
W (497) pcnt(legacy): legacy driver is deprecated, please migrate to `driver/pulse_cnt.h`
W (505) timer_group: legacy driver is deprecated, please migrate to `driver/gptimer.h`
I (515) main_task: Started on CPU0
I (525) main_task: Calling app_main()
I (525) esp_candle: TMN: Initializing ESP32...
I (525) esp_candle: TMN: Using device: Cpu
I (855) esp_candle: Model: Model { first: Linear { weight: Tensor[dims 784, 50; f32], bias: Tensor[dims 50; f32] }, second: Linear { weight: Tensor[dims 50, 10; f32], bias: Tensor[dims 10; f32] } }
I (875) esp_candle: Digit Tensor[dims 1, 10; f32]

```