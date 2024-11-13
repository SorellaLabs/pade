use pade::{PadeDecode, PadeEncode};
use pade_macro::{PadeDecode, PadeEncode};

#[test]
fn enums_have_correct_variant_bit_width() {
    #[derive(PadeEncode)]
    enum OneOption {
        Dave
    }

    #[derive(PadeEncode)]
    enum TwoOptions {
        Dave,
        #[allow(dead_code)]
        Knave
    }

    #[derive(PadeEncode)]
    #[allow(dead_code)]
    enum ThreeOptions {
        Dave,
        Knave,
        ToBlave
    }

    #[derive(PadeEncode)]
    #[allow(dead_code)]
    enum FiveOptions {
        Dave,
        Knave,
        ToBlave,
        Shave,
        Grave
    }

    assert_eq!(
        OneOption::Dave.pade_variant_map_bits(),
        1,
        "Wrong number of variant bits for One option"
    );
    assert_eq!(
        TwoOptions::Dave.pade_variant_map_bits(),
        1,
        "Wrong number of variant bits for Two option"
    );
    assert_eq!(
        ThreeOptions::Dave.pade_variant_map_bits(),
        2,
        "Wrong number of variant bits for Three option"
    );
    assert_eq!(
        FiveOptions::Dave.pade_variant_map_bits(),
        3,
        "Wrong number of variant bits for Five option"
    );
}

#[test]
fn supports_struct_with_enum() {
    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    struct OuterStruct {
        #[pade_width(3)]
        x:      i32,
        enum1:  Cases,
        list:   Vec<u128>,
        inside: Inside,
        enum2:  Cases
    }

    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    struct Inside {
        number:  u128,
        another: u128,
        enum1:   Cases
    }

    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    pub enum Cases {
        Once { x: u128, y: u128 },
        Twice { a: u128, b: u128 },
        // memes
        Thrice { a: u128, b: u128 }
    }

    let outer = OuterStruct {
        x:      34342,
        enum1:  Cases::Twice { a: 10, b: 2000000 },
        list:   vec![1, 2, 3, 4023, 323424],
        inside: Inside {
            enum1:   Cases::Thrice { a: 123, b: 423 },
            number:  234093323,
            another: 234234
        },
        enum2:  Cases::Thrice { a: 100, b: 2000000 }
    };

    let encoded = outer.pade_encode();
    let mut slice = encoded.as_slice();
    let decoded = OuterStruct::pade_decode(&mut slice, None).unwrap();

    assert_eq!(outer, decoded);
}

#[test]
fn bool_ordering_more_than_1byte() {
    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    struct OuterStruct {
        a:  bool,
        b:  bool,
        c:  bool,
        c1: bool,
        c3: bool,
        c2: bool,
        d:  Cases,
        e:  Cases
    }

    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    pub enum Cases {
        Once { x: u128, y: u128 },
        Twice { a: u128, b: u128 },
        // memes
        Thrice { a: u128, b: u128 }
    }

    let outer = OuterStruct {
        a:  true,
        b:  true,
        c:  true,
        c1: false,
        c2: false,
        c3: true,
        d:  Cases::Twice { a: 0, b: 0 },
        e:  Cases::Thrice { a: 0, b: 0 }
    };

    let encoded = outer.pade_encode();
    let mut slice = encoded.as_slice();
    println!("{:08b}", slice[0]);
    println!("{:08b}", slice[1]);
    let decoded = OuterStruct::pade_decode(&mut slice, None).unwrap();

    assert_eq!(outer, decoded);
}

#[test]
fn bool_ordering_more_than_1byte_diff_size() {
    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    struct OuterStruct {
        a:  bool,
        b:  bool,
        c:  bool,
        c1: bool,
        c2: bool,
        d:  Cases,
        e:  Cases
    }

    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    pub enum Cases {
        Once { x: u128, y: u128 },
        Twice { a: u128, b: u128 },
        // memes
        Thrice { a: u128, b: u128 }
    }

    let outer = OuterStruct {
        a:  true,
        b:  true,
        c:  true,
        c1: false,
        c2: false,
        d:  Cases::Twice { a: 0, b: 0 },
        e:  Cases::Thrice { a: 0, b: 0 }
    };

    let encoded = outer.pade_encode();
    let mut slice = encoded.as_slice();
    let decoded = OuterStruct::pade_decode(&mut slice, None).unwrap();

    assert_eq!(outer, decoded);
}

#[test]
fn bool_ordering_lower() {
    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    struct OuterStruct {
        a:  bool,
        b:  bool,
        c:  bool,
        c1: bool,
        c3: bool,
        c2: bool
    }

    let outer = OuterStruct { a: true, b: true, c: true, c1: false, c2: false, c3: true };

    let encoded = outer.pade_encode();
    let mut slice = encoded.as_slice();
    println!("{:08b}", slice[0]);
    let decoded = OuterStruct::pade_decode(&mut slice, None).unwrap();

    assert_eq!(outer, decoded);
}

#[test]
fn option_struct() {
    #[derive(Debug, PadeEncode, PadeDecode, PartialEq, Eq)]
    struct TestStruct {
        pub number:     u32,
        pub option:     Option<u128>,
        pub number_two: u32,
        pub bool:       bool
    }

    let s = TestStruct { number: 100, option: Some(95), number_two: 200, bool: true };
    let bytes = s.pade_encode();
    let mut slice = bytes.as_slice();
    let decoded = TestStruct::pade_decode(&mut slice, None).unwrap();

    assert_eq!(s, decoded);
}

#[test]
fn super_specific_dave_test() {
    #[derive(Debug, PadeEncode, PadeDecode, PartialEq, Eq)]
    pub enum OrderQuantities {
        Exact { quantity: u128 },
        Partial { min_quantity_in: u128, max_quantity_in: u128, filled_quantity: u128 }
    }

    #[derive(Debug, PadeEncode, PadeDecode, PartialEq, Eq)]
    enum Signature {
        TypeOne,
        TypeTwo
    }
    #[derive(Debug, PadeEncode, PadeDecode, PartialEq, Eq)]
    struct UserOrder {
        pub ref_id:               u32,
        pub use_internal:         bool,
        pub pair_index:           u16,
        pub min_price:            alloy::primitives::U256,
        pub recipient:            Option<alloy::primitives::Address>,
        pub hook_data:            Option<alloy::primitives::Bytes>,
        pub zero_for_one:         bool,
        pub standing_validation:  Option<u8>,
        pub order_quantities:     OrderQuantities,
        pub max_extra_fee_asset0: u128,
        pub extra_fee_asset0:     u128,
        pub exact_in:             bool,
        pub signature:            Signature
    }

    let item = UserOrder {
        ref_id:               25,
        use_internal:         false,
        pair_index:           50,
        min_price:            alloy::primitives::U256::from(29769_u128),
        recipient:            None,
        hook_data:            None,
        zero_for_one:         true,
        standing_validation:  None,
        order_quantities:     OrderQuantities::Partial {
            min_quantity_in: 0,
            max_quantity_in: 99,
            filled_quantity: 0
        },
        max_extra_fee_asset0: 0,
        extra_fee_asset0:     0,
        exact_in:             false,
        signature:            Signature::TypeTwo
    };
    let bytes = item.pade_encode();
    let mut slice = bytes.as_slice();
    let decoded = UserOrder::pade_decode(&mut slice, None).unwrap();

    assert_eq!(item, decoded);
}
