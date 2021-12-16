mod app;
pub use app::PWA;

use eframe::egui::{
    CentralPanel,
    CtxRef,
    ScrollArea,
    Vec2,
};

use eframe::epi::{
    App,
    Frame,
    Storage,
};

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run(canvas_id: &str) {
    let pwa = PWA::new();
    let window = Box::new(pwa);
    eframe::start_web(canvas_id, window).unwrap();
}

impl App for PWA {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_title(ui);
                self.render_body(ui);
            });
        });
    }

    fn setup(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        self.configure_font(ctx);
        self.configure_size(ctx);
    }

    fn name(&self) -> &str {
        "PWA"
    }

    fn max_size_points(&self) -> Vec2 {
        self.size
    }
}
