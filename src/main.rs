#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, FontFamily, FontId, Vec2};

mod fonts;
pub mod model;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([600., 400.])
            .with_max_inner_size([600., 400.])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "egui State Machine",
        options,
        Box::new(|cc| {
            // This gives us image support (note that
            // egui_extras in cargo.toml needs all_loaders)
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(cc)))
        }),
    )
}

// A place to store our data
pub struct MyApp {
    model: model::StateMachine,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            model: Default::default(),
        }
    }
}

impl MyApp {
    // Called once before the first frame.
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load custom fonts
        fonts::setup_custom_fonts(&cc.egui_ctx);
        Self {
            model: Default::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Display depending upon super_state
        match self.model.super_state {
            // First state
            model::SuperState::A => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label(
                        egui::RichText::new("State A")
                            .color(egui::Color32::YELLOW)
                            .font(FontId {
                                size: 80.0,
                                family: FontFamily::Name("bungee".into()),
                            }),
                    );
                    if ui
                        .add(egui::Button::new("Go to state A").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::AtoA;
                    }
                    if ui
                        .add(egui::Button::new("Go to state B").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::AtoB;
                    }
                });

                egui::TopBottomPanel::bottom("my_bottom_panel").show(ctx, |ui| {
                    ui.label(
                        egui::RichText::new("🯰🯱🯲🯳🯴🯵🯶🯷🯸🯹")
                            .color(egui::Color32::LIGHT_GRAY)
                            .font(FontId {
                                size: 12.0,
                                family: FontFamily::Name("seven_seg".into()),
                            }),
                    );
                    // ui.label(
                    //     egui::RichText::new("State A")
                    //         .color(egui::Color32::YELLOW)
                    //         .font(FontId {
                    //             size: 12.0,
                    //             family: FontFamily::Name("bungee".into()),
                    //         }),
                    // );
                });
            }
            // Second state
            model::SuperState::B => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label(
                        egui::RichText::new("State B")
                            .color(egui::Color32::YELLOW)
                            .font(FontId {
                                size: 80.0,
                                family: FontFamily::Name("bungee".into()),
                            }),
                    );
                    if ui
                        .add(egui::Button::new("Go to state B").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::BtoB;
                    }
                    if ui
                        .add(egui::Button::new("Go to state C").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::BtoC;
                    }
                    if ui
                        .add(egui::Button::new("Go to state D").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::BtoD;
                    }
                    if ui
                        .add(egui::Button::new("Go to state E").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::BtoE;
                    }
                });
            }
            model::SuperState::C => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label(
                        egui::RichText::new("State C")
                            .color(egui::Color32::YELLOW)
                            .font(FontId {
                                size: 80.0,
                                family: FontFamily::Name("bungee".into()),
                            }),
                    );
                    if ui
                        .add(egui::Button::new("Go to state C").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::CtoC;
                    }
                    if ui
                        .add(egui::Button::new("Go to state E").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::CtoE;
                    }
                });
            }
            model::SuperState::D => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label(
                        egui::RichText::new("State D")
                            .color(egui::Color32::YELLOW)
                            .font(FontId {
                                size: 80.0,
                                family: FontFamily::Name("bungee".into()),
                            }),
                    );
                    if ui
                        .add(egui::Button::new("Go to state D").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::DtoD;
                    }
                    if ui
                        .add(egui::Button::new("Go to state E").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::DtoE;
                    }
                });
            }
            model::SuperState::E => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label(
                        egui::RichText::new("State E")
                            .color(egui::Color32::YELLOW)
                            .font(FontId {
                                size: 80.0,
                                family: FontFamily::Name("bungee".into()),
                            }),
                    );
                    if ui
                        .add(egui::Button::new("Go to state E").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::EtoE;
                    }
                    if ui
                        .add(egui::Button::new("Go to state A").min_size(Vec2::new(120., 32.)))
                        .clicked()
                    {
                        self.model.transition = model::Transition::EtoA;
                    }
                });
            }
        }
        self.model.update_state_machine();
    }
}
