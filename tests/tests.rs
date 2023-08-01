use xsd::core::Xsd;

#[test]
fn it_adds_two() {
    let xsd = Xsd::new(2, 2);

    assert_eq!(4, xsd.add());
}
