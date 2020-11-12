use std::collections::HashMap;

mod language;
mod lite_parse;
mod parse;

use language::{ExpressionShape, Scope};
use parse::parse;

fn main() {
    let input1 = "this 34";
    let mut commands = HashMap::new();
    commands.insert("this".to_string(), vec![ExpressionShape::Integer]);

    let scope = Box::new(Scope {
        parent: None,
        commands,
    });

    println!("{:#?}", parse(input1, 0, &scope));
}
