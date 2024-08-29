use smoothy::assert_that;

mod with_numbers {
    use super::*;

    mod u8 {
        use super::*;

        #[test]
        fn min() {
            assert_that(u8::MIN).equals(0);
        }

        #[test]
        fn max() {
            assert_that(u8::MAX).equals(255);
        }

        #[test]
        fn random_value() {
            let random_value: u8 = 123;
            assert_that(random_value).equals(123);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: u8 = 123;
            assert_that(random_value).equals(42);
        }
    }

    mod u16 {
        use super::*;

        #[test]
        fn min() {
            assert_that(u16::MIN).equals(0u16);
        }

        #[test]
        fn max() {
            assert_that(u16::MAX).equals(65535u16);
        }

        #[test]
        fn random_value() {
            let random_value: u16 = 321;
            assert_that(random_value).equals(321u16);
        }

        #[test]
        fn can_be_compared_to_u8() {
            let random_value: u16 = 123;
            assert_that(random_value).equals(123u8);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: u16 = 123;
            assert_that(random_value).equals(321u16);
        }
    }

    mod u32 {
        use super::*;

        #[test]
        fn min() {
            assert_that(u32::MIN).equals(0u32);
        }

        #[test]
        fn max() {
            assert_that(u32::MAX).equals(4_294_967_295_u32);
        }

        #[test]
        fn random_value() {
            let random_value: u32 = 321;
            assert_that(random_value).equals(321u32);
        }

        #[test]
        fn can_be_compared_to_u8() {
            let random_value: u32 = 123;
            assert_that(random_value).equals(123u8);
        }

        #[test]
        fn can_be_compared_to_u16() {
            let random_value: u32 = 123;
            assert_that(random_value).equals(123u16);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: u32 = 123;
            assert_that(random_value).equals(321u32);
        }
    }

    mod u64 {
        use super::*;

        #[test]
        fn min() {
            assert_that(u64::MIN).equals(0u64);
        }

        #[test]
        fn max() {
            assert_that(u64::MAX).equals(18_446_744_073_709_551_615_u64);
        }

        #[test]
        fn random_value() {
            let random_value: u64 = 321;
            assert_that(random_value).equals(321u64);
        }

        #[test]
        fn can_be_compared_to_u8() {
            let random_value: u64 = 123;
            assert_that(random_value).equals(123u8);
        }

        #[test]
        fn can_be_compared_to_u16() {
            let random_value: u64 = 123;
            assert_that(random_value).equals(123u16);
        }

        #[test]
        fn can_be_compared_to_u32() {
            let random_value: u64 = 123;
            assert_that(random_value).equals(123u32);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: u64 = 123;
            assert_that(random_value).equals(321u64);
        }
    }

    mod u128 {
        use super::*;

        #[test]
        fn min() {
            assert_that(u128::MIN).equals(0u128);
        }

        #[test]
        fn max() {
            assert_that(u128::MAX).equals(340_282_366_920_938_463_463_374_607_431_768_211_455_u128);
        }

        #[test]
        fn random_value() {
            let random_value: u128 = 321;
            assert_that(random_value).equals(321u128);
        }

        #[test]
        fn can_be_compared_to_u8() {
            let random_value: u128 = 123;
            assert_that(random_value).equals(123u8);
        }

        #[test]
        fn can_be_compared_to_u16() {
            let random_value: u128 = 123;
            assert_that(random_value).equals(123u16);
        }

        #[test]
        fn can_be_compared_to_u32() {
            let random_value: u128 = 123;
            assert_that(random_value).equals(123u32);
        }

        #[test]
        fn can_be_compared_to_u64() {
            let random_value: u128 = 123;
            assert_that(random_value).equals(123u64);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: u128 = 123;
            assert_that(random_value).equals(321u128);
        }
    }

    mod i8 {
        use super::*;

        #[test]
        fn min() {
            assert_that(i8::MIN).equals(-128);
        }

