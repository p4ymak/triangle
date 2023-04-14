#[cfg(test)]
mod tests;
use num_traits::{Float, FloatConst};
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Point<F> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float + FloatConst> Point<F> {
    ///Creates new Point.
    pub fn new(x: F, y: F, z: F) -> Point<F> {
        Point { x, y, z }
    }

    ///Cross product of two Points coordinates.
    pub fn cross(&self, pt: &Point<F>) -> Point<F> {
        Point {
            x: self.y * pt.z - pt.y * self.z,
            y: self.z * pt.x - pt.z * self.x,
            z: self.x * pt.y - pt.x * self.y,
        }
    }

    ///Dot product of two points coordinates.
    pub fn dot(&self, pt: &Point<F>) -> F {
        self.x * pt.x + self.y * pt.y + self.z * pt.z
    }

    ///Calculates distance to another Point.
    pub fn distance_to(&self, pt: &Point<F>) -> F {
        ((self.x - pt.x).powi(2) + (self.y - pt.y).powi(2) + (self.z - pt.z).powi(2)).sqrt()
    }

    ///Normalize coordinates of the Point.
    pub fn normalized(&self) -> Point<F> {
        let mut n = self.distance_to(&Point {
            x: F::zero(),
            y: F::zero(),
            z: F::zero(),
        });
        if n == F::zero() {
            n = F::one();
        }
        Point {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

impl<F: Float + FloatConst> Add for Point<F> {
    type Output = Self;
    ///Calculates sum of two Points.
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<F: Float + FloatConst> Sub for Point<F> {
    type Output = Self;
    ///Calculates subtraction of two Points.
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Triangle<F> {
    pub a: Point<F>,
    pub b: Point<F>,
    pub c: Point<F>,
}

impl<F: Float + FloatConst> Triangle<F> {
    ///Creates new Triangle.
    pub fn new(a: Point<F>, b: Point<F>, c: Point<F>) -> Triangle<F> {
        Triangle { a, b, c }
    }

    ///Creates new Triangle from array of Points.
    pub fn from_array(points: [Point<F>; 3]) -> Triangle<F> {
        Triangle {
            a: points[0],
            b: points[1],
            c: points[2],
        }
    }

    ///Returns two opposite points of axis-aligned bounding box.
    pub fn aabb(&self) -> [Point<F>; 2] {
        let mut c_x = [self.a.x, self.b.x, self.c.x];
        let mut c_y = [self.a.y, self.b.y, self.c.y];
        let mut c_z = [self.a.z, self.b.z, self.c.z];
        c_x.sort_by(|i, j| i.partial_cmp(j).unwrap());
        c_y.sort_by(|i, j| i.partial_cmp(j).unwrap());
        c_z.sort_by(|i, j| i.partial_cmp(j).unwrap());
        [
            Point {
                x: c_x[0],
                y: c_y[0],
                z: c_z[0],
            },
            Point {
                x: c_x[2],
                y: c_y[2],
                z: c_z[2],
            },
        ]
    }

    ///Gets angles of the triangle.
    pub fn angles(&self) -> Option<[F; 3]> {
        if self.is_collinear() {
            return None;
        }
        let [la, lb, lc] = self.sides();
        let two = F::from(2).expect("cast from 2");
        let alpha = ((lb.powi(2) + lc.powi(2) - la.powi(2)) / (two * lb * lc)).acos();
        let beta = ((la.powi(2) + lc.powi(2) - lb.powi(2)) / (two * la * lc)).acos();
        let gamma = F::PI() - alpha - beta;
        Some([alpha, beta, gamma])
    }

    ///Gets area of the triangle.
    pub fn area(&self) -> F {
        let s = self.semiperimeter();
        let [la, lb, lc] = self.sides();
        (s * (s - la) * (s - lb) * (s - lc)).sqrt()
    }

    ///Converts barycentric coordinates of given point to cartesian coordinate system.
    pub fn barycentric_to_cartesian(&self, pt: &Point<F>) -> Point<F> {
        let x = pt.x * self.a.x + pt.y * self.b.x + pt.z * self.c.x;
        let y = pt.x * self.a.y + pt.y * self.b.y + pt.z * self.c.y;
        let z = pt.x * self.a.z + pt.y * self.b.z + pt.z * self.c.z;
        Point { x, y, z }
    }

    ///Converts cartesian coordinates of given point to barycentric coordinate system.
    pub fn cartesian_to_barycentric(&self, pt: &Point<F>) -> Point<F> {
        let v0 = Point {
            x: self.b.x - self.a.x,
            y: self.b.y - self.a.y,
            z: self.b.z - self.a.z,
        };
        let v1 = Point {
            x: self.c.x - self.a.x,
            y: self.c.y - self.a.y,
            z: self.c.z - self.a.z,
        };
        let v2 = Point {
            x: pt.x - self.a.x,
            y: pt.y - self.a.y,
            z: pt.z - self.a.z,
        };
        let one = F::from(1).expect("cast from 1");
        let den = one / (v0.x * v1.y - v1.x * v0.y);
        let v = (v2.x * v1.y - v1.x * v2.y) * den;
        let w = (v0.x * v2.y - v2.x * v0.y) * den;
        let u = one - v - w;
        Point { x: u, y: v, z: w }
    }

    ///Gets centroid of the triangle.
    pub fn centroid(&self) -> Point<F> {
        let three = F::from(3).expect("cast from 3");
        Point {
            x: (self.a.x + self.b.x + self.c.x) / three,
            y: (self.a.y + self.b.y + self.c.y) / three,
            z: (self.a.z + self.b.z + self.c.z) / three,
        }
    }

    ///Gets radius of a circle that passes through all of the triangle's vertices, so called
    ///circumradius.
    pub fn circumradius(&self) -> Option<F> {
        if self.is_collinear() {
            return None;
        }
        let one = F::from(1).expect("cast from 1");
        let four = F::from(4).expect("cast from 4");
        Some(self.sides().iter().fold(one, |acc, x| acc * *x) / (four * self.area()))
    }

    ///Checks whether a given point lies inside the triangle.
    pub fn has_point(&self, pt: Point<F>) -> bool {
        let zero = F::from(0).expect("cast from 0");
        let d1 = sign(&pt, &self.a, &self.b);
        let d2 = sign(&pt, &self.b, &self.c);
        let d3 = sign(&pt, &self.c, &self.a);
        let has_neg = (d1 < zero) || (d2 < zero) || (d3 < zero);
        let has_pos = (d1 > zero) || (d2 > zero) || (d3 > zero);
        !(has_neg && has_pos)
    }

    ///Gets the heights of the triangle.
    pub fn heights(&self) -> Option<[F; 3]> {
        if self.is_collinear() {
            return None;
        }
        let two = F::from(2).expect("cast from 2");
        let double_area = two * self.area();
        let [la, lb, lc] = self.sides();
        Some([double_area / la, double_area / lb, double_area / lc])
    }

    ///Gets radius of a circle which is tangent to each side of the triangle, so called inradius.
    pub fn inradius(&self) -> Option<F> {
        if self.is_collinear() {
            return None;
        }
        Some(self.area() / self.semiperimeter())
    }

    ///Checks if points of triangle are collinear.
    pub fn is_collinear(&self) -> bool {
        self.area().eq(&F::from(0).expect("cast from 0"))
    }

    ///Checks if the triangle is equilateral.
    pub fn is_equilateral(&self) -> bool {
        let sides = self.sides();
        sides[0].eq(&sides[1]) && sides[1].eq(&sides[2])
    }

    ///Checks if the triangle is golden or sublime.
    pub fn is_golden(&self) -> bool {
        if !self.is_isosceles() {
            return false;
        }
        let mut sides = self.sides();
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let min = sides[0];
        let max = sides[2];
        let one = F::from(1).expect("cast from 1");
        let two = F::from(2).expect("cast from 2");
        let five = F::from(5).expect("cast from 5");

        (max / min).eq(&((one + five.sqrt()) / two))
    }

    ///Checks if the triangle is isosceles.
    pub fn is_isosceles(&self) -> bool {
        let sides = self.sides();
        sides[0].eq(&sides[1]) || sides[1].eq(&sides[2]) || sides[2].eq(&sides[0])
    }

    ///Checks if the triangle is right-angled.
    pub fn is_right(&self) -> bool {
        if self.is_collinear() {
            return false;
        }
        let angles = self.angles().unwrap();
        let half_pi = F::FRAC_PI_2();
        angles[0].eq(&half_pi) || angles[1].eq(&half_pi) || angles[2].eq(&half_pi)
    }

    ///Gets medians of the triangle.
    pub fn medians(&self) -> [F; 3] {
        let [la, lb, lc] = self.sides();
        let two = F::from(2).expect("cast from 2");
        let ma = (two * lb.powi(2) + two * lc.powi(2) - la.powi(2)).sqrt() / two;
        let mb = (two * lc.powi(2) + two * la.powi(2) - lb.powi(2)).sqrt() / two;
        let mc = (two * la.powi(2) + two * lb.powi(2) - lc.powi(2)).sqrt() / two;
        [ma, mb, mc]
    }

    ///Gets normal of the triangle, depending on vertices order.
    pub fn normal(&self) -> Option<Point<F>> {
        if self.is_collinear() {
            return None;
        }
        let u = self.b - self.a;
        let v = self.c - self.a;
        let n = Point {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        };
        Some(n.normalized())
    }

    ///Gets perimeter of the triangle.
    pub fn perimeter(&self) -> F {
        let zero = F::from(0).expect("cast from 0");
        return self.sides().iter().fold(zero, |acc, x| acc + *x);
    }

    ///Gets distance from ray origin to intersection with triangle. Möller & f64rumbore algorithm.
    pub fn ray_intersection(&self, ray_orig: &Point<F>, ray_dir: &Point<F>) -> Option<F> {
        if self.is_collinear() {
            return None;
        }

        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let pvec = ray_dir.cross(&e2);
        let det = e1.dot(&pvec);
        let zero = F::from(0).expect("cast from 0");
        let one = F::from(1).expect("cast from 1");
        if det.abs() < F::min_value() {
            return None;
        }

        let inv_det = one / det;
        let tvec = *ray_orig - self.a;
        let u = tvec.dot(&pvec) * inv_det;
        if u < zero || u > one {
            return None;
        }

        let qvec = tvec.cross(&e1);
        let v = ray_dir.dot(&qvec) * inv_det;
        if v < zero || (u + v) > one {
            return None;
        }

        Some(e2.dot(&qvec) * inv_det)
    }

    ///Gets semiperimeter of the triangle.
    pub fn semiperimeter(&self) -> F {
        let two = F::from(2).expect("cast from 2");
        self.perimeter() / two
    }

    ///Gets lengths of sides opposite to points.
    pub fn sides(&self) -> [F; 3] {
        [
            self.b.distance_to(&self.c),
            self.c.distance_to(&self.a),
            self.a.distance_to(&self.b),
        ]
    }

    ///Checks if Triangle Points are sorted by axis.
    pub fn is_sorted_by(self, axis_name: char) -> bool {
        match axis_name {
            'x' | 'X' | '0' => self.a.x <= self.b.x && self.b.x <= self.c.x,
            'z' | 'Z' | '2' => self.a.z <= self.b.z && self.b.z <= self.c.z,
            _ => self.a.y <= self.b.y && self.b.y <= self.c.y,
        }
    }

    ///Creates new Triangle with Points sorted by axis.
    pub fn sorted_by(self, axis_name: char) -> Triangle<F> {
        let mut sorted = [self.a, self.b, self.c];
        match axis_name {
            'x' | 'X' | '0' => sorted.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap()),
            'z' | 'Z' | '2' => sorted.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap()),
            _ => sorted.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap()),
        };
        Triangle::from_array(sorted)
    }
}

fn sign<F: Float + FloatConst>(a: &Point<F>, b: &Point<F>, c: &Point<F>) -> F {
    (a.x - c.x) * (b.y - c.y) - (b.x - c.x) * (a.y - c.y)
}
