use crate::shapes::*;
use crate::data::*;
use na::geometry::{Translation};
use std::collections::hash_map::HashMap;

pub struct DataBox<S> {
    pub shape: BoundingBox<f32>,
    pub data: S, 
}

impl<S> DataBox<S> {
    pub fn new(shape: BoundingBox<f32>, data: S) -> Self {
        Self {
            shape,
            data,
        }
    }
}

pub struct Frame {
    centre: [u32; 2],
    rect: AABB<u32>,    
    active_hitboxes: Vec<usize>,
}

pub struct AnimationData {
    frame_data: Vec<Frame>,
    data_boxes: Vec<DataBox<(String, u32)>>,
    index_generators: HashMap<String, IndexGenerator>,
    data: DataStorage<u32>,
    fps: u16,
}

impl AnimationData {
    pub fn new(fps: u16) -> AnimationData {
        AnimationData {
            frame_data: vec![],
            data_boxes: vec![],
            index_generators: HashMap::new(),
            data: DataStorage::new(),
            fps,
        }
    }

    pub fn new_data_type(&mut self, name: String) {
        self.data.new_data_type(name);
    }

    pub fn generate_data(&mut self, dtype: &str) -> Option<(String, u32)> {
        let index_gen = self.index_generators.entry(dtype.to_string()).or_insert(IndexGenerator::new());
        let index = index_gen.next_index();

        if let Some(ref mut data) = self.data.data.get_mut(dtype) {
            data.gen_new(index);
            Some((dtype.to_string(), index))
        }
        else {
            None
        }
    }

    pub fn new_data_box(&mut self, dtype: &str, bounding_box: BoundingBox<f32>) {
        let data = match self.generate_data(dtype) {
            Some(data) => data,
            None => return,
        };

        self.data_boxes.push(DataBox::new(bounding_box, data));
    }

    pub fn remove_data_box(&mut self, index: usize) {
        if index < self.data_boxes.len() {
            let data_box = self.data_boxes.remove(index);
            let (dtype, index) = data_box.data;

            if let Some(gen) = self.index_generators.get_mut(&dtype) {
                gen.remove_index(index);
            }
        }
    }
}

