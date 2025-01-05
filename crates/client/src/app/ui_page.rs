use tracing::info;

use crate::TestApp;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[serde(default)]
pub struct UiPage {
    #[serde(skip)]
    prev_ui_options: Option<egui::Options>,
}

pub enum ShouldClose {
    Close,
    LeaveOpen,
}

impl UiPage {
    fn save_current_ui_options(&mut self, ctx: egui::Context) {
        let current_ui_options = ctx.options(|o| o.clone());
        self.prev_ui_options = Some(current_ui_options);
        let visuals = ctx.style().visuals.clone();
        ctx.data_mut(|w| w.insert_persisted(egui::Id::new(TestApp::VISUALS_KEY), visuals));
        info!("Saved UI Visuals");
    }

    pub fn show(&mut self, ctx: egui::Context) -> ShouldClose {
        let mut is_open = true;
        egui::Window::new("UI Settings")
            .vscroll(true)
            .hscroll(true)
            .open(&mut is_open)
            .show(&ctx.clone(), |ui| {
                ctx.settings_ui(ui);
                match self.prev_ui_options.as_ref() {
                    Some(prev) => {
                        if ctx.options(|o| o != prev) {
                            self.save_current_ui_options(ctx)
                        }
                    }
                    None => self.save_current_ui_options(ctx),
                }
            });
        if is_open {
            ShouldClose::LeaveOpen
        } else {
            ShouldClose::Close
        }
    }
}
