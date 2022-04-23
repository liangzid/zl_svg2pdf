use eframe::{egui, epi};
mod svg2pdff;
use rfd;
use svg2pdff::mysvg2pdf;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    // label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    // value: f32,
    is_show: i32,
    dst_path: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            // label: "Hello World!".to_owned(),
            // value: 2.7,
            is_show: 0,
            dst_path: "./newfile.pdf".to_owned(),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "SVG2PDF"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
    //     let Self {
    //         _,
    //         _,
    //         _,
    //         ..
    //     } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("SVG2PDF");
            // let mut my_string="Please insert your target path";

            let mut src_path: Option<String> = Some("D://1.svg".to_owned());

            if ui.button("read svg files...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    src_path = Some(path.display().to_string());
                }
            }
            if let Some(src_path) = &src_path {
                ui.horizontal(|ui| {
                    ui.label("Src file: ");
                    ui.label(src_path);
                });
            }

            ui.label("Pelase insert your target path");
            ui.end_row();
            let _dst_from = ui.text_edit_singleline(&mut self.dst_path);
            ui.end_row();
            // if _dst_from.changed(){
            // 	_dst_from.show(&mut dst_path);
            // }

            if ui.button("transform!").clicked() {
                mysvg2pdf(&src_path.unwrap(), &self.dst_path);
                self.is_show = 1;
            }
            if self.is_show == 1 {
                ui.label("Transform DONE!");
                ui.end_row();
                ui.label("Target path: ".to_owned() + &self.dst_path);
                ui.end_row();
            }

            egui::warn_if_debug_build(ui);
        });

        //     if false {
        //         egui::Window::new("Window").show(ctx, |ui| {
        //             ui.label("Windows can be moved by dragging them.");
        //             ui.label("They are automatically sized based on contents.");
        //             ui.label("You can turn on resizing and scrolling if you like.");
        //             ui.label("You would normally chose either panels OR windows.");
        //         });
        //     }
    }
}
