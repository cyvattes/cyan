mod ngram;

use std::any::Any;
use futures::join;
use plotters::chart::SeriesAnno;
use plotters::coord::types::RangedSlice;
use plotters::prelude::*;
use plotters::prelude::SegmentValue::CenterOf;
use tokio::task::spawn_blocking;

const NGRAM_FIELD_LENGTH: usize = 10;

pub async fn plot(src: &Vec<String>, abs: &Vec<String>) {
    join!(
        plot_n_grams("ng_src", src),
        // plot_n_grams("ng_abs", abs),
        plot_freq(src, abs),
    );
}

async fn plot_n_grams(name: &'static str, text: &Vec<String>) {
    let t = text.to_owned();
    spawn_blocking(move || {
        let top = ngram::get_top(t, NGRAM_FIELD_LENGTH);
        plot_histogram(name, &top);
        println!("{:?}", top);
    })
        .await
        .unwrap();
}

async fn plot_freq(_src: &Vec<String>, _abs: &Vec<String>) {
    spawn_blocking(move || {
        // 1: calculate POS for src, abs
        // 2: graph
    })
        .await
        .unwrap();
}

fn plot_histogram(name: &'static str, data: &Vec<(String, usize)>) {
    let file = &format!("cyan_api/web/static/img/{}.png", name);
    let root = BitMapBackend::new(file, (288, 288)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(60)
        .margin(5)
        .build_cartesian_2d(
            0u32..data[0].1 as u32 + 2,
            (1u32..NGRAM_FIELD_LENGTH as u32).into_segmented(),
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_y_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .x_desc("Occurrences")
        .y_label_formatter(&|sv| {
            let i: usize = match sv {
                CenterOf(val) => *val,
                _ => 0,
            } as usize;
            format!("{:?}", data[i-1].0)
        })
        .axis_desc_style(("sans-serif", 12))
        .draw()
        .unwrap();

    chart.draw_series((1..).zip(data.iter()).map(|(y, (_, x))| {
        let mut bar = Rectangle::new([
                                         (0, SegmentValue::Exact(y)),
                                         (*x as u32, SegmentValue::Exact(y + 1))
                                     ], RED.mix(0.5).filled());
        bar.set_margin(5, 5, 0, 0);
        bar
    }))
        .unwrap();

    root.present().expect("Directory not found");
}
