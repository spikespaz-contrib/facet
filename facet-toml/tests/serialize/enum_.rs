//! Tests for TOML values to structs.

use facet::Facet;
use facet_testhelpers::test;

use crate::assert_serialize;

#[test]
fn test_unit_only_enum() {
    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: UnitOnlyEnum,
    }

    #[derive(Debug, Facet, PartialEq)]
    #[repr(u8)]
    enum UnitOnlyEnum {
        VariantA,
        VariantB,
    }

    assert_serialize!(
        Root,
        Root {
            value: UnitOnlyEnum::VariantA,
        }
    );
    assert_serialize!(
        Root,
        Root {
            value: UnitOnlyEnum::VariantB,
        },
    );
}

#[test]
fn test_single_value_on_non_unit_enum() {
    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: WithNonUnitVariant,
    }

    #[derive(Debug, Facet, PartialEq)]
    #[repr(u8)]
    enum WithNonUnitVariant {
        VariantA,
        #[allow(dead_code)]
        VariantB(i32),
    }

    assert_serialize!(
        Root,
        Root {
            value: WithNonUnitVariant::VariantA
        }
    );
    assert_serialize!(
        Root,
        Root {
            value: WithNonUnitVariant::VariantB(1)
        }
    );
}

#[test]
fn test_tuple_enum() {
    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: WithTupleVariants,
    }

    #[derive(Debug, Facet, PartialEq)]
    #[repr(u8)]
    enum WithTupleVariants {
        OneField(f32),
        TwoFields(bool, i16),
    }

    assert_serialize!(
        Root,
        Root {
            value: WithTupleVariants::OneField(0.5)
        }
    );
    assert_serialize!(
        Root,
        Root {
            value: WithTupleVariants::TwoFields(true, 1)
        }
    );
}

#[test]
fn test_struct_enum() {
    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: WithStructVariants,
    }

    #[derive(Debug, Facet, PartialEq)]
    #[repr(u8)]
    enum WithStructVariants {
        OneField { one: f64 },
        TwoFields { first: bool, second: u8 },
    }

    assert_serialize!(
        Root,
        Root {
            value: WithStructVariants::OneField { one: 0.5 }
        }
    );
    assert_serialize!(
        Root,
        Root {
            value: WithStructVariants::TwoFields {
                first: true,
                second: 1
            }
        }
    );
}

#[test]
fn test_nested_struct_enum() {
    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: WithNestedStructVariants,
    }

    #[derive(Debug, Facet, PartialEq)]
    #[repr(u8)]
    enum WithNestedStructVariants {
        OneField { one: NestedStructs },
        TwoFields { first: NestedStructs, second: u8 },
    }

    #[derive(Debug, Facet, PartialEq)]
    #[repr(u8)]
    enum NestedStructs {
        NestedOneField {
            nested_one: f64,
        },
        NestedTwoFields {
            nested_first: bool,
            nested_second: i8,
        },
    }

    assert_serialize!(
        Root,
        Root {
            value: WithNestedStructVariants::OneField {
                one: NestedStructs::NestedOneField { nested_one: 0.5 }
            }
        }
    );
    assert_serialize!(
        Root,
        Root {
            value: WithNestedStructVariants::TwoFields {
                first: NestedStructs::NestedTwoFields {
                    nested_first: false,
                    nested_second: 8
                },
                second: 1
            }
        }
    );
}

#[test]
fn test_enum_root() {
    #[derive(Debug, Facet, PartialEq)]
    #[repr(u8)]
    enum Root {
        A { value: u16 },
        B(u32),
        C,
    }

    assert_serialize!(Root, Root::A { value: 1 });
    assert_serialize!(Root, Root::B(2));
    assert_serialize!(Root, Root::C);
}
