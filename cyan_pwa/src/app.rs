use eframe::egui::{Color32, CtxRef, Direction, FontDefinitions, FontFamily, Layout, Separator, TextStyle, Ui, Vec2};
use cyan_nlg;

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

pub struct PWA {
    pub(crate) size: Vec2,
}

impl PWA {
    pub fn new() -> PWA {
        PWA {
            size: Vec2::new(800., 600.),
        }
    }

    pub fn configure_font(&self, ctx: &CtxRef) {
        let mut fonts = FontDefinitions::default();
        fonts.family_and_size.insert(
            TextStyle::Heading,
            (FontFamily::Proportional, 35.),
        );
        fonts.family_and_size.insert(
            TextStyle::Body,
            (FontFamily::Proportional, 20.),
        );
        ctx.set_fonts(fonts);
    }

    pub fn configure_size(&mut self, ctx: &CtxRef) {
        self.size = Vec2::new(
            ctx.input().screen_rect().width(),
            ctx.input().screen_rect().height(),
        );
    }

    pub fn render_title(&self, ui: &mut Ui) {
        let title: String = String::from("Cyan Text Abstraction Tool");
        ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
            ui.colored_label(WHITE, title);
        });
        ui.add(Separator::default());
    }

    pub fn render_body(&self, ui: &mut Ui) {
        let body: String = cyan_nlg::run_short().join(" ");
        // ui.label(body);
        ui.with_layout(Layout::left_to_right(), |ui| {
            ui.label(body);
        });
    }
}
