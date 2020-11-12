use std::collections::HashMap;

mod language;
mod lite_parse;
mod parse;

use language::{ExpressionShape, Scope};
use parse::parse;

fn main() {
    let input = "this rick";
    let mut commands = HashMap::new();
    commands.insert("this".to_string(), vec![ExpressionShape::Integer]);
    commands.insert("that".to_string(), vec![ExpressionShape::Integer]);

    let scope = Box::new(Scope {
        parent: None,
        commands,
    });

    parse(input, 0, &scope);
    //println!("{:?}", parse(input, 0, &scope));
}
