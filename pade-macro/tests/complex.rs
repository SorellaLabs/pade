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

fn roundtrip_decode_encode_decode<T: PadeEncode + PadeDecode + PartialEq + Eq + std::fmt::Debug>(
    slice: &mut &[u8]
) {
    if let Ok(decoded) = T::pade_decode(slice, None) {
        let bytes = decoded.pade_encode();
        let decoded2 = T::pade_decode(&mut bytes.as_slice(), None).unwrap();
        assert_eq!(decoded, decoded2);
    }
}

#[test_fuzz::test_fuzz]
fn roundtrip_outer_struct_a(slice: &mut &[u8]) {
    roundtrip_decode_encode_decode::<OuterStructA>(slice);
}

#[test_fuzz::test_fuzz]
fn roundtrip_outer_struct_b(slice: &mut &[u8]) {
    roundtrip_decode_encode_decode::<OuterStructB>(slice);
}

#[test_fuzz::test_fuzz]
fn roundtrip_user_order(slice: &mut &[u8]) {
    roundtrip_decode_encode_decode::<UserOrder>(slice);
}

#[test_fuzz::test_fuzz]
fn roundtrip_test_struct(slice: &mut &[u8]) {
    roundtrip_decode_encode_decode::<TestStruct>(slice);
}

#[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
struct OuterStructA {
    #[pade_width(3)]
    x:      i32,
    enum1:  CasesA,
    list:   Vec<u128>,
    inside: Inside,
    enum2:  CasesA
}

#[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
struct Inside {
    number:  u128,
    another: u128,
    enum1:   CasesA
}

#[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
pub enum CasesA {
    Once { x: u128, y: u128 },
    Twice { a: u128, b: u128 },
    // memes
    Thrice { a: u128, b: u128 }
}

#[test]
fn supports_struct_with_enum() {
    let outer = OuterStructA {
        x:      34342,
        enum1:  CasesA::Twice { a: 10, b: 2000000 },
        list:   vec![1, 2, 3, 4023, 323424],
        inside: Inside {
            enum1:   CasesA::Thrice { a: 123, b: 423 },
            number:  234093323,
            another: 234234
        },
        enum2:  CasesA::Thrice { a: 100, b: 2000000 }
    };

    let encoded = outer.pade_encode();
    let mut slice = encoded.as_slice();
    roundtrip_outer_struct_a(&mut slice);
}

#[test]
fn regression_panic_1() {
    let mut bytes: &[u8] = &[
        9, 0, 134, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 38, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 132, 128, 0, 0,
        80, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 15, 183, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 239, 96, 2, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 13, 243, 251, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 146, 250, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 123, 0, 0, 0, 0, 0, 0, 0, 255, 245, 0, 0, 0, 0,
        0, 1, 167, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ];
    let _ = OuterStructA::pade_decode(&mut bytes, None);
}

#[test]
fn regression_panic_2() {
    let mut bytes: &[u8] = &[0];
    let _ = OuterStructA::pade_decode(&mut bytes, None);
}

#[test]
fn regression_panic_3() {
    let mut bytes: &[u8] = &[
        246, 0, 134, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 30, 132, 128, 0, 0, 80
    ];
    let _ = OuterStructA::pade_decode(&mut bytes, None);
}

#[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
struct OuterStructB {
    a:  bool,
    b:  bool,
    c:  bool,
    c1: bool,
    c3: bool,
    c2: bool,
    d:  CasesB,
    e:  CasesB
}

#[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
pub enum CasesB {
    Once { x: u128, y: u128 },
    Twice { a: u128, b: u128 },
    // memes
    Thrice { a: u128, b: u128 }
}

#[test]
fn bool_ordering_more_than_1byte() {
    let outer = OuterStructB {
        a:  true,
        b:  true,
        c:  true,
        c1: false,
        c2: false,
        c3: true,
        d:  CasesB::Twice { a: 0, b: 0 },
        e:  CasesB::Thrice { a: 0, b: 0 }
    };

    let encoded = outer.pade_encode();
    let mut slice = encoded.as_slice();
    println!("{:08b}", slice[0]);
    println!("{:08b}", slice[1]);
    roundtrip_outer_struct_b(&mut slice);
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

#[derive(Debug, PadeEncode, PadeDecode, PartialEq, Eq)]
struct TestStruct {
    pub number:     u32,
    pub option:     Option<u128>,
    pub number_two: u32,
    pub bool:       bool
}

#[test]
fn option_struct() {
    let s = TestStruct { number: 100, option: Some(95), number_two: 200, bool: true };
    let bytes = s.pade_encode();
    let mut slice = bytes.as_slice();
    roundtrip_test_struct(&mut slice);
}

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
#[test]
fn super_specific_dave_test() {
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
    roundtrip_user_order(&mut slice);
}
