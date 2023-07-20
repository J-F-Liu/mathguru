use mathguru::{Matrix, Poly, Quat, Vector3};
type Vec3 = Vector3<Poly<i32>>;

#[test]
fn test_matrix() {
    let m = Matrix::<Poly<i32>, 3, 3>::new(
        "m11".into(),
        "m12".into(),
        "m13".into(),
        "m21".into(),
        "m22".into(),
        "m23".into(),
        "m31".into(),
        "m32".into(),
        "m33".into(),
    );
    let a = Matrix::<Poly<i32>, 1, 3>::new("x".into(), "y".into(), 1.into());
    let b = Matrix::<Poly<i32>, 3, 1>::new("u".into(), "v".into(), 1.into());
    println!();
    println!("{}", &m);
    println!("{}", a * (&m * &b));
    println!("{}", &m * &m);
}

#[test]
fn test_determinant() {
    use num_traits::Zero;
    let q = Quat::<Poly<i32>>::new("w".into(), "x".into(), "y".into(), "z".into());
    let r = (q.left_mul_matrix() * q.conjugate().right_mul_matrix()).block(1, 1);
    println!("{}", r);

    let a = Vec3::new(format!("a_x").into(), format!("a_y").into(), 1.into());
    let b = Vec3::new(format!("b_x").into(), format!("b_y").into(), 1.into());
    let c = Vec3::new(format!("c_x").into(), format!("c_y").into(), 1.into());
    let d = Vec3::new(format!("d_x").into(), format!("d_y").into(), 1.into());
    let e = Vec3::new(format!("e_x").into(), format!("e_y").into(), 1.into());
    let f = Vec3::new(format!("f_x").into(), format!("f_y").into(), 1.into());

    fn concat(u: Vec3, v: Vec3) -> [Poly<i32>; 6] {
        [u.x(), u.y(), u.z(), v.x(), v.y(), v.z()]
    }

    let m = Matrix {
        data: [
            concat(&r * &a, &r * &a),
            concat(-&b, -&b),
            concat(-(&r * &c), Vec3::zero()),
            concat(d, Vec3::zero()),
            concat(Vec3::zero(), -(&r * &e)),
            concat(Vec3::zero(), f),
        ],
    };

    println!("{}", m);

    let m11 = m.block::<3, 3>(0, 0);
    let m12 = m.block::<3, 3>(0, 3);
    let m21 = m.block::<3, 3>(3, 0);
    let m22 = m.block::<3, 3>(3, 3);

    println!("{}", &m11 * &m12 == &m12 * &m11);
    println!("{}", &m21 * &m22 == &m22 * &m21);
    println!("{}", &m11 * &m21 == &m21 * &m11);
    println!("{}", &m12 * &m22 == &m22 * &m12);

    let constraint: Poly<i32> = Poly::from("w") * Poly::from("w")
        + Poly::from("x") * Poly::from("x")
        + Poly::from("y") * Poly::from("y")
        + Poly::from("z") * Poly::from("z");
    let mut det = m11.determinant().simplify_by_identity(constraint, 1.into());
    det.expand();
    dbg!(det.terms.len());

    det.group_by(vec!["w".into(), "x".into(), "y".into(), "z".into()]);
    dbg!(det.terms.len());
    println!("{}", det);

    let mut det = m.determinant();
    dbg!(det.terms.len());

    det.group_by(vec!["w".into(), "x".into(), "y".into(), "z".into()]);
    dbg!(det.terms.len());
    // println!("{}", det);
}
