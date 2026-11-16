mod image_processor;
mod updater;

use std::io::Write;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, fs};

use iced::font::Weight;
use iced::theme::{Mode, Palette};
use iced::widget::canvas::{self, Frame, Stroke};
use iced::widget::image::Handle as RasterHandle;
use iced::widget::text::Wrapping;
use iced::widget::{
    Canvas, Column, Row, Space, Stack, button, checkbox, column, container, image, scrollable,
    slider, svg, text,
};
use iced::window::{self, icon};
use iced::{
    Alignment, Border, Color, ContentFit, Element, Font, Length, Point, Rectangle, Shadow, Size,
    Subscription, Task, Theme,
};

const APP_NAME: &str = "BinVec";

const KEY_WIDTH: f32 = 304.0;
const KEY_RULE: f32 = 1.0;
const TOP_BAR_HEIGHT: f32 = 56.0;
const BOTTOM_BAR_HEIGHT: f32 = 34.0;
const FIGURE_PADDING: f32 = 24.0;
const CAPTION_HEIGHT: f32 = 24.0;
const CAPTION_GAP: f32 = 8.0;
const PLATE_MARGIN: f32 = 24.0;
const PLATE_FRAME: f32 = 1.0;
const CHROME_RULES: f32 = 2.0;

/// Manual zoom window: `−` / `+` step by `ZOOM_STEP` between these bounds.
const MIN_ZOOM: f32 = 0.05;
const MAX_ZOOM: f32 = 4.0;
const ZOOM_STEP: f32 = 1.25;

/// Logical-pixel ceiling for the vector proof's display box. iced_wgpu
/// rasterizes SVG content at the widget bounds times the window scale factor
/// on every size change, so the vector proof is capped by both its longest
/// edge and its area to keep each raster allocation bounded. The source proof
/// draws a fixed texture and keeps the full manual zoom range.
const VECTOR_RASTER_MAX_EDGE: f32 = 2048.0;
const VECTOR_RASTER_MAX_PIXELS: f32 = 4.0 * 1024.0 * 1024.0;

const PLATE_PAPER: Color = Color::from_rgb8(0xF3, 0xF0, 0xE7);
const PLATE_PAPER_RAISED: Color = Color::from_rgb8(0xF8, 0xF5, 0xEC);
const PLATE_PAPER_DEEP: Color = Color::from_rgb8(0xE8, 0xE3, 0xD6);
const PLATE_INK: Color = Color::from_rgb8(0x15, 0x16, 0x1A);
const PLATE_INK_HOVER: Color = Color::from_rgb8(0x2B, 0x2D, 0x33);
const PLATE_INK_PRESSED: Color = Color::from_rgb8(0x00, 0x00, 0x00);
const PLATE_MUTED: Color = Color::from_rgb8(0x6E, 0x6A, 0x60);
const PLATE_RULE: Color = Color::from_rgb8(0xD8, 0xD2, 0xC3);
const PLATE_TEAL: Color = Color::from_rgb8(0x00, 0xA8, 0xBE);
const PLATE_TEAL_DARK: Color = Color::from_rgb8(0x00, 0x70, 0x7C);
const PLATE_TEAL_WASH: Color = Color::from_rgba8(0x00, 0xA8, 0xBE, 0.14);
const PLATE_RED: Color = Color::from_rgb8(0xE1, 0x41, 0x34);
const PLATE_RED_DARK: Color = Color::from_rgb8(0xB3, 0x26, 0x1E);
const PLATE_GREEN: Color = Color::from_rgb8(0x2F, 0x6B, 0x4F);
const PLATE_OCHRE: Color = Color::from_rgb8(0x8A, 0x5A, 0x00);

const SEMIBOLD: Font = Font {
    weight: Weight::Semibold,
    ..Font::DEFAULT
};
const BOLD: Font = Font {
    weight: Weight::Bold,
    ..Font::DEFAULT
};

pub fn main() -> iced::Result {
    prefer_dx12_and_vulkan_on_windows();

    updater::update();

    let window_settings = iced::window::Settings {
        icon: icon::from_file_data(include_bytes!("../.desktop/icon.png"), None).ok(),
        size: Size::new(950.0, 700.0),
        min_size: Some(Size::new(800.0, 600.0)),
        ..iced::window::Settings::default()
    };

    iced::application(UiState::new, UiState::update, UiState::view)
        .title(APP_NAME)
        .theme(|state: &UiState| state.theme())
        .subscription(UiState::subscription)
        .window(window_settings)
        .run()
}

/// Keeps the OpenGL backend out of the default graphics backends on Windows.
///
/// wgpu initializes every requested backend as soon as the instance is
/// created, and `Backends::all()` (the default used by iced) includes OpenGL.
/// On Windows, that backend creates a hidden window with a WGL context, which
/// makes Windows load the OpenGL ICD of the installed GPU driver. If that
/// driver component is broken or missing — as seen on hybrid-graphics
/// laptops and in VMs — the driver interrupts startup with a modal
/// "LoadLibrary failed with error 126" dialog.
///
/// Direct3D 12 and Vulkan cover Windows (WARP can serve as a software
/// fallback), so only those two are requested by default. An explicitly set
/// `WGPU_BACKEND` environment variable always wins.
fn prefer_dx12_and_vulkan_on_windows() {
    if cfg!(windows) && std::env::var_os("WGPU_BACKEND").is_none() {
        // SAFETY: Runs first in `main`, before any other thread is spawned.
        unsafe { std::env::set_var("WGPU_BACKEND", "dx12,vulkan") };
    }
}

#[derive(Clone, Copy)]
struct Colors {
    shell_bg: Color,
    shell_raised: Color,
    shell_deep: Color,
    shell_text: Color,
    shell_muted: Color,
    shell_rule: Color,
    primary_fill: Color,
    primary_text: Color,
    primary_hover: Color,
    primary_pressed: Color,
    disabled_fill: Color,
    disabled_text: Color,
    disabled_border: Color,
    teal: Color,
    teal_text: Color,
    red: Color,
    red_text: Color,
    green: Color,
    ochre: Color,
}

impl Colors {
    const LIGHT: Self = Self {
        shell_bg: PLATE_PAPER,
        shell_raised: PLATE_PAPER_RAISED,
        shell_deep: PLATE_PAPER_DEEP,
        shell_text: PLATE_INK,
        shell_muted: PLATE_MUTED,
        shell_rule: PLATE_RULE,
        primary_fill: PLATE_INK,
        primary_text: PLATE_PAPER,
        primary_hover: PLATE_INK_HOVER,
        primary_pressed: PLATE_INK_PRESSED,
        disabled_fill: PLATE_PAPER,
        disabled_text: PLATE_MUTED,
        disabled_border: PLATE_RULE,
        teal: PLATE_TEAL,
        teal_text: PLATE_TEAL_DARK,
        red: PLATE_RED,
        red_text: PLATE_RED_DARK,
        green: PLATE_GREEN,
        ochre: PLATE_OCHRE,
    };

    const DARK: Self = Self {
        shell_bg: Color::from_rgb8(0x17, 0x16, 0x14),
        shell_raised: Color::from_rgb8(0x20, 0x1E, 0x1A),
        shell_deep: Color::from_rgb8(0x29, 0x25, 0x1F),
        shell_text: Color::from_rgb8(0xF1, 0xEB, 0xDD),
        shell_muted: Color::from_rgb8(0xA3, 0x9B, 0x8B),
        shell_rule: Color::from_rgb8(0x3B, 0x37, 0x30),
        primary_fill: Color::from_rgb8(0xF1, 0xEB, 0xDD),
        primary_text: Color::from_rgb8(0x17, 0x16, 0x14),
        primary_hover: Color::from_rgb8(0xFF, 0xF8, 0xE8),
        primary_pressed: Color::from_rgb8(0xD8, 0xCF, 0xBD),
        disabled_fill: Color::from_rgb8(0x17, 0x16, 0x14),
        disabled_text: Color::from_rgb8(0xA3, 0x9B, 0x8B),
        disabled_border: Color::from_rgb8(0x3B, 0x37, 0x30),
        teal: Color::from_rgb8(0x3F, 0xD6, 0xE8),
        teal_text: Color::from_rgb8(0x5F, 0xD9, 0xE8),
        red: Color::from_rgb8(0xFF, 0x6B, 0x5E),
        red_text: Color::from_rgb8(0xFF, 0x7A, 0x6B),
        green: Color::from_rgb8(0x6F, 0xD3, 0xA3),
        ochre: Color::from_rgb8(0xE0, 0xB4, 0x5A),
    };

