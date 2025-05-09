use facet::Facet;

#[derive(Debug, Facet, PartialEq)]
#[repr(u8)]
enum EmptyRenameEnum {
    #[facet(rename = "")]
    EmptyVariant,
    RegularVariant,
}

fn main() {}
