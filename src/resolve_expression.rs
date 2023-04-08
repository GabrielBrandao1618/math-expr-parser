use crate::math::resolve_operation;
use crate::parser::parse_input;

pub fn resolve_expression(expression: &str) -> i64 {
    let operation = parse_input(expression);
    let result = resolve_operation(&operation);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let expected = 10;
        let result = resolve_expression("4 + 2 * 3");
        assert_eq!(expected, result);
    }
    #[test]
    fn test_sub() {
        let expected = 20;
        let result = resolve_expression("10 + 20 - 10");
        assert_eq!(expected, result);
    }
    #[test]
    fn test_div() {
        let expected = 2;
        let result = resolve_expression("8 / 2 / 2");
        assert_eq!(expected, result);
    }
    #[test]
    fn test_mul() {
        let expected = 7;
        let result = resolve_expression("5 * 2 - 3");
        assert_eq!(expected, result);
    }
    #[test]
    fn test_sub_expression() {
        let expected = 5;
        let result = resolve_expression("10 - (4 + 1)");
        assert_eq!(expected, result);
    }
}
