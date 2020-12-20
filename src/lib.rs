pub trait Point3: Sized + Copy {
    fn from_tuple(tup: (f64, f64, f64)) -> Self;
    fn from_array(arr: [f64; 3]) -> Self {
        Self::from_tuple((arr[0], arr[1], arr[2]))
    }
    fn from_other<P: Point3>(other: P) -> Self {
        let (x, y, z) = other.into_tuple();
        Self::from_tuple((x, y, z))
    }
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn square_elements(&self) -> Self {
        let (x, y, z) = self.into_tuple();
        Self::from_tuple((x * x, y * y, z * z))
    }
    fn sub_elements(&self, rhs: &Self) -> Self {
        let t1 = self.into_tuple();
        let t2 = rhs.into_tuple();
        Self::from_tuple((t1.0 - t2.0, t1.1 - t2.1, t1.2 - t2.2))
    }
    fn sum_elements(&self) -> f64 {
        self.x() + self.y() + self.z()
    }
    fn distance(&self, rhs: &Self) -> f64 {
        self.sub_elements(rhs)
            .square_elements()
            .sum_elements()
            .sqrt()
    }
    fn into_tuple(&self) -> (f64, f64, f64) {
        (self.x(), self.y(), self.z())
    }
    fn into_array(&self) -> [f64; 3] {
        let (x, y, z) = self.into_tuple();
        [x, y, z]
    }
}
pub trait Trig<P: Point3>: Sized + Copy {
    fn from_tuple(tup: (P, P, P)) -> Self;
    fn a(&self) -> P;
    fn b(&self) -> P;
    fn c(&self) -> P;
    fn into_tuple(&self) -> (P, P, P) {
        (self.a(), self.b(), self.c())
    }
    fn into_array(&self) -> [P; 3] {
        let (a, b, c) = self.into_tuple();
        [a, b, c]
    }
    fn from_other<F: Point3, T: Trig<F>>(other: &T) -> Self {
        let (a, b, c) = other.into_tuple();
        Self::from_tuple((P::from_other(a), P::from_other(b), P::from_other(c)))
    }
    fn aabb(&self) -> [P; 2] {
        let arr = Triangle::from_other(self).aabb();
        [P::from_other(arr[0]), P::from_other(arr[1])]
    }
    fn area(&self) -> f64 {
        Triangle::area(&Triangle::from_other(self))
    }
    fn barycentric_to_cartesian(&self, pt: &P) -> P {
        P::from_other(Triangle::barycentric_to_cartesian(
            &Triangle::from_other(self),
            &Point::from_other(*pt),
        ))
    }
    fn cartesian_to_barycentric(&self, pt: &P) -> P {
        P::from_other(Triangle::cartesian_to_barycentric(
            &Triangle::from_other(self),
            &Point::from_other(*pt),
        ))
    }
    fn has_point(&self, pt: P) -> bool {
        Triangle::has_point(&Triangle::from_other(self), Point::from_other(pt))
    }
    fn perimeter(&self) -> f64 {
        Triangle::perimeter(&Triangle::from_other(self))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 for Point {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
    fn from_tuple(tup: (f64, f64, f64)) -> Self {
        Self {
            x: tup.0,
            y: tup.1,
            z: tup.2,
        }
    }
}

impl Point {
    ///Calculates distance to another Point.
    pub fn distance_to(&self, pt: &Point) -> f64 {
        self.distance(pt)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Trig<Point> for Triangle {
    fn from_tuple(tup: (Point, Point, Point)) -> Self {
        Self {
            a: tup.0,
            b: tup.1,
            c: tup.2,
        }
    }
    fn a(&self) -> Point {
        self.a
    }
    fn b(&self) -> Point {
        self.b
    }
    fn c(&self) -> Point {
        self.c
    }
}

fn square<T: std::ops::Mul<T , Output = T> + Copy>(t: T) -> T{
    t * t
}

fn sort_tuple3(tup: (f64 , f64 , f64)) -> (f64 ,f64 ,f64  ){
    use std::mem::swap;
    let (mut a ,mut  b ,mut c) = tup;
    if a > b{
        swap(&mut a ,&mut b);
    }
    if b > c{
        swap(&mut b , &mut c)
    }
    if a > b{
       swap(&mut a , &mut b) 
    }
    (a , b ,  c)
}


impl Triangle {
    ///Returns two opposite points of axis-aligned bounding box.
    pub fn aabb(&self) -> [Point; 2] {
        let c_x = sort_tuple3((self.a.x, self.b.x, self.c.x));
        let c_y = sort_tuple3((self.a.y, self.b.y, self.c.y));
        let c_z = sort_tuple3((self.a.z, self.b.z, self.c.z));
        [
            Point {
                x: c_x.0,
                y: c_y.0,
                z: c_z.0,
            },
            Point {
                x: c_x.2,
                y: c_y.2,
                z: c_z.2,
            },
        ]
    }

    ///Gets area of the triangle.
    pub fn area(&self) -> f64 {
        (square(self.b.x * self.a.y - self.c.x * self.a.y - self.a.x * self.b.y
            + self.c.x * self.b.y
            + self.a.x * self.c.y
            - self.b.x * self.c.y)
            + square(self.b.x * self.a.z - self.c.x * self.a.z - self.a.x * self.b.z
                + self.c.x * self.b.z
                + self.a.x * self.c.z
                - self.b.x * self.c.z)
            + square(self.b.y * self.a.z - self.c.y * self.a.z - self.a.y * self.b.z
                + self.c.y * self.b.z
                + self.a.y * self.c.z
                - self.b.y * self.c.z))
        .sqrt()
            / 2.0
    }

    ///Converts barycentric coordinates of given point to cartesian coordinate system.
    pub fn barycentric_to_cartesian(&self, pt: &Point) -> Point {
        let x = pt.x * self.a.x + pt.y * self.b.x + pt.z * self.c.x;
        let y = pt.x * self.a.y + pt.y * self.b.y + pt.z * self.c.y;
        let z = pt.x * self.a.z + pt.y * self.b.z + pt.z * self.c.z;
        Point { x, y, z }
    }

    ///Converts cartesian coordinates of given point to barycentric coordinate system.
    pub fn cartesian_to_barycentric(&self, pt: &Point) -> Point {
        let v0 = self.b.sub_elements(&self.a);
        let v1 = self.c.sub_elements(&self.a);
        let v2 = pt.sub_elements(&self.a);

        let den = 1.0 / (v0.x * v1.y - v1.x * v0.y);
        let v = (v2.x * v1.y - v1.x * v2.y) * den;
        let w = (v0.x * v2.y - v2.x * v0.y) * den;
        let u = 1.0 - v - w;
        Point { x: u, y: v, z: w }
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

    ///Gets perimeter of the triangle.
    pub fn perimeter(&self) -> f64 {
        self.a.distance_to(&self.b) + self.b.distance_to(&self.c) + self.c.distance_to(&self.a)
    }
}
