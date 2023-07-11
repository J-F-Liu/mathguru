use mathguru::Poly;

#[test]
fn test_factorize() {
    let t: Poly<i32> = Poly::from("t");

    let mut p = t.clone() * t.clone() * t.clone() + t.clone() * t.clone() + t.clone() * 3.into();

    let factors = p.extract_common_factors();
    println!(
        "factors: {}",
        factors
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .concat()
    );
    println!("p={}", p);
}
