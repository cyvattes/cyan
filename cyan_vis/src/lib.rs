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
        plot_n_grams("ng_abs", abs),
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

    // let fields: Box<[String]> = Box::from(data.iter().map(|(field, _)| field).collect());
    // let b =(1u32..10u32).into_segmented();
    // println!("{:?}", b);
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .build_cartesian_2d(
            (1u32..NGRAM_FIELD_LENGTH as u32).into_segmented(),
            0u32..data[0].1 as u32 + 2,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("N-Gram")
        .x_label_formatter(&|sv| {
            let x: usize = match sv {
                CenterOf(val) => *val,
                _ => 0,
            } as usize;
            format!("{:?}", data[x-1].0)
        })
        .axis_desc_style(("sans-serif", 12))
        .draw()
        .unwrap();

    let mut c = 0;
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data
                .iter()
                .map(|(_, y)| {
                    c += 1;
                    (c, *y as u32)
                })
            )
    ).unwrap();

    root.present().expect("Directory not found");
}
