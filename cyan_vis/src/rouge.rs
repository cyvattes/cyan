use crate::utils;
use plotters::prelude::*;

pub(crate) fn plot(matrix: [[[f32; 4]; 3]; 3]) {
    let file = "cyan_api/web/static/img/rouge.png";
    let root = BitMapBackend::new(file, (688, 576)).into_drawing_area();
    root.fill(&utils::BGC).unwrap();

    println!("{:?}", matrix);
    let (upper, lower) = root.split_vertically(288);
    let (ll, lr) = lower.split_horizontally(344);
    let panels = [
        (ll, "Recall"),
        (lr, "Precision"),
        (upper, "F1 Means"),
    ];

    for i in 0..3 {
        let mut matrix = [[0 as f32; 3]; 4];
        matrix = [
            [0.0, 0.5, 0.3],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.7, 0.5, 1.0],
        ];

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

        let series = (1 as u32..)
            .zip(matrix.iter())
            .map(|(x, reference)| (1 as u32..)
                .zip(reference.iter())
                .map(move |(y, v)| (x, y, v)))
            .flatten()
            .map(|(x, y, value)| {
                utils::heatbar(x, y, *value)
            });

        chart.draw_series(series)
            .unwrap();
    }

    root.present().expect("Directory not found");
}