    fn for_mode(mode: Mode) -> Self {
        if mode == Mode::Dark {
            Self::DARK
        } else {
            Self::LIGHT
        }
    }

    fn for_theme(theme: &Theme) -> Self {
        if theme.extended_palette().is_dark {
            Self::DARK
        } else {
            Self::LIGHT
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeChoice {
    Auto,
    Light,
    Dark,
}

impl ThemeChoice {
    fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "auto" => Some(Self::Auto),
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            _ => None,
        }
    }

    fn load() -> Option<Self> {
        let path = theme_config_path()?;
        let value = fs::read_to_string(path).ok()?;
        Self::parse(value.trim())
    }

    fn save(self) {
        let Some(path) = theme_config_path() else {
            return;
        };

        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Err(error) = fs::write(&path, self.as_str()) {
            eprintln!(
                "Failed to save theme preference to {}: {error}",
                path.display()
            );
        }
    }
}

fn theme_config_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        env::var_os("APPDATA")
            .map(PathBuf::from)
            .map(|base| base.join("binvec").join("theme"))
    }

    #[cfg(not(target_os = "windows"))]
    {
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
            .map(|base| base.join("binvec").join("theme"))
    }
}

/// Represents the overall state of the UI.
#[derive(Clone)]
struct UiState {
    image_path: Option<PathBuf>,
    source_handle: Option<RasterHandle>,
    source_readable: bool,
    source_size: Option<(u32, u32)>,
    vector_image: Option<String>,
    vector_handle: Option<svg::Handle>,
    is_rendering: bool,
    render_seq: u64,
    rerender_pending: bool,
    pending_image: Option<PathBuf>,
    render_error: Option<String>,
    vector_image_config: VectorImageConfig,
    is_saving: bool,
    save_seq: u64,
    save_result: Option<Result<PathBuf, String>>,
    proof: Proof,
    fit_zoom: bool,
    zoom: f32,
    window_size: Size,
    drag_hovering: bool,
    scan_phase: f32,
    register_offset: f32,
    settling: bool,
    reduce_motion: bool,
    brand_icon: RasterHandle,
    theme_choice: ThemeChoice,
    system_mode: Mode,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            image_path: None,
            source_handle: None,
            source_readable: false,
            source_size: None,
            vector_image: None,
            vector_handle: None,
            is_rendering: false,
            render_seq: 0,
            rerender_pending: false,
            pending_image: None,
            render_error: None,
            vector_image_config: VectorImageConfig::default(),
            is_saving: false,
            save_seq: 0,
            save_result: None,
            proof: Proof::Vector,
            fit_zoom: true,
            zoom: 1.0,
            window_size: Size::new(950.0, 700.0),
            drag_hovering: false,
            scan_phase: 0.0,
            register_offset: 0.0,
            settling: false,
            reduce_motion: env::var_os("BINVEC_REDUCE_MOTION").is_some_and(|value| value != "0"),
            brand_icon: RasterHandle::from_bytes(include_bytes!("../.desktop/icon.png").to_vec()),
            theme_choice: ThemeChoice::load().unwrap_or(ThemeChoice::Auto),
            system_mode: Mode::Light,
        }
    }
}

/// Configuration options for vector image processing.
#[derive(Clone, Copy)]
struct VectorImageConfig {
    with_color: bool,
    ignore_alpha_channel: bool,
    filter_speckle: usize,
    binarize_threshold: u8,
    invert_binary: bool,
    color_precision: u8,
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
struct RenderedPlate {
    svg: String,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone)]
struct RenderFailure {
    message: String,
    source_size: Option<(u32, u32)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Proof {
    Vector,
    Source,
}

#[derive(Debug, Clone)]
enum UiMessage {
    OpenImageButtonPressed,
    RasterGraphicSelected(Option<PathBuf>),
    SvgImageRendered {
        seq: u64,
        result: Result<RenderedPlate, RenderFailure>,
    },
    WithColorToggled(bool),
    IgnoreAlphaChannelToggled(bool),
    FilterSpeckleChanged(u32),
    BinarizeThresholdChanged(u32),
    InvertBinaryToggled(bool),
    ColorPrecisionChanged(u32),
    GradientStepChanged(u32),
    SaveToSvgPressed,
    SvgSaved {
        seq: u64,
        result: Result<PathBuf, String>,
    },
    ProofSelected(Proof),
    ZoomIn,
    ZoomOut,
    ZoomFit,
    WindowResized(Size),
    FileHovered(bool),
    ThemeSelected(ThemeChoice),
    SystemThemeChanged(Mode),
    Tick,
}

impl UiState {
    fn new() -> (Self, Task<UiMessage>) {
        let state = Self::default();
        let task = iced::system::theme().map(UiMessage::SystemThemeChanged);
        (state, task)
    }

    fn resolved_mode(&self) -> Mode {
        match self.theme_choice {
            ThemeChoice::Auto => {
                if self.system_mode == Mode::Dark {
                    Mode::Dark
                } else {
                    Mode::Light
                }
            }
            ThemeChoice::Light => Mode::Light,
            ThemeChoice::Dark => Mode::Dark,
        }
    }

    fn colors(&self) -> Colors {
        Colors::for_mode(self.resolved_mode())
    }

    fn theme(&self) -> Theme {
        let colors = self.colors();
        let name = if self.resolved_mode() == Mode::Dark {
            "binvec-plate-dark"
        } else {
            "binvec-plate-light"
        };

        Theme::custom(
            name,
            Palette {
                background: colors.shell_bg,
                text: colors.shell_text,
                primary: colors.teal,
                success: colors.green,
                warning: colors.ochre,
                danger: colors.red_text,
            },
        )
    }

