use super::{Itp, RgbU8};
use std::ops::Index;

const M1: f32 = 0.15930176; // 0.1593017578125
const M2: f32 = 78.84375;
const C1: f32 = C3 - C2 + 1.0;
const C2: f32 = 18.851563; // 18.8515625
const C3: f32 = 18.6875;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Lms(pub [f32; 3]);

impl Index<usize> for Lms {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<RgbU8> for Lms {
    fn from(rgb: RgbU8) -> Self {
        let rgb = rgb.0.map(|rgb| f32::from(rgb) / 255.0);
        Lms([
            // 0.412109375, 0.52392578125, 0.06396484375
            0.41210938 * rgb[0] + 0.5239258 * rgb[1] + 0.063964844 * rgb[2],
            // 0.166748046875, 0.720458984375, 0.11279296875
            0.16674805 * rgb[0] + 0.720459 * rgb[1] + 0.11279297 * rgb[2],
            // 0.024169921875, 0.075439453125, 0.900390625
            0.024169922 * rgb[0] + 0.07543945 * rgb[1] + 0.9003906 * rgb[2],
        ])
    }
}

impl From<NonLinearLms> for Lms {
    fn from(nllms: NonLinearLms) -> Self {
        let e1m2 = nllms.0.map(|e1m2| e1m2.powf(1.0 / M2));
        let top = e1m2.map(|e1m2| (e1m2 - C1).max(0.0));
        let bottom = e1m2.map(|e1m2| C2 - C3 * e1m2);
        Lms([top[0] / bottom[0], top[1] / bottom[1], top[2] / bottom[2]]
            .map(|y| 10_000.0 * y.powf(1.0 / M1)))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NonLinearLms(pub [f32; 3]);

impl Index<usize> for NonLinearLms {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<Lms> for NonLinearLms {
    fn from(lms: Lms) -> Self {
        let ym1 = lms.0.map(|lms| lms / 10_000.0).map(|y| y.powf(M1));
        let top = ym1.map(|ym1| C1 + C2 * ym1);
        let bottom = ym1.map(|ym1| 1.0 + C3 * ym1);
        NonLinearLms(
            [top[0] / bottom[0], top[1] / bottom[1], top[2] / bottom[2]].map(|x| x.powf(M2)),
        )
    }
}

impl From<Itp> for NonLinearLms {
    fn from(itp: Itp) -> Self {
        NonLinearLms([
            itp[0] + 0.0172181 * itp[1] + 0.11103 * itp[2],
            itp[0] - 0.0172181 * itp[1] - 0.11103 * itp[2],
            itp[0] + 1.12006 * itp[1] - 0.320627 * itp[2],
        ])
    }
}
