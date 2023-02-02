use pest::{iterators::Pair, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "grammar/expression.pest"]
struct ExpressionParser;

pub fn parse(input: &str) -> i32 {
    let expression = ExpressionParser::parse(Rule::expression, input)
        .unwrap()
        .next()
        .unwrap();
    let mut initial_value = parse_number_value(expression.clone().into_inner().nth(0).unwrap());

    for inner in expression.into_inner() {
        if inner.as_rule() == Rule::operation {
            initial_value = resolve_operation(&initial_value, inner);
        }
    }
    return initial_value;
}

fn resolve_operation(acc: &i32, operation: Pair<Rule>) -> i32 {
    let operator = extract_operator_from_operation(operation.clone()).unwrap();
    let number = extract_number_from_operation(operation).unwrap();

    match operator.as_rule() {
        Rule::add => acc + number,
        Rule::sub => acc - number,
        Rule::mul => acc * number,
        Rule::div => acc / number,
        _ => unreachable!(),
    }
}

fn extract_operator_from_operation(operation: Pair<Rule>) -> Option<Pair<Rule>> {
    for i in operation.into_inner() {
        if i.as_rule() == Rule::operator {
            return i.into_inner().next();
        }
    }
    None
}
fn extract_number_from_operation(operation: Pair<Rule>) -> Option<i32> {
    for i in operation.into_inner() {
        if i.as_rule() == Rule::number {
            return Some(parse_number_value(i));
        }
    }
    return None;
}

fn parse_number_value(number_pair: Pair<Rule>) -> i32 {
    number_pair.as_str().parse::<i32>().unwrap()
}


#[cfg(test)]
mod test {
    use super::parse;
    #[test]
    fn should_calc() {
        assert_eq!(parse("5 + 9"), 14);
    }
}
