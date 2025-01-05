//! Original structure taken from https://www.egui.rs/#http

mod cookies;

use anyhow::Context as _;
use cookies::{Cookie, CookieDisplayMode};
use reqwest_cross::{
    fetch_plus,
    reqwest::{self, StatusCode},
    Awaiting, DataState,
};

struct ResponseData {
    url: String,
    text: String,
    status: StatusCode,
    size_kb: Option<f32>,
    headers: Vec<(String, String)>,
    cookies: Vec<Cookie>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UiRequestTest {
    url: String,
    cookie_display_mode: CookieDisplayMode,

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
            cookie_display_mode: Default::default(),
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

        ui.horizontal(|ui| {
            ui.label("Cookie Display Mode:");
            ui.radio_value(
                &mut self.cookie_display_mode,
                CookieDisplayMode::NameOnly,
                CookieDisplayMode::NameOnly.to_string(),
            );
            ui.radio_value(
                &mut self.cookie_display_mode,
                CookieDisplayMode::NameValue,
                CookieDisplayMode::NameValue.to_string(),
            );
            ui.radio_value(
                &mut self.cookie_display_mode,
                CookieDisplayMode::Pretty,
                CookieDisplayMode::Pretty.to_string(),
            );
            ui.radio_value(
                &mut self.cookie_display_mode,
                CookieDisplayMode::SingleLine,
                CookieDisplayMode::SingleLine.to_string(),
            );
        });

        ui.separator();

        if let Some(resp_data) = self.resp_data.as_mut() {
            if let DataState::Present(resp) = resp_data {
                ui_resource(ui, resp, self.cookie_display_mode);
            } else {
                let ctx = ui.ctx().clone();
                let url = self.url.clone();
                resp_data.get(|| {
                    let req = self.client.get(&self.url);
                    let response_handler = |resp: reqwest::Result<reqwest::Response>| async {
                        let resp = resp.context("failed to get response, request failed")?;
                        let status = resp.status();
                        let size_kb = resp.content_length().map(|x| x as f32 / 1000.0);
                        let headers = resp
                            .headers()
                            .iter()
                            .map(|(name, value)| {
                                (
                                    name.to_string(),
                                    value.to_str().unwrap_or_default().to_string(),
                                )
                            })
                            .collect();
                        let cookies = resp.cookies().map(|x| x.into()).collect();
                        let text = resp
                            .text()
                            .await
                            .context("failed to get text from response")?;
                        Ok(ResponseData {
                            url,
                            text,
                            status,
                            size_kb,
                            headers,
                            cookies,
                        })
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
        });

        if frame.is_web() {
            ui.label("HINT: try pasting the url of this page into the field above!");
        }
    }

    /// Idempotent as long in all practical respects, unless the fetch already started, but then what else should we do
    fn trigger_fetch(&mut self) {
        self.resp_data = Some(Default::default()); // Clear any current data and set to some to activate the loading and polling
    }
}

fn ui_resource(ui: &mut egui::Ui, resp: &ResponseData, cookie_display_mode: CookieDisplayMode) {
    ui.monospace(format!("url:          {}", resp.url));
    ui.monospace(format!("status:       {}", resp.status,));
    ui.monospace(format!(
        "size:         {}",
        if let Some(size) = resp.size_kb.as_ref() {
            format!("{size:.1} kB")
        } else {
            "[Not Available]".to_string()
        }
    ));
    if ui
        .button("📋 Copy Response")
        .on_hover_text("Click to copy the response body")
        .clicked()
    {
        ui.ctx().copy_text(resp.text.clone());
    }

    ui.separator();

    egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
            egui::CollapsingHeader::new("Response headers")
                .default_open(false)
                .show(ui, |ui| {
                    egui::Grid::new("response_headers")
                        .spacing(egui::vec2(ui.spacing().item_spacing.x * 2.0, 0.0))
                        .show(ui, |ui| {
                            for (k, v) in &resp.headers {
                                ui.label(k);
                                ui.label(v);
                                ui.end_row();
                            }
                        })
                });
            ui.separator();

            egui::CollapsingHeader::new("Cookie Jar")
                .default_open(false)
                .show(ui, |ui| {
                    egui::Grid::new("cookies").show(ui, |ui| {
                        for cookie in &resp.cookies {
                            ui.label(cookie.display(cookie_display_mode));
                            ui.end_row();
                        }
                    })
                });
            ui.separator();

            ui.add(egui::Label::new(&resp.text).selectable(true));
            // TODO 3: Enable syntax highlighting (Let user choose extension)
            use egui_extras as _; // Remove line after implementing syntax highlighting
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
