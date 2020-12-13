pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}
impl Triangle {
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

    ///Convert cartesian coordinates of given point to barycentric coordinate system.
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
    ///Converts barycentric coordinates of given point to cartesian coordinate system.
    pub fn barycentric_to_cartesian(&self, pt: &Point) -> Point {
        let x = pt.x * self.a.x + pt.y * self.b.x + pt.z * self.c.x;
        let y = pt.x * self.a.y + pt.y * self.b.y + pt.z * self.c.y;
        let z = pt.x * self.a.z + pt.y * self.b.z + pt.z * self.c.z;
        Point { x: x, y: y, z: z }
    }
}
