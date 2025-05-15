# Question
Could I get an AI model on an ESP 32?

# Notes
Memory on an ESP32: 160 Kb - DRAM
https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-guides/memory-types.html


Memory on an ARM Cortex M: 4 Gb
https://developer.arm.com/documentation/dui0552/a/the-cortex-m3-processor/memory-model


Use the Candle Tutorial for MNIST
https://huggingface.github.io/candle/guide/hello_world.html
https://www.kaggle.com/datasets/hojjatk/mnist-dataset


- Create tensors of width 784 (Images are 128px x 128px), thn fill with random numbers to simulate a dataset.

- with 100 lines each layer is aproxomately ~314kb (784 * 100 * 4)

- This exceeds the memory space of the ESP32