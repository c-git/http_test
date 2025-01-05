use log::info;
use ui_page::UiPage;
use ui_request_test::UiRequestTest;

mod ui_page;
pub mod ui_request_test;

const HEADING: &str = concat!("Test App ", env!("CARGO_PKG_VERSION"));

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TestApp {
    ui_page: Option<UiPage>,
    ui_request_test: UiRequestTest,
}

impl TestApp {
    const VISUALS_KEY: &str = "visuals";

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        if let Some(visuals) = cc
            .egui_ctx
            .data_mut(|r| r.get_persisted::<egui::Visuals>(egui::Id::new(Self::VISUALS_KEY)))
        {
            info!("Found saved Visuals. Loading...");
            cc.egui_ctx.set_visuals(visuals);
        } else {
            info!("Unable to load Visuals, no saved version found");
        }

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            info!("Attempting to load previous app state");
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TestApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        info!("Saving app state");
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if let Some(ui_page) = self.ui_page.as_mut() {
            match ui_page.show(ctx.clone()) {
                ui_page::ShouldClose::Close => self.ui_page = None,
                ui_page::ShouldClose::LeaveOpen => {}
            };
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                ui.menu_button("File", |ui| {
                    if ui.button("Open UI Settings").clicked() {
                        self.ui_page = Some(Default::default());
                        ui.close_menu();
                    }
                    if !is_web && ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(HEADING);

            self.ui_request_test.show(ui, frame);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
