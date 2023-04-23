use crate::math::resolve_operation;
use crate::parser::parse_input;

pub fn resolve_expression(expression: &str) -> f64 {
    let operation = parse_input(expression);
    let result = resolve_operation(&operation);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(resolve_expression("4 + 2 * 3"), 10.0);
    }
    #[test]
    fn test_sub() {
        assert_eq!(resolve_expression("10 + 20 - 10"), 20.0);
    }
    #[test]
    fn test_div() {
        assert_eq!(resolve_expression("8 / 2 / 2"), 2.0);
    }
    #[test]
    fn test_mul() {
        assert_eq!(resolve_expression("5 * 2 - 3"), 7.0);
    }
    #[test]
    fn test_sub_expression() {
        assert_eq!(resolve_expression("10 - (4 + 1)"), 5.0);
    }

    #[test]
    fn test_power_expression() {
        assert_eq!(resolve_expression("2 + 2 ^ 2 - 2"), 4.0);
        assert_eq!(resolve_expression("2 + 2 ^ (2 - 2)"), 3.0);
        assert_eq!(resolve_expression("(2 + 2) ^ 2 - 2"), 14.0);
        assert_eq!(resolve_expression("2 + 2 ^ 2 + 2"), 8.0);
        assert_eq!(resolve_expression("2 + 2 ^ (2 + 2) ^ 2"), 258.0);
    }
    #[test]
    fn test_real_multiplication() {
        assert_eq!(resolve_expression("5 * 0.5"), 5.0 * 0.5);
    }
    #[test]
    fn test_real_division() {
        assert_eq!(resolve_expression("5 / 2"), 5.0 / 2.0);
    }
}
