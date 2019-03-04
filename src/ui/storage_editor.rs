use crate::{
    data::*,
};
use imgui::*;

pub struct EditorConfig {
    type_menu_name: ImString,
    new_type_label: ImString,

    display_height: f32,
    width_padding: f32,
    display_color: [f32; 4],

    type_list: [ImString; 6],
}

impl Default for EditorConfig {
    fn default() -> Self {
        let type_list = [
            ImString::new("F32"),
            ImString::new("I32"),
            ImString::new("U32"),
            ImString::new("Char"),
            ImString::new("Bool"),
            ImString::new("String"),
        ];
        

        Self {
            type_menu_name: ImString::new("Types"),
            new_type_label: ImString::new("New Type"),
            display_height: 80.0,
            width_padding: 15.0,
            display_color: [0.5, 0.5, 0.5, 0.5],

            type_list,
        }
    }
}

pub struct StorageEditor {
    config: EditorConfig,

    selected_type: Option<ImString>,
    type_names: Vec<ImString>,
    name_buffer: ImString,

    field_name: ImString,
    field_type: (ImString, DataType),
    field_selected: i32,
    header_open: bool,

    error_message: Option<String>,
}   

impl StorageEditor {
    pub fn new(config: EditorConfig) -> StorageEditor {
        StorageEditor {
            config,
            selected_type: None,
            type_names: vec![],
            name_buffer: ImString::with_capacity(20),

            field_name: ImString::with_capacity(20),
            field_type: (ImString::new("F32"), DataType::F32),
            field_selected: 0,
            header_open: false,

            error_message: None,
        }
    }

    pub fn new_type_adder<K: Eq + std::hash::Hash>(&mut self, storage: &mut DataStorage<K>, ui: &Ui) {
        if self.selected_type.is_some() {
            return;
        }

        ui.input_text(im_str!("Type Name"), &mut self.name_buffer).build();
        if ui.small_button(im_str!("Create Type")) {
            if storage.get_storage(self.name_buffer.to_str()).is_some() {
                self.error_message = Some(String::from("Type already exists"));
            }
            else {
                self.error_message = None;
                storage.new_data_type(self.name_buffer.to_str().to_string());
                self.selected_type = Some(ImString::new(self.name_buffer.to_str()));
            }
        }
    }

    pub fn field_adder<K: Eq + std::hash::Hash>(&mut self, storage: &mut DataStorage<K>, ui: &Ui) {
        let selected = match self.selected_type.as_ref() {
            Some(value) => value,
            None => return,
        };
        let storage = match storage.get_storage_mut(selected.to_str()) {
            Some(value) => value,
            None => return,
        };

        if ui.small_button(im_str!("Add Field")) {
            if storage.has_field_with_name(self.field_name.to_str()) {
                self.error_message = Some(String::from("Already has that field"));
            }
            else {
                self.error_message = None;
                storage.add_field(self.field_name.to_str().to_string(), self.field_type.1);
                self.field_name.clear();
            }
        }

        ui.input_text(im_str!("Field Name"), &mut self.field_name).build();
        if ui.collapsing_header(&self.field_type.0).default_open(self.header_open).build() {
            self.header_open = true;
            let mut selected = self.field_selected;
            let items = [
                self.config.type_list[0].as_ref(),
                self.config.type_list[1].as_ref(),
                self.config.type_list[2].as_ref(),
                self.config.type_list[3].as_ref(),
                self.config.type_list[4].as_ref(),
                self.config.type_list[5].as_ref(),
            ];

            ui.list_box(im_str!("##Field Type List"), &mut selected, &items[..], 7);
            
            if selected != self.field_selected {
                self.field_selected = selected;
                match selected {
                    0 => {
                        self.field_type.0.clear();
                        self.field_type.0.push_str("F32");
                        self.field_type.1 = DataType::F32;
                    }
                    1 => {
                        self.field_type.0.clear();
                        self.field_type.0.push_str("I32");
                        self.field_type.1 = DataType::I32;
                    }
                    2 => {
                        self.field_type.0.clear();
                        self.field_type.0.push_str("U32");
                        self.field_type.1 = DataType::U32;
                    }
                    3 => {
                        self.field_type.0.clear();
                        self.field_type.0.push_str("Char");
                        self.field_type.1 = DataType::Char;
                    }
                    4 => {
                        self.field_type.0.clear();
                        self.field_type.0.push_str("Bool");
                        self.field_type.1 = DataType::Bool;
                    }
                    5 => {
                        self.field_type.0.clear();
                        self.field_type.0.push_str("String");
                        self.field_type.1 = DataType::OwnedString;
                    }
                    _ => {}
                }
            }
            
        }

    }

