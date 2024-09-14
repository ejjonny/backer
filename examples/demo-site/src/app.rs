#[derive(Default)]
pub struct TemplateApp {}

// impl Default for TemplateApp {
//     fn default() -> Self {
//         Self {
//         }
//     }
// }

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
