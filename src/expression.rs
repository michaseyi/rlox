use crate::scanner::token::{Literal as LiteralEnum, Token, TokenType,};

pub trait Expression {
    fn accept<T>(&self, visitor: &impl Visitor<T>) -> T;
}
pub struct Binary<T: Expression, V: Expression> {
    left: Box<T>,
    operator: Token,
    right: Box<V>,
}

impl<T: Expression, V: Expression> Binary<T, V> {
    pub fn new(left: Box<T>, operator: Token, right: Box<V>) -> Self {
        Self {
            left,
            right,
            operator,
        }
    }
}

impl<T: Expression, V: Expression> Expression for Binary<T, V> {
    fn accept<U>(&self, visitor: &impl Visitor<U>) -> U {
        visitor.visit_binary_expression(self)
    }
}
pub struct Grouping<T: Expression> {
    expression: Box<T>,
}

impl<T: Expression> Grouping<T> {
    pub fn new(expression: Box<T>) -> Self {
        Self { expression }
    }

}

impl<T: Expression> Expression for Grouping<T> {
    fn accept<U>(&self, visitor: &impl Visitor<U>) -> U {
        visitor.visit_grouping_expression(self)
    }
}
pub struct Literal {
    value: LiteralEnum,
}

impl Literal {
    pub fn new(value: LiteralEnum) -> Self {
        Self { value }
    }
}

impl Expression for Literal {
    fn accept<T>(&self, visitor: &impl Visitor<T>) -> T {
        visitor.visit_literal_expression(self)
    }
}

pub struct Unary<T: Expression> {
    operator: Token,
    right: Box<T>,
}

impl<T: Expression> Unary<T> {
    pub fn new(operator: Token, right: Box<T>) -> Self {
        Self { operator, right }
    }

}

impl <T: Expression> Expression for Unary<T> {
    fn accept<U>(&self, visitor: &impl Visitor<U>) -> U {
        visitor.visit_unary_expression(self)
    }
}

pub trait Visitor<T> {
    fn visit_binary_expression(&self, expression: &Binary<impl Expression, impl Expression>) -> T;
    fn visit_grouping_expression(&self, expression: &Grouping<impl Expression>) -> T;
    fn visit_literal_expression(&self, expression: &Literal) -> T;
    fn visit_unary_expression(&self, expression: &Unary<impl Expression>) -> T;
}
pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_binary_expression(&self, expression: &Binary<impl Expression, impl Expression>) -> String {
        parenthesize_double(
            &expression.operator.lexeme,
            &expression.left,
            &expression.right
        )
    }

    fn visit_grouping_expression(&self, expression: &Grouping<impl Expression>) -> String {
        parenthesize_single("group", &expression.expression)
    }

    fn visit_literal_expression(&self, expression: &Literal) -> String {
        match &expression.value {
            LiteralEnum::Nil => "nil".to_string(),
            LiteralEnum::False => "false".to_string(),
            LiteralEnum::True => "true".to_string(),
            LiteralEnum::Number(number) => number.to_string(),
            LiteralEnum::String(string) => string.clone(),
        }
    }
    fn visit_unary_expression(&self, expression: &Unary<impl Expression>) -> String {
        parenthesize_single(&expression.operator.lexeme, &expression.right)
    }
}

impl AstPrinter {
    fn print(&self, expression: &Box<impl Expression>) -> String {
      expression.accept(self)
    }
}

fn parenthesize_single(name: &str, first_expression: &Box<impl Expression>) -> String {

    let printer = AstPrinter{};

    let first_expression_evaluation = first_expression.accept(&printer);
    if name == "group" {
        return format!("(group {})", first_expression_evaluation);
    }
    return format!("({} {})", name, first_expression_evaluation)
}

fn parenthesize_double(name: &str, first_expression: &Box<impl Expression>, second_expression: &Box<impl Expression>) -> String {

    let printer = AstPrinter{};

    let first_expression_evaluation = first_expression.accept(&printer);

    let second_expression_evaluation = second_expression.accept(&printer);
    return format!("({} {} {})", first_expression_evaluation, name, second_expression_evaluation);
}


pub fn create() {
    let expression = Box::new(Binary::new(
        Box::new(Unary::new(Token::new(TokenType::Minus, "-".to_string(), 1, None), Box::new(Literal::new(LiteralEnum::Number(10.1))))),
        Token::new(TokenType::Star, "*".to_string(), 1, None),
        Box::new(Grouping::new(Box::new(Literal::new(LiteralEnum::Number(10.33)))))
    ));

    let printer = AstPrinter{};


    println!("{}", printer.print(&expression));
}

