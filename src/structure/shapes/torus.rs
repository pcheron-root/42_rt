use crate::constants::EPSILON;
use crate::{Intersect, LocalIntersection, Point, Ray, Vector};
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Torus {
    pub major_radius: f64, // Distance from center of tube to center of torus
    pub minor_radius: f64, // Radius of the tube
}

impl Torus {
    pub fn new(major_radius: f64, minor_radius: f64) -> Self {
        Self {
            major_radius,
            minor_radius,
        }
    }
}

impl Intersect for Torus {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection> {
        let d = ray.direction;
        let o = ray.origin - Point::new(0.0, 0.0, 0.0);

        let min_r = self.minor_radius;
        let big_r = self.major_radius;

        // Precompute
        let dd = d.dot(&d);
        let od = o.dot(&d);
        let oo = o.dot(&o);

        let k = oo - (min_r).powf(2.) + (big_r).powf(2.);
        let dxy2 = d.x * d.x + d.y * d.y;
        let oxy_dot = o.x * d.x + o.y * d.y;
        let oxy2 = o.x * o.x + o.y * o.y;

        // Quartic coefficients
        let a4 = dd * dd;
        let a3 = 4.0 * dd * od;
        let a2 = 2.0 * dd * k + 4.0 * od * od - 4.0 * (big_r).powf(2.) * dxy2;
        let a1 = 4.0 * od * k - 8.0 * (big_r).powf(2.) * oxy_dot;
        let a0 = k * k - 4.0 * (big_r).powf(2.) * oxy2;

        // Solve quartic
        let mut roots = solve_quartic(a4, a3, a2, a1, a0);

        // Keep only positive roots > EPSILON
        roots.retain(|&t| t.is_finite() && t > EPSILON);

        if roots.is_empty() {
            return None;
        }

        // Find nearest intersection
        let t = roots.into_iter().fold(f64::INFINITY, |acc, x| acc.min(x));

        let point = ray.position(t);
        let normal = self.normal_at(point);
        Some(LocalIntersection { point, normal, t })
    }

    fn normal_at(&self, p: Point) -> Vector {
        let big_r = self.major_radius;
        let min_r = self.minor_radius;

        let x = p.x;
        let y = p.y;
        let z = p.z;

        let inner = x * x + y * y + z * z + big_r * big_r - min_r * min_r;

        // Gradient of F(x,y,z)
        let nx = 4.0 * x * inner - 8.0 * big_r * big_r * x;
        let ny = 4.0 * y * inner - 8.0 * big_r * big_r * y;
        let nz = 4.0 * z * inner;

        -Vector::new(nx, ny, nz).normalize()
    }
}

// Solve quadratic ax^2+bx+c=0
fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    if a.abs() < EPSILON {
        if b.abs() < EPSILON {
            return vec![];
        }
        return vec![-c / b];
    }
    let disc = b * b - 4.0 * a * c;
    if disc.abs() < EPSILON {
        return vec![-b / (2.0 * a)];
    }
    if disc > 0.0 {
        let sqrt_disc = disc.sqrt();
        vec![(-b + sqrt_disc) / (2.0 * a), (-b - sqrt_disc) / (2.0 * a)]
    } else {
        vec![]
    }
}

// Solve cubic x^3 + ax^2 + bx + c = 0, return all real roots
fn solve_cubic(a: f64, b: f64, c: f64) -> Vec<f64> {
    let a2 = a * a;
    let q = (3.0 * b - a2) / 9.0;
    let r = (9.0 * a * b - 27.0 * c - 2.0 * a2 * a) / 54.0;
    let disc = q * q * q + r * r;

    if disc.abs() < EPSILON {
        let s = r.cbrt();
        return vec![2.0 * s - a / 3.0, -s - a / 3.0];
    }

    if disc > 0.0 {
        // one real root
        let s = (r + disc.sqrt()).cbrt();
        let t = (r - disc.sqrt()).cbrt();
        return vec![s + t - a / 3.0];
    } else {
        // three real roots
        let rho = (-q * q * q).sqrt();
        let theta = (r / rho).acos();
        let r13 = (-q).sqrt();
        return vec![
            2.0 * r13 * (theta / 3.0).cos() - a / 3.0,
            2.0 * r13 * ((theta + 2.0 * PI) / 3.0).cos() - a / 3.0,
            2.0 * r13 * ((theta + 4.0 * PI) / 3.0).cos() - a / 3.0,
        ];
    }
}

// Quartic solver (Ferrari's method)
fn solve_quartic(a: f64, b: f64, c: f64, d: f64, e: f64) -> Vec<f64> {
    if a.abs() < EPSILON {
        // Handle degenerate cases (cubic or lower)
        if b.abs() < EPSILON {
            if c.abs() < EPSILON {
                if d.abs() < EPSILON {
                    return vec![];
                }
                return vec![-e / d];
            }
            return solve_quadratic(c, d, e);
        }
        return solve_cubic(c / b, d / b, e / b);
    }
    // Normalize coefficients
    let b = b / a;
    let c = c / a;
    let d = d / a;
    let e = e / a;

    // Compute depressed quartic coefficients
    let p = c - 3.0 / 8.0 * b * b;
    let q = d - 0.5 * b * c + b * b * b / 8.0;
    let r = e - 0.25 * b * d + (b * b * c) / 16.0 - 3.0 * b * b * b * b / 256.0;

    // Handle biquadratic case
    if q.abs() < EPSILON {
        let mut roots = vec![];
        for y2 in solve_quadratic(1.0, p, r) {
            if y2 >= 0.0 {
                let y = y2.sqrt();
                roots.push(y - b / 4.0);
                roots.push(-y - b / 4.0);
            }
        }
        return roots;
    }

    // Solve cubic resolvent
    let cubic_a = -p / 2.0;
    let cubic_b = -r;
    let cubic_c = (4.0 * r * p - q * q) / 8.0;
    let mut cubic_roots = solve_cubic(cubic_a, cubic_b, cubic_c);

    // Filter and sort roots
    cubic_roots.retain(|x| x.is_finite());
    cubic_roots.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    const U_EPS: f64 = 1e-7;
    let mut z0 = None;

    // Find suitable cubic root
    for root in cubic_roots {
        let u_sq = 2.0 * root - p;
        if u_sq < 0.0 {
            continue;
        }
        let u_val = u_sq.sqrt();
        if u_val < U_EPS {
            continue;
        }
        z0 = Some(root);
        break;
    }

    let z0 = match z0 {
        Some(z) => z,
        None => return vec![],
    };

    let u = (2.0 * z0 - p).sqrt();
    let v = q / (2.0 * u);

    // Solve resulting quadratics
    let mut roots = vec![];
    roots.extend(solve_quadratic(1.0, u, z0 + v));
    roots.extend(solve_quadratic(1.0, -u, z0 - v));

    // Convert back from depressed coordinates
    for t in roots.iter_mut() {
        *t -= b / 4.0;
    }
    roots
}
