#[derive(PartialEq, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(PartialEq, Debug)]
pub enum OperationPrimitive<'a> {
    Number {
        val: i32
    },
    Operation {
        val: &'a Operation<'a>
    }
}

#[derive(Debug, PartialEq)]
pub struct Operation<'a> {
    pub a: OperationPrimitive<'a>,
    pub b: OperationPrimitive<'a>,
    pub operator: Operator
}

