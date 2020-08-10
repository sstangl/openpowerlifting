//! Parts of the [opltypes] library, expressed for GraphQL.

/// The GraphQL variant of [opltypes::Equipment].
#[derive(GraphQLEnum)]
pub enum Equipment {
    Raw,
    Wraps,
    SinglePly,
    MultiPly,
    Unlimited,
    Straps,
}

impl From<opltypes::Equipment> for Equipment {
    fn from(o: opltypes::Equipment) -> Equipment {
        match o {
            opltypes::Equipment::Raw => Equipment::Raw,
            opltypes::Equipment::Wraps => Equipment::Wraps,
            opltypes::Equipment::Single => Equipment::SinglePly,
            opltypes::Equipment::Multi => Equipment::MultiPly,
            opltypes::Equipment::Unlimited => Equipment::Unlimited,
            opltypes::Equipment::Straps => Equipment::Straps,
        }
    }
}
