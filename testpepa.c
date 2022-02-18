#include <stdio.h>

// 01

struct Point2D_01 {
        float x;
        float y;
};

struct Box2D_01 {
        struct Point2D_01 x;
        struct Point2D_01 y;
};

void test01(struct Box2D_01 a, int b);
struct Box2D_01 test01_ret();

// 02

struct Point2D_02 {
        float x;
        signed char y;
};

struct Box2D_02 {
        struct Point2D_02 x;
        struct Point2D_02 y;
};

void test02(struct Box2D_02 a, int b);
struct Box2D_02 test02_ret();

// 03

struct Point2D_03 {
        float x;
        signed char y;
};

struct Point2D_03b {
        signed char x;
        int y;
};

struct Box2D_03 {
        struct Point2D_03 x;
        struct Point2D_03b y;
};

void test03(struct Box2D_03 a, int b);
struct Box2D_03 test03_ret();

// 04

struct Point2D_04 {
        float x;
        signed char y;
        short z;
};

struct Box2D_04 {
        struct Point2D_04 x;
};

void test04(struct Box2D_04 a, int b);
struct Box2D_04 test04_ret();

// 05

struct Point2D_05b {
        float x;
};

struct Point2D_05 {
        struct Point2D_05b x;
        short y;
        signed char z;
};

struct Box2D_05 {
        struct Point2D_05 x;
};

void test05(struct Box2D_05 a, int b);
struct Box2D_05 test05_ret();

// 06

struct Point2D_06b {
        float x;
	signed char y;
};

struct Point2D_06 {
        struct Point2D_06b x;
        signed char y;
};

struct Box2D_06 {
        struct Point2D_06 x;
};

void test06(struct Box2D_06 a, int b);
struct Box2D_06 test06_ret();

// 07

struct Point2D_07 {
        float x;
};

struct Box2D_07 {
        struct Point2D_07 x;
        struct Point2D_07 y;
};

void test07(struct Box2D_07 a, int b);
struct Box2D_07 test07_ret();

// 08 original test
struct Franta {
        float a;
        float b;
        float c;
        float d;
};

void funkce(struct Franta a, int b);
struct Franta funkce_ret();

// 09 firefox PNG decoder issue

typedef struct {
        double x;
        double y;
        double Y;
} qcms_CIE_xyY;

typedef struct
{
        qcms_CIE_xyY red;
        qcms_CIE_xyY green;
        qcms_CIE_xyY blue;
} qcms_CIE_xyYTRIPLE;

void qcms_profile_create_rgb_with_gamma(
                qcms_CIE_xyY white_point,
                qcms_CIE_xyYTRIPLE primaries,
                float gamma);
qcms_CIE_xyY qcms_profile_create_rgb_with_gamma_ret1();
qcms_CIE_xyYTRIPLE qcms_profile_create_rgb_with_gamma_ret2();

// 10 union

typedef union {
        int f1;
        float f2;
} Test10Union;

void test10(Test10Union a, int b);
Test10Union test10_ret();

// 11 union

typedef union {
        int f1;
        float f2;
} Test11Union;

typedef struct {
        Test11Union x;
        float y;
} Test11Struct;

void test11(Test11Struct a, int b);
Test11Struct test11_ret();

// 12

typedef struct {
	float x;
	int y;
	int z;
} Test12Struct;

void test12(Test12Struct a, int b);
Test12Struct test12_ret();

// 13

typedef struct {
	float x;
	long y;
} Test13Struct;

void test13(Test13Struct a, int b);
Test13Struct test13_ret();

// 14 - broken FF case (scalar pair, offset 1 before float)

typedef struct {
	signed char x;
	float y;
} Test14Struct;

void test14(Test14Struct a, int b);
Test14Struct test14_ret();

// 15 - broken FF case (addinng additional int into raw)

typedef struct {
	float x;
} Test15Struct;

void test15(Test15Struct a, int b);
Test15Struct test15_ret();

// 16

struct Point2D_16 {
        short x;
        short y;
};

struct Box2D_16 {
        float n;
        struct Point2D_16 min;
};

void test16(struct Box2D_16 a, int b);
struct Box2D_16 test16_ret();

// 17

struct Point2D_17 {
	float x;
};

struct Box2D_17 {
	struct Point2D_17 a;
	struct Point2D_17 b;
	struct Point2D_17 c;
	struct Point2D_17 d;
	struct Point2D_17 e;
	struct Point2D_17 f;
};

void test17(struct Box2D_17 a, int b);
struct Box2D_17 test17_ret();

// 18

struct Box2D_18 {
	double a;
	double b;
	double c;
};

void test18(struct Box2D_18 a, int b);
struct Box2D_18 test18_ret();

int main() {
  printf("RUNNING TESTS:\n");
  { test01(test01_ret(), 7); }
  { test02(test02_ret(), 7); }
  { test03(test03_ret(), 7); }
  { test04(test04_ret(), 7); }
  { test05(test05_ret(), 7); }
  { test06(test06_ret(), 7); }
  { test07(test07_ret(), 7); }
  { funkce(funkce_ret(), 7); }
  { qcms_profile_create_rgb_with_gamma(qcms_profile_create_rgb_with_gamma_ret1(), qcms_profile_create_rgb_with_gamma_ret2(), 777.777); }
  { test10(test10_ret(), 7); }
  { test11(test11_ret(), 7); }
  { test12(test12_ret(), 7); }
  { test13(test13_ret(), 7); }
  { test14(test14_ret(), 7); }
  { test15(test15_ret(), 7); }
  { test16(test16_ret(), 7); }
  { test17(test17_ret(), 7); }
  { test18(test18_ret(), 7); }
  printf("DONE\n");
  return 0;
}
