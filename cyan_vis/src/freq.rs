use std::cmp;
use plotters::prelude::*;

const BGC: RGBColor = RGBColor(251, 241, 199);
const WHT: RGBColor = RGBColor(60, 56, 54);
const BLU: RGBColor = RGBColor(69, 133, 136);
const PPL: RGBColor = RGBColor(177, 98, 134);

pub(crate) fn plot(src: &Vec<(String, u32)>, abs: &Vec<(String, u32)>) {
    let file = "cyan_api/web/static/img/freq.png";
    let root = BitMapBackend::new(file, (608, 288)).into_drawing_area();
    root.fill(&BGC).unwrap();

    let len = cmp::max(src.len(), abs.len()) as u32;
    let max = cmp::max(src[0].1, abs[0].1) as u32;
    let lng = match len == src.len() as u32 {
        true => src.clone(),
        false => abs.clone(),
    };

    println!("{:?}, {:?}", len, max);
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(35)
        .margin(5)
        .build_cartesian_2d(
            (1u32..len).into_segmented(),
            0u32..max + 1,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHT.mix(0.3))
        .y_desc("Count")
        .x_label_formatter(&|sv| {
            println!("{:?}", sv);
            let i: usize = match sv {
                SegmentValue::Exact(val) => *val,
                SegmentValue::CenterOf(val) => *val,
                _ => len-1,
            } as usize;
            format!("{}", lng[i-1].0)
        })
        .axis_desc_style(("sans-serif", 12))
        .draw()
        .unwrap();

    let mut x: u32 = 0;
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLU.mix(0.5).filled())
            .data(src.iter().map(|(_, y)| {
                x += 1;
                (x, *y)
            })),
    ).unwrap();
    // chart.draw_series((1..).zip(src.iter()).map(|(x, (_, y))| {
    //     let mut bar = Rectangle::new([
    //         (SegmentValue::Exact(x), 0),
    //         (SegmentValue::Exact(x+1), *y),
    //     ], BLU.mix(0.5).filled());
    //     bar.set_margin(5, 5, 0, 0);
    //     bar
    // })).unwrap();

    root.present().expect("Directory not found");
}
