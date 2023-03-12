mod fft;
mod tools;
use tools::get_lframe;
use plotters::prelude::*;
use std::fs::File;
use fft::{fft, hann};
use wav::BitDepth;
use num_complex::Complex64;
use clap::Parser;

#[derive(Debug, Parser)]
struct  Args { 
    #[arg(help = "対象のファイル")]
    target_file: String,

    #[arg(short = 'o', help = "出力するグラフのパス")]
    out_file: Option<String>
}

fn main() {
    let args = Args::parse();

    let target_file = args.target_file;
    let mut out_path = "plot.png".to_string();

    if let Some(path) = args.out_file {
        out_path = path;
    }

    // wavを読み込む
    let mut f = File::open(&target_file).expect("file not found");
    let (header, data) = wav::read(&mut f).unwrap();
    let fs = header.sampling_rate;

    // wavのフレームをleftのみ取り出す
    let is_stereo = header.channel_count == 2;
    let mx;
    let mut frames = match data {
        BitDepth::Eight(v) => { mx = std::u8::MAX as f64; get_lframe(&v, is_stereo) },
        BitDepth::Sixteen(v) => { mx = std::i16::MAX as f64; get_lframe(&v, is_stereo) },
        BitDepth::ThirtyTwoFloat(v) => { mx = std::f32::MAX as f64; get_lframe(&v, is_stereo) },
        BitDepth::TwentyFour(v) => { mx = std::i32::MAX as f64; get_lframe(&v, is_stereo) },
        BitDepth::Empty => panic!("faild to load frames"),
    };  

    // 2の冪乗になるように後ろを切り落とす
    let x = (frames.len() as f64).log2() as usize;
    let mut n = 1<<x;
    frames.truncate(n);

    // 周波数分解能
    let df = fs as f64 / n as f64; 

    // ハン窓関数を適応してFFTする
    let hann_win = hann(n);
    let hanned_frames: Vec<f64> = frames.iter().zip(hann_win.iter()).map(|(f, w)| f*w).collect();
    let hanned_f: Vec<Complex64> = fft(&hanned_frames);
    // 2.0はハン窓関数の補正
    let f: Vec<Complex64> = hanned_f.iter().map(|f| f * 2.0).collect();

    // 正規化 -> 絶対値 -> dB
    let mut amp_log10: Vec<f64> = f.iter()
                                .map(|v| v / Complex64::new(n as f64 / 2.0, 0.0))
                                .map(|v| v.norm())
                                .map(|v| 20.0 * (v / mx).log10())
                                .collect();

    // 前半半分のみ取る
    n /= 2;
    amp_log10.truncate(n);

    // グラフの周波数軸を作る
    let freq: Vec<f64> = (0..n).map(|i| df * i as f64).collect();

    // ここから描画処理
    let root = BitMapBackend::new(&out_path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(&target_file, ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..(n as f64), -40.0..1.0).unwrap();

    chart.configure_mesh()
        .x_desc("Hz")
        .y_desc("dB")
        .axis_desc_style(("sans-serif", 15))
        .draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            freq.iter().zip(amp_log10).map(|(a, b)| (*a as f64, b as f64)),
            &BLUE,
        )).unwrap();

    root.present().unwrap();
}
