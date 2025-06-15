mod image_processor;
mod updater;

use iced::widget::{Column, Row, Rule, button, checkbox, column, svg};
use iced::{Alignment, Center, Task, Theme};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const APP_NAME: &str = "BinVec";

pub fn main() -> iced::Result {
    updater::update();

    iced::application(APP_NAME, UiState::update, UiState::view)
        .theme(|_| Theme::TokyoNight)
        .run()
}

/// Represents the overall state of the UI.
#[derive(Default, Clone)]
struct UiState {
    /// Path to the currently loaded raster image.
    image_path: Option<PathBuf>,
    /// SVG string of the vectorized image.
    vector_image: Option<String>,
    /// Flag indicating if vectorization is in progress.
    is_rendering: bool,
    /// Configuration for the vector image generation.
    vector_image_config: VectorImageConfig,
    /// Flag indicating if saving the SVG is in progress.
    is_saving: bool,
    /// Result of the last save operation (Some(true) for success, Some(false) for failure, None if no save attempted or in progress).
    save_result: Option<bool>,
}

/// Configuration options for vector image processing.
#[derive(Clone, Copy)]
struct VectorImageConfig {
    /// Whether to include color in the vectorized output.
    with_color: bool,
    /// Whether to ignore the alpha channel of the input image.
    ignore_alpha_channel: bool,
    /// Speckle filtering size.
    filter_speckle: usize,
    /// Threshold for binarization (0-255).
    binarize_threshold: u8,
    /// Whether to invert the binary image (black becomes white and vice-versa).
    invert_binary: bool,
    /// Precision for color quantization.
    color_precision: u8,
    /// Step for gradient calculation.
    gradient_step: u8,
}

impl Default for VectorImageConfig {
    fn default() -> Self {
        Self {
            with_color: false,
            ignore_alpha_channel: false,
            filter_speckle: 4,
            binarize_threshold: 128,
            invert_binary: false,
            color_precision: 5,
            gradient_step: 16,
        }
    }
}

#[derive(Debug, Clone)]
enum UiMessage {
    OpenImageButtonPressed,
    RasterGraphicSelected(Option<PathBuf>),
    SvgImageRendered(Option<String>),
    WithColorToggled(bool),
    IgnoreAlphaChannelToggled(bool),
    FilterSpeckleChanged(u32),
    BinarizeThresholdChanged(u32),
    InvertBinaryToggled(bool),
    ColorPrecisionChanged(u32),
    GradientStepChanged(u32),
    SaveToSvgPressed,
    SvgSaved(bool),
}

impl UiState {
    fn update(&mut self, message: UiMessage) -> Task<UiMessage> {
        match message {
            UiMessage::OpenImageButtonPressed => Task::perform(
                UiState::open_image_select_dialog(),
                UiMessage::RasterGraphicSelected,
            ),
            UiMessage::RasterGraphicSelected(path) => {
                self.save_result = None; // Reset save status
                if let Some(path) = path {
                    self.image_path = Some(path.clone());
                    self.is_rendering = true;

                    Task::perform(
                        UiState::render_svg_image(path.clone(), self.vector_image_config),
                        UiMessage::SvgImageRendered,
                    )
                } else {
                    self.is_rendering = false;
                    // If selection was cancelled, clear relevant fields
                    self.image_path = None;
                    self.vector_image = None;
                    Task::none()
                }
            }
            UiMessage::SvgImageRendered(vector_image) => {
                self.is_rendering = false;
                self.vector_image = vector_image;
                Task::none()
            }
            UiMessage::WithColorToggled(checked) => {
                self.vector_image_config.with_color = checked;

                // Automatically re-render when the checkbox is toggled
                if let Some(path) = &self.image_path {
                    self.is_rendering = true;
                    return self.perform_svg_rendering_task(path.clone());
                }

                Task::none()
            }
            UiMessage::IgnoreAlphaChannelToggled(checked) => {
                self.vector_image_config.ignore_alpha_channel = checked;

                // Automatically re-render when the checkbox is toggled
                if let Some(path) = &self.image_path {
                    self.is_rendering = true;
                    return Task::perform(
                        UiState::render_svg_image(path.clone(), self.vector_image_config),
                        UiMessage::SvgImageRendered,
                    );
                }

                Task::none()
            }
            UiMessage::FilterSpeckleChanged(value) => {
                self.vector_image_config.filter_speckle = value as usize;

                // Automatically re-render when the value is changed
                if let Some(path) = &self.image_path {
                    self.is_rendering = true;
                    return Task::perform(
                        UiState::render_svg_image(path.clone(), self.vector_image_config),
                        UiMessage::SvgImageRendered,
                    );
                }

                Task::none()
            }
            UiMessage::BinarizeThresholdChanged(value) => {
                self.vector_image_config.binarize_threshold = value as u8;

                // Automatically re-render when the value is changed
                if let Some(path) = &self.image_path {
                    self.is_rendering = true;
                    return Task::perform(
                        UiState::render_svg_image(path.clone(), self.vector_image_config),
                        UiMessage::SvgImageRendered,
                    );
                }

                Task::none()
            }
            UiMessage::InvertBinaryToggled(checked) => {
                self.vector_image_config.invert_binary = checked;

                // Automatically re-render when the checkbox is toggled
                if let Some(path) = &self.image_path {
                    self.is_rendering = true;
                    return Task::perform(
                        UiState::render_svg_image(path.clone(), self.vector_image_config),
                        UiMessage::SvgImageRendered,
                    );
                }

                Task::none()
            }
            UiMessage::ColorPrecisionChanged(value) => {
                self.vector_image_config.color_precision = value as u8;

                // Automatically re-render when the value is changed
                if let Some(path) = &self.image_path {
                    self.is_rendering = true;
                    return Task::perform(
                        UiState::render_svg_image(path.clone(), self.vector_image_config),
                        UiMessage::SvgImageRendered,
                    );
                }

                Task::none()
            }
            UiMessage::GradientStepChanged(value) => {
                self.vector_image_config.gradient_step = value as u8;

                // Automatically re-render when the value is changed
                if let Some(path) = &self.image_path {
                    self.is_rendering = true;
                    return Task::perform(
                        UiState::render_svg_image(path.clone(), self.vector_image_config),
                        UiMessage::SvgImageRendered,
                    );
                }

                Task::none()
            }
            UiMessage::SaveToSvgPressed => {
                if let Some(svg_data) = &self.vector_image {
                    if let Some(image_path) = &self.image_path {
                        self.is_saving = true;
                        self.save_result = None; // Indicate save operation started/in progress
                        return Task::perform(
                            UiState::save_svg_to_file(svg_data.clone(), image_path.clone()),
                            UiMessage::SvgSaved,
                        );
                    }
                }

                Task::none()
            }
            UiMessage::SvgSaved(success) => {
                self.is_saving = false;
                self.save_result = Some(success); // Store the outcome of the save operation
                Task::none()
            }
        }
    }

