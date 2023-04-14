use super::*;

#[test]
fn cartesian_barycentric() {
    let point_a = Point::<f64>::new(10.153, 20.21, 0.0);
    let point_b = Point::<f64>::new(1.21, -2.531, 0.0);
    let point_c = Point::<f64>::new(-42.332, 0.0, 0.0);
    let triangle = Triangle::new(point_a, point_b, point_c);

    let point_d = Point::<f64>::new(-0.09823, 0.2131, 0.0);
    let point_d1 = triangle.barycentric_to_cartesian(&triangle.cartesian_to_barycentric(&point_d));
    assert!(point_d.distance_to(&point_d1) <= 0.0001);
}

#[test]
fn collinear() {
    let point_a = Point::<f32>::new(1.0, 2.0, -3.0);
    let point_b = Point::<f32>::new(1.0, 2.0, 0.0);
    let point_c = Point::<f32>::new(1.0, 2.0, 19.0);
    let triangle = Triangle::new(point_a, point_b, point_c);

    assert!(triangle.is_collinear());
}

#[test]
fn aabb() {
    let point_a = Point::<f64>::new(-21.1, 2.0, -3.0);
    let point_b = Point::<f64>::new(84.2, -9.0, 12.9832);
    let point_c = Point::<f64>::new(-1.1, 32.0, 19.0);
    let triangle1 = Triangle::new(point_a, point_b, point_c);
    let triangle2 = Triangle::new(point_c, point_a, point_b);
    assert_eq!(triangle1.aabb(), triangle2.aabb());
}

#[test]
fn area() {
    let point_a = Point::<f64>::new(-21.0, 2.0, -3.0);
    let point_b = Point::<f64>::new(84.0, -9.0, 12.0);
    let point_c = Point::<f64>::new(19.0, 32.0, 19.0);
    let triangle1 = Triangle::new(point_a, point_b, point_c);
    let triangle2 = Triangle::new(point_b, point_c, point_a);
    assert!((triangle1.area() - triangle2.area()).abs() <= 0.0001);
}

#[test]
fn perimeter() {
    let point_a = Point::<f32>::new(21.0, 2.239, 3.305);
    let point_b = Point::<f32>::new(-84.328, -9.896, 12.98);
    let point_c = Point::<f32>::new(19.0, -32.3249, 19.8432);
    let triangle1 = Triangle::new(point_a, point_b, point_c);
    let triangle2 = Triangle::new(point_b, point_c, point_a);
    assert!((triangle1.perimeter() - triangle2.perimeter()).abs() <= 0.0001);
}
