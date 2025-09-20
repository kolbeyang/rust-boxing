use burn::module::Module;
use burn::nn::{Linear, LinearConfig, Relu};
use burn::prelude::*;


#[derive(Module, Debug)]
pub struct DQN<B: Backend> {
    fc0: Linear<B>,
    fc1: Linear<B>,
    fc2: Linear<B>,
    out: Linear<B>,
    activation: Relu,
}

#[derive(Config, Debug)]
pub struct DQNConfig {
    input_size: usize,
    output_size: usize,
}

impl DQNConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> DQN<B> {
        DQN {
            fc0: LinearConfig::new(self.input_size, 128).init(device),
            fc1: LinearConfig::new(128, 128).init(device),
            fc2: LinearConfig::new(128, 64).init(device),
            out: LinearConfig::new(64, self.output_size).init(device),
            activation: Relu::new(),
        }
    }
}

impl<B: Backend> DQN<B> {
    pub fn forward(&self, obs_tensor: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = obs_tensor.clone();
        let x = self.fc0.forward(x);
        let x = self.activation.forward(x);
        let x = self.fc1.forward(x);
        let x = self.activation.forward(x);
        let x = self.fc2.forward(x);
        let x = self.activation.forward(x);
        self.out.forward(x)
    }
}
