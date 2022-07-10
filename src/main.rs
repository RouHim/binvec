use std::path::{Path, PathBuf};

use gtk::prelude::*;
use gtk::{FileChooserDialog, FileFilter};
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};
use visioncortex::PathSimplifyMode;
use vtracer::{ColorMode, Hierarchical};

struct AppModel {
    image_path: PathBuf,
    svg_path: Option<PathBuf>,
    filter_speckle: usize,
}

enum AppMsg {
    OpenImageChooser,
    ImageSelected(PathBuf),
    CreateVectorPreview,
    ChangeFilterSpeckle(usize),
    VectorPreviewGenerated(PathBuf),
    SaveSvg,
}

struct AppWidgets {
    window: gtk::ApplicationWindow,
    vbox: gtk::Box,
    btn_open_image: gtk::Button,
    btn_save_svg: gtk::Button,
    img_preview: gtk::Image,
    slider_threshold: gtk::Scale,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::OpenImageChooser => {
                open_image_chooser(sender);
            }
            AppMsg::ImageSelected(image_path) => {
                self.image_path = image_path;
                send!(sender, AppMsg::CreateVectorPreview);
            }
            AppMsg::CreateVectorPreview => {
                let svg_path = create_vector_preview(&self.image_path, self.filter_speckle);
                send!(sender, AppMsg::VectorPreviewGenerated(svg_path));
            }
            AppMsg::VectorPreviewGenerated(svg_path) => {
                self.svg_path = Some(svg_path);
            }
            AppMsg::ChangeFilterSpeckle(filter_speckle) => {
                self.filter_speckle = filter_speckle;
                let svg_path = create_vector_preview(&self.image_path, filter_speckle);
                send!(sender, AppMsg::VectorPreviewGenerated(svg_path));
            }
            AppMsg::SaveSvg => {
                save_vector_image(self.image_path.as_ref(), self.filter_speckle);
                self.svg_path = Some(self.image_path.with_extension("svg"));
            }
        }
        true
    }
}

fn open_image_chooser(sender: Sender<AppMsg>) {
    let file_filter = FileFilter::new();
    file_filter.set_name(Some("Image files"));
    file_filter.add_pattern("*.png");
    file_filter.add_pattern("*.jpg");
    file_filter.add_pattern("*.jpeg");
    file_filter.add_pattern("*.bmp");
    file_filter.add_pattern("*.gif");
    file_filter.add_pattern("*.ico");
    file_filter.add_pattern("*.tiff");
    file_filter.add_pattern("*.webp");
    file_filter.add_pattern("*.pnm");
    file_filter.add_pattern("*.avif");
    file_filter.add_pattern("*.dds");
    file_filter.add_pattern("*.tga");

    let image_chooser = FileChooserDialog::builder()
        .title("Select an image")
        .action(gtk::FileChooserAction::Open)
        .modal(true)
        .filter(&file_filter)
        .build();
    image_chooser.add_button("Cancel", gtk::ResponseType::Cancel);
    image_chooser.add_button("Open", gtk::ResponseType::Accept);
    image_chooser.connect_response(move |chooser, response| {
        if response == gtk::ResponseType::Accept {
            let path = chooser.file().unwrap().path().unwrap();
            send!(sender, AppMsg::ImageSelected(path));
        }

        chooser.destroy();
    });

    image_chooser.show();
}

fn create_vector_preview(image_path: &Path, filter_speckle: usize) -> PathBuf {
    save_vector_image(image_path, filter_speckle)
}

fn save_vector_image(input_path: &Path, filter_speckle: usize) -> PathBuf {
    let image_path = input_path.to_path_buf();
    let svg_path = image_path.with_extension("svg");

    vtracer::convert_image_to_svg(vtracer::Config {
        input_path: image_path,
        output_path: svg_path.clone(),
        color_mode: ColorMode::Binary,
        hierarchical: Hierarchical::Cutout,
        mode: PathSimplifyMode::Spline,
        filter_speckle,
        color_precision: 6,
        layer_difference: 16,
        corner_threshold: 60,
        length_threshold: 4.0,
        splice_threshold: 45,
        max_iterations: 10,
        path_precision: Some(8),
    })
    .expect("Failed to convert image to svg");

    svg_path
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    /// Initialize the UI.
    fn init_view(model: &AppModel, _parent_widgets: &(), sender: Sender<AppMsg>) -> Self {
        let window = gtk::ApplicationWindow::builder()
            .title("BinVec")
            .default_width(1024)
            .default_height(768)
            .build();
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();
        vbox.set_margin_all(5);

        let btn_open_image = gtk::Button::with_label("Open image");
        let btn_save_svg = gtk::Button::with_label("Save SVG");

        let img_preview = gtk::Image::builder()
            .height_request(750)
            .width_request(750)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        let slider_threshold_sender = sender.clone();
        let slider_threshold = gtk::Scale::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        slider_threshold.set_range(0.0, 128.0);
        slider_threshold.set_value(model.filter_speckle as f64);
        slider_threshold.connect_value_changed(move |slider| {
            let value = slider.value() as usize;
            send!(slider_threshold_sender, AppMsg::ChangeFilterSpeckle(value));
        });

        // Connect the widgets
        window.set_child(Some(&vbox));
        vbox.append(&btn_open_image);
        vbox.append(&img_preview);
        vbox.append(&slider_threshold);
        vbox.append(&btn_save_svg);

        // Connect events
        let btn_open_image_sender = sender.clone();
        btn_open_image.connect_clicked(move |_| {
            send!(btn_open_image_sender, AppMsg::OpenImageChooser);
        });

        let btn_save_svg_sender = sender;
        btn_save_svg.connect_clicked(move |_| {
            send!(btn_save_svg_sender, AppMsg::SaveSvg);
        });

        Self {
            window,
            vbox,
            btn_open_image,
            btn_save_svg,
            img_preview,
            slider_threshold,
        }
    }

    /// Return the root widget.
    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }

    /// Update the view to represent the updated model.
    fn view(&mut self, model: &AppModel, _sender: Sender<AppMsg>) {
        if model.svg_path.is_some() {
            let vector_image = model.svg_path.as_ref();
            self.img_preview.set_file(vector_image.unwrap().to_str());
        }
    }
}

fn main() {
    let model = AppModel {
        image_path: Default::default(),
        svg_path: None,
        filter_speckle: 4,
    };
    let app = RelmApp::new(model);
    app.run();
}