        #[test]
        fn max() {
            assert_that(i8::MAX).equals(127);
        }

        #[test]
        fn random_value() {
            let random_value: i8 = 123;
            assert_that(random_value).equals(123);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: i8 = 123;
            assert_that(random_value).equals(42);
        }
    }

    mod i16 {
        use super::*;

        #[test]
        fn min() {
            assert_that(i16::MIN).equals(-32768i16);
        }

        #[test]
        fn max() {
            assert_that(i16::MAX).equals(32767i16);
        }

        #[test]
        fn random_value() {
            let random_value: i16 = 321;
            assert_that(random_value).equals(321i16);
        }

        #[test]
        fn can_be_compared_to_i8() {
            let random_value: i16 = 123;
            assert_that(random_value).equals(123i8);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: i16 = 123;
            assert_that(random_value).equals(321i16);
        }
    }

    mod i32 {
        use super::*;

        #[test]
        fn min() {
            assert_that(i32::MIN).equals(-2_147_483_648_i32);
        }

        #[test]
        fn max() {
            assert_that(i32::MAX).equals(2_147_483_647_i32);
        }

        #[test]
        fn random_value() {
            let random_value: i32 = 321;
            assert_that(random_value).equals(321i32);
        }

        #[test]
        fn can_be_compared_to_i8() {
            let random_value: i32 = 123;
            assert_that(random_value).equals(123i8);
        }

        #[test]
        fn can_be_compared_to_i16() {
            let random_value: i32 = 123;
            assert_that(random_value).equals(123i16);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: u32 = 123;
            assert_that(random_value).equals(321u32);
        }
    }

    mod i64 {
        use super::*;

        #[test]
        fn min() {
            assert_that(i64::MIN).equals(-9_223_372_036_854_775_808_i64);
        }

        #[test]
        fn max() {
            assert_that(i64::MAX).equals(9_223_372_036_854_775_807_i64);
        }

        #[test]
        fn random_value() {
            let random_value: i64 = 321;
            assert_that(random_value).equals(321i64);
        }

        #[test]
        fn can_be_compared_to_i8() {
            let random_value: i64 = 123;
            assert_that(random_value).equals(123i8);
        }

        #[test]
        fn can_be_compared_to_i16() {
            let random_value: i64 = 123;
            assert_that(random_value).equals(123i16);
        }

        #[test]
        fn can_be_compared_to_i32() {
            let random_value: i64 = 123;
            assert_that(random_value).equals(123i32);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: i64 = 123;
            assert_that(random_value).equals(321i64);
        }
    }

    mod i128 {
        use super::*;

        #[test]
        fn min() {
            assert_that(i128::MIN)
                .equals(-170_141_183_460_469_231_731_687_303_715_884_105_728_i128);
        }

        #[test]
        fn max() {
            assert_that(i128::MAX).equals(170_141_183_460_469_231_731_687_303_715_884_105_727_i128);
        }

        #[test]
        fn random_value() {
            let random_value: i128 = 321;
            assert_that(random_value).equals(321i128);
        }

        #[test]
        fn can_be_compared_to_i8() {
            let random_value: i128 = 123;
            assert_that(random_value).equals(123i8);
        }

        #[test]
        fn can_be_compared_to_i16() {
            let random_value: i128 = 123;
            assert_that(random_value).equals(123i16);
        }

        #[test]
        fn can_be_compared_to_i32() {
            let random_value: i128 = 123;
            assert_that(random_value).equals(123i32);
        }

        #[test]
        fn can_be_compared_to_i64() {
            let random_value: i128 = 123;
            assert_that(random_value).equals(123i64);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: i128 = 123;
            assert_that(random_value).equals(321i128);
        }
    }

    mod f32 {
        use super::*;

        #[test]
        fn min() {
            assert_that(f32::MIN).equals(-3.402_823_5e38);
        }

        #[test]
        fn max() {
            assert_that(f32::MAX).equals(3.402_823_5e38);
        }

