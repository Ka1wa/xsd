use xsd::core::Xsd;

#[test]
fn xsd_loads() {
    let xsd = Xsd::load("./tests/assets/simple.xsd");

    assert!(xsd.is_ok());
}
