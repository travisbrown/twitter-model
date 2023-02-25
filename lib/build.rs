use schemars::schema::Schema;
use std::path::Path;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let content = std::fs::read_to_string("../schemas/api.json").unwrap();
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_ref_types(schema.definitions).unwrap();
    type_space.add_type(&Schema::Object(schema.schema)).unwrap();

    let contents = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        type_space.to_string()
    );

    std::fs::write(Path::new("src/api.rs"), contents).unwrap();
}
