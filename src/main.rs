mod image_processor;

use iced::Center;
use iced::widget::{Column, button, column, svg};
use std::path::PathBuf;

pub fn main() -> iced::Result {
    iced::run("BinVec", UiState::update, UiState::view)
}

#[derive(Default)]
struct UiState {
    image_path: Option<PathBuf>,
    with_color: bool,
    ignore_alpha_channel: bool,
    is_rendering: bool,
}

#[derive(Debug, Clone, Copy)]
enum UiMessage {
    OpenImageDialog,
}

impl UiState {
    fn update(&mut self, message: UiMessage) {
        match message {
            UiMessage::OpenImageDialog => {
                self.open_image_select_dialog();
            }
        }
    }

    fn open_image_select_dialog(&mut self) {
        // Open a file dialog to select an image
        let file_dialog = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .pick_file();

        if let Some(path) = file_dialog {
            self.image_path = Some(path);
        }
    }

    fn view(&self) -> Column<UiMessage> {
        let mut ui_content = column![button("Open Image").on_press(UiMessage::OpenImageDialog),]
            .padding(20)
            .align_x(Center);

        if let Some(path) = &self.image_path {
            let image_data = image_processor::load_image(path);
            let binary_xml_svg_image = image_processor::generate_svg(
                image_data,
                self.with_color,
                self.ignore_alpha_channel,
                0,
                128,
                false,
                0,
                0,
            )
            .unwrap();
            let image_handle = svg::Handle::from_memory(binary_xml_svg_image.as_bytes().to_vec());
            ui_content = ui_content.push(svg(image_handle));
        }

        ui_content
    }
}
