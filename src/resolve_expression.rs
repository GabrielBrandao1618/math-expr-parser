use crate::math::resolve_operation;
use crate::parser::parse_input;

pub fn resolve_expression(expression: &str) -> i64 {
    let operation = parse_input(expression);
    let result = resolve_operation(&operation);
    return result;
}
