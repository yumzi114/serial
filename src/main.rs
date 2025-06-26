#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::{menu, Button};
use crate::app_menu::main_menu::main_view;
use crate::app_menu::serial_menu::serial_view;
use crate::app_menu::{App_Menu, Device_info};
use crate::app_threads::seral_task_thread;
use crate::style::setup_custom_fonts;
mod app_menu;
mod style;
mod app_threads;
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App", 
        native_options, 
        Box::new(|cc| {
            let app=MyEguiApp::new(cc);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            seral_task_thread();
            Ok(Box::new(app))
        }
            
        ));
}

#[derive(Default)]
struct MyEguiApp {
    app_menu:App_Menu,
    device_info:Device_info,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
       egui::CentralPanel::default().show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("MAIN", |ui| {
                    if ui.button("MAIN").clicked() {
                        self.app_menu=app_menu::App_Menu::Main_Menu;
                        // …
                    }
                    
                });
                ui.menu_button("DEVICE", |ui| {
                    if ui.button("SERIAL").clicked() {
                        self.app_menu=app_menu::App_Menu::Serial_Menu;
                        // …
                    }
                });
            });
            match self.app_menu {
                app_menu::App_Menu::Main_Menu=>{
                    main_view(ui, ctx);
                    // ui.label("MAIN");
                },
                app_menu::App_Menu::Serial_Menu=>{
                    serial_view(self,ui,ctx,);
                },
            }
            // ui.horizontal_top(|ui|{
            //     ui.button("sssss");
            //     ui.button("TEST");
            // });
        //    ui.heading("Hello World!");
       });
   }
}