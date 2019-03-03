use imgui::*;
use crate::data::*;

pub struct DataEditor {
    field_buffers: Vec<ImString>,
    error_message: Option<String>,
}

impl DataEditor {
    pub fn new() -> Self {
        Self {
            field_buffers: vec![],
            error_message: None,
        }
    }

    pub fn create_ui<K: Eq + std::hash::Hash>(&mut self, key: &K, data: &mut DataTypeStorage<K>, ui: &Ui) {
        let description = std::mem::replace(&mut data.description, vec![]);

        if let Some(values) = data.get_mut(key) {
            for (i, value) in values.iter_mut().enumerate() {
                let buffer = {
                    if i < self.field_buffers.len() {
                        &mut self.field_buffers[i]
                    }
                    else {
                        self.field_buffers.push(ImString::with_capacity(10));
                        &mut self.field_buffers[i]
                    }
                };
        
                let name = format!("{}: {:?}: {}", description[i].name, description[i].dtype, value);
                ui.text(name);
                ui.same_line(100.0);
                
                if ui.input_text(im_str!("##Value{}", i), &mut self.field_buffers[i]).enter_returns_true(true).build() {
                    match value.set_value(self.field_buffers[i].to_str()) {
                        Ok(_) => self.field_buffers[i].clear(),
                        Err(_) => {}
                    }
                }
            }
        }    

        std::mem::replace(&mut data.description, description);
    }
}