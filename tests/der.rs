use mathguru::{Poly, Vector3};

type Vec3 = Vector3<Poly<i32>>;

fn sin_cos(t: Poly<i32>) -> (Poly<i32>, Poly<i32>) {
    let sin = t.apply("sin");
    let cos = t.apply("cos");
    (sin, cos)
}

fn rotate_perp(a: &Vec3, n: &Vec3, t: Poly<i32>) -> Vec3 {
    let (sin, cos) = sin_cos(t);
    a * cos + n.cross(a) * sin
}

fn rotate(a: &Vec3, n: &Vec3, t: Poly<i32>) -> Vec3 {
    let (sin, cos) = sin_cos(t);
    let para = n * n.dot(a);
    let perp = a - &para;
    para + perp * cos + n.cross(a) * sin
}

#[test]
fn test_derive() {
    let a = Vec3::new("a_x".into(), "a_y".into(), 1.into());
    let b = Vec3::new("b_x".into(), "b_y".into(), 1.into());

    let (sin_theta, cos_theta) = sin_cos("θ".into());
    println!(
        "{} = 1",
        sin_theta.clone() * sin_theta + cos_theta.clone() * cos_theta
    );

    let (sin_phi, cos_phi) = sin_cos("ψ".into());
    let (sin_omega, cos_omega) = sin_cos("ω".into());
    let n = Vec3::new(sin_phi.clone() * cos_omega, sin_phi * sin_omega, cos_phi);
    let ra = rotate_perp(&a, &n, "θ".into());
    println!("rotate_perp = {}", &ra);

    let ra = rotate(&a, &n, "θ".into());
    println!("rotate = {}", &ra);
    let rab = ra.cross(&b);
    let bra = -b.cross(&ra);
    assert_eq!(rab, bra);
}
