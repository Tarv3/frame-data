#![allow(dead_code)]
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer as imrender;
extern crate nalgebra_glm as glm;
extern crate nalgebra as na;

mod support;
mod window;
mod shapes;
mod input;
mod state;
mod frame;
mod data;
mod ui;
mod util;
mod render;

use glium::Surface;

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let mut window = window::Window::from_builder(&mut events_loop, |window, context| {
        (window, context.with_vsync(true))
    }).unwrap();

    let mut editor = ui::storage_editor::StorageEditor::new(Default::default());
    let mut data_editor = ui::data_editor::DataEditor::new();

    let mut data: data::DataStorage<u32> =  data::DataStorage::new();

    data.new_data_type("TestType".to_string());
    {
        let storage = data.get_storage_mut("TestType").unwrap();
        storage.add_field("alpha".to_string(), data::DataType::F32);
        storage.add_field("beta".to_string(), data::DataType::F32);
        let value = storage.gen_new(1);
        value[0].set_value("1.0").unwrap();
        value[1].set_value("2.0").unwrap();
    }

    support::run(&mut window, &mut events_loop, |_target, ui, _events, _dt, _no_render | {

        ui.window(im_str!("Window2"))
            .position((300.0, 0.0), imgui::ImGuiCond::Appearing)
            .size((200.0, 200.0), imgui::ImGuiCond::Appearing)
            .build(|| {
                editor.create_ui(&mut data, ui);
            });

        ui.window(im_str!("Window"))
            .position((0.0, 0.0), imgui::ImGuiCond::Appearing)
            .size((200.0, 200.0), imgui::ImGuiCond::Appearing)
            .build(|| {
                data_editor.create_ui(&1, data.get_storage_mut("TestType").unwrap(), ui);
            });
        _target.clear_color(1.0, 1.0, 1.0, 0.0);
        true
    });

}
