use num_complex::Complex64;
use std::f64::consts::PI;

fn bit_reverse(num: usize, d: usize) -> usize {
    let mut rev_num = 0;
    for i in 0..d {
        if num >> i & 1 == 1 {
            rev_num |= 1 << (d-1 - i);
        }
    }

    rev_num
}

// ハン窓関数
pub fn hann(n: usize) -> Vec<f64> {
    let mut resu = vec![0.0; n];
    for i    in 0..n {
        resu[i] = 0.5 - 0.5 * (2.0*PI*i as f64 / (n as f64 - 1.0)).cos();
    }

    resu
}

// 時間間引きによるFFT
pub fn fft(frames:& Vec<f64>) -> Vec<Complex64>{
    let n = frames.len();
    let d = (n as f64).log2() as usize;

    assert_eq!(n, 1<<d);

    let mut rev_frames = vec![Complex64::new(0.0, 0.0); n];
    for i in 0..n {
        let rev_i = bit_reverse(i, d);
        rev_frames[i] = frames[rev_i].into();
    }

    let mut segment = 1;
    while segment < n {
        segment <<= 1;
        for i in (0..n).step_by(segment) {
            for j in 0..segment/2 {
                let l = i + j;
                let r = i + j + segment/2;

                let w = Complex64::new(0.0, -2.0 * PI * j as f64 /segment as f64).exp();
                let tmp = rev_frames[l];
                rev_frames[l] = rev_frames[l] + w * rev_frames[r];
                rev_frames[r] = tmp - w * rev_frames[r];
            }
        }
    }

    rev_frames
}

#[cfg(test)]
mod test {
    use num_complex::{Complex64, ComplexFloat};

    use super::{bit_reverse, fft, hann};

    #[test]
    fn test_bit_reverse () {
        assert_eq!(0b000000, bit_reverse(0b000000, 6), "failed 0b000000");
        assert_eq!(0b111111, bit_reverse(0b111111, 6), "failed 0b111111");
        assert_eq!(0b101010, bit_reverse(0b010101, 6), "failed 0b010101");
        assert_eq!(0b1101110, bit_reverse(0b0111011, 7), "failed 0b0111011");
    }

    #[test]
    fn test_fft() {
        let frames = vec![1.0, 2.0, 3.0, 4.0];

        let res = fft(&frames);
        // MATLABで計算した値
        let expected_resu = vec![
            Complex64::new(10.0, 0.0),
            Complex64::new(-2.0, 2.0),
            Complex64::new(-2.0, 0.0),
            Complex64::new(-2.0, -2.0),
        ];

        assert!((res[0] - expected_resu[0]).abs() < 1e-9, "failed 0");
        assert!((res[1] - expected_resu[1]).abs() < 1e-9, "failed 1");
        assert!((res[2] - expected_resu[2]).abs() < 1e-9, "failed 2");
        assert!((res[3] - expected_resu[3]).abs() < 1e-9, "failed 3");
    }

    #[test]
    fn test_hann() {
        let n = 5;
        let ret = hann(n);
        // MATLABで計算した値
        let expected = vec![0.0, 0.5, 1.0, 0.5, 0.0];

        for i in 0..n {
            assert!((ret[i] - expected[i]).abs() < 1e-9, "failed {i}");
        }
    }
}
