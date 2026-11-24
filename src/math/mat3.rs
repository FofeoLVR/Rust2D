use std::fmt;

#[derive(Clone, Copy)]
pub struct Mat3 {
    pub data: [f32; 9],
}

impl Mat3 {
    pub(crate) fn identity() -> Self {
        Self {
            data: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub(crate) fn zero() -> Self {
        Self {
            data: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }

    pub(crate) fn transform_point(&self, point: [f32; 3]) -> [f32; 3] {
        let x = self.data[0] * point[0] + self.data[1] * point[1] + self.data[2] * point[2];
        let y = self.data[3] * point[0] + self.data[4] * point[1] + self.data[5] * point[2];
        let w = self.data[6] * point[0] + self.data[7] * point[1] + self.data[8] * point[2];
        [x, y, w]
    }

    pub(crate) fn perspective_divide(point: [f32; 3]) -> Result<[f32; 2], String> {
        if point[2] != 0.0 && point[2].is_finite() && point[1].is_finite() && point[0].is_finite() {
            let x = point[0] / point[2];
            let y = point[1] / point[2];

            if x.is_finite() && y.is_finite() {
                Ok([x, y])
            } else {
                Err(String::from("X or Y must be finite"))
            }
        } else {
            Err(String::from("W can't be zero"))
        }
    }

    pub(crate) fn multiply(&self, other: &Mat3) -> Mat3 {
        let mut result: Mat3 = Mat3::zero();
        for row in 0..3 {
            for col in 0..3 {
                for k in 0..3 {
                    let a = self.data[row * 3 + k];
                    let b = other.data[k * 3 + col];
                    result.data[row * 3 + col] += a * b;
                }
            }
        }
        result
    }

    pub(crate) fn new(i: [f32; 9]) -> Mat3 {
        Self { data: i }
    }
    pub(crate) fn translation(tx: f32, ty: f32) -> Self {
        let mut matrix = Self::identity();
        matrix.data[2] = matrix.data[2] + tx;
        matrix.data[5] = matrix.data[5] + ty;
        matrix
    }

    pub(crate) fn scale(sx: f32, sy: f32) -> Self {
        let mut matrix = Self::identity();

        matrix.data[0] = sx;
        matrix.data[4] = sy;

        matrix
    }

    pub(crate) fn rotation(angle: f32) -> Self {
        let mut matrix = Self::identity();
        let angle = angle.to_radians();
        matrix.data[1] = -angle.sin();
        matrix.data[0] = angle.cos();
        matrix.data[3] = angle.sin();
        matrix.data[4] = angle.cos();
        matrix
    }
}

impl fmt::Display for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..3 {
            writeln!(
                f,
                "[{}, {}, {}]",
                self.data[row * 3],
                self.data[row * 3 + 1],
                self.data[row * 3 + 2]
            )?;
        }

        Ok(())
    }
}
