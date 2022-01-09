use plotters::prelude::{*, SegmentValue};

const BGC: RGBColor = RGBColor(251, 241, 199);
const WHT: RGBColor = RGBColor(60, 56, 54);

pub(crate) fn plot(name: &'static str, data: &Vec<(String, u32)>) {
    let file = &format!("cyan_api/web/static/img/{}.png", name);
    let root = BitMapBackend::new(file, (288, 288)).into_drawing_area();
    root.fill(&BGC).unwrap();

    let t: Vec<_> = data.first().unwrap().0
        .split_ascii_whitespace()
        .collect();
    let margin: u32 = (30 * t.len() as u32) + 10;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(margin)
        .margin(5)
        .build_cartesian_2d(
            0u32..data.last().unwrap().1 + 1,
            (1u32..crate::NGRAM_FIELD_LENGTH).into_segmented(),
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .bold_line_style(&WHT.mix(0.3))
        .x_desc("Occurrences")
        .y_label_formatter(&|sv| {
            let i: usize = match sv {
                SegmentValue::Exact(val) => *val,
                SegmentValue::CenterOf(val) => *val,
                _ => crate::NGRAM_FIELD_LENGTH,
            } as usize;
            format!("{:?}", data[i-1].0)
        })
        .axis_desc_style(("sans-serif", 12))
        .draw()
        .unwrap();

    chart.draw_series((1..).zip(data.iter()).map(|(y, (_, x))| {
        let mut bar = Rectangle::new([
                                         (0, SegmentValue::Exact(y)),
                                         (*x, SegmentValue::Exact(y + 1))
                                     ], RED.mix(0.5).filled());
        bar.set_margin(5, 5, 0, 0);
        bar
    }))
        .unwrap();

    root.present().expect("Directory not found");
}
