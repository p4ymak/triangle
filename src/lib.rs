#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Point {
    ///Calculates distance to another Point.
    pub fn distance_to(&self, pt: &Point) -> f64 {
        ((self.x - pt.x).powi(2) + (self.y - pt.y).powi(2) + (self.z - pt.z).powi(2)).sqrt()
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}
impl Triangle {
    ///Returns two opposite points of axis-aligned bounding box.
    pub fn aabb(&self) -> [Point; 2] {
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
    pub fn angles(&self) -> Option<[f64; 3]> {
        if self.is_collinear() {
            return None;
        }
        let [la, lb, lc] = self.sides();
        let alpha = ((lb.powi(2) + lc.powi(2) - la.powi(2)) / (2.0 * lb * lc)).acos();
        let beta = ((la.powi(2) + lc.powi(2) - lb.powi(2)) / (2.0 * la * lc)).acos();
        let gamma = std::f64::consts::PI - alpha - beta;
        Some([alpha, beta, gamma])
    }

    ///Gets area of the triangle.
    pub fn area(&self) -> f64 {
        let s = self.semiperimeter();
        let [la, lb, lc] = self.sides();
        (s * (s - la) * (s - lb) * (s - lc)).sqrt()
    }

    ///Converts barycentric coordinates of given point to cartesian coordinate system.
    pub fn barycentric_to_cartesian(&self, pt: &Point) -> Point {
        let x = pt.x * self.a.x + pt.y * self.b.x + pt.z * self.c.x;
        let y = pt.x * self.a.y + pt.y * self.b.y + pt.z * self.c.y;
        let z = pt.x * self.a.z + pt.y * self.b.z + pt.z * self.c.z;
        Point { x: x, y: y, z: z }
    }

    ///Converts cartesian coordinates of given point to barycentric coordinate system.
    pub fn cartesian_to_barycentric(&self, pt: &Point) -> Point {
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
        let den = 1.0 / (v0.x * v1.y - v1.x * v0.y);
        let v = (v2.x * v1.y - v1.x * v2.y) * den;
        let w = (v0.x * v2.y - v2.x * v0.y) * den;
        let u = 1.0 - v - w;
        Point { x: u, y: v, z: w }
    }

    ///Gets centroid of the triangle.
    pub fn centroid(&self) -> Point {
        let cx = (self.a.x + self.b.x + self.c.x) / 3.0;
        let cy = (self.a.y + self.b.y + self.c.y) / 3.0;
        let cz = (self.a.z + self.b.z + self.c.z) / 3.0;
        Point {
            x: cx,
            y: cy,
            z: cz,
        }
    }

    ///Gets radius of a circle that passes through all of the triangle's vertices, so called
    ///circumradius.
    pub fn circumradius(&self) -> Option<f64> {
        if self.is_collinear() {
            return None;
        }
        Some(self.sides().iter().product::<f64>() / (4.0 * self.area()))
    }

    ///Checks whether a given point lies inside the triangle.
    pub fn has_point(&self, pt: Point) -> bool {
        fn sign(a: &Point, b: &Point, c: &Point) -> f32 {
            ((a.x - c.x) * (b.y - c.y) - (b.x - c.x) * (a.y - c.y)) as f32
        }
        let d1 = sign(&pt, &self.a, &self.b);
        let d2 = sign(&pt, &self.b, &self.c);
        let d3 = sign(&pt, &self.c, &self.a);
        let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
        let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
        !(has_neg && has_pos)
    }

    ///Gets the heights of the triangle.
    pub fn heights(&self) -> Option<[f64; 3]> {
        if self.is_collinear() {
            return None;
        }
        let double_area = 2.0 * self.area();
        let [la, lb, lc] = self.sides();
        Some([double_area / la, double_area / lb, double_area / lc])
    }

    ///Gets radius of a circle which is tangent to each side of the triangle, so called inradius.
    pub fn inradius(&self) -> Option<f64> {
        if self.is_collinear() {
            return None;
        }
        Some(self.area() / self.semiperimeter())
    }

    ///Checks if points of triangle are collinear.
    pub fn is_collinear(&self) -> bool {
        self.area() == 0.0
    }

    ///Gets perimeter of the triangle.
    pub fn perimeter(&self) -> f64 {
        self.sides().iter().sum()
    }

    ///Gets semiperimeter of the triangle.
    pub fn semiperimeter(&self) -> f64 {
        self.perimeter() / 2.0
    }

    ///Gets lengths of sides opposite to points.
    pub fn sides(&self) -> [f64; 3] {
        [
            self.b.distance_to(&self.c),
            self.c.distance_to(&self.a),
            self.a.distance_to(&self.b),
        ]
    }
}
