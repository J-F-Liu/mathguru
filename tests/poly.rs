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
    let rab = ra.cross(&b);
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

    let n1 = create_normal("a", "b", &n, "t".into(), "s".into());
    let n2 = create_normal("c", "d", &n, "t".into(), "s".into());
    let n3 = create_normal("e", "f", &n, "t".into(), "s".into());
    let res = n1.cross(&n2).dot(&n3);
    println!("(n1×n2)·n3 = {}", res);
    dbg!(res.terms.len());
}
