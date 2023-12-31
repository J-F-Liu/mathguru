use mathguru::{Poly, Vector3};

type Vec3 = Vector3<Poly<i32>>;

fn rotate_perp(a: &Vec3, n: &Vec3, c: Poly<i32>, s: Poly<i32>) -> Vec3 {
    a * c + n.cross(a) * s
}

fn rotate(a: &Vec3, n: &Vec3, c: Poly<i32>, s: Poly<i32>) -> Vec3 {
    let para = n * n.dot(a);
    let perp = a - &para;
    para + perp * c + n.cross(a) * s
}

fn create_normal(a: &str, b: &str, n: &Vec3, c: Poly<i32>, s: Poly<i32>) -> Vec3 {
    let a = Vec3::new(format!("{a}_x").into(), format!("{a}_y").into(), 1.into());
    let b = Vec3::new(format!("{b}_x").into(), format!("{b}_y").into(), 1.into());
    let ra = rotate(&a, &n, c, s);
    ra.cross(&b)
}

#[test]
fn test_polynomial() {
    let a = Vec3::new("a_x".into(), "a_y".into(), 1.into());
    let b = Vec3::new("b_x".into(), "b_y".into(), 1.into());
    println!("a = {}", &a);
    println!("a + b = {}", &a + &b);

    let n = Vec3::new("u".into(), "v".into(), "w".into());
    let ra = rotate(&a, &n, "c".into(), "s".into());
    let mut rab = ra.cross(&b);
    let bra = -b.cross(&ra);
    assert_eq!(rab, bra);

    println!("a.dot(&n) = {}", a.dot(&n));
    println!("a.cross(&n) = {}", a.cross(&n));
    println!(
        "rotate_perp = {}",
        rotate_perp(&a, &n, "c".into(), "s".into())
    );
    println!("rotate = {}", &ra);
    println!("rab = {}", &rab);

    for value in &mut rab.data {
        value.group_by(vec![
            "u".into(),
            "v".into(),
            "w".into(),
            "c".into(),
            "s".into(),
        ]);
    }
    println!("rab = {}", &rab);

    let n1 = create_normal("a", "b", &n, "t".into(), "s".into());
    let n2 = create_normal("c", "d", &n, "t".into(), "s".into());
    let n3 = create_normal("e", "f", &n, "t".into(), "s".into());
    let res = n1.cross(&n2).dot(&n3);
    dbg!(res.terms.len());

    let constraint: Poly<i32> = Poly::from("u") * Poly::from("u")
        + Poly::from("v") * Poly::from("v")
        + Poly::from("w") * Poly::from("w");
    let mut sim = res.simplify_by_identity(constraint, 1.into());
    sim.expand();
    dbg!(sim.terms.len());

    let constraint: Poly<i32> = Poly::from("v") * Poly::from("v")
        + Poly::from("u") * Poly::from("u")
        + Poly::from("w") * Poly::from("w");
    let mut sim = sim.simplify_by_identity(constraint, 1.into());
    sim.expand();
    dbg!(sim.terms.len());

    let constraint: Poly<i32> = Poly::from("w") * Poly::from("w")
        + Poly::from("u") * Poly::from("u")
        + Poly::from("v") * Poly::from("v");
    let mut sim = sim.simplify_by_identity(constraint, 1.into());
    sim.expand();
    dbg!(sim.terms.len());

    let constraint: Poly<i32> =
        Poly::from("t") * Poly::from("t") + Poly::from("s") * Poly::from("s");
    let mut sim = sim.simplify_by_identity(constraint, 1.into());
    sim.expand();
    dbg!(sim.terms.len());

    sim.group_by(vec![
        "u".into(),
        "v".into(),
        "w".into(),
        "t".into(),
        "s".into(),
    ]);
    dbg!(sim.terms.len());
    println!("(n1×n2)·n3 = {}", sim);
}
