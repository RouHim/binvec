mod image_processor;

use iced::widget::{Column, button, column, svg};
use iced::{Center, Task};
use std::path::PathBuf;

pub fn main() -> iced::Result {
    iced::run("BinVec", UiState::update, UiState::view)
}

#[derive(Default, Clone)]
struct UiState {
    image_path: Option<PathBuf>,
    vector_image: Option<String>,
    is_rendering: bool,
}

#[derive(Default, Clone, Copy)]
struct VectorImageConfig {
    with_color: bool,
    ignore_alpha_channel: bool,
    filter_speckle: usize,
    binarize_threshold: u8,
    invert_binary: bool,
    color_precision: i32,
    gradient_step: i32,
}

#[derive(Debug, Clone)]
enum UiMessage {
    OpenImageButtonPressed,
    RasterGraphicSelected(Option<PathBuf>),
    SvgImageRendered(Option<String>),
}

impl UiState {
    fn update(&mut self, message: UiMessage) -> Task<UiMessage> {
        match message {
            UiMessage::OpenImageButtonPressed => Task::perform(
                UiState::open_image_select_dialog(),
                UiMessage::RasterGraphicSelected,
            ),
            UiMessage::RasterGraphicSelected(path) => {
                if let Some(path) = path {
                    self.image_path = Some(path.clone());
                    self.is_rendering = true;
                    // TODO: fill with config values from the ui widets
                    let vector_image_config = VectorImageConfig {
                        with_color: true, // Example config, can be customized
                        ignore_alpha_channel: false,
                        filter_speckle: 10,
                        binarize_threshold: 128,
                        invert_binary: false,
                        color_precision: 4,
                        gradient_step: 2,
                    };
                    Task::perform(
                        UiState::render_svg_image(path.clone(), self.vector_image_config.clone()),
                        UiMessage::SvgImageRendered,
                    )
                } else {
                    self.is_rendering = false;
                    Task::none()
                }
            }
            UiMessage::SvgImageRendered(vector_image) => {
                self.is_rendering = false;
                self.vector_image = vector_image;
                Task::none()
            }
        }
    }

    fn view(&self) -> Column<UiMessage> {
        let mut ui_content = column![].padding(20).align_x(Center);

        // Button to open image file dialog
        ui_content = ui_content.push(if self.is_rendering {
            button("Open Image") // No on_press when disabled
        } else {
            button("Open Image").on_press(UiMessage::OpenImageButtonPressed)
        });

        // Display the selected image path if available
        let svg_handle = if let Some(vector_image) = &self.vector_image {
            svg::Handle::from_memory(vector_image.as_bytes().to_vec())
        } else {
            svg::Handle::from_memory("".as_bytes().to_vec())
        };
        ui_content = ui_content.push(svg(svg_handle));

        ui_content
    }
}

impl UiState {
    async fn render_svg_image(image_path: PathBuf, config: VectorImageConfig) -> Option<String> {
        let image_data = image_processor::load_image(&image_path);
        image_processor::generate_svg(image_data, config).ok()
    }

    async fn open_image_select_dialog() -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .pick_file()
    }
}
