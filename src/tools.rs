// 左の音声のみ取り出す
pub fn get_lframe<T>(frames:& Vec<T>, is_stereo: bool) -> Vec<f64>
    where T: Copy + std::convert::Into<f64>
{
    if is_stereo {
        let retu: Vec<f64> = frames.iter()
            .enumerate()
            .filter(|&(i, _)| i%2 == 0)
            .map(|(_, &val)| val.into())
            .collect();

        retu
    }else {
        let retu: Vec<f64> =  frames.iter()
            .enumerate()
            .map(|(_, &val)| val.into())
            .collect();

        retu
    }
}