use crate::constants::EPSILON;
use crate::{Point, Ray, Vector};
use std::ops::{Index, IndexMut, Mul, MulAssign};

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: [[f32; 4]; 4],
}

impl Index<usize> for Matrix {
    type Output = [f32; 4];

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn from_col(cols: [[f32; 4]; 4]) -> Self {
        Matrix { data: cols }
    }

    pub fn from_row(rows: [[f32; 4]; 4]) -> Self {
        let mut matrix = Matrix::new();
        for r in 0..4 {
            for c in 0..4 {
                matrix[c][r] = rows[r][c];
            }
        }

        matrix
    }

    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for col in 0..4 {
            for row in 0..4 {
                if (self.data[col][row] - other.data[col][row]).abs() > EPSILON {
                    return false;
                }
            }
        }

        true
    }
}

impl Mul<Ray> for Matrix {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        Ray {
            origin: self.clone() * rhs.origin,
            direction: self.clone() * rhs.direction,
        }
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let x = rhs.data.x * self[0][0]
            + rhs.data.y * self[1][0]
            + rhs.data.z * self[2][0]
            + rhs.data.w * self[3][0];
        let y = rhs.data.x * self[0][1]
            + rhs.data.y * self[1][1]
            + rhs.data.z * self[2][1]
            + rhs.data.w * self[3][1];
        let z = rhs.data.x * self[0][2]
            + rhs.data.y * self[1][2]
            + rhs.data.z * self[2][2]
            + rhs.data.w * self[3][2];
        let w = rhs.data.x * self[0][3]
            + rhs.data.y * self[1][3]
            + rhs.data.z * self[2][3]
            + rhs.data.w * self[3][3];

        if w != 0.0 {
            Point::new([x / w, y / w, z / w])
        } else {
            Point::new([x, y, z])
        }
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, v: Vector) -> Self::Output {
        Vector::new([
            v.data.x * self[0][0]
                + v.data.y * self[1][0]
                + v.data.z * self[2][0]
                + v.data.w * self[3][0],
            v.data.x * self[0][1]
                + v.data.y * self[1][1]
                + v.data.z * self[2][1]
                + v.data.w * self[3][1],
            v.data.x * self[0][2]
                + v.data.y * self[1][2]
                + v.data.z * self[2][2]
                + v.data.w * self[3][2],
        ])
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut result = Matrix::new();

        for p in 0..4 {
            for r in 0..4 {
                let mut sum = 0.0;
                for c in 0..4 {
                    sum += self[c][r] * rhs[p][c];
                }
                result[p][r] = sum;
            }
        }

        result
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Matrix) {
        for p in 0..4 {
            for r in 0..4 {
                let mut sum = 0.0;
                for c in 0..4 {
                    sum = sum + self[c][r] * rhs[p][c];
                }
                self[p][r] = sum;
            }
        }
    }
}

impl Matrix {
    pub fn transpose(&self) -> Matrix {
        Matrix::from_row(self.data)
    }

    pub fn rotation(pitch: f32, yaw: f32, roll: f32) -> Matrix {
        Matrix::rotation_z(roll) * Matrix::rotation_y(yaw) * Matrix::rotation_x(pitch)
    }

    pub fn rotation_x(angle: f32) -> Matrix {
        let mut r = Matrix::identity();

        r[1][1] = f32::cos(angle);
        r[1][2] = f32::sin(angle);
        r[2][1] = -f32::sin(angle);
        r[2][2] = f32::cos(angle);

        r
    }
    pub fn rotation_y(angle: f32) -> Matrix {
        let mut r = Matrix::identity();

        r[0][0] = f32::cos(angle);
        r[0][2] = -f32::sin(angle);
        r[2][0] = f32::sin(angle);
        r[2][2] = f32::cos(angle);

        r
    }
    pub fn rotation_z(angle: f32) -> Matrix {
        let mut r = Matrix::identity();

        r[0][0] = f32::cos(angle);
        r[0][1] = f32::sin(angle);
        r[1][0] = -f32::sin(angle);
        r[1][1] = f32::cos(angle);

        r
    }

    pub fn translation(vector: Vector) -> Matrix {
        let mut t = Matrix::identity();

        t[3][0] = vector.data.x;
        t[3][1] = vector.data.y;
        t[3][2] = vector.data.z;

        t
    }

