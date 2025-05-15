# Example using PyTorch to create and save a model
import torch
import numpy as np

# Create your model
model = torch.nn.Sequential(
    torch.nn.Linear(784, 100),
    torch.nn.ReLU(),
    torch.nn.Linear(100, 10)
)

# Train or initialize the model
# For demonstration, we will just initialize it
# Save the model architecture


# Save the weights in a compact format
weights = {
    "layer1.weight": model[0].weight.data.numpy(),
    "layer1.bias": model[0].bias.data.numpy(),
    "layer2.weight": model[2].weight.data.numpy(),
    "layer2.bias": model[2].bias.data.numpy(),
}


for np_name, tensors in weights.items():
    if np_name in weights:
        file_name = np_name.replace(".", "_")
        tensor = tensors.astype(np.float32)
        with open(f"model/{file_name}.bin", "wb") as f:
            f.write(tensor.tobytes(order='C'))
        print(f"Saved {file_name}.bin, shape: {tensor.shape}, size: {tensor.size*4} bytes")


# # Save as a binary file (using numpy's format or a custom format)
# import numpy as np
# np.savez_compressed("model_weights.npz", **weights)