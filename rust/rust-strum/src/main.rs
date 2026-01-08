use anyhow::Result;
use strum::EnumString;

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
enum NewType {
    Text,

    // Allow multiple string to be parsed into this Enum
    #[strum(serialize = "number", serialize = "decimal")]
    Number,

    #[allow(dead_code)]
    #[strum(disabled)]
    Generic(NewGenericType, Box<NewType>), // Generic with 1 type field
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
enum NewGenericType {
    Array,
    Nullable,
}

fn parse_type(ty: &str) -> Result<NewType> {
    if let Some((left, right)) = ty.split_once(":") {
        return Ok(NewType::Generic(
            left.parse()?,
            Box::new(parse_type(right)?),
        ));
    }
    Ok(ty.parse()?)
}

fn main() -> Result<()> {
    let ok_inputs = &[
        "text",
        "number",
        "decimal",
        "nullable:text",
        "array:nullable:number",
    ];

    let ok_outputs = ok_inputs
        .iter()
        .map(|x| parse_type(x))
        .collect::<Result<Vec<_>>>();

    println!("Inputs:");
    dbg!(ok_inputs);
    println!("Outputs:");
    let _ = dbg!(ok_outputs);

    println!("Invalids:");
    let _ = dbg!(parse_type("invalid"));
    let _ = dbg!(parse_type("array:invalid"));
    Ok(())
}