        #[test]
        fn random_value() {
            let random_value: f32 = 123.45;
            assert_that(random_value).equals(123.45);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: f32 = 123.;
            assert_that(random_value).equals(42.);
        }
    }

    mod f64 {
        use super::*;

        #[test]
        fn min() {
            assert_that(f64::MIN).equals(-1.797_693_134_862_315_7e308);
        }

        #[test]
        fn max() {
            assert_that(f64::MAX).equals(1.797_693_134_862_315_7e308);
        }

        #[test]
        fn random_value() {
            let random_value: f64 = 123.45;
            assert_that(random_value).equals(123.45);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn cannot_be_compared_to_f32() {
            let random_value: f64 = 123.45;
            assert_that(random_value).equals(123.45f32);
        }

        #[test]
        #[should_panic = "assertion failed: `(left == right)`"]
        fn fails() {
            let random_value: f64 = 123.45;
            assert_that(random_value).equals(321.01);
        }
    }
}

mod with_chars {
    use smoothy::assert_that;

    #[test]
    fn succeeds() {
        assert_that('a').equals('a');
    }

    #[test]
    #[should_panic = "assertion failed: `(left == right)`"]
    fn fails_for_wrong_char() {
        assert_that('a').equals('b');
    }

    #[test]
    #[should_panic = "assertion failed: `(left == right)`"]
    fn fails_for_wrong_casing() {
        assert_that('a').equals('A');
    }
}

mod with_bools {
    use smoothy::assert_that;

    #[test]
    fn succeeds_with_true() {
        assert_that(true).equals(true);
    }

    #[test]
    fn succeeds_with_false() {
        assert_that(false).equals(false);
    }

    #[test]
    fn succeeds_with_expression() {
        const fn true_func() -> bool {
            true
        }
        assert_that(true_func()).equals(true);
    }

    #[test]
    #[should_panic = "assertion failed: `(left == right)`"]
    fn fails() {
        assert_that(true).equals(false);
    }
}

mod with_unit_type {
    use smoothy::assert_that;

    #[test]
    fn succeeds() {
        assert_that(()).equals(());
    }
}

mod with_tuples {
    use smoothy::assert_that;

    #[test]
    fn succeeds() {
        assert_that((1, 2)).equals((1, 2));
    }

    #[test]
    #[should_panic = "assertion failed: `(left == right)`"]
    fn fails_with_different_order() {
        assert_that((1, 2)).equals((2, 1));
    }
}

mod with_vecs {
    use smoothy::assert_that;

    #[test]
    fn succeeds_with_const() {
        assert_that([1, 2, 3]).equals([1, 2, 3]);
    }

    #[test]
    fn succeeds_with_slices() {
        assert_that(&[1, 2, 3]).equals(&[1, 2, 3]);
    }

    #[test]
    fn succeeds_with_vecs() {
        assert_that(vec![1, 2, 3]).equals(vec![1, 2, 3]);
        assert_that(vec![1, 2, 3]).equals([1, 2, 3]);
    }

    #[test]
    #[should_panic = "assertion failed: `(left == right)`"]
    fn fails_with_different_order() {
        assert_that([1, 2, 3]).equals([3, 2, 1]);
    }

    #[test]
    #[should_panic = "assertion failed: `(left == right)`"]
    fn fails_with_different_size() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2];
        assert_that(a).equals(b);
    }
}

mod with_structs {
    use smoothy::assert_that;

    #[derive(Debug, PartialEq, Eq)]
    struct Test {
        data: String,
    }

    #[test]
    fn succeeds() {
        let a = Test {
            data: String::new(),
        };
        let b = Test {
            data: String::new(),
        };

        assert_that(a).equals(b);
    }

    #[test]
    #[should_panic = "assertion failed: `(left == right)`"]
    fn fails() {
        let a = Test {
            data: String::new(),
        };
        let b = Test {
            data: String::from("yo"),
        };

        assert_that(a).equals(b);
    }
}
