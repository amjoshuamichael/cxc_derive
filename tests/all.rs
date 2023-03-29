#![allow(dead_code)]

use std::collections::HashMap;
use std::rc::Rc;

use cxc::XcReflect;
use cxc::xc_opaque;

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

#[derive(XcReflect)]
struct MultidimensionaArray {
    num: f32,
    array: [[i32; 2]; 2],
}

#[test]
fn multidimensional_array() {
    assert_eq!(
        MultidimensionaArray::alias_code(),
        "MultidimensionaArray = { num : f32, array : [2][2]i32, }"
    );
}

const THIRTY_TWO: usize = 32;
const TWO: usize = 2;

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
struct MultiArrayWithConstSize {
    array: [[i32; THIRTY_TWO]; TWO],
}

#[test]
fn multi_array_with_const_size() {
    assert_eq!(
        MultiArrayWithConstSize::alias_code(),
        "MultiArrayWithConstSize = { array : [2][32]i32, }"
    );
}

#[derive(XcReflect)]
struct MultiArrayWithExprSize {
    array: [[i32; THIRTY_TWO * 2]; TWO + THIRTY_TWO],
}

#[test]
fn multi_array_with_expr_size() {
    assert_eq!(
        MultiArrayWithExprSize::alias_code(),
        "MultiArrayWithExprSize = { array : [34][64]i32, }"
    );
}

#[derive(XcReflect)]
struct TupleInStruct {
    tuple: (i32, f32),
}

#[test]
fn tuple_in_struct() {
    assert_eq!(
        TupleInStruct::alias_code(),
        "TupleInStruct = { tuple : { i32, f32, }, }"
    );
}

#[derive(XcReflect)]
struct WithPointers {
    pointer_to_something: *const (i32, f32),
}

#[test]
fn raw_pointer() {
    assert_eq!(
        WithPointers::alias_code(),
        "WithPointers = { pointer_to_something : &{ i32, f32, }, }"
    );
}

#[derive(XcReflect)]
struct WithMutPointers {
    pointer_to_something: *mut (i32, f32),
}

#[test]
fn mut_pointer() {
    assert_eq!(
        WithMutPointers::alias_code(),
        "WithMutPointers = { pointer_to_something : &{ i32, f32, }, }"
    );
}

#[derive(XcReflect)]
struct HoldingVec {
    inner: Vec<u32>,
}

#[test]
fn holding_vec() {
    assert_eq!(
        HoldingVec::alias_code(),
        "HoldingVec = { inner : Vec < u32, >, }"
    );
}

#[derive(XcReflect)]
struct HoldingTupleVec {
    inner: Vec<(u32, u32)>,
}

#[test]
fn holding_tuple_vec() {
    assert_eq!(
        HoldingTupleVec::alias_code(),
        "HoldingTupleVec = { inner : Vec < { u32, u32, }, >, }"
    );
}

#[derive(XcReflect)]
struct HoldingTupleHashMap {
    inner: HashMap<(u32, u32), (u32, u32)>,
}

#[test]
fn holding_tuple_hashmap() {
    assert_eq!(
        HoldingTupleHashMap::alias_code(),
        "HoldingTupleHashMap = { inner : HashMap < { u32, u32, }, { u32, u32, }, >, }"
    );
}

#[derive(XcReflect)]
struct FnHolder {
    contains: fn(i32) -> i32,
}

#[test]
fn fn_holder() {
    assert_eq!(
        FnHolder::alias_code(),
        "FnHolder = { contains : (i32, ); i32, }"
    );
}

#[derive(XcReflect)]
struct FnHolderVoid {
    contains: fn(i32),
}

#[test]
fn fn_holder_void() {
    assert_eq!(
        FnHolderVoid::alias_code(),
        "FnHolderVoid = { contains : (i32, ), }"
    );
}

#[derive(XcReflect)]
enum IntOrFloatNamed {
    Int { the_int: u32 },
    Float { the_float: f32 },
}

#[test]
fn int_or_float_named() {
    assert_eq!(
        IntOrFloatNamed::alias_code(),
        "IntOrFloatNamed = { Int : { the_int : u32, } / Float : { the_float : f32, } / }"
    )
}

#[derive(XcReflect)]
enum IntOrFloatUnnamed {
    Int(u32, u32),
    Float(f32, f32),
}

#[test]
fn int_or_float_unnamed() {
    assert_eq!(
        IntOrFloatUnnamed::alias_code(),
        "IntOrFloatUnnamed = { Int : { u32, u32, } / Float : { f32, f32, } / }"
    )
}

#[derive(XcReflect)]
enum IntOrFloatMixed {
    Int { i: u32, u: u32 },
    Float(f32, f32),
}

#[test]
fn int_or_float_mixed() {
    assert_eq!(
        IntOrFloatMixed::alias_code(),
        "IntOrFloatMixed = { Int : { i : u32, u : u32, } / Float : { f32, f32, } / }"
    )
}

#[derive(XcReflect)]
enum NoDataEnum {
    Int,
    Float,
}

#[test]
fn no_data_enum() {
    assert_eq!(
        NoDataEnum::alias_code(),
        "NoDataEnum = { Int : { } / Float : { } / }"
    )
}

#[derive(XcReflect)]
struct ComplexTuple(i32, Vec<(u32, u32)>);

#[test]
fn complex_tuple() {
    assert_eq!(
        ComplexTuple::alias_code(),
        "ComplexTuple = { i32, Vec < { u32, u32, }, >, }"
    )
}

#[derive(XcReflect)]
struct Nothing;

#[test]
fn nothing() {
    assert_eq!(
        Nothing::alias_code(),
        "Nothing = {}"
    )
}

#[derive(XcReflect)]
#[xc_opaque]
struct CrazyOpaque {
    tuple: (u32, u32), 
    num: u32,
    float: f32,
}

fn size_over_alignment<T>() -> usize {
    std::mem::size_of::<CrazyOpaque>() / std::mem::align_of::<CrazyOpaque>()
}

#[test]
fn crazy_opaque() {
    assert_eq!(
        CrazyOpaque::alias_code(),
        format!("CrazyOpaque = {{ [ {} ] u32 }}", size_over_alignment::<CrazyOpaque>())
    )
}

#[derive(XcReflect)]
#[xc_opaque]
struct CrazyOpaqueWithPtr {
    some_ptr: Rc<(u32, u32)>,
    some_num: u32, 
}

#[test]
fn crazy_opaque_with_ptr() {
    assert_eq!(
        CrazyOpaqueWithPtr::alias_code(),
        "CrazyOpaqueWithPtr = { bool, u64, }"
    )
}
