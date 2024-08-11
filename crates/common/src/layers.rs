use std::sync::{
    Arc,
    Mutex,
    MutexGuard,
};

use freya_native_core::NodeId;
use rustc_hash::FxHashMap;

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: Arc<Mutex<FxHashMap<i16, Vec<NodeId>>>>,
}

impl Layers {
    pub fn insert_node_in_layer(&self, node_id: NodeId, layer_n: i16) {
        let mut layers = self.layers.lock().unwrap();
        let layer = layers.entry(layer_n).or_default();
        if layer.contains(&node_id) {
            return;
        }
        layer.push(node_id);
    }

    pub fn remove_node_from_layer(&self, node_id: NodeId, layer_n: i16) -> bool {
        let mut layers = self.layers.lock().unwrap();
        let layer = layers.get_mut(&layer_n).unwrap();
        layer.retain(|id| *id != node_id);

        let layer_is_empty = layer.is_empty();

        if layer_is_empty {
            layers.remove(&layer_n);
        }

        layer_is_empty
    }

    pub fn layers(&self) -> MutexGuard<FxHashMap<i16, Vec<NodeId>>> {
        self.layers.lock().unwrap()
    }

    pub fn len_layers(&self) -> usize {
        self.layers.lock().unwrap().len()
    }
}
