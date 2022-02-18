Tests Rustc to C ABI

Purely done to fix broken Rust SPARC64 ABI.

Example:
```
$ RUSTC=/opt/my-test-rust/bin/rustc ./run.sh
RUNNING TESTS:
test01 Box2D_01 { min: Point2D_01 { x: 0.1, y: 1.1 }, max: Point2D_01 { x: 2.2, y: 3.3 } } 7
test02 Box2D_02 { min: Point2D_02 { x: 0.1, y: 11 }, max: Point2D_02 { x: 2.2, y: 33 } } 7
test03 Box2D_03 { min: Point2D_03 { x: 0.1, y: 11 }, max: Point2D_03b { x: 22, y: 33 } } 7
test04 Box2D_04 { min: Point2D_04 { x: 0.1, y: 11, z: 111 } } 7
test05 Box2D_05 { min: Point2D_05 { x: Point2D_05b { x: 0.1 }, y: 11, z: 111 } } 7
test06 Box2D_06 { min: Point2D_06 { x: Point2D_06b { x: 0.1, y: 11 }, y: 22 } } 7
test07 Box2D_07 { min: Point2D_07 { x: 0.1 }, max: Point2D_07 { x: 1.1 } } 7
franta Franta { a: 0.1, b: 1.1, c: 2.2, d: 3.3 } 7
qcms_profile_create_rgb_with_gamma qcms_CIE_xyY { x: 0.1, y: 1.1, Y: 2.2 } qcms_CIE_xyYTRIPLE { red: qcms_CIE_xyY { x: 3.3, y: 4.4, Y: 5.5 }, green: qcms_CIE_xyY { x: 6.6, y: 7.7, Y: 8.8 }, blue: qcms_CIE_xyY { x: 9.0, y: 9.1, Y: 9.2 } } 777.777
test10 0.1 7
test11 0.1 1.1 7
test12 Box2D_12 { x: 0.1, y: 11, z: 22 } 7
test13 Box2D_13 { x: 0.1, y: 11 } 7
test14 Box2D_14 { x: 1, y: 1.1 } 7
test15 Box2D_15 { x: 0.1 } 7
test16 Box2D_16 { n: 0.1, min: Point2D_16 { x: 11, y: 22 } } 7
test17 Box2D_17 { a: Point2D_17 { x: 0.1 }, b: Point2D_17 { x: 1.1 }, c: Point2D_17 { x: 2.2 }, d: Point2D_17 { x: 3.3 }, e: Point2D_17 { x: 4.4 }, f: Point2D_17 { x: 5.5 } } 7
test18 Box2D_18 { a: 0.1, b: 1.1, c: 2.2 } 7
DONE
```
