mod image_processor;
mod updater;

use iced::widget::{Column, Row, button, checkbox, column, container, svg, text};
use iced::window::icon;
use iced::{Center, Length, Task, Theme};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const APP_NAME: &str = "BinVec";

pub fn main() -> iced::Result {
    updater::update();

    let window_settings = iced::window::Settings {
        icon: icon::from_file_data(include_bytes!("../.desktop/icon.png"), None).ok(),
        size: iced::Size::new(950.0, 700.0),
        min_size: Some(iced::Size::new(800.0, 600.0)),
        ..iced::window::Settings::default()
    };

    iced::application(APP_NAME, UiState::update, UiState::view)
        .theme(|_| Theme::TokyoNight)
        .window(window_settings)
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
                    self.perform_svg_rendering_task()
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
                self.perform_svg_rendering_task()
            }
            UiMessage::IgnoreAlphaChannelToggled(checked) => {
                self.vector_image_config.ignore_alpha_channel = checked;

                // Automatically re-render when the checkbox is toggled
                self.perform_svg_rendering_task()
            }
            UiMessage::FilterSpeckleChanged(value) => {
                self.vector_image_config.filter_speckle = value as usize;

                // Automatically re-render when the value is changed
                self.perform_svg_rendering_task()
            }
            UiMessage::BinarizeThresholdChanged(value) => {
                self.vector_image_config.binarize_threshold = value as u8;

                // Automatically re-render when the value is changed
                self.perform_svg_rendering_task()
            }
            UiMessage::InvertBinaryToggled(checked) => {
                self.vector_image_config.invert_binary = checked;

                // Automatically re-render when the checkbox is toggled
                self.perform_svg_rendering_task()
            }
            UiMessage::ColorPrecisionChanged(value) => {
                self.vector_image_config.color_precision = value as u8;

                // Automatically re-render when the value is changed
                self.perform_svg_rendering_task()
            }
            UiMessage::GradientStepChanged(value) => {
                self.vector_image_config.gradient_step = value as u8;

                // Automatically re-render when the value is changed
                self.perform_svg_rendering_task()
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
        // Create a header only if we have a file name to display
        let header = if let Some(path) = &self.image_path {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let row = Row::new()
                    .spacing(20)
                    .padding(20)
                    .push(text(format!("File: {name}")).size(16));

                Some(container(row).width(Length::Fill))
            } else {
                None
            }
        } else {
            None
        };

        // Controls panel with settings
        let controls_panel = {
            let mut controls = column![].spacing(15).width(Length::Fill).padding(20);

            // Section title styling
            let make_title = |title: &str| -> container::Container<'_, _, _> {
                container(text(title.to_string()).size(18))
                    .width(Length::Fill)
                    .padding(10)
            };

            // File operations section
            let file_actions = {
                let mut col = column![].spacing(10).width(Length::Fill);

                col = col.push(make_title("File Actions"));

                // Open image button
                let open_button = if self.is_rendering {
                    button(text("Rendering...").size(16))
                        .width(Length::Fill)
                        .padding(8)
                } else {
                    button(text("Open Image").size(16))
                        .on_press(UiMessage::OpenImageButtonPressed)
                        .width(Length::Fill)
                        .padding(8)
                };

                col = col.push(open_button);

                // Save SVG button
                let save_button = if self.is_saving {
                    button(text("Saving...").size(16))
                        .width(Length::Fill)
                        .padding(8)
                } else if self.vector_image.is_none() {
                    button(text("Save to SVG").size(16))
                        .width(Length::Fill)
                        .padding(8)
                } else {
                    button(text("Save to SVG").size(16))
                        .on_press(UiMessage::SaveToSvgPressed)
                        .width(Length::Fill)
                        .padding(8)
                };

                col = col.push(save_button);

                // Show status message when saving
                if let Some(success) = self.save_result {
                    let status_message = if success {
                        text("✓ SVG saved successfully!").size(14)
                    } else {
                        text("❌ Failed to save SVG").size(14)
                    };
                    col = col.push(status_message);
                }

                container(col).width(Length::Fill).padding(10)
            };

            // Settings section
            let settings = {
                let mut col = column![].spacing(10).width(Length::Fill);

                col = col.push(make_title("Settings"));

                // With colors checkbox
                let color_checkbox =
                    checkbox("Generate with colors", self.vector_image_config.with_color)
                        .on_toggle(UiMessage::WithColorToggled);

                col = col.push(color_checkbox);

                // Mode-specific settings
                if !self.vector_image_config.with_color {
                    // Black & white mode settings
                    col = col.push(text("Black & White Options").size(16));

                    // Ignore alpha channel checkbox
                    let alpha_checkbox = checkbox(
                        "Ignore alpha channel",
                        self.vector_image_config.ignore_alpha_channel,
                    )
                    .on_toggle(UiMessage::IgnoreAlphaChannelToggled);

                    col = col.push(alpha_checkbox);

                    // Threshold slider
                    col = col.push(text("Black / White Threshold").size(14));
                    col = col.push(
                        iced::widget::slider(
                            0..=255,
                            self.vector_image_config.binarize_threshold as u32,
                            UiMessage::BinarizeThresholdChanged,
                        )
                        .step(1u32),
                    );
                    col = col.push(
                        text(format!(
                            "Value: {}",
                            self.vector_image_config.binarize_threshold
                        ))
                        .size(12),
                    );

                    // Invert black/white checkbox
                    let invert_checkbox = checkbox(
                        "Invert black / white",
                        self.vector_image_config.invert_binary,
                    )
                    .on_toggle(UiMessage::InvertBinaryToggled);

                    col = col.push(invert_checkbox);
                } else {
                    // Color mode settings
                    col = col.push(text("Color Options").size(16));

                    // Color precision slider
                    col = col.push(text("Color Precision").size(14));
                    col = col.push(
                        iced::widget::slider(
                            0..=8,
                            self.vector_image_config.color_precision as u32,
                            UiMessage::ColorPrecisionChanged,
                        )
                        .step(1u32),
                    );
                    col = col.push(
                        text(format!(
                            "Value: {}",
                            self.vector_image_config.color_precision
                        ))
                        .size(12),
                    );

                    // Gradient step slider
                    col = col.push(text("Color Gradient Step").size(14));
                    col = col.push(
                        iced::widget::slider(
                            0..=128,
                            self.vector_image_config.gradient_step as u32,
                            UiMessage::GradientStepChanged,
                        )
                        .step(1u32),
                    );
                    col = col.push(
                        text(format!("Value: {}", self.vector_image_config.gradient_step)).size(12),
                    );
                }

                // Common filter settings
                col = col.push(text("Filter Options").size(16));
                col = col.push(text("Detail Filter Threshold").size(14));
                col = col.push(
                    iced::widget::slider(
                        0..=128,
                        self.vector_image_config.filter_speckle as u32,
                        UiMessage::FilterSpeckleChanged,
                    )
                    .step(1u32),
                );
                col = col.push(
                    text(format!(
                        "Value: {}",
                        self.vector_image_config.filter_speckle
                    ))
                    .size(12),
                );

                container(col).width(Length::Fill).padding(10)
            };

            // Combine sections
            controls = controls.push(file_actions);
            controls = controls.push(settings);

            // Return controls directly without making it scrollable
            controls
        };

        // SVG preview area
        let svg_view = {
            let svg_handle = if let Some(vector_image) = &self.vector_image {
                svg::Handle::from_memory(vector_image.as_bytes().to_vec())
            } else {
                svg::Handle::from_memory("".as_bytes().to_vec())
            };

            // Use a direct approach - wrap everything in containers from the start
            let content = if self.vector_image.is_some() {
                container(svg(svg_handle).width(Length::Fill).height(Length::Fill))
                    .width(Length::Fill)
                    .height(Length::Fill)
            } else {
                container(text("No image loaded. Open an image to start.").size(18))
                    .width(Length::Fill)
                    .height(Length::Fill)
            };

            // Wrapper container for consistent styling
            container(content)
                .width(Length::FillPortion(7))
                .height(Length::Fill)
                .padding(20)
        };

        // Main layout
        let main_row = Row::new()
            .push(
                container(controls_panel)
                    .width(Length::FillPortion(3))
                    .height(Length::Fill),
            )
            .push(svg_view)
            .spacing(15);

        // Combine everything - conditionally include the header only if it exists
        let mut content = Column::new().padding(10).spacing(10).align_x(Center);

        // Only add header if it exists
        if let Some(header_content) = header {
            content = content.push(header_content);
        }

        content.push(main_row)
    }
}

impl UiState {
    async fn render_svg_image(image_path: PathBuf, config: VectorImageConfig) -> Option<String> {
        let image_data = image_processor::load_image(&image_path);
        image_processor::generate_svg(image_data, config).ok()
    }

    async fn open_image_select_dialog() -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter(
                "Images",
                &[
                    "jpg", "jpeg", "png", "gif", "bmp", "webp", "ico", "tiff", "avif", "pnm",
                    "dds", "tga",
                ],
            )
            .pick_file()
    }

    async fn save_svg_to_file(svg_data: String, image_path: PathBuf) -> bool {
        let target_svg_path = image_path.with_extension("svg");
        match fs::File::create(&target_svg_path) {
            Ok(mut file) => file.write_all(svg_data.as_bytes()).is_ok(),
            Err(_) => false,
        }
    }

    fn perform_svg_rendering_task(&mut self) -> Task<UiMessage> {
        // Don't start a new render if one is already in progress
        if self.is_rendering {
            return Task::none();
        }

        if let Some(path) = &self.image_path {
            self.is_rendering = true;
            return Task::perform(
                UiState::render_svg_image(path.clone(), self.vector_image_config),
                UiMessage::SvgImageRendered,
            );
        }
        Task::none()
    }
}
