use eframe::egui::{self, InnerResponse, Ui};



pub fn main_view(
    ui: &mut Ui,
    ctx: &egui::Context,
)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.heading("MAIN VIEW");
    })
}