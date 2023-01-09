use cxc::XcReflect;

#[derive(XcReflect)]
struct Point2D {
    x: f32,
    y: f32,
}

#[test]
fn basic() {
    assert_eq!(Point2D::alias_code(), "Point2D = { x : f32, y : f32 }");
}

#[derive(XcReflect)]
struct NumAndArray {
    num: f32,
    array: [i32; 2],
}

#[test]
fn array() {
    assert_eq!(
        NumAndArray::alias_code(),
        "NumAndArray = { num : f32, array : [2] i32 }"
    );
}
