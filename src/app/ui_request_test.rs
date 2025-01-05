//! Original structure taken from https://www.egui.rs/#http

use anyhow::Context as _;
use reqwest_cross::{fetch_plus, reqwest, Awaiting, DataState};

struct ResponseData {
    url: String,
    text: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UiRequestTest {
    url: String,

    #[serde(skip)]
    client: reqwest::Client,

    #[serde(skip)]
    /// When Option::None we don't want to load anything otherwise try to load current url
    resp_data: Option<DataState<ResponseData>>,
}

impl Default for UiRequestTest {
    fn default() -> Self {
        Self {
            url: "https://github.com/c-git/reqwest_w_egui_testing/blob/main/README.md".to_owned(),
            resp_data: Default::default(),
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .expect("failed to create reqwest client"),
        }
    }
}

impl UiRequestTest {
    pub fn show(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.ui_url(ui, frame);

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("HTTP requests made using ");
            ui.hyperlink_to("reqwest-cross", "https://github.com/c-git/reqwest-cross");
            ui.label(".");
        });

        ui.separator();

        if let Some(resp_data) = self.resp_data.as_mut() {
            if let DataState::Present(resp) = resp_data {
                ui_resource(ui, resp);
            } else {
                let ctx = ui.ctx().clone();
                let url = self.url.clone();
                resp_data.get(|| {
                    let req = self.client.get(&self.url);
                    let response_handler = |resp: reqwest::Result<reqwest::Response>| async {
                        let text = resp
                            .context("failed to get response, request failed")?
                            .text()
                            .await
                            .context("failed to get text from response")?;
                        Ok(ResponseData { url, text })
                    };
                    let ui_notify = move || {
                        ctx.request_repaint();
                    };
                    let rx = fetch_plus(req, response_handler, ui_notify);
                    Awaiting(rx)
                });
            }
        }
    }

    fn ui_url(&mut self, ui: &mut egui::Ui, frame: &eframe::Frame) {
        ui.horizontal(|ui| {
            ui.label("URL:");
            if ui
                .add(egui::TextEdit::singleline(&mut self.url).desired_width(f32::INFINITY))
                .lost_focus()
            {
                self.trigger_fetch();
            }
            // TODO 2: Add a submit button (maybe use a right to left layout and put the button first)
        });

        if frame.is_web() {
            ui.label("HINT: paste the url of this page into the field above!");
        }
    }

    /// Idempotent as long in all practical respects, unless the fetch already started, but then what else should we do
    fn trigger_fetch(&mut self) {
        self.resp_data = Some(Default::default()); // Clear any current data and set to some to activate the loading and polling
    }
}

fn ui_resource(ui: &mut egui::Ui, resp: &ResponseData) {
    ui.monospace(format!("url:          {}", resp.url));
    // ui.monospace(format!(
    //     "status:       {} ({})",
    //     response.status, response.status_text
    // ));
    // ui.monospace(format!(
    //     "content-type: {}",
    //     response.content_type().unwrap_or_default()
    // ));
    // ui.monospace(format!(
    //     "size:         {:.1} kB",
    //     response.bytes.len() as f32 / 1000.0
    // ));

    ui.separator();

    egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
            // egui::CollapsingHeader::new("Response headers")
            //     .default_open(false)
            //     .show(ui, |ui| {
            //         egui::Grid::new("response_headers")
            //             .spacing(egui::vec2(ui.spacing().item_spacing.x * 2.0, 0.0))
            //             .show(ui, |ui| {
            //                 for (k, v) in &response.headers {
            //                     ui.label(k);
            //                     ui.label(v);
            //                     ui.end_row();
            //                 }
            //             })
            //     });

            // ui.separator();

            let tooltip = "Click to copy the response body";
            if ui.button("📋").on_hover_text(tooltip).clicked() {
                ui.ctx().copy_text(resp.text.clone());
            }
            ui.separator();

            ui.add(egui::Label::new(&resp.text).selectable(true));
        });
}

// ----------------------------------------------------------------------------
// Syntax highlighting:

// fn syntax_highlighting(
//     ctx: &egui::Context,
//     response: &ehttp::Response,
//     text: &str,
// ) -> Option<ColoredText> {
//     let extension_and_rest: Vec<&str> = response.url.rsplitn(2, '.').collect();
//     let extension = extension_and_rest.first()?;
//     let theme = egui_extras::syntax_highlighting::CodeTheme::from_style(&ctx.style());
//     Some(ColoredText(egui_extras::syntax_highlighting::highlight(
//         ctx,
//         &ctx.style(),
//         &theme,
//         text,
//         extension,
//     )))
// }

// struct ColoredText(egui::text::LayoutJob);

// impl ColoredText {
//     pub fn ui(&self, ui: &mut egui::Ui) {
//         let mut job = self.0.clone();
//         job.wrap.max_width = ui.available_width();
//         let galley = ui.fonts(|f| f.layout_job(job));
//         ui.add(egui::Label::new(galley).selectable(true));
//     }
// }
