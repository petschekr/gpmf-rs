use std::fmt::Display;

/// Generic quaternion orientation data struct for
/// - Camera Orientation
/// - Image Orientation
#[derive(Debug, Default, Clone, Copy)]
pub struct OrientationField {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Display for OrientationField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<w: {:>3.08}, x: {:>3.08}, y: {:>3.08}, z: {:>3.08}>", self.w, self.x, self.y, self.z)
    }
}

impl OrientationField {
    pub fn new(
        wxyz: &[f64],
        scale: f64,
    ) -> Option<Self> {
        Some(Self{
            w: wxyz.get(0)? / scale,
            x: wxyz.get(1)? / scale,
            y: wxyz.get(2)? / scale,
            z: wxyz.get(3)? / scale,
        })
    }
}
