use rand::prelude::*;
use crate::utils::{MUTATION_RATE, HIDDEN_NEURON_NUM};
use crate::Direction;

#[derive(Clone)]
pub struct Brain {
    pub input: Vec<f32>,
    pub hidden_layers: Vec<Vec<f32>>,
    pub weights: Vec<Vec<Vec<f32>>>,
    pub num_hidden_layers: usize
}

impl Brain {
    pub fn new(num_inputs: usize, num_hidden_layers: usize) -> Self {
        let mut rng = rand::thread_rng();

        let mut weights = Vec::new();

        weights.push(
            (0..HIDDEN_NEURON_NUM)
                .map(|_| (0..num_inputs).map(|_| rng.gen_range(-1.0..1.0)).collect())
                .collect()
        );

        for _ in 1..num_hidden_layers {
            weights.push(
                (0..HIDDEN_NEURON_NUM)
                    .map(|_| (0..HIDDEN_NEURON_NUM).map(|_| rng.gen_range(-1.0..1.0)).collect())
                    .collect()
            );
        }

        weights.push(
            (0..4)
                .map(|_| (0..HIDDEN_NEURON_NUM).map(|_| rng.gen_range(-1.0..1.0)).collect())
                .collect()
        );

        Self {
            input: vec![0.0; num_inputs],
            hidden_layers: vec![vec![0.0; HIDDEN_NEURON_NUM]; num_hidden_layers],
            weights,
            num_hidden_layers
        }
    }

    pub fn process_input(&mut self, eye_data: Vec<f32>) -> Direction {
        self.input = eye_data;

        let mut layer_input = self.input.clone();
        for (layer_idx, layer_weights) in self.weights.iter().enumerate() {
            let mut new_layer = vec![0.0; layer_weights.len()];
            for (neuron_idx, neuron_weights) in layer_weights.iter().enumerate() {
                let sum: f32 = neuron_weights.iter().zip(&layer_input).map(|(w, v)| w * v).sum();
                new_layer[neuron_idx] = sum.tanh();
            }
            layer_input = new_layer.clone();
            if layer_idx < self.hidden_layers.len() {
                self.hidden_layers[layer_idx] = new_layer;
            }
        }

        let max_index = layer_input
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(4);

        match max_index {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => Direction::None,
        }
    }

    pub fn mutate(&mut self, brain_num: usize) {
        let mut rng = rand::thread_rng();
        
        if rng.gen::<f32>() < MUTATION_RATE {
            for layer in self.weights.iter_mut() {
                for neuron in layer.iter_mut() {
                    for weight in neuron.iter_mut() {
                        *weight += rng.gen_range(-MUTATION_RATE..MUTATION_RATE);
                    }
                }
            }
        }

        if brain_num != self.num_hidden_layers {
            if brain_num > self.num_hidden_layers {
                let new_layer: Vec<Vec<f32>> = (0..HIDDEN_NEURON_NUM)
                    .map(|_| (0..HIDDEN_NEURON_NUM).map(|_| rng.gen_range(-1.0..1.0)).collect())
                    .collect();
                self.weights.insert(1, new_layer);
                self.hidden_layers.push(vec![0.0; HIDDEN_NEURON_NUM]);
                self.num_hidden_layers += 1;
            } else {
                self.weights.pop();
                self.hidden_layers.pop();
                self.num_hidden_layers -= 1;
            }
        }
    }

    pub fn child_brain(&self, brain_num: usize) -> Brain {
        let mut new_brain: Brain = self.clone();
        new_brain.mutate(brain_num);
        new_brain
    }
}
