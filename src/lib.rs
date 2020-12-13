pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}
impl Triangle {
    fn has_point(&self, pt: Point) -> bool {
        ///Checks whether a given point lies inside the triangle.
        fn sign(a: &Point, b: &Point, c: &Point) -> f32 {
            (a.x - c.x) * (b.y - c.y) - (b.x - c.x) * (a.y - c.y)
        }
        let d1 = sign(&pt, &self.a, &self.b);
        let d2 = sign(&pt, &self.b, &self.c);
        let d3 = sign(&pt, &self.c, &self.a);
        let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
        let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
        !(has_neg && has_pos)
    }
    fn aabb(&self) -> [Point; 2] {
        ///Returns two opposite points of axis-aligned bounding box.
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
    fn cartesian_to_barycentric(&self, pt: &Point) -> Point {
        ///Convert cartesian coordinates of given point to barycentric coordinate system of the
        ///Triangle.
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
    fn barycentric_to_cartesian(&self, pt: &Point) -> Point {
        ///Converts barycentric coordinates of given point to cartesian coordinate system.
        let x = pt.x * self.a.x + pt.y * self.b.x + pt.z * self.c.x;
        let y = pt.x * self.a.y + pt.y * self.b.y + pt.z * self.c.y;
        let z = pt.x * self.a.z + pt.y * self.b.z + pt.z * self.c.z;
        Point { x: x, y: y, z: z }
    }
}
