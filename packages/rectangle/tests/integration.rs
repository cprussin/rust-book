use rectangle::Rectangle;

#[test]
fn it_calculates_area() {
    let rectangle = Rectangle {
        width: 5,
        height: 10,
    };
    assert_eq!(rectangle.area(), 50);
}
