mod borsh_type;
mod field;
mod borsh_container;
pub use borsh_type::BorshType;
pub use field::LayoutField;

/// Indicates whether the layout should be generated for a `struct` or an
/// `enum` type.
#[derive(Debug)]
pub enum Kind {
    Enum,
    Struct,
}

/// The layout of a Rust data structure that is straghtforward to convert into
/// a TypeScript class and the respective borsh schema.
#[derive(Debug)]
pub struct Layout {
    pub name: String,
    pub kind: Kind,
    pub fields: Vec<LayoutField>,
}

impl Layout {

    /// Converts the layout into a TypeScript class.
    pub fn to_ts_class(&self) -> String {
        let class_fields = self
            .fields
            .iter()
            .filter(|field| !field.should_skip())
            .map(|field| String::from("\n  ") + &field.to_class_field() + " | undefined;")
            .collect::<String>();
        format!(
            r#"export class {} extends {:?} {{{}
}};

"#,
            self.name, self.kind, class_fields
        )
    }

    /// Converts the layout into a borsh schema.
    pub fn to_borsh_schema(&self) -> String {
        let first_line = match self.kind {
            Kind::Struct => "kind: 'struct', fields:",
            Kind::Enum => "kind: 'enum', field: 'enum', values:",
        };
        let borsh_schema_fields = self
            .fields
            .iter()
            .filter(|field| !field.should_skip())
            .map(|field| String::from("\n\t\t\t") + &field.to_borsh_schema() + ",")
            .collect::<String>();
        // NOTE don't change this string (tabs are included in the output string)
        format!(
            r#"
    [
            {},
            {{
                {} [{}
                ],
            }},
    ],"#,
            self.name, first_line, borsh_schema_fields,
        )
    }
}
