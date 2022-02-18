// mimics wr_dp_push_stacking_context() issue

// 01 - original ff test case
#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_01 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_01 {
    pub min: Point2D_01,
    pub max: Point2D_01,
}

#[no_mangle]
pub extern "C" fn test01_ret() -> Box2D_01 {
    Box2D_01 { min: Point2D_01 { x: 0.1, y: 1.1 }, max: Point2D_01 { x: 2.2, y: 3.3 } }
}

#[no_mangle]
pub extern "C" fn test01(a: Box2D_01, b: usize) {
    println!("test01 {:?} {}", a, b);
    assert!(a.min.x == 0.1);
    assert!(a.min.y == 1.1);
    assert!(a.max.x == 2.2);
    assert!(a.max.y == 3.3);
    assert!(b == 7);
}

// 02

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_02 {
    pub x: f32,
    pub y: i8,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_02 {
    pub min: Point2D_02,
    pub max: Point2D_02,
}

#[no_mangle]
pub extern "C" fn test02_ret() -> Box2D_02 {
    Box2D_02 { min: Point2D_02 { x: 0.1, y: 11 }, max: Point2D_02 { x: 2.2, y: 33 } }
}

#[no_mangle]
pub extern "C" fn test02(a: Box2D_02, b: usize) {
    println!("test02 {:?} {}", a, b);
    assert!(a.min.x == 0.1);
    assert!(a.min.y == 11);
    assert!(a.max.x == 2.2);
    assert!(a.max.y == 33);
    assert!(b == 7);
}

// 03

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_03 {
    pub x: f32,
    pub y: i8,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_03b {
    pub x: i8,
    pub y: i32,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_03 {
    pub min: Point2D_03,
    pub max: Point2D_03b,
}

#[no_mangle]
pub extern "C" fn test03_ret() -> Box2D_03 {
    Box2D_03 { min: Point2D_03 { x: 0.1, y: 11 }, max: Point2D_03b { x: 22, y: 33 } }
}

#[no_mangle]
pub extern "C" fn test03(a: Box2D_03, b: usize) {
    println!("test03 {:?} {}", a, b);
    assert!(a.min.x == 0.1);
    assert!(a.min.y == 11);
    assert!(a.max.x == 22);
    assert!(a.max.y == 33);
    assert!(b == 7);
}

// 04

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_04 {
    pub x: f32,
    pub y: i8,
    pub z: i16,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_04 {
    pub min: Point2D_04,
}

#[no_mangle]
pub extern "C" fn test04_ret() -> Box2D_04 {
    Box2D_04 { min: Point2D_04 { x: 0.1, y: 11, z: 111 } }
}

#[no_mangle]
pub extern "C" fn test04(a: Box2D_04, b: usize) {
    println!("test04 {:?} {}", a, b);
    assert!(a.min.x == 0.1);
    assert!(a.min.y == 11);
    assert!(a.min.z == 111);
    assert!(b == 7);
}

// 05

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_05b {
    pub x: f32,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_05 {
    pub x: Point2D_05b,
    pub y: i16,
    pub z: i8,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_05 {
    pub min: Point2D_05,
}

#[no_mangle]
pub extern "C" fn test05_ret() -> Box2D_05 {
    Box2D_05 { min: Point2D_05 { x: Point2D_05b { x: 0.1 }, y: 11, z: 111 } }
}

#[no_mangle]
pub extern "C" fn test05(a: Box2D_05, b: usize) {
    println!("test05 {:?} {}", a, b);
    assert!(a.min.x.x == 0.1);
    assert!(a.min.y == 11);
    assert!(a.min.z == 111);
    assert!(b == 7);
}

// 06 

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_06b {
    pub x: f32,
    pub y: i8,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_06 {
    pub x: Point2D_06b,
    pub y: i8,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_06 {
    pub min: Point2D_06,
}

#[no_mangle]
pub extern "C" fn test06_ret() -> Box2D_06 {
    Box2D_06 { min: Point2D_06 { x: Point2D_06b { x: 0.1, y: 11 } , y: 22 } }
}

#[no_mangle]
pub extern "C" fn test06(a: Box2D_06, b: usize) {
    println!("test06 {:?} {}", a, b);
    assert!(a.min.x.x == 0.1);
    assert!(a.min.x.y == 11);
    assert!(a.min.y == 22);
    assert!(b == 7);
}

// 07 - offset issue (update only if bigger than existing)
#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_07 {
    pub x: f32,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_07 {
    pub min: Point2D_07,
    pub max: Point2D_07,
}

#[no_mangle]
pub extern "C" fn test07_ret() -> Box2D_07 {
    Box2D_07 { min: Point2D_07 { x: 0.1 }, max: Point2D_07 { x: 1.1 } }
}

#[no_mangle]
pub extern "C" fn test07(a: Box2D_07, b: usize) {
    println!("test07 {:?} {}", a, b);
    assert!(a.min.x == 0.1);
    assert!(a.max.x == 1.1);
    assert!(b == 7);
}

// 08 original use case (prior to nested structure handling - aggregate)
#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Franta {
    a:f32,
    b:f32,
    c:f32,
    d:f32,
}

#[no_mangle]
pub extern "C" fn funkce_ret() -> Franta {
    Franta { a: 0.1, b: 1.1, c: 2.2, d: 3.3 }
}

#[no_mangle]
pub extern "C" fn funkce(arg: Franta, i: i32)
{
    println!("franta {:?} {}", arg, i);
    assert!(arg.a == 0.1);
    assert!(arg.b == 1.1);
    assert!(arg.c == 2.2);
    assert!(arg.d == 3.3);
    assert!(i == 7);
}

// 09

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct qcms_CIE_xyY {
    pub x: f64,
    pub y: f64,
    pub Y: f64,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct qcms_CIE_xyYTRIPLE {
    pub red: qcms_CIE_xyY,
    pub green: qcms_CIE_xyY,
    pub blue: qcms_CIE_xyY,
}

#[no_mangle]
pub extern "C" fn qcms_profile_create_rgb_with_gamma_ret1() -> qcms_CIE_xyY {
    qcms_CIE_xyY { x: 0.1, y: 1.1, Y: 2.2 }
}

#[no_mangle]
pub extern "C" fn qcms_profile_create_rgb_with_gamma_ret2() -> qcms_CIE_xyYTRIPLE {
    qcms_CIE_xyYTRIPLE { red:   qcms_CIE_xyY { x: 3.3, y: 4.4, Y: 5.5 }, 
                         green: qcms_CIE_xyY { x: 6.6, y: 7.7, Y: 8.8 },
                         blue:  qcms_CIE_xyY { x: 9.0, y: 9.1, Y: 9.2 }  }
}

#[no_mangle]
pub extern "C" fn qcms_profile_create_rgb_with_gamma(
    white_point: qcms_CIE_xyY,
    primaries: qcms_CIE_xyYTRIPLE,
    gamma: f32,
) {
    println!("qcms_profile_create_rgb_with_gamma {:?} {:?} {}", white_point, primaries, gamma);
    assert!(white_point.x == 0.1);
    assert!(white_point.y == 1.1);
    assert!(white_point.Y == 2.2);
    assert!(primaries.red.x == 3.3);
    assert!(primaries.red.y == 4.4);
    assert!(primaries.red.Y == 5.5);
    assert!(primaries.green.x == 6.6);
    assert!(primaries.green.y == 7.7);
    assert!(primaries.green.Y == 8.8);
    assert!(primaries.blue.x == 9.0);
    assert!(primaries.blue.y == 9.1);
    assert!(primaries.blue.Y == 9.2);
}

// 10 - unions

#[no_mangle]
#[repr(C)]
pub union Test10Union {
    f1: u32,
    f2: f32,
}

#[no_mangle]
pub extern "C" fn test10_ret() -> Test10Union {
    Test10Union { f2: 0.1 }
}

#[no_mangle]
pub extern "C" fn test10(a: Test10Union, b: usize) {
    unsafe { println!("test10 {:?} {}", a.f2, b); }
    unsafe { assert!(a.f2 == 0.1); }
    unsafe { assert!(b == 7); }
}

// 11 - union in structure

#[no_mangle]
#[repr(C)]
pub union Test11Union {
    f1: u32,
    f2: f32,
}

#[no_mangle]
#[repr(C)]
pub struct Test11Struct {
    x: Test11Union,
    y: f32,
}

#[no_mangle]
pub extern "C" fn test11_ret() -> Test11Struct {
    Test11Struct { x: Test11Union { f2: 0.1 }, y: 1.1 }
}

#[no_mangle]
pub extern "C" fn test11(a: Test11Struct, b: usize) {
    unsafe { println!("test11 {:?} {:?} {}", a.x.f2, a.y, b); }
    unsafe { assert!(a.x.f2 == 0.1); }
    unsafe { assert!(a.y == 1.1); }
    unsafe { assert!(b == 7); }
}

// 12

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_12 {
    pub x: f32,
    pub y: i32,
    pub z: i32,
}

#[no_mangle]
pub extern "C" fn test12_ret() -> Box2D_12 {
    Box2D_12 { x: 0.1, y: 11, z: 22 }
}

#[no_mangle]
pub extern "C" fn test12(a: Box2D_12, b: usize) {
    println!("test12 {:?} {}", a, b);
    unsafe { assert!(a.x == 0.1); }
    unsafe { assert!(a.y == 11); }
    unsafe { assert!(a.z == 22); }
    unsafe { assert!(b == 7); }
}

// 13

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_13 {
    pub x: f32,
    pub y: i64,
}

#[no_mangle]
pub extern "C" fn test13_ret() -> Box2D_13 {
    Box2D_13 { x: 0.1, y: 11 }
}

#[no_mangle]
pub extern "C" fn test13(a: Box2D_13, b: usize) {
    println!("test13 {:?} {}", a, b);
    unsafe { assert!(a.x == 0.1); }
    unsafe { assert!(a.y == 11); }
    unsafe { assert!(b == 7); }
}

// 14 - broken FF case (scalar pair, offset 1 before float)

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_14 {
    pub x: i8,
    pub y: f32,
}

#[no_mangle]
pub extern "C" fn test14_ret() -> Box2D_14 {
    Box2D_14 { x: 1, y: 1.1 }
}

#[no_mangle]
pub extern "C" fn test14(a: Box2D_14, b: usize) {
    println!("test14 {:?} {}", a, b);
    unsafe { assert!(a.x == 1); }
    unsafe { assert!(a.y == 1.1); }
    unsafe { assert!(b == 7); }
}

// 15 - broken FF case (addinng additional int into raw)

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_15 {
    pub x: f32,
}

#[no_mangle]
pub extern "C" fn test15_ret() -> Box2D_15 {
    Box2D_15 { x: 0.1 }
}

#[no_mangle]
pub extern "C" fn test15(a: Box2D_15, b: usize) {
    println!("test15 {:?} {}", a, b);
    unsafe { assert!(a.x == 0.1); }
    unsafe { assert!(b == 7); }
}

// 16

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_16 {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_16 {
    pub n: f32,
    pub min: Point2D_16,
}

#[no_mangle]
pub extern "C" fn test16_ret() -> Box2D_16 {
    Box2D_16 { n: 0.1, min: Point2D_16 { x: 11, y: 22 } }
}

#[no_mangle]
pub extern "C" fn test16(a: Box2D_16, b: usize) {
    println!("test16 {:?} {}", a, b);
    assert!(a.n == 0.1);
    assert!(a.min.x == 11);
    assert!(a.min.y == 22);
    assert!(b == 7);
}

// 17 FF example
//  (Arbitrary { offsets: [Size { raw: 0 }, Size { raw: 4 }, Size { raw: 8 }, Size { raw: 12 }, Size { raw: 16 }, Size { raw: 20 }], memory_index: [0, 1, 2, 3, 4, 5] })

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Point2D_17 {
    pub x: f32,
}

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_17 {
    pub a: Point2D_17,
    pub b: Point2D_17,
    pub c: Point2D_17,
    pub d: Point2D_17,
    pub e: Point2D_17,
    pub f: Point2D_17,
}

#[no_mangle]
pub extern "C" fn test17_ret() -> Box2D_17 {
    Box2D_17 { a: Point2D_17 { x: 0.1}, b: Point2D_17 { x: 1.1}, c: Point2D_17 { x: 2.2}, d: Point2D_17 { x: 3.3}, e: Point2D_17 { x: 4.4}, f: Point2D_17 { x: 5.5} }
}

#[no_mangle]
pub extern "C" fn test17(a: Box2D_17, b: usize) {
    println!("test17 {:?} {}", a, b);
    assert!(a.a.x == 0.1);
    assert!(a.b.x == 1.1);
    assert!(a.c.x == 2.2);
    assert!(a.d.x == 3.3);
    assert!(a.e.x == 4.4);
    assert!(a.f.x == 5.5);
    assert!(b == 7);
}

// 18 FF another real example

#[derive(Debug)]
#[no_mangle]
#[repr(C)]
pub struct Box2D_18 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

#[no_mangle]
pub extern "C" fn test18_ret() -> Box2D_18 {
    Box2D_18 { a: 0.1, b: 1.1, c: 2.2 }
}

#[no_mangle]
pub extern "C" fn test18(a: Box2D_18, b: usize) {
    println!("test18 {:?} {}", a, b);
    assert!(a.a == 0.1);
    assert!(a.b == 1.1);
    assert!(a.c == 2.2);
    assert!(b == 7);
}