    fn update(&mut self, message: UiMessage) -> Task<UiMessage> {
        match message {
            UiMessage::OpenImageButtonPressed => Task::perform(
                Self::open_image_select_dialog(),
                UiMessage::RasterGraphicSelected,
            ),
            UiMessage::RasterGraphicSelected(path) => {
                let Some(path) = path else {
                    return Task::none();
                };

                self.drag_hovering = false;
                self.image_path = Some(path.clone());
                self.source_handle = Some(RasterHandle::from_path(path.clone()));
                self.source_readable = false;
                self.source_size = None;
                self.vector_image = None;
                self.vector_handle = None;
                self.render_error = None;
                self.save_seq = self.save_seq.wrapping_add(1);
                self.save_result = None;
                self.proof = Proof::Vector;
                self.fit_zoom = true;
                self.zoom = 1.0;
                self.register_offset = 3.0;
                self.settling = false;

                if self.is_rendering {
                    self.render_seq = self.render_seq.wrapping_add(1);
                    self.pending_image = Some(path);
                    Task::none()
                } else {
                    self.start_render()
                }
            }
            UiMessage::SvgImageRendered { seq, result } => {
                if seq != self.render_seq {
                    self.is_rendering = false;

                    if let Some(path) = self.pending_image.take()
                        && self.image_path.as_ref() == Some(&path)
                    {
                        return self.start_render();
                    }

                    return Task::none();
                }

                self.is_rendering = false;

                match result {
                    Ok(plate) => {
                        self.source_readable = true;
                        self.source_size = Some((plate.width, plate.height));
                        self.vector_handle =
                            Some(svg::Handle::from_memory(plate.svg.clone().into_bytes()));
                        self.vector_image = Some(plate.svg);
                        self.render_error = None;
                        self.register_offset = if self.reduce_motion { 0.0 } else { 3.0 };
                        self.settling = !self.reduce_motion;
                    }
                    Err(failure) => {
                        self.source_readable = failure.source_size.is_some();
                        self.source_size = failure.source_size;
                        self.vector_handle = None;
                        self.vector_image = None;
                        self.render_error = Some(failure.message);
                        self.proof = Proof::Vector;
                        self.register_offset = 0.0;
                        self.settling = false;
                    }
                }

                if self.rerender_pending {
                    self.rerender_pending = false;
                    return self.start_render();
                }

                Task::none()
            }
            UiMessage::WithColorToggled(checked) => {
                self.vector_image_config.with_color = checked;
                self.config_changed()
            }
            UiMessage::IgnoreAlphaChannelToggled(checked) => {
                self.vector_image_config.ignore_alpha_channel = checked;
                self.config_changed()
            }
            UiMessage::FilterSpeckleChanged(value) => {
                self.vector_image_config.filter_speckle = value as usize;
                self.config_changed()
            }
            UiMessage::BinarizeThresholdChanged(value) => {
                self.vector_image_config.binarize_threshold = value as u8;
                self.config_changed()
            }
            UiMessage::InvertBinaryToggled(checked) => {
                self.vector_image_config.invert_binary = checked;
                self.config_changed()
            }
            UiMessage::ColorPrecisionChanged(value) => {
                self.vector_image_config.color_precision = value as u8;
                self.config_changed()
            }
            UiMessage::GradientStepChanged(value) => {
                self.vector_image_config.gradient_step = value as u8;
                self.config_changed()
            }
            UiMessage::SaveToSvgPressed => {
                if self.is_rendering || self.is_saving {
                    return Task::none();
                }

                let (Some(svg_data), Some(image_path)) = (&self.vector_image, &self.image_path)
                else {
                    return Task::none();
                };

                self.is_saving = true;
                self.save_result = None;
                self.save_seq = self.save_seq.wrapping_add(1);
                let seq = self.save_seq;
                Task::perform(
                    Self::save_svg_to_file(svg_data.clone(), image_path.clone()),
                    move |result| UiMessage::SvgSaved { seq, result },
                )
            }
            UiMessage::SvgSaved { seq, result } => {
                self.is_saving = false;
                if seq == self.save_seq {
                    self.save_result = Some(result);
                }
                Task::none()
            }
            UiMessage::ProofSelected(proof) => {
                if proof == Proof::Vector && self.vector_handle.is_none() {
                    return Task::none();
                }
                if proof == Proof::Source && self.source_handle.is_none() {
                    return Task::none();
                }
                self.proof = proof;
                Task::none()
            }
            UiMessage::ZoomIn => {
                if !self.zoom_available() {
                    return Task::none();
                }
                // Step from the unclamped scale so a FIT scale outside
                // 5%–400% still moves in the direction of the control.
                let current = self.effective_zoom();
                self.fit_zoom = false;
                self.zoom = (current * ZOOM_STEP).clamp(MIN_ZOOM, MAX_ZOOM).max(current);
                Task::none()
            }
            UiMessage::ZoomOut => {
                if !self.zoom_available() {
                    return Task::none();
                }
                // Step from the unclamped scale and never grow past it, so a
                // FIT scale outside 5%–400% cannot invert the direction.
                let current = self.effective_zoom();
                self.fit_zoom = false;
                self.zoom = (current / ZOOM_STEP).clamp(MIN_ZOOM, MAX_ZOOM).min(current);
                Task::none()
            }
            UiMessage::ZoomFit => {
                self.fit_zoom = true;
                Task::none()
            }
            UiMessage::WindowResized(size) => {
                self.window_size = size;
                Task::none()
            }
            UiMessage::FileHovered(hovered) => {
                self.drag_hovering = hovered;
                Task::none()
            }
            UiMessage::ThemeSelected(choice) => {
                self.theme_choice = choice;
                choice.save();
                Task::none()
            }
            UiMessage::SystemThemeChanged(mode) => {
                self.system_mode = mode;
                Task::none()
            }
            UiMessage::Tick => {
                if self.is_rendering {
                    self.scan_phase = (self.scan_phase + 0.018) % 1.0;
                }
                if self.settling {
                    self.register_offset = (self.register_offset - 0.5).max(0.0);
                    if self.register_offset <= 0.0 {
                        self.settling = false;
                    }
                }
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<UiMessage> {
        let window_events = iced::event::listen_with(|event, _status, _window| match event {
            iced::Event::Window(window::Event::Opened { size, .. })
            | iced::Event::Window(window::Event::Resized(size)) => {
                Some(UiMessage::WindowResized(size))
            }
            iced::Event::Window(window::Event::FileDropped(path)) => {
                Some(UiMessage::RasterGraphicSelected(Some(path)))
            }
            iced::Event::Window(window::Event::FileHovered(_)) => {
                Some(UiMessage::FileHovered(true))
            }
            iced::Event::Window(window::Event::FilesHoveredLeft) => {
                Some(UiMessage::FileHovered(false))
            }
            _ => None,
        });

        let mut subscriptions = vec![
            window_events,
            iced::system::theme_changes().map(UiMessage::SystemThemeChanged),
        ];

        if !self.reduce_motion
            && (self.settling || (self.is_rendering && self.proof == Proof::Vector))
        {
            subscriptions
                .push(iced::time::every(Duration::from_millis(16)).map(|_| UiMessage::Tick));
        }

        Subscription::batch(subscriptions)
    }

    fn view(&self) -> Element<'_, UiMessage> {
        let c = self.colors();
        let main_row = Row::new()
            .push(self.key_column())
            .push(vertical_hairline(c))
            .push(self.figure_column())
            .height(Length::Fill);

        container(
            column![
                self.top_bar(),
                hairline(c),
                main_row,
                hairline(c),
                self.bottom_bar(),
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(c.shell_bg.into()),
            ..Default::default()
        })
        .into()
    }

    fn top_bar(&self) -> Element<'_, UiMessage> {
        let c = self.colors();
        let compact = self.window_size.width < 900.0;
        let subtitle = if compact {
            "PLATE 01"
        } else {
            "PLATE 01 / RASTER → VECTOR"
        };

        let brand = Row::new()
            .push(
                container(
                    image(self.brand_icon.clone())
                        .width(Length::Fixed(28.0))
                        .height(Length::Fixed(28.0))
                        .content_fit(ContentFit::Contain),
                )
                .padding(2)
                .style(move |_| container::Style {
                    border: Border {
                        color: c.shell_rule,
                        width: 1.0,
                        radius: 2.0.into(),
                    },
                    ..Default::default()
                }),
            )
            .push(
                column![
                    Row::new()
                        .push(text("BIN").size(16).font(BOLD).color(c.teal_text))
                        .push(text("VEC").size(16).font(BOLD).color(c.red_text)),
                    text(subtitle)
                        .size(11)
                        .font(Font::MONOSPACE)
                        .color(c.shell_muted)
                        .wrapping(Wrapping::None),
                ]
                .spacing(0),
            )
            .spacing(12)
            .align_y(Alignment::Center);

        let (status, status_color) = self.status();
        let status = Row::new()
            .push(
                container(Space::new().width(8).height(8)).style(move |_| container::Style {
                    background: Some(status_color.into()),
                    border: Border {
                        color: status_color,
                        width: 1.0,
                        radius: 1.0.into(),
                    },
                    ..Default::default()
                }),
            )
            .push(
                text(status)
                    .size(11)
                    .font(Font::MONOSPACE)
                    .color(c.shell_text),
            )
            .spacing(8)
            .align_y(Alignment::Center);

        let open = secondary_button(c, "OPEN IMAGE", UiMessage::OpenImageButtonPressed);

        let save_enabled = self.vector_handle.is_some() && !self.is_rendering && !self.is_saving;
        let save_label = if self.is_saving {
            "WRITING"
        } else {
            "SAVE SVG"
        };
        let save = primary_button(
            c,
            save_label,
            save_enabled,
            save_enabled.then_some(UiMessage::SaveToSvgPressed),
        );

        container(
            Row::new()
                .push(brand)
                .push(Space::new().width(Length::Fill))
                .push(self.theme_control())
                .push(Space::new().width(16))
                .push(status)
                .push(Space::new().width(16))
                .push(open)
                .push(save)
                .spacing(8)
                .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fixed(TOP_BAR_HEIGHT))
        .padding([0, 24])
        .into()
    }

    fn theme_control(&self) -> Element<'_, UiMessage> {
        let c = self.colors();
        let choices = [
            (ThemeChoice::Auto, "AUTO"),
            (ThemeChoice::Light, "LIGHT"),
            (ThemeChoice::Dark, "DARK"),
        ];
        let mut row = Row::new().spacing(2);

        for (choice, label) in choices {
            let active = self.theme_choice == choice;
            row = row.push(theme_button(
                c,
                label,
                active,
                (!active).then_some(UiMessage::ThemeSelected(choice)),
            ));
        }

        container(row).padding(2).style(theme_frame_style).into()
    }

    fn key_column(&self) -> Element<'_, UiMessage> {
        let content = column![
            self.source_section(),
            self.mode_section(),
            self.key_section(),
            self.record_section(),
        ]
        .spacing(24)
        .padding(24)
        .width(Length::Fill);