    pub fn type_display<K: Eq + std::hash::Hash>(&self, storage: &mut DataStorage<K>, ui: &Ui) {
        if self.selected_type.is_none() {
            return;
        }

        let selected = self.selected_type.as_ref().unwrap();
        let data_description = match storage.get_storage_mut(selected.to_str()) {
            Some(value) => value,
            None => return 
        };
        let mut to_remove = None;
        let mut to_move = None;

        let (x, _) = ui.get_window_size();
        let child_x = x - self.config.width_padding;
        
        ui.with_color_var(ImGuiCol::ChildBg, self.config.display_color, || {
            ui.separator();
            ui.text(&selected);

            ui.child_frame(im_str!("Fields_Child"), (child_x, self.config.display_height))
            .movable(false)
            .build(|| {
                for (i, desc)in data_description.get_desc().iter().enumerate() {
                    let text = format!("{}: {:?}", desc.name, desc.dtype);
                    let remove = im_str!("Remove##{}", i);
                    let up = im_str!("u##{}", i);
                    let down = im_str!("d##{}", i);

                    ui.text(&text);
                    ui.same_line(child_x - 100.0 - self.config.width_padding);
                    if ui.small_button(up) {
                        to_move = Some((i, true));
                    }

                    ui.same_line(child_x - 75.0 - self.config.width_padding);
                    if ui.small_button(down) {
                        to_move = Some((i, false));
                    }

                    ui.same_line(child_x - 50.0 - self.config.width_padding);
                    if ui.small_button(remove) {
                        to_remove = Some(i);
                    }

                }
            });  
        });

        if let Some(index) = to_remove {
            data_description.remove_field(index);
        }

        match to_move {
            Some((i, true)) => data_description.move_field_up(i),
            Some((i, false)) => data_description.move_field_down(i),
            _ => {}
        }
    }

    pub fn type_selector<K>(&mut self, storage: &DataStorage<K>, ui: &Ui) {
        let mut names = std::mem::replace(&mut self.type_names, vec![]);
        let mut selected = self.selected_type.take();

        ui.menu(&self.config.type_menu_name).build(|| {
            for (i, (name, _)) in storage.data.iter().enumerate() {
                let lable = 
                if i >= names.len() {
                    names.push(ImString::new(name.clone()));
                    &names[i]
                }
                else if names[i].to_str() != name {
                    names[i] = ImString::new(name.clone());
                    &names[i]

                }
                else {
                    &names[i]
                };

                if ui.menu_item(lable).build() {
                    selected = Some(lable.clone());
                }
            }

            if ui.menu_item(&self.config.new_type_label).build() {
                selected = None;
            }
        });

        self.selected_type = selected;
        std::mem::replace(&mut self.type_names, names);
    }

    pub fn create_ui<K: Eq + std::hash::Hash>(&mut self, data: &mut DataStorage<K>, ui: &Ui) {
        self.type_selector(&*data, ui);
        self.type_display(data, ui);
        self.field_adder(data, ui);
        self.new_type_adder(data, ui);

        if let Some(message) = &self.error_message {
            ui.separator();
            ui.text(message);
        }
    }
}