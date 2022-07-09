use std::fs;
use std::path::{Path, PathBuf};

use gtk::prelude::*;
use gtk::FileChooserDialog;
use image::imageops::FilterType;
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};
use visioncortex::PathSimplifyMode;
use vtracer::{ColorMode, Hierarchical};

struct AppModel {
    image_path: PathBuf,
    preview_svg_path: Option<PathBuf>,
}

enum AppMsg {
    OpenImage(FileChooserDialog),
    ImageSelected(PathBuf),
    CreateVectorPreview(PathBuf),
    VectorPreviewGenerated(PathBuf),
    SaveSvg,
}

struct AppWidgets {
    window: gtk::ApplicationWindow,
    vbox: gtk::Box,
    btn_open_image: gtk::Button,
    btn_save_svg: gtk::Button,
    img_preview: gtk::Image,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::OpenImage(image_chooser) => {
                image_chooser.show();
            }
            AppMsg::ImageSelected(image_path) => {
                self.image_path = image_path.clone();
                send!(sender, AppMsg::CreateVectorPreview(image_path));
            }
            AppMsg::CreateVectorPreview(image_path) => {
                let preview_svg_path = create_vector_preview(&image_path);
                send!(sender, AppMsg::VectorPreviewGenerated(preview_svg_path));
            }
            AppMsg::VectorPreviewGenerated(preview_svg_path) => {
                self.preview_svg_path = Some(preview_svg_path);
            }
            AppMsg::SaveSvg => {
                save_vector_image(self.image_path.as_ref(), 4);
                fs::remove_file(self.preview_svg_path.as_ref().unwrap())
                    .expect("failed to remove svg preview file");
            }
        }
        true
    }
}

fn create_vector_preview(image_path: &PathBuf) -> PathBuf {
    let preview_image_path = create_preview_image(image_path);

    let filter_speckle = 4; // 0-128 threshold

    let svg_image_preview = save_vector_image(&preview_image_path, filter_speckle);

    // Delete preview image
    fs::remove_file(preview_image_path.as_path()).expect("Failed to delete preview image");

    svg_image_preview
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

fn create_preview_image(image_path: &PathBuf) -> PathBuf {
    let preview_image_path = image_path.with_extension("preview.png");

    // Convert the original image to 250px x 250px image
    let image = image::io::Reader::open(image_path)
        .unwrap()
        .decode()
        .unwrap();
    let scaled_image = image.resize(250, 250, FilterType::Lanczos3);
    scaled_image
        .save_with_format(&preview_image_path, image::ImageFormat::Png)
        .unwrap();

    preview_image_path
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    /// Initialize the UI.
    fn init_view(_model: &AppModel, _parent_widgets: &(), sender: Sender<AppMsg>) -> Self {
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

        let chooser_sender = sender.clone();
        let image_chooser = FileChooserDialog::builder()
            .title("Select an image")
            .action(gtk::FileChooserAction::Open)
            .modal(true)
            .build();
        image_chooser.connect_response(move |chooser, response| {
            if response == gtk::ResponseType::Accept {
                let path = chooser.file().unwrap().path().unwrap();
                send!(chooser_sender, AppMsg::ImageSelected(path));
            }
            chooser.destroy();
        });
        image_chooser.add_button("Cancel", gtk::ResponseType::Cancel);
        image_chooser.add_button("Open", gtk::ResponseType::Accept);

        let btn_open_image = gtk::Button::with_label("Open image");
        let btn_save_svg = gtk::Button::with_label("Save SVG");

        let img_preview = gtk::Image::builder()
            .height_request(250)
            .width_request(250)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        // Connect the widgets
        window.set_child(Some(&vbox));
        vbox.append(&btn_open_image);
        vbox.append(&img_preview);
        vbox.append(&btn_save_svg);

        // Connect events
        let btn_open_image_sender = sender.clone();
        btn_open_image.connect_clicked(move |_| {
            send!(
                btn_open_image_sender,
                AppMsg::OpenImage(image_chooser.clone())
            );
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
        }
    }

    /// Return the root widget.
    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }

    /// Update the view to represent the updated model.
    fn view(&mut self, model: &AppModel, _sender: Sender<AppMsg>) {
        if model.preview_svg_path.is_some() {
            let vector_image = model.preview_svg_path.as_ref();
            self.img_preview.set_file(vector_image.unwrap().to_str());
        }
    }
}

fn main() {
    let model = AppModel {
        image_path: Default::default(),
        preview_svg_path: None,
    };
    let app = RelmApp::new(model);
    app.run();
}