        container(
            scrollable(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(scrollable_style),
        )
        .width(Length::Fixed(KEY_WIDTH))
        .height(Length::Fill)
        .into()
    }

    fn figure_column(&self) -> Element<'_, UiMessage> {
        let caption = self.caption_row();
        let plate = self.plate();

        container(
            column![caption, plate]
                .spacing(CAPTION_GAP)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(FIGURE_PADDING as u16)
        .into()
    }

    fn caption_row(&self) -> Element<'_, UiMessage> {
        let c = self.colors();
        let mut caption = Row::new()
            .push(
                text("FIG. 1")
                    .size(11)
                    .font(Font::MONOSPACE)
                    .color(c.shell_text),
            )
            .push(text("—").size(11).color(c.shell_muted))
            .push(
                text(self.caption())
                    .size(11)
                    .font(SEMIBOLD)
                    .color(c.shell_muted),
            )
            .push(Space::new().width(Length::Fill))
            .spacing(8)
            .align_y(Alignment::Center);

        if let Some(toggle) = self.proof_toggle() {
            caption = caption.push(toggle);
        }

        container(caption)
            .height(Length::Fixed(CAPTION_HEIGHT))
            .width(Length::Fill)
            .into()
    }

    fn proof_toggle(&self) -> Option<Element<'_, UiMessage>> {
        self.image_path.as_ref()?;
        let c = self.colors();

        let vector_available = self.vector_handle.is_some();
        let source_available = self.source_readable && self.source_handle.is_some();

        if !vector_available && !source_available {
            return None;
        }

        let vector_active = self.proof == Proof::Vector && vector_available;
        let source_active = self.proof == Proof::Source && source_available;

        let vector = proof_button(
            c,
            "VECTOR",
            vector_active,
            vector_available,
            (!vector_active).then_some(UiMessage::ProofSelected(Proof::Vector)),
        );
        let source = proof_button(
            c,
            "SOURCE",
            source_active,
            source_available,
            (!source_active).then_some(UiMessage::ProofSelected(Proof::Source)),
        );

