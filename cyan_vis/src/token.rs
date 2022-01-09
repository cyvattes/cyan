use crate::utils;
use std::cmp;
use plotters::prelude::*;

pub(crate) fn plot(src: &Vec<(String, u32)>, abs: &Vec<(String, u32)>) {
    let file = "cyan_api/web/static/img/freq.png";
    let root = BitMapBackend::new(file, (608, 288)).into_drawing_area();
    root.fill(&utils::BGC).unwrap();

    let len = cmp::max(src.len(), abs.len()) as u32;
    let max = cmp::max(src[0].1, abs[0].1) as u32;
    let lng = match len == src.len() as u32 {
        true => src.clone(),
        false => abs.clone(),
    };

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
        .bold_line_style(&utils::BLK.mix(0.3))
        .y_desc("Count")
        .x_labels(len as usize)
        .x_label_formatter(&|sv| {
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

    let src_color = utils::TextSource::SRC.color();
    let left = (1..).zip(src.iter())
        .map(|(x, (_, y))| utils::rectangle(x, *y, true, src_color));
    chart.draw_series(left)
        .unwrap()
        .label("SRC")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], src_color));

    let abs_color = utils::TextSource::ABS.color();
    let right = (1..).zip(abs.iter())
        .map(|(x, (_, y))| utils::rectangle(x, *y, false, abs_color));
    chart.draw_series(right)
        .unwrap()
        .label("ABS")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], abs_color));

    chart.configure_series_labels()
        .border_style(&utils::FGC)
        .background_style(&utils::WHT.mix(0.8))
        .draw()
        .unwrap();

    root.present().expect("Directory not found");
}
