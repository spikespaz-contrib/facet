use facet::Facet;

#[derive(Debug, Facet, PartialEq)]
#[repr(u8)]
enum EmptyRenameField {
    StructVariant {
        #[facet(rename = "")]
        field: String,
    },
}

fn main() {}