        Some(
            container(Row::new().push(vector).push(source).spacing(2))
                .padding(2)
                .style(segment_frame_style)
                .into(),
        )
    }

    fn plate(&self) -> Element<'_, UiMessage> {
        let viewport = self.plate_viewport();
        let hatched = self.vector_handle.is_none() && self.proof == Proof::Vector;
        let scan = (self.is_rendering && self.proof == Proof::Vector && !self.reduce_motion)
            .then_some(self.scan_phase);

        let ground = Canvas::new(PlateGround { hatched })
            .width(Length::Fill)
            .height(Length::Fill);
        let marks = Canvas::new(PlateMarks {
            scan,
            register: self.register_offset,
        })
        .width(Length::Fill)
        .height(Length::Fill);

        let stack = Stack::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Space::new().width(Length::Fill).height(Length::Fill))
            .push(ground)
            .push(self.plate_content(viewport))
            .push(marks);

        container(stack)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(PLATE_FRAME)
            .clip(true)
            .style(plate_frame_style)
            .into()
    }

    fn plate_content(&self, viewport: Size) -> Element<'_, UiMessage> {
        if self.drag_hovering {
            return self.drag_overlay();
        }

        if self.proof == Proof::Source
            && self.source_readable
            && let Some(handle) = &self.source_handle
        {
            return match self.source_size {
                Some(size) => self.media_frame(
                    image(handle.clone())
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(ContentFit::Contain)
                        .into(),
                    size,
                    viewport,
                    false,
                ),
                None => container(
                    image(handle.clone())
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(ContentFit::Contain),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(PLATE_MARGIN as u16)
                .into(),
            };
        }

        if self.render_error.is_some() {
            return self.error_state();
        }

        if let Some(handle) = &self.vector_handle
            && let Some(size) = self.source_size
        {
            return self.media_frame(
                svg(handle.clone())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(ContentFit::Contain)
                    .into(),
                size,
                viewport,
                true,
            );
        }

        if self.is_rendering {
            return self.loading_state();
        }

        self.empty_state()
    }

    fn media_frame<'a>(
        &self,
        media: Element<'a, UiMessage>,
        source_size: (u32, u32),
        viewport: Size,
        bound_raster: bool,
    ) -> Element<'a, UiMessage> {
        let zoom = self.effective_zoom();
        let display = Size::new(source_size.0 as f32 * zoom, source_size.1 as f32 * zoom);
        // iced_wgpu re-rasterizes SVG content at the widget bounds times the
        // window scale factor, keyed by size, so the vector proof's display
        // box is capped; manual zoom stays at 5%–400%, but the raster request
        // stops growing once the cap is reached. Scroll geometry follows the
        // capped box, so the plate never holds a small proof in a large void.
        let display = if bound_raster {
            Self::bounded_vector_display(display)
        } else {
            display
        };
        let padding = 2.0 * PLATE_MARGIN;
        let content_size = Size::new(
            (display.width + padding).max(viewport.width),
            (display.height + padding).max(viewport.height),
        );

        let inner = container(
            container(media)
                .width(Length::Fixed(display.width))
                .height(Length::Fixed(display.height)),
        )
        .width(Length::Fixed(content_size.width))
        .height(Length::Fixed(content_size.height))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        scrollable(inner)
            .direction(scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::new(),
                horizontal: scrollable::Scrollbar::new(),
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .style(scrollable_style)
            .into()
    }

    /// Bounds the raster iced_wgpu allocates for the vector proof. The display
    /// box is capped by its longest edge and by its area, both in logical
    /// pixels, so the raster stays a few tens of MiB even on a HiDPI window.
    /// Sizes at or below both caps keep the full display extent, so the vector
    /// proof stays aligned with the source proof at normal zoom.
    fn bounded_vector_display(display: Size) -> Size {
        let area = display.width * display.height;
        if area <= 0.0 {
            return display;
        }

        let longest = display.width.max(display.height);
        let mut scale = (VECTOR_RASTER_MAX_EDGE / longest).min(1.0);
        if area * scale * scale > VECTOR_RASTER_MAX_PIXELS {
            scale = (VECTOR_RASTER_MAX_PIXELS / area).sqrt();
        }
        if scale >= 1.0 {
            return display;
        }

        Size::new(
            (display.width * scale).round().max(1.0),
            (display.height * scale).round().max(1.0),
        )
    }

    fn empty_state(&self) -> Element<'_, UiMessage> {
        container(
            container(
                column![
                    text("NO PLATE LOADED")
                        .size(20)
                        .font(SEMIBOLD)
                        .color(PLATE_INK),
                    text("Drop a raster onto the plate, or open a file.")
                        .size(13)
                        .color(PLATE_MUTED)
                        .width(Length::Fill)
                        .line_height(1.4)
                        .wrapping(Wrapping::WordOrGlyph),
                    text("PNG · JPG · WEBP · TIFF · BMP · GIF · ICO · TGA · AVIF · PNM · DDS")
                        .size(11)
                        .font(Font::MONOSPACE)
                        .color(PLATE_MUTED)
                        .width(Length::Fill)
                        .line_height(1.4)
                        .wrapping(Wrapping::WordOrGlyph),
                    plate_secondary_button("OPEN IMAGE", UiMessage::OpenImageButtonPressed),
                ]
                .spacing(12)
                .align_x(Alignment::Center)
                .width(Length::Fixed(360.0)),
            )
            .padding(24)
            .style(state_block_style),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }

    fn loading_state(&self) -> Element<'_, UiMessage> {
        container(
            container(
                column![
                    text("TRACING PLATE")
                        .size(20)
                        .font(SEMIBOLD)
                        .color(PLATE_INK),
                    text("Cutting paths from the raster.")
                        .size(13)
                        .color(PLATE_MUTED),
                ]
                .spacing(8)
                .align_x(Alignment::Center),
            )
            .padding(24)
            .style(state_block_style),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }

    fn error_state(&self) -> Element<'_, UiMessage> {
        container(
            container(
                column![
                    text("PLATE FAILED")
                        .size(20)
                        .font(SEMIBOLD)
                        .color(PLATE_RED_DARK),
                    text(self.render_error.clone().unwrap_or_else(|| {
                        "The tracer could not cut a path from this image.".into()
                    }),)
                    .size(13)
                    .color(PLATE_MUTED)
                    .width(Length::Fill)
                    .line_height(1.4)
                    .wrapping(Wrapping::WordOrGlyph),
                    plate_secondary_button("OPEN ANOTHER IMAGE", UiMessage::OpenImageButtonPressed),
                ]
                .spacing(12)
                .align_x(Alignment::Center)
                .width(Length::Fixed(360.0)),
            )
            .padding(24)
            .style(state_block_style),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }

    fn drag_overlay(&self) -> Element<'_, UiMessage> {
        container(
            column![
                text("RELEASE TO PLATE")
                    .size(20)
                    .font(SEMIBOLD)
                    .color(PLATE_INK),
                text("One raster replaces the current plate.")
                    .size(13)
                    .color(PLATE_MUTED),
            ]
            .spacing(8)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_| container::Style {
            background: Some(PLATE_PAPER.into()),
            border: Border {
                color: PLATE_INK,
                width: 2.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    fn source_section(&self) -> Element<'_, UiMessage> {
        let c = self.colors();
        let content: Element<'_, UiMessage> = if let Some(path) = &self.image_path {
            let name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string());
            let format = path
                .extension()
                .map(|extension| extension.to_string_lossy().to_uppercase())
                .unwrap_or_else(|| "IMAGE".into());
            let size = match self.source_size {
                Some((width, height)) => format!("{width} × {height} px"),
                None if self.render_error.is_some() => "UNREADABLE".into(),
                None => "SIZE PENDING".into(),
            };

            column![
                text(name)
                    .size(13)
                    .font(Font::MONOSPACE)
                    .color(c.shell_text)
                    .width(Length::Fill)
                    .line_height(1.4)
                    .wrapping(Wrapping::WordOrGlyph),
                Row::new()
                    .push(
                        text(size)
                            .size(11)
                            .font(Font::MONOSPACE)
                            .color(c.shell_muted),
                    )
                    .push(Space::new().width(Length::Fill))
                    .push(
                        text(format)
                            .size(11)
                            .font(Font::MONOSPACE)
                            .color(c.shell_muted),
                    )
                    .align_y(Alignment::Center),
                text("Drop a new raster anywhere on the window.")
                    .size(11)
                    .color(c.shell_muted)
                    .width(Length::Fill)
                    .line_height(1.4)
                    .wrapping(Wrapping::WordOrGlyph),
            ]
            .spacing(8)
            .into()
        } else {
            column![
                text("NO SOURCE")
                    .size(12)
                    .font(Font::MONOSPACE)
                    .color(c.shell_muted),
                text("Drop a raster on the plate or use OPEN IMAGE.")
                    .size(11)
                    .color(c.shell_muted)
                    .width(Length::Fill)
                    .line_height(1.4)
                    .wrapping(Wrapping::WordOrGlyph),
            ]
            .spacing(8)
            .into()
        };

        section(c, "SOURCE", content)
    }

    fn mode_section(&self) -> Element<'_, UiMessage> {
        let c = self.colors();
        let with_color = self.vector_image_config.with_color;
        let binary_border = if !with_color {
            c.primary_text
        } else {
            c.shell_text
        };
        let color_border = if with_color {
            c.primary_text
        } else {
            c.shell_text
        };
        let binary = mode_button(
            c,
            "BINARY",
            binary_swatch(c, binary_border),
            !with_color,
            if with_color {
                Some(UiMessage::WithColorToggled(false))
            } else {
                None
            },
        );
        let color = mode_button(
            c,
            "COLOR",
            color_swatch(c, color_border),
            with_color,
            if with_color {
                None
            } else {
                Some(UiMessage::WithColorToggled(true))
            },
        );

        let content = column![
            container(Row::new().push(binary).push(color).spacing(2))
                .padding(2)
                .style(segment_frame_style),
            text("Binary cuts ink from paper. Color keeps spot layers.")
                .size(11)
                .color(c.shell_muted)
                .width(Length::Fill)
                .line_height(1.4)
                .wrapping(Wrapping::WordOrGlyph),
        ]
        .spacing(8);

        section(c, "MODE", content.into())
    }

    fn key_section(&self) -> Element<'_, UiMessage> {
        let c = self.colors();
        let config = self.vector_image_config;

        let content: Element<'_, UiMessage> = if config.with_color {
            column![
                param_slider(
                    c,
                    "COLOR PRECISION",
                    config.color_precision.to_string(),
                    0..=8,
                    config.color_precision as u32,
                    UiMessage::ColorPrecisionChanged,
                    "Higher keeps more color detail.",
                ),
                param_slider(
                    c,
                    "GRADIENT STEP",
                    config.gradient_step.to_string(),
                    0..=128,
                    config.gradient_step as u32,
                    UiMessage::GradientStepChanged,
                    "Larger merges nearby colors into fewer layers.",
                ),
                param_slider(
                    c,
                    "SPECKLE FILTER",
                    config.filter_speckle.to_string(),
                    0..=128,
                    config.filter_speckle as u32,
                    UiMessage::FilterSpeckleChanged,
                    "Discards details smaller than N pixels.",
                ),
            ]
            .spacing(16)
            .into()
        } else {
            column![
                param_slider(
                    c,
                    "THRESHOLD",
                    config.binarize_threshold.to_string(),
                    0..=255,
                    config.binarize_threshold as u32,
                    UiMessage::BinarizeThresholdChanged,
                    "Cutoff between ink and paper (0-255).",
                ),
                column![
                    checkbox(config.ignore_alpha_channel)
                        .label("IGNORE ALPHA")
                        .text_size(12)
                        .size(16)
                        .spacing(8)
                        .on_toggle(UiMessage::IgnoreAlphaChannelToggled)
                        .style(checkbox_style),
                    text("Use transparency as the cut mask.")
                        .size(11)
                        .color(c.shell_muted)
                        .width(Length::Fill)
                        .line_height(1.4)
                        .wrapping(Wrapping::WordOrGlyph),
                ]
                .spacing(4),
                checkbox(config.invert_binary)
                    .label("INVERT BLACK / WHITE")
                    .text_size(12)
                    .size(16)
                    .spacing(8)
                    .on_toggle(UiMessage::InvertBinaryToggled)
                    .style(checkbox_style),
                param_slider(
                    c,
                    "SPECKLE FILTER",
                    config.filter_speckle.to_string(),
                    0..=128,
                    config.filter_speckle as u32,
                    UiMessage::FilterSpeckleChanged,
                    "Discards details smaller than N pixels.",
                ),
            ]
            .spacing(16)
            .into()
        };

        section(c, "KEY", content)
    }

    fn record_section(&self) -> Element<'_, UiMessage> {
        let c = self.colors();
        let output = self
            .output_path()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "—".into());

        let mut rows = column![
            column![
                text("OUTPUT").size(11).font(SEMIBOLD).color(c.shell_muted),
                text(output)
                    .size(11)
                    .font(Font::MONOSPACE)
                    .color(c.shell_text)
                    .width(Length::Fill)
                    .line_height(1.4)
                    .wrapping(Wrapping::WordOrGlyph),
            ]
            .spacing(4),
            record_row(c, "FORMAT", "SVG 1.1"),
            record_row(c, "BUILD", env!("CARGO_PKG_VERSION")),
        ]
        .spacing(12);

        rows = rows.push(match &self.save_result {
            Some(Ok(path)) => text(format!("WRITTEN → {}", path.display()))
                .size(11)
                .font(Font::MONOSPACE)
                .color(c.green)
                .width(Length::Fill)
                .line_height(1.4)
                .wrapping(Wrapping::WordOrGlyph),
            Some(Err(message)) => text(message.clone())
                .size(11)
                .color(c.red_text)
                .width(Length::Fill)
                .line_height(1.4)
                .wrapping(Wrapping::WordOrGlyph),
            None => text("NOT WRITTEN")
                .size(11)
                .font(Font::MONOSPACE)
                .color(c.shell_muted),
        });

        section(c, "PLATE RECORD", rows.into())
    }

    fn bottom_bar(&self) -> Element<'_, UiMessage> {
        let c = self.colors();
        let output = self
            .output_path()
            .map(|path| shorten_path(&path, 64))
            .unwrap_or_else(|| "—".into());
        let output_color = if matches!(self.save_result, Some(Ok(_))) {
            c.green
        } else {
            c.shell_muted
        };

        container(
            Row::new()
                .push(
                    text(format!("SVG → {output}"))
                        .size(11)
                        .font(Font::MONOSPACE)
                        .color(output_color)
                        .width(Length::Fill)
                        .wrapping(Wrapping::None),
                )
                .push(self.zoom_controls())
                .spacing(16)
                .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fixed(BOTTOM_BAR_HEIGHT))
        .padding([0, 24])
        .into()
    }

    fn zoom_controls(&self) -> Element<'_, UiMessage> {
        let c = self.colors();
        let enabled = self.zoom_available();
        let percent = if enabled {
            format!("{:.0}%", self.effective_zoom() * 100.0)
        } else {
            "—".into()
        };

        let group = Row::new()
            .push(zoom_button(c, "−", enabled.then_some(UiMessage::ZoomOut)))
            .push(
                text(percent)
                    .size(11)
                    .font(Font::MONOSPACE)
                    .color(c.shell_text)
                    .width(Length::Fixed(44.0))
                    .align_x(Alignment::End),
            )
            .push(zoom_button(c, "+", enabled.then_some(UiMessage::ZoomIn)))
            .push(fit_button(c, enabled, self.fit_zoom && enabled))
            .spacing(2)
            .align_y(Alignment::Center);

        Row::new()
            .push(
                text("ZOOM")
                    .size(11)
                    .font(Font::MONOSPACE)
                    .color(c.shell_muted),
            )
            .push(container(group).padding(2).style(zoom_frame_style))
            .spacing(8)
            .align_y(Alignment::Center)
            .into()
    }

    fn status(&self) -> (&'static str, Color) {
        let c = self.colors();
        if matches!(self.save_result, Some(Err(_))) {
            ("SAVE FAILED", c.red_text)
        } else if self.render_error.is_some() {
            if self.source_size.is_some() {
                ("TRACE FAILED", c.red_text)
            } else {
                ("READ FAILED", c.red_text)
            }
        } else if self.is_saving {
            ("WRITING", c.shell_text)
        } else if self.is_rendering {
            ("TRACING", c.shell_text)
        } else if matches!(self.save_result, Some(Ok(_))) {
            ("SAVED", c.green)
        } else if self.vector_handle.is_some() {
            ("READY", c.teal_text)
        } else {
            ("EMPTY", c.shell_muted)
        }
    }

    fn caption(&self) -> &'static str {
        if self.drag_hovering {
            "DROP TO PLATE"
        } else if self.proof == Proof::Source
            && self.source_readable
            && self.source_handle.is_some()
        {
            "SOURCE RASTER"
        } else if self.render_error.is_some() {
            "ERRATUM"
        } else if self.is_rendering {
            "TRACING"
        } else if self.vector_handle.is_some() {
            "VECTOR TRACE"
        } else {
            "NO PLATE"
        }
    }

    fn plate_viewport(&self) -> Size {
        Size::new(
            (self.window_size.width
                - KEY_WIDTH
                - KEY_RULE
                - 2.0 * FIGURE_PADDING
                - 2.0 * PLATE_FRAME)
                .max(120.0),
            (self.window_size.height
                - TOP_BAR_HEIGHT
                - BOTTOM_BAR_HEIGHT
                - CHROME_RULES
                - 2.0 * FIGURE_PADDING
                - 2.0 * PLATE_FRAME
                - CAPTION_HEIGHT
                - CAPTION_GAP)
                .max(120.0),
        )
    }

    fn fit_scale(&self) -> f32 {
        let Some((width, height)) = self.source_size else {
            return 1.0;
        };
        let viewport = self.plate_viewport();
        let available_width = (viewport.width - 2.0 * PLATE_MARGIN).max(1.0);
        let available_height = (viewport.height - 2.0 * PLATE_MARGIN).max(1.0);
        (available_width / width as f32)
            .min(available_height / height as f32)
            .clamp(0.01, 16.0)
    }

    fn effective_zoom(&self) -> f32 {
        if self.fit_zoom {
            self.fit_scale()
        } else {
            self.zoom
        }
    }

    /// Whether the visible proof can be zoomed and panned.
    fn zoom_available(&self) -> bool {
        match self.proof {
            Proof::Vector => self.vector_handle.is_some(),
            Proof::Source => self.source_readable && self.source_handle.is_some(),
        }
    }

    fn output_path(&self) -> Option<PathBuf> {
        if self.render_error.is_some() && !self.source_readable {
            return None;
        }

        self.image_path
            .as_ref()
            .map(|path| path.with_extension("svg"))
    }

    fn config_changed(&mut self) -> Task<UiMessage> {
        self.save_seq = self.save_seq.wrapping_add(1);
        self.save_result = None;
        self.request_render()
    }

    fn request_render(&mut self) -> Task<UiMessage> {
        if self.image_path.is_none() {
            return Task::none();
        }

        if self.is_rendering {
            if self.pending_image.is_none() {
                self.rerender_pending = true;
            }
            return Task::none();
        }

        self.start_render()
    }

    fn start_render(&mut self) -> Task<UiMessage> {
        let Some(path) = self.image_path.clone() else {
            return Task::none();
        };

        self.render_seq = self.render_seq.wrapping_add(1);
        let seq = self.render_seq;
        self.is_rendering = true;
        self.rerender_pending = false;
        self.render_error = None;
        self.register_offset = 3.0;
        self.settling = false;
        self.scan_phase = 0.0;

        Task::perform(
            Self::render_svg_image(path, self.vector_image_config),
            move |result| UiMessage::SvgImageRendered { seq, result },
        )
    }

    async fn render_svg_image(
        image_path: PathBuf,
        config: VectorImageConfig,
    ) -> Result<RenderedPlate, RenderFailure> {
        let image_data = image_processor::load_image(&image_path).map_err(|error| {
            eprintln!("Failed to load {}: {error}", image_path.display());
            RenderFailure {
                message:
                    "This file could not be read as an image. Try another format or another copy."
                        .to_string(),
                source_size: None,
            }
        })?;
        let (width, height) = image_processor::dimensions(&image_data);
        let svg = image_processor::generate_svg(image_data, config).map_err(|_| {
            RenderFailure {
                message:
                    "The tracer could not cut a path from this image. Try different key values or another file."
                        .to_string(),
                source_size: Some((width, height)),
            }
        })?;

        Ok(RenderedPlate { svg, width, height })
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

    async fn save_svg_to_file(svg_data: String, image_path: PathBuf) -> Result<PathBuf, String> {
        let target_svg_path = image_path.with_extension("svg");
        let mut file = fs::File::create(&target_svg_path).map_err(|_| {
            "Could not create the SVG beside the source. Check folder permissions.".to_string()
        })?;
        file.write_all(svg_data.as_bytes()).map_err(|_| {
            "The SVG could not be written. Check free space and folder permissions.".to_string()
        })?;
        Ok(target_svg_path)
    }
}

