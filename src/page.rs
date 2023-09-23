use std::sync::Arc;

use eframe::{egui, epaint::mutex::Mutex};

#[derive(Default)]
pub struct Report {
    pub text: Arc<Mutex<String>>,
}

impl eframe::App for Report {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Response:");
                ui.text_edit_multiline(&mut *self.text.lock());

                if ui.button("Api Call").clicked() {
                    let url = "http://127.0.0.1:8000/reportstream";
                    let request = ehttp::Request::get(url);
                    let text = self.text.clone();

                    ehttp::streaming::fetch(
                        request,
                        move |result: ehttp::Result<ehttp::streaming::Part>| {
                            let part = match result {
                                Ok(part) => part,
                                Err(_) => {
                                    return std::ops::ControlFlow::Break(());
                                }
                            };

                            match part {
                                ehttp::streaming::Part::Response(response) => {
                                    tracing::debug!("RESPONSE");
                                    tracing::debug!("Status code: {:?}", response.status);
                                    if response.ok {
                                        std::ops::ControlFlow::Continue(())
                                    } else {
                                        std::ops::ControlFlow::Break(())
                                    }
                                }
                                ehttp::streaming::Part::Chunk(chunk) => {
                                    if chunk.is_empty() {
                                        std::ops::ControlFlow::Break(())
                                    } else {
                                        let chunk = String::from_utf8(chunk).unwrap();
                                        text.lock().push_str(&chunk);
                                        std::ops::ControlFlow::Continue(())
                                    }
                                }
                            }
                        },
                    );
                }
            })
        });
    }
}
