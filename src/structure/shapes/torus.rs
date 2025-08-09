use std::mem::swap;

use crate::constants::EPSILON;
use crate::{Intersect, LocalIntersection, Point, Ray, Vector};

#[derive(Debug, Clone)]
pub struct Torus {
    pub major_radius: f32, // Distance from center of tube to center of torus
    pub minor_radius: f32, // Radius of the tube
}

impl Torus {
    pub fn new(major_radius: f32, minor_radius: f32) -> Self {
        Self {
            major_radius,
            minor_radius,
        }
    }
}

impl Intersect for Torus {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        let mut po = 1.0;
        let Ra2 = self.major_radius * self.major_radius;
        let ra2 = self.minor_radius * self.minor_radius;
        let ro = Vector::new(ray.origin.x, ray.origin.y, ray.origin.z);
        let rd = ray.direction;
        let m = ro.dot(&ro);
        let n = ro.dot(&rd);
        let h = n * n - m
            + (self.major_radius + self.minor_radius) * (self.major_radius + self.minor_radius);
        if h < 0.0 {
            return None;
        }
        let k = (m - ra2 - Ra2) / 2.0;
        let mut k3 = n;
        let mut k2 = n * n + Ra2 * rd.z * rd.z + k;
        let mut k1 = k * n + Ra2 * ro.z * rd.z;
        let mut k0 = k * k + Ra2 * ro.z * ro.z - Ra2 * ra2;

        if (k3 * (k3 * k3 - k2) + k1).abs() < 0.01 {
            po = -1.0;
            swap(&mut k1, &mut k3);
            k0 = 1.0 / k0;
            k1 = k1 * k0;
            k2 = k2 * k0;
            k3 = k3 * k0;
        }

        let c2 = (2.0 * k2 - 3.0 * k3 * k3) / 3.0;
        let c1 = (k3 * (k3 * k3 - k2) + k1) * 2.0;
        let c0 = (k3 * (k3 * (-3.0 * k3 * k3 + 4.0 * k2) - 8.0 * k1) + 4.0 * k0) / 3.0;

        let Q = c2 * c2 + c0;
        let R = 3.0 * c0 * c2 - c2 * c2 * c2 - c1 * c1;
        let mut h = R * R - Q * Q * Q;
        let mut z = 0.0;

        if h < 0.0 {
            let sQ = Q.sqrt();
            z = c2 - (2.0 * sQ * ((R / (sQ * Q)).acos() / 3.0).cos());
        } else {
            let sQ = (h.sqrt() + R.abs()).powf(1.0 / 3.0);
            z = c2 - (R.signum() * (sQ + Q / sQ).abs());
        }

        let mut d1 = z - 3.0 * c2;
        let mut d2 = z * z - 3.0 * c0;
        if d1.abs() < EPSILON {
            if d2 < 0.0 {
                return None;
            }
            d2 = d2.sqrt();
        } else {
            if d1 < 0.0 {
                return None;
            }
            d1 = (d1 / 2.0).sqrt();
            d2 = c1 / d1;
        }

        let mut result = 1e20;

        h = d1 * d1 - z + d2;
        if h > 0.0 {
            h = (h).sqrt();
            let t1 = if po < 0.0 {
                2.0 / (-d1 - h - k3)
            } else {
                -d1 - h - k3
            };
            let t2 = if po < 0.0 {
                2.0 / (-d1 + h - k3)
            } else {
                -d1 + h - k3
            };
            if t1 > 0.0 {
                result = t1
            };
            if t2 > 0.0 {
                result = result.min(t2)
            };
        }

        h = d1 * d1 - z - d2;
        if h > 0.0 {
            h = (h).sqrt();
            let t1 = if po < 0.0 {
                2.0 / (d1 - h - k3)
            } else {
                d1 - h - k3
            };
            let t2 = if po < 0.0 {
                2.0 / (d1 + h - k3)
            } else {
                d1 + h - k3
            };
            if t1 > 0.0 {
                result = result.min(t1);
            };
            if t2 > 0.0 {
                result = result.min(t2);
            };
        }

        let t = result;
        let point = ray.position(t);
        Some(LocalIntersection {
            point,
            normal: self.normal_at(point),
            t,
        })
    }

    fn normal_at(&self, point: Point) -> Vector {
        // For a torus centered at origin with major radius R and minor radius r:
        // The normal at point P is calculated as follows:

        let x = point.x;
        let y = point.y;
        let z = point.z;

        // Distance from the point to the Z-axis (in XY plane)
        let rho = (x * x + y * y).sqrt();

        // If we're exactly on the Z-axis, handle the degenerate case
        if rho < f32::EPSILON {
            // Point is on the Z-axis, normal points radially outward in XY plane
            // This is a degenerate case that shouldn't normally occur on a torus surface
            return Vector::new(1.0, 0.0, 0.0);
        }

        // Unit vector pointing from Z-axis toward the point in XY plane
        let u_rho = Vector::new(x / rho, y / rho, 0.0);

        // Center of the tube circle that this point lies on
        // This is at distance major_radius from the origin in the XY plane
        let tube_center = u_rho * self.major_radius;

        // Vector from tube center to the point
        let to_point = Vector::new(x, y, z) - tube_center;

        // The normal is this vector normalized
        to_point.normalize()
    }
}
