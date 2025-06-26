use better_default::Default;


pub mod main_menu;
pub mod serial_menu;
#[derive(Default,Clone, Copy)]
pub enum App_Menu{
    #[default]
    Main_Menu,
    Serial_Menu,
}

#[derive(Default)]
pub struct Device_info{
    serial_port:Option<String>,
}
impl Device_info {
    pub fn default(&self)->Self{
        Self{
            serial_port:None,
        }
    }
}