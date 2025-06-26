use eframe::egui::{self, Color32, InnerResponse, Ui};
use serialport::SerialPortType;

use crate::MyEguiApp;



pub fn serial_view(
    app:&mut MyEguiApp,
    ui: &mut Ui,
    ctx: &egui::Context,
    
)->InnerResponse<()>{
    ui.vertical_centered(|ui|{
        ui.heading("SERIAL VIEW");
        let serial = serialport::available_ports();
        if let Ok(ports)=serial{
            ui.vertical_centered(|ui|{
                egui::Grid::new("Grid")
                    .min_col_width(80.0)
                    
                    .striped(true)
                    .show(ui, |ui| {
                        ui.heading("NAME");
                        ui.heading("PORT");
                        ui.heading("TYPE");
                        ui.heading("PID");
                        ui.heading("BAURATE");
                        ui.heading("CONNECTED");
                        ui.end_row();
                        for p in &ports{
                            match &p.port_type {
                                SerialPortType::UsbPort(info)=>{
                                    ui.label(elide_text(info.product.as_deref().unwrap(), 40));
                                    ui.label(p.port_name.clone());
                                    ui.label(info.manufacturer.as_deref().unwrap());
                                    ui.label(info.pid.to_string());
                                    ui.label("ddd");
                                    if let Some(port)=app.device_info.serial_port.as_deref(){
                                        if port ==p.port_name.as_str(){
                                            ui.add(egui::Spinner::new());
                                        }else{
                                            if ui.button("Connect").clicked(){
                                              app.device_info.serial_port=Some(p.port_name.to_string());
                                            };
                                        }
                                        
                                    }else{
                                        if ui.button("Connect").clicked(){
                                            app.device_info.serial_port=Some(p.port_name.to_string());
                                        };
                                        // ui.label("Not connected");
                                    };
                                    
                                    ui.end_row();
                                },
                                _=>{}
                            }
                        };
                       
                        

                    });
                // ui.columns(5, |columns|{
                //     columns[0].vertical_centered(|ui|{
                //         ui.label("NAME");
                        
                //         for p in &ports{
                //             match &p.port_type {
                //                 SerialPortType::UsbPort(info)=>{
                //                     // ui.add_sized([250.0, 10.0], egui::Label::new(elide_text(info.product.as_deref().unwrap(), 30)));
                //                     // if ui.button(elide_text(info.product.as_deref().unwrap(), 20)).clicked(){

                //                     // };
                //                     ui.label(elide_text(info.product.as_deref().unwrap(), 30));
                //                 },
                //                 _=>{}
                //             }
                //         };
                //         // for i in ports{
                //         //     ui.label(i.port_name);
                //         // }
                //     });
                //     columns[1].vertical_centered(|ui|{
                //         ui.label("PORT");
                //         for i in &ports{
                //             ui.label(i.port_name.as_str());
                //         }
                //     });
                //     columns[2].vertical_centered(|ui|{
                //         ui.label("TYPE");
                //         for p in &ports{
                //             match &p.port_type {
                //                 SerialPortType::UsbPort(info)=>{
                                    
                //                     ui.label(info.manufacturer.as_deref().unwrap());
                //                 },
                //                 _=>{}
                //             }
                //         };
                //         // for i in ports{
                //         //     ui.label(i.port_name);
                //         // }
                //     });
                //     columns[3].vertical_centered(|ui|{
                //         ui.label("PID");
                //         for p in &ports{
                //             match &p.port_type {
                //                 SerialPortType::UsbPort(info)=>{
                                    
                //                     ui.label(info.pid.to_string());
                //                 },
                //                 _=>{}
                //             }
                //         };
                //     });
                //     columns[4].vertical_centered(|ui|{
                //         ui.label("CONECTED");
                //         for p in &ports{
                //             ui.label("FALSE");
                //         };
                //     });
                //     // for p in ports {
                //     //     println!("{}", p.port_name);
                //     // }
                // });
                
            });
            
        }else{
            ui.label("No ports found!");
        }
        
    })
}


fn elide_text(text: &str, max_len: usize) -> String {
    if text.chars().count() > max_len {
        let short: String = text.chars().take(max_len - 3).collect();
        format!("{}...", short)
    } else {
        text.to_string()
    }
}