/// The plate ground: a paper checkerboard plus the hatched no-data field.
struct PlateGround {
    hatched: bool,
}

impl canvas::Program<UiMessage> for PlateGround {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let size = bounds.size();

        frame.fill_rectangle(Point::new(0.0, 0.0), size, PLATE_PAPER);

        let cell = 16.0;
        let columns = (size.width / cell).ceil() as usize;
        let rows = (size.height / cell).ceil() as usize;
        for row in 0..rows {
            for column in 0..columns {
                if (row + column) % 2 == 0 {
                    frame.fill_rectangle(
                        Point::new(column as f32 * cell, row as f32 * cell),
                        Size::new(cell, cell),
                        PLATE_PAPER_DEEP,
                    );
                }
            }
        }

        if self.hatched {
            draw_hatch(&mut frame, size, PLATE_RULE);
        }

        vec![frame.into_geometry()]
    }
}

/// The plate marks: crop corners, registration offset, and the trace scan line.
struct PlateMarks {
    scan: Option<f32>,
    register: f32,
}

impl canvas::Program<UiMessage> for PlateMarks {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let size = bounds.size();
        let inset = 12.0;
        let arm = 14.0;
        let offset = self.register;
        let stroke = Stroke::default().with_color(PLATE_INK).with_width(1.0);

        let corners = [
            (inset - offset, inset - offset, 1.0, 1.0),
            (size.width - inset + offset, inset - offset, -1.0, 1.0),
            (inset - offset, size.height - inset + offset, 1.0, -1.0),
            (
                size.width - inset + offset,
                size.height - inset + offset,
                -1.0,
                -1.0,
            ),
        ];

