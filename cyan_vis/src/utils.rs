use plotters::style::RGBColor;
use plotters::prelude::*;

pub(crate) const NGRAM_FIELD_LENGTH: u32 = 10;

pub(crate) const FGC: RGBColor = RGBColor(60, 56, 54);
pub(crate) const BGC: RGBColor = RGBColor(251, 241, 199);
pub(crate) const WHT: RGBColor = RGBColor(252, 253, 252);
pub(crate) const BLK: RGBColor = RGBColor(60, 56, 54);
pub(crate) const BLU: RGBColor = RGBColor(69, 133, 136);
pub(crate) const RED: RGBColor = RGBColor(204, 36, 29);

pub enum TextSource {
    SRC,
    ABS,
}

impl TextSource {
    pub(crate) fn color(&self) -> RGBColor {
        match *self {
            TextSource::SRC => BLU,
            TextSource::ABS => RED,
        }
    }

    pub(crate) fn file(&self) -> &'static str {
        match *self {
            TextSource::SRC => "cyan_api/web/static/img/ng_src.png",
            TextSource::ABS => "cyan_api/web/static/img/ng_abs.png",
        }
    }
}

pub(crate) fn rectangle(x: u32, y: u32, left: bool, color: RGBColor) -> Rectangle<(SegmentValue<u32>, u32)> {
    let mut bar = Rectangle::new([
        (SegmentValue::Exact(x), 0),
        (SegmentValue::Exact(x+1), y),
    ], color.mix(0.8).filled());
    if left {
        bar.set_margin(0, 0, 5, 15);
    } else {
        bar.set_margin(0, 0, 15, 5);
    }
    bar
}

pub(crate) fn heatbar(x: u32, y: u32, v: f32) -> Rectangle<(u32, u32)> {
    Rectangle::new([
        (x, y),
        (x+1, y+1),
    ], HSLColor(
        0.66 - (0.66 * v as f64),
        0.5,
        0.5
    ).filled())
}