    pub fn scaling(vector: Vector) -> Matrix {
        let mut s = Matrix::new();

        s[0][0] = vector.data.x;
        s[1][1] = vector.data.y;
        s[2][2] = vector.data.z;
        s[3][3] = 1.;

        s
    }

    pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix {
        let mut s = Matrix::identity();

        s[1][0] = xy;
        s[2][0] = xz;

        s[0][1] = yx;
        s[2][1] = yz;

        s[0][2] = zx;
        s[1][2] = zy;

        s
    }

    pub fn view(from: Point, to: Point, up: Vector) -> Matrix {
        let forward = (from.clone() - to).normalize();
        let up = up.normalize();
        let right = up.cross(&forward).normalize();
        let up = forward.cross(&right).normalize();

        let orientation = Matrix::from_col([
            [right.data.x, up.data.x, forward.data.x, 0.],
            [right.data.y, up.data.y, forward.data.y, 0.],
            [right.data.z, up.data.z, forward.data.z, 0.],
            [0., 0., 0., 1.],
        ]);

        orientation * Matrix::translation(Vector::new([-from.data.x, -from.data.y, -from.data.z]))
    }

    pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix {
        let tan_half_fov = (fov / 2.0).tan();
        let fov_factor = 1. / tan_half_fov;
        let range = near - far;

        Matrix::from_col([
            [fov_factor / ratio, 0., 0., 0.],
            [0., -fov_factor, 0., 0.],
            [0., 0., far / range, -1.],
            [0., 0., (far * near) / range, 0.],
        ])
    }

    fn lu_decomposition(&self) -> (Matrix, Matrix, Vec<usize>, usize) {
        let mut l = Matrix::new();
        let mut u = self.clone();
        let mut p: Vec<usize> = (0..4).collect();
        let mut s = 0;

        for i in 0..4 {
            let mut max_row = i;
            for row in i..4 {
                if u.data[i][row].abs() > u.data[i][max_row].abs() {
                    max_row = row;
                }
            }

            if max_row != i {
                for col in 0..4 {
                    u.data[col].swap(i, max_row);
                }
                for col in 0..i {
                    l.data[col].swap(i, max_row);
                }
                p.swap(i, max_row); // Update permutation vector
                s += 1;
            }

            // Compute L and U
            let pivot = u.data[i][i];
            for row in i..4 {
                l.data[i][row] = u.data[i][row] / pivot;
            }

            for row in (i + 1)..4 {
                let factor = l.data[i][row];
                for col in i..4 {
                    u.data[col][row] -= factor * u.data[col][i];
                }
            }
        }

        // Set diagonal of L to 1
        for i in 0..4 {
            l.data[i][i] = 1.0;
        }

        (l, u, p, s)
    }

    // Function to compute the determinant using LU Decomposition
    pub fn determinant(&self) -> f32 {
        let (_, u, _, s) = self.lu_decomposition();
        let mut determinant = 1.;

        // Product of diagonal elements of U
        for i in 0..4 {
            determinant = determinant * u[i][i];
        }

        // Adjust for row swaps
        determinant *= (-1.0f32).powi(s as i32);

        determinant
    }

    // Function to compute the inverse using LU Decomposition
    pub fn inverse(&self) -> Option<Matrix> {
        let (l, u, p, _) = self.lu_decomposition();
        let mut inverse = Matrix::new();

        // Check determinant (product of U's diagonal)
        let mut det = 1.0;
        for i in 0..4 {
            det *= u.data[i][i];
        }
        if det == 0.0 {
            return None;
        }

        // Solve for each column of the identity matrix (permuted by p)
        for col in 0..4 {
            // Apply permutation p to the identity column
            let mut b = [0.0; 4];
            for i in 0..4 {
                if p[i] == col {
                    b[i] = 1.0;
                    break;
                }
            }

            // Forward substitution: solve L * y = b
            let mut y = [0.0; 4];
            for row in 0..4 {
                y[row] = b[row];
                for k in 0..row {
                    y[row] -= l.data[k][row] * y[k];
                }
            }

            // Backward substitution: solve U * x = y
            let mut x = [0.0; 4];
            for row in (0..4).rev() {
                x[row] = y[row];
                for k in (row + 1)..4 {
                    x[row] -= u.data[k][row] * x[k];
                }
                x[row] /= u.data[row][row];
            }

            // Assign to inverse matrix (column-major)
            for row in 0..4 {
                inverse.data[col][row] = x[row];
            }
        }

        Some(inverse)
    }
}