        for (x, y, dx, dy) in corners {
            frame.stroke(
                &canvas::Path::line(Point::new(x, y), Point::new(x + dx * arm, y)),
                stroke,
            );
            frame.stroke(
                &canvas::Path::line(Point::new(x, y), Point::new(x, y + dy * arm)),
                stroke,
            );
        }

        let mid_x = size.width / 2.0;
        let mid_y = size.height / 2.0;
        let tick = 4.0;
        let edge_ticks = [
            (
                Point::new(mid_x + offset, inset - tick - offset),
                Point::new(mid_x + offset, inset + tick - offset),
            ),
            (
                Point::new(mid_x + offset, size.height - inset - tick + offset),
                Point::new(mid_x + offset, size.height - inset + tick + offset),
            ),
            (
                Point::new(inset - tick - offset, mid_y + offset),
                Point::new(inset + tick - offset, mid_y + offset),
            ),
            (
                Point::new(size.width - inset - tick + offset, mid_y + offset),
                Point::new(size.width - inset + tick + offset, mid_y + offset),
            ),
        ];

        for (from, to) in edge_ticks {
            frame.stroke(&canvas::Path::line(from, to), stroke);
        }

        if let Some(progress) = self.scan {
            let x = inset + progress * (size.width - 2.0 * inset);
            frame.fill_rectangle(
                Point::new(x - 3.0, inset),
                Size::new(6.0, size.height - 2.0 * inset),
                PLATE_TEAL_WASH,
            );
            frame.stroke(
                &canvas::Path::line(Point::new(x, inset), Point::new(x, size.height - inset)),
                Stroke::default().with_color(PLATE_TEAL).with_width(1.0),
            );
        }

        vec![frame.into_geometry()]
    }
}

fn draw_hatch(frame: &mut Frame, size: Size, color: Color) {
    let step = 14.0;
    let height = size.height;
    let width = size.width;
    let stroke = Stroke::default().with_color(color).with_width(1.0);
    let mut offset = -height;

    while offset < width {
        let x0 = offset.max(0.0);
        let y0 = (x0 - offset).max(0.0);
        let x1 = (offset + height).min(width);
        let y1 = (x1 - offset).min(height);

        if x1 > x0 && y1 > y0 {
            frame.stroke(
                &canvas::Path::line(Point::new(x0, y0), Point::new(x1, y1)),
                stroke,
            );
        }

        offset += step;
    }
}

fn section<'a>(
    c: Colors,
    title: &'static str,
    content: Element<'a, UiMessage>,
) -> Element<'a, UiMessage> {
    column![
        column![
            text(title).size(11).font(SEMIBOLD).color(c.shell_muted),
            hairline(c),
        ]
        .spacing(8),
        content,
    ]
    .spacing(12)
    .width(Length::Fill)
    .into()
}

fn param_slider<'a>(
    c: Colors,
    label: &'static str,
    value: String,
    range: RangeInclusive<u32>,
    current: u32,
    on_change: impl Fn(u32) -> UiMessage + 'a,
    helper: &'static str,
) -> Column<'a, UiMessage> {
    column![
        Row::new()
            .push(text(label).size(12).font(SEMIBOLD).color(c.shell_text),)
            .push(Space::new().width(Length::Fill))
            .push(
                text(value)
                    .size(12)
                    .font(Font::MONOSPACE)
                    .color(c.shell_text),
            )
            .align_y(Alignment::Center),
        slider(range, current, on_change)
            .step(1u32)
            .style(slider_style),
        text(helper)
            .size(11)
            .color(c.shell_muted)
            .width(Length::Fill)
            .line_height(1.4)
            .wrapping(Wrapping::WordOrGlyph),
    ]
    .spacing(8)
}

fn record_row(c: Colors, label: &'static str, value: &'static str) -> Element<'static, UiMessage> {
    Row::new()
        .push(
            text(label)
                .size(11)
                .font(SEMIBOLD)
                .color(c.shell_muted)
                .width(Length::Fixed(56.0)),
        )
        .push(
            text(value)
                .size(11)
                .font(Font::MONOSPACE)
                .color(c.shell_text),
        )
        .align_y(Alignment::Center)
        .into()
}

fn primary_button<'a>(
    c: Colors,
    label: &'a str,
    enabled: bool,
    on_press: Option<UiMessage>,
) -> Element<'a, UiMessage> {
    button(text(label).size(12).font(SEMIBOLD).color(if enabled {
        c.primary_text
    } else {
        c.disabled_text
    }))
    .padding([8, 12])
    .on_press_maybe(on_press)
    .style(primary_style)
    .into()
}

fn secondary_button<'a>(c: Colors, label: &'a str, on_press: UiMessage) -> Element<'a, UiMessage> {
    button(text(label).size(12).font(SEMIBOLD).color(c.shell_text))
        .padding([8, 12])
        .on_press(on_press)
        .style(secondary_style)
        .into()
}

fn plate_secondary_button<'a>(label: &'a str, on_press: UiMessage) -> Element<'a, UiMessage> {
    button(text(label).size(12).font(SEMIBOLD).color(PLATE_INK))
        .padding([8, 12])
        .on_press(on_press)
        .style(plate_secondary_style)
        .into()
}

fn mode_button<'a>(
    c: Colors,
    label: &'static str,
    swatch: Element<'a, UiMessage>,
    active: bool,
    on_press: Option<UiMessage>,
) -> Element<'a, UiMessage> {
    button(
        Row::new()
            .push(swatch)
            .push(text(label).size(12).font(SEMIBOLD).color(if active {
                c.primary_text
            } else {
                c.shell_text
            }))
            .spacing(8)
            .align_y(Alignment::Center),
    )
    .padding([8, 12])
    .width(Length::Fill)
    .on_press_maybe(on_press)
    .style(move |theme, status| segment_style(theme, status, active))
    .into()
}

fn proof_button<'a>(
    c: Colors,
    label: &'static str,
    active: bool,
    enabled: bool,
    on_press: Option<UiMessage>,
) -> Element<'a, UiMessage> {
    let on_press = enabled.then_some(on_press).flatten();
    button(text(label).size(11).font(SEMIBOLD).color(if active {
        c.primary_text
    } else if enabled {
        c.shell_text
    } else {
        c.shell_muted
    }))
    .padding([4, 8])
    .on_press_maybe(on_press)
    .style(move |theme, status| segment_style(theme, status, active))
    .into()
}

fn zoom_button<'a>(
    c: Colors,
    label: &'a str,
    on_press: Option<UiMessage>,
) -> Element<'a, UiMessage> {
    let enabled = on_press.is_some();
    button(
        text(label)
            .size(11)
            .font(Font::MONOSPACE)
            .color(if enabled { c.shell_text } else { c.shell_muted }),
    )
    .padding([4, 8])
    .on_press_maybe(on_press)
    .style(ghost_style)
    .into()
}

fn fit_button(c: Colors, enabled: bool, active: bool) -> Element<'static, UiMessage> {
    button(text("FIT").size(11).font(SEMIBOLD).color(if active {
        c.primary_text
    } else if enabled {
        c.shell_text
    } else {
        c.shell_muted
    }))
    .padding([4, 8])
    .on_press_maybe(enabled.then_some(UiMessage::ZoomFit))
    .style(move |theme, status| segment_style(theme, status, active))
    .into()
}

fn binary_swatch<'a>(_c: Colors, border: Color) -> Element<'a, UiMessage> {
    swatch(PLATE_INK, PLATE_PAPER, border)
}

fn color_swatch<'a>(c: Colors, border: Color) -> Element<'a, UiMessage> {
    swatch(c.teal, c.red, border)
}

fn swatch<'a>(left: Color, right: Color, border: Color) -> Element<'a, UiMessage> {
    container(
        Row::new()
            .push(
                container(Space::new().width(6).height(12)).style(move |_| container::Style {
                    background: Some(left.into()),
                    ..Default::default()
                }),
            )
            .push(
                container(Space::new().width(6).height(12)).style(move |_| container::Style {
                    background: Some(right.into()),
                    ..Default::default()
                }),
            ),
    )
    .padding(1)
    .style(move |_| container::Style {
        border: Border {
            color: border,
            width: 1.0,
            radius: 1.0.into(),
        },
        ..Default::default()
    })
    .into()
}

