use std::error::Error;
use chrono::offset::{Local, TimeZone};
use chrono::{Date, Duration};
use futures::future::try_join_all;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use tokio;

pub async fn plot(src: &Vec<String>, abs: &Vec<String>) {
    let handles = vec![
        async { plot_comp(src, abs) },
        async { plot_freq(src, abs) },
    ];
    try_join_all(handles).await;
}

fn plot_comp(_src: &Vec<String>, _abs: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let data = get_data();
    let (to_date, from_date) = (
        parse_time(data[0].0) + Duration::days(1),
        parse_time(data[29].0) - Duration::days(1),
    );

    const FILENAME_SRC: &'static str = "cyan_api/web/static/img/ngram_src.png";
    let root_src = BitMapBackend::new(FILENAME_SRC, (288, 288))
        .into_drawing_area();
    root_src.fill(&WHITE)?;
    let mut chart_src = ChartBuilder::on(&root_src)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(from_date..to_date, 110f32..135f32)?;
    draw_histogram(&data, &mut chart_src);

    const FILENAME_ABS: &'static str = "cyan_api/web/static/img/ngram_abs.png";
    let root_abs = BitMapBackend::new(FILENAME_ABS, (288, 288))
        .into_drawing_area();
    root_abs.fill(&WHITE)?;
    let mut chart_abs = ChartBuilder::on(&root_abs)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(from_date..to_date, 110f32..135f32)?;
    draw_histogram(&data, &mut chart_abs);

    root_src.present().expect("Unable to write file");
    root_abs.present().expect("Unable to write file");
    println!("Files written to {}, {}", FILENAME_SRC, FILENAME_ABS);

    Ok(())
}

fn plot_freq(_src: &Vec<String>, _abs: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let data = get_data();
    let (to_date, from_date) = (
        parse_time(data[0].0) + Duration::days(1),
        parse_time(data[29].0) - Duration::days(1),
    );

    const FILENAME: &'static str = "cyan_api/web/static/img/freq.png";
    let root = BitMapBackend::new(FILENAME, (608, 288))
        .into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(from_date..to_date, 110f32..135f32)?;
    draw_histogram(&data, &mut chart);

    root.present().expect("Unable to write file");
    println!("File written to {}", FILENAME);

    Ok(())
}

fn draw_histogram(data: &Vec<(&str, f32, f32, f32, f32)>, chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedDate<Date<Local>>, RangedCoordf32>>) -> Result<(), Box<dyn std::error::Error>> {
    chart.configure_mesh().light_line_style(&WHITE).draw()?;
    chart.draw_series(
        data.iter()
            .map(|x| CandleStick::new(
                parse_time(x.0),
                x.1,
                x.2,
                x.3,
                x.4,
                &GREEN,
                &RED,
                15)
            ),
    )?;
    Ok(())
}

fn parse_time(t: &str) -> Date<Local> {
    Local
        .datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M")
        .unwrap()
        .date()
}

fn get_data() -> Vec<(&'static str, f32, f32, f32, f32)> {
    return vec![
        ("2019-04-25", 130.0600, 131.3700, 128.8300, 129.1500),
        ("2019-04-24", 125.7900, 125.8500, 124.5200, 125.0100),
        ("2019-04-23", 124.1000, 125.5800, 123.8300, 125.4400),
        ("2019-04-22", 122.6200, 124.0000, 122.5700, 123.7600),
        ("2019-04-18", 122.1900, 123.5200, 121.3018, 123.3700),
        ("2019-04-17", 121.2400, 121.8500, 120.5400, 121.7700),
        ("2019-04-16", 121.6400, 121.6500, 120.1000, 120.7700),
        ("2019-04-15", 120.9400, 121.5800, 120.5700, 121.0500),
        ("2019-04-12", 120.6400, 120.9800, 120.3700, 120.9500),
        ("2019-04-11", 120.5400, 120.8500, 119.9200, 120.3300),
        ("2019-04-10", 119.7600, 120.3500, 119.5400, 120.1900),
        ("2019-04-09", 118.6300, 119.5400, 118.5800, 119.2800),
        ("2019-04-08", 119.8100, 120.0200, 118.6400, 119.9300),
        ("2019-04-05", 119.3900, 120.2300, 119.3700, 119.8900),
        ("2019-04-04", 120.1000, 120.2300, 118.3800, 119.3600),
        ("2019-04-03", 119.8600, 120.4300, 119.1500, 119.9700),
        ("2019-04-02", 119.0600, 119.4800, 118.5200, 119.1900),
        ("2019-04-01", 118.9500, 119.1085, 118.1000, 119.0200),
        ("2019-03-29", 118.0700, 118.3200, 116.9600, 117.9400),
        ("2019-03-28", 117.4400, 117.5800, 116.1300, 116.9300),
        ("2019-03-27", 117.8750, 118.2100, 115.5215, 116.7700),
        ("2019-03-26", 118.6200, 118.7050, 116.8500, 117.9100),
        ("2019-03-25", 116.5600, 118.0100, 116.3224, 117.6600),
        ("2019-03-22", 119.5000, 119.5900, 117.0400, 117.0500),
        ("2019-03-21", 117.1350, 120.8200, 117.0900, 120.2200),
        ("2019-03-20", 117.3900, 118.7500, 116.7100, 117.5200),
        ("2019-03-19", 118.0900, 118.4400, 116.9900, 117.6500),
        ("2019-03-18", 116.1700, 117.6100, 116.0500, 117.5700),
        ("2019-03-15", 115.3400, 117.2500, 114.5900, 115.9100),
        ("2019-03-14", 114.5400, 115.2000, 114.3300, 114.5900),
    ];
}
