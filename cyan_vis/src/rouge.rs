use crate::utils;
use plotters::prelude::*;

pub(crate) fn plot(matrix: [[[f32; 3]; 4]; 3]) {
    let file = "cyan_api/web/static/img/rouge.png";
    let root = BitMapBackend::new(file, (608, 480)).into_drawing_area();
    root.fill(&utils::BGC).unwrap();

    let (upper, lower) = root.split_vertically(240);
    let (ll, lr) = lower.split_horizontally(304);
    let panels = [
        (ll, "Recall"),
        (lr, "Precision"),
        (upper, "F1 Means"),
    ];

    for i in 0..3 {
        let mut chart = ChartBuilder::on(&panels[i].0)
            .x_label_area_size(35)
            .y_label_area_size(35)
            .margin(15)
            .caption(panels[i].1, ("sans-serif", 20))
            .build_cartesian_2d(
                1u32..5u32,
                1u32..4u32,
            )
            .unwrap();

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_labels(4)
            .y_labels(3)
            .x_desc("N-Gram Length")
            .y_desc("Reference")
            .x_label_offset(30)
            .y_label_offset(-30)
            .axis_desc_style(("sans-serif", 12))
            .draw()
            .unwrap();

        let points = (1 as u32..)
            .zip(matrix[i].iter())
            .map(|(x, r)| (1 as u32..)
                .zip(r.iter())
                .map(move |(y, v)| (x, y, v)))
            .flatten();

        let series = points
            .clone()
            .map(|(x, y, v)| {
                utils::heatbar(x, y, *v)
            });

        chart
            .draw_series(series)
            .unwrap();

        chart
            .draw_series(PointSeries::of_element(
                points.map(|(x, y, v)| (x, y, v)),
                5,
                ShapeStyle::from(&utils::FGC).filled(),
                &|v, _size, _style| {
                    EmptyElement::at((v.0, v.1))
                    + Text::new(
                        format!("{:.3}", v.2),
                        (25, -35),
                        ("sans-serif", 15)
                    )
                }
            ))
            .unwrap();
    }

    root.present().expect("Directory not found");
}
