use mathguru::{Poly, Quat};

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
}