    fn view(&self) -> Column<UiMessage> {
        let mut controls_column = column![].padding(20).spacing(10).align_x(Center);

        // Button to open image file dialog
        controls_column = controls_column.push(if self.is_rendering {
            button("Rendering...") // No on_press while rendering
        } else {
            button("Open Image").on_press(UiMessage::OpenImageButtonPressed)
        });

        // Add With colors checkbox
        controls_column = controls_column.push(
            checkbox("With colors", self.vector_image_config.with_color)
                .on_toggle(UiMessage::WithColorToggled),
        );

        // Add controls that are common or specific to black and white mode
        if !self.vector_image_config.with_color {
            // Ignore Alpha Channel checkbox (only for B&W mode)
            controls_column = controls_column.push(
                checkbox(
                    "Ignore alpha channel",
                    self.vector_image_config.ignore_alpha_channel,
                )
                .on_toggle(UiMessage::IgnoreAlphaChannelToggled),
            );

            // Black / White Threshold slider (only for B&W mode)
            controls_column = controls_column.push(iced::widget::text("Black / White Threshold"));
            controls_column = controls_column.push(
                iced::widget::slider(
                    0..=255,
                    self.vector_image_config.binarize_threshold as u32,
                    UiMessage::BinarizeThresholdChanged,
                )
                .step(1_u32),
            );

            // Invert black / white checkbox (only for B&W mode)
            controls_column = controls_column.push(
                checkbox(
                    "Invert black / white",
                    self.vector_image_config.invert_binary,
                )
                .on_toggle(UiMessage::InvertBinaryToggled),
            );
        }

        // General filter threshold slider (common to both modes)
        controls_column = controls_column.push(iced::widget::text("General filter threshold"));
        controls_column = controls_column.push(
            iced::widget::slider(
                0..=128,
                self.vector_image_config.filter_speckle as u32,
                UiMessage::FilterSpeckleChanged,
            )
            .step(1_u32),
        );

        // Add controls specific to color mode
        if self.vector_image_config.with_color {
            // Color count slider (only for color mode)
            controls_column = controls_column.push(iced::widget::text("Color count"));
            controls_column = controls_column.push(
                iced::widget::slider(
                    0..=8,
                    self.vector_image_config.color_precision as u32,
                    UiMessage::ColorPrecisionChanged,
                )
                .step(1_u32),
            );

            // Gradient Step slider (only for color mode)
            controls_column =
                controls_column.push(iced::widget::text("Stepping of the Color gradient"));
            controls_column = controls_column.push(
                iced::widget::slider(
                    0..=128,
                    self.vector_image_config.gradient_step as u32,
                    UiMessage::GradientStepChanged,
                )
                .step(1_u32),
            );
        }

        // Add Save to SVG button
        controls_column = controls_column.push(if self.is_saving {
            button("Saving...")
        } else {
            button("Save to SVG").on_press(UiMessage::SaveToSvgPressed)
        });

        // Display save status message (only error)
        if let Some(false) = self.save_result {
            controls_column = controls_column.push(iced::widget::text("Failed to save SVG."));
        }

        let svg_handle = if let Some(vector_image) = &self.vector_image {
            svg::Handle::from_memory(vector_image.as_bytes().to_vec())
        } else {
            svg::Handle::from_memory("".as_bytes().to_vec())
        };

        let svg_view = iced::widget::container(
            svg(svg_handle)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
        )
        .width(iced::Length::FillPortion(7)) // Take 70% of the available width
        .height(iced::Length::Fill)
        .padding(20.0);

        let main_row = Row::new()
            .push(
                iced::widget::container(controls_column)
                    .width(iced::Length::FillPortion(3)) // Take 30% of the available width
                    .height(iced::Length::Fill)
                    .padding(10.0)
                    .align_y(iced::alignment::Vertical::Center), // Center content vertically
            )
            .push(Rule::vertical(1))
            .push(svg_view)
            .align_y(Alignment::Center)
            .spacing(20);

        // The main view is a column that contains the row
        // This allows for potential future elements above or below the main row if needed
        column![main_row].padding(20).align_x(Center)
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

    async fn save_svg_to_file(svg_data: String, image_path: PathBuf) -> bool {
        let target_svg_path = image_path.with_extension("svg");
        match fs::File::create(&target_svg_path) {
            Ok(mut file) => file.write_all(svg_data.as_bytes()).is_ok(),
            Err(_) => false,
        }
    }
}
