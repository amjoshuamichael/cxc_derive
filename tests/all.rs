#![allow(dead_code)]

use cxc::XcReflect;

#[derive(XcReflect)]
struct Point2D {
    x: f32,
    y: f32,
}

#[test]
fn basic() {
    assert_eq!(Point2D::alias_code(), "Point2D = { x : f32, y : f32, }");
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
        "NumAndArray = { num : f32, array : [2]i32, }"
    );
}

const THIRTY_TWO: usize = 32;

#[derive(XcReflect)]
struct ArrayWithConstSize {
    array: [i32; THIRTY_TWO],
}

#[test]
fn array_with_const_size() {
    assert_eq!(
        ArrayWithConstSize::alias_code(),
        "ArrayWithConstSize = { array : [32]i32, }"
    );
}

#[derive(XcReflect)]
struct TupleInStruct {
    array: (i32, f32),
}

#[test]
fn tuple_in_struct() {
    assert_eq!(
        TupleInStruct::alias_code(),
        "TupleInStruct = { array : { i32, f32, }, }"
    );
}
