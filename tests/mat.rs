use mathguru::{Matrix, Poly, Vector};

#[test]
fn test_matrix() {
    let m = Matrix::<Poly<i32>, 2, 2>::new("a".into(), "b".into(), "b".into(), "c".into());
    let v = Vector::<Poly<i32>, 2>::new("e".into(), "f".into());
    println!();
    println!("{}", &m);
    println!("{}", &m * &v);
    println!("{}", &m * &m);
}
