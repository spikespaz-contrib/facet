use facet::Facet;

#[derive(Debug, Facet, PartialEq)]
struct EmptyRename {
    #[facet(rename = "")]
    empty_name: String,
}

fn main() {}
