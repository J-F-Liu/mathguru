use mathguru::{Poly, Quat, Vector3};

type Vec3 = Vector3<Poly<i32>>;

fn create_normal(a: &str, b: &str, q: &Quat<Poly<i32>>) -> Vec3 {
    let a = Vec3::new(format!("{a}_x").into(), format!("{a}_y").into(), 1.into());
    let b = Vec3::new(format!("{b}_x").into(), format!("{b}_y").into(), 1.into());
    let ra = q.rotate(&a);
    ra.cross(&b)
}

#[test]
fn test_quaternion() {
    let q1 = Quat::<Poly<i32>>::new("a".into(), "b".into(), "c".into(), "d".into());
    let q2 = Quat::<Poly<i32>>::new("e".into(), "f".into(), "g".into(), "h".into());
    assert_eq!(
        &q1.left_mul_matrix() * q2.as_vector(),
        &q2.right_mul_matrix() * q1.as_vector()
    );
    println!();
    println!("{}", &q1);
    println!("{}", &q1 * &q2);

    let q = Quat::<Poly<i32>>::new("q_0".into(), "q_1".into(), "q_2".into(), "q_3".into());
    let v = Vector3::<Poly<i32>>::new("x".into(), "y".into(), "z".into());
    let r = (q.left_mul_matrix() * q.conjugate().right_mul_matrix()).block(1, 1);
    assert_eq!(q.rotate(&v), &r * &v);
    println!("{}", r);

    let n1 = create_normal("a", "b", &q);
    let n2 = create_normal("c", "d", &q);
    let n3 = create_normal("e", "f", &q);
    let res = n1.cross(&n2).dot(&n3);
    dbg!(res.terms.len());

    let constraint: Poly<i32> = Poly::from("q_0") * Poly::from("q_0")
        + Poly::from("q_1") * Poly::from("q_1")
        + Poly::from("q_2") * Poly::from("q_2")
        + Poly::from("q_3") * Poly::from("q_3");
    let mut sim = res.simplify_by_identity(constraint, 1.into());
    sim.expand();
    dbg!(sim.terms.len());

    sim.group_by(vec!["q_0".into(), "q_1".into(), "q_2".into(), "q_3".into()]);
    dbg!(sim.terms.len());
    // println!("(n1×n2)·n3 = {}", sim);

    for (index, term) in sim.terms.iter().enumerate() {
        for factor in &term.factors {
            match factor.base {
                mathguru::Base::Poly(_) => {
                    print!("p_{{{}}} ", index + 1);
                }
                _ => {
                    if factor.power == 1 {
                        print!("{} ", factor.base);
                    } else {
                        print!("{}^{} ", factor.base, factor.power);
                    }
                }
            }
        }
        print!("+ ");
    }
}