fn theme_button<'a>(
    c: Colors,
    label: &'static str,
    active: bool,
    on_press: Option<UiMessage>,
) -> Element<'a, UiMessage> {
    button(text(label).size(11).font(SEMIBOLD).color(if active {
        c.primary_text
    } else {
        c.shell_text
    }))
    .padding([4, 8])
    .on_press_maybe(on_press)
    .style(move |theme, status| segment_style(theme, status, active))
    .into()
}

fn theme_frame_style(theme: &Theme) -> container::Style {
    let c = Colors::for_theme(theme);
    container::Style {
        border: Border {
            color: c.shell_rule,
            width: 1.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    }
}

fn hairline(c: Colors) -> Element<'static, UiMessage> {
    container(Space::new().width(Length::Fill).height(1))
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(move |_| container::Style {
            background: Some(c.shell_rule.into()),
            ..Default::default()
        })
        .into()
}

fn vertical_hairline(c: Colors) -> Element<'static, UiMessage> {
    container(Space::new().width(1).height(Length::Fill))
        .width(Length::Fixed(KEY_RULE))
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(c.shell_rule.into()),
            ..Default::default()
        })
        .into()
}

fn state_block_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(PLATE_PAPER.into()),
        ..Default::default()
    }
}

fn plate_frame_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(PLATE_PAPER.into()),
        border: Border {
            color: PLATE_INK,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

fn segment_frame_style(theme: &Theme) -> container::Style {
    let c = Colors::for_theme(theme);
    container::Style {
        border: Border {
            color: c.shell_text,
            width: 1.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    }
}

fn zoom_frame_style(theme: &Theme) -> container::Style {
    let c = Colors::for_theme(theme);
    container::Style {
        border: Border {
            color: c.shell_rule,
            width: 1.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    }
}

fn primary_style(theme: &Theme, status: button::Status) -> button::Style {
    let c = Colors::for_theme(theme);
    let (background, text_color, border) = match status {
        button::Status::Active => (c.primary_fill, c.primary_text, c.primary_fill),
        button::Status::Hovered => (c.primary_hover, c.primary_text, c.primary_hover),
        button::Status::Pressed => (c.primary_pressed, c.primary_text, c.primary_pressed),
        button::Status::Disabled => (c.disabled_fill, c.disabled_text, c.disabled_border),
    };

    button::Style {
        background: Some(background.into()),
        text_color,
        border: Border {
            color: border,
            width: 1.0,
            radius: 2.0.into(),
        },
        shadow: Shadow::default(),
        ..Default::default()
    }
}

fn secondary_style(theme: &Theme, status: button::Status) -> button::Style {
    let c = Colors::for_theme(theme);
    let (background, text_color, border) = match status {
        button::Status::Active => (Color::TRANSPARENT, c.shell_text, c.shell_text),
        button::Status::Hovered => (c.shell_raised, c.shell_text, c.shell_text),
        button::Status::Pressed => (c.shell_deep, c.shell_text, c.shell_text),
        button::Status::Disabled => (Color::TRANSPARENT, c.disabled_text, c.disabled_border),
    };

    button::Style {
        background: Some(background.into()),
        text_color,
        border: Border {
            color: border,
            width: 1.0,
            radius: 2.0.into(),
        },
        shadow: Shadow::default(),
        ..Default::default()
    }
}

fn plate_secondary_style(_theme: &Theme, status: button::Status) -> button::Style {
    let (background, text_color, border) = match status {
        button::Status::Active => (Color::TRANSPARENT, PLATE_INK, PLATE_INK),
        button::Status::Hovered => (PLATE_PAPER_RAISED, PLATE_INK, PLATE_INK),
        button::Status::Pressed => (PLATE_PAPER_DEEP, PLATE_INK, PLATE_INK),
        button::Status::Disabled => (Color::TRANSPARENT, PLATE_MUTED, PLATE_RULE),
    };

    button::Style {
        background: Some(background.into()),
        text_color,
        border: Border {
            color: border,
            width: 1.0,
            radius: 2.0.into(),
        },
        shadow: Shadow::default(),
        ..Default::default()
    }
}

fn ghost_style(theme: &Theme, status: button::Status) -> button::Style {
    let c = Colors::for_theme(theme);
    let (background, text_color) = match status {
        button::Status::Active => (Color::TRANSPARENT, c.shell_text),
        button::Status::Hovered => (c.shell_deep, c.shell_text),
        button::Status::Pressed => (c.shell_rule, c.shell_text),
        button::Status::Disabled => (Color::TRANSPARENT, c.disabled_text),
    };

    button::Style {
        background: Some(background.into()),
        text_color,
        border: Border::default(),
        shadow: Shadow::default(),
        ..Default::default()
    }
}

fn segment_style(theme: &Theme, status: button::Status, active: bool) -> button::Style {
    let c = Colors::for_theme(theme);
    if active {
        let background = match status {
            button::Status::Hovered => c.primary_hover,
            button::Status::Pressed => c.primary_pressed,
            _ => c.primary_fill,
        };

        button::Style {
            background: Some(background.into()),
            text_color: c.primary_text,
            border: Border {
                color: c.primary_fill,
                width: 1.0,
                radius: 1.0.into(),
            },
            shadow: Shadow::default(),
            ..Default::default()
        }
    } else {
        let (background, text_color) = match status {
            button::Status::Hovered => (c.shell_deep, c.shell_text),
            button::Status::Pressed => (c.shell_rule, c.shell_text),
            button::Status::Disabled => (Color::TRANSPARENT, c.disabled_text),
            button::Status::Active => (Color::TRANSPARENT, c.shell_text),
        };

        button::Style {
            background: Some(background.into()),
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 1.0.into(),
            },
            shadow: Shadow::default(),
            ..Default::default()
        }
    }
}

fn checkbox_style(theme: &Theme, status: checkbox::Status) -> checkbox::Style {
    let c = Colors::for_theme(theme);
    let (is_checked, is_hovered, is_disabled) = match status {
        checkbox::Status::Active { is_checked } => (is_checked, false, false),
        checkbox::Status::Hovered { is_checked } => (is_checked, true, false),
        checkbox::Status::Disabled { is_checked } => (is_checked, false, true),
    };

    checkbox::Style {
        background: if is_checked {
            c.primary_fill
        } else {
            c.shell_bg
        }
        .into(),
        icon_color: if is_checked {
            c.primary_text
        } else {
            c.shell_text
        },
        border: Border {
            color: if is_disabled {
                c.disabled_border
            } else if is_checked || is_hovered {
                c.shell_text
            } else {
                c.shell_muted
            },
            width: 1.0,
            radius: 1.0.into(),
        },
        text_color: Some(if is_disabled {
            c.disabled_text
        } else {
            c.shell_text
        }),
    }
}

fn slider_style(theme: &Theme, status: slider::Status) -> slider::Style {
    let c = Colors::for_theme(theme);
    let filled = if matches!(status, slider::Status::Active) {
        c.teal
    } else {
        c.teal_text
    };
    let handle_background = if matches!(status, slider::Status::Dragged) {
        c.shell_text
    } else {
        c.shell_bg
    };

    slider::Style {
        rail: slider::Rail {
            backgrounds: (filled.into(), c.shell_rule.into()),
            width: 2.0,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 1.0.into(),
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Rectangle {
                width: 8,
                border_radius: 1.0.into(),
            },
            background: handle_background.into(),
            border_width: 1.0,
            border_color: c.shell_text,
        },
    }
}

fn scrollable_style(theme: &Theme, _status: scrollable::Status) -> scrollable::Style {
    let c = Colors::for_theme(theme);
    let rail = scrollable::Rail {
        background: None,
        border: Border::default(),
        scroller: scrollable::Scroller {
            background: Color {
                a: 0.55,
                ..c.shell_muted
            }
            .into(),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 2.0.into(),
            },
        },
    };

    scrollable::Style {
        container: container::Style::default(),
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: None,
        auto_scroll: scrollable::AutoScroll {
            background: c.shell_bg.into(),
            border: Border::default(),
            shadow: Shadow::default(),
            icon: c.shell_text,
        },
    }
}

fn shorten_path(path: &Path, max_chars: usize) -> String {
    let value = path.display().to_string();
    let chars: Vec<char> = value.chars().collect();

    if chars.len() <= max_chars {
        return value;
    }

    let head = max_chars / 2 - 2;
    let tail = max_chars - head - 1;
    let head: String = chars[..head].iter().collect();
    let tail: String = chars[chars.len() - tail..].iter().collect();
    format!("{head}…{tail}")
}
