use api_rust::add;

#[test]
fn can_add() {
    assert_eq!(add(2, 2), 4);
}
