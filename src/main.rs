// Andriella Wagner

enum Expression {
    Integer(i32),
    FixedPoint(i32, i32),
    Addition(Vec<Expression>)
}

fn evaluate_integer(expression: &Expression) -> f64 {
    if let Expression::Integer(value) = expression {
        *value as f64
    } else {
        panic!("not an integer, how could we even get here???");
    }
}

fn evaluate_fixed_point(expression: &Expression) -> f64 {
    if let Expression::FixedPoint(valueone, valuetwo) = expression {
        let x = *valueone as f64;
        let y = *valuetwo as f64;
        let deci = y / (100 as f64);
        let fixedpoint = x + deci;
        fixedpoint
    } else {
        panic!("not a fixed point");
    }
}

fn evaluate_add_integers(expressions: &Vec<Expression>) -> Expression {
    let mut total = 0;
    for each in expressions {
        if let Expression::Integer(value) = each {
            total = total + value;
        } else {
            panic!("I only can add integers");
        }
    }

    Expression::Integer(total)
}

fn evaluate_add_fixed_point(expressions: &Vec<Expression>) -> Expression {
    let mut total = 0 as f64;
    for each in expressions {
        if let Expression::FixedPoint(valueone, valuetwo) = each {
            let value = evaluate_fixed_point(each);
            total = total + value;
        } else {
            panic!("I only can add fixed points");
        }
    }
    let x = total as i64;
    let y = (total - (x as f64)) * (100 as f64);
    Expression::FixedPoint(x as i32, y as i32)
}

fn evaluate_addition(expression: &Expression) -> Expression {
    if let Expression::Addition(expressions) = expression {
        match expressions[0] {
            Expression::Integer(_) => evaluate_add_integers(expressions),
            _ => panic!("I only know how to add integers")
        }
    } else {
        panic!("how could you do this to me? I expected an addition")
    }
}

fn evaluate(expression: &Expression) -> f64 {
    match expression {
        Expression::Addition(_) => evaluate(&evaluate_addition(expression)),
        Expression::Integer(_) => evaluate_integer(expression),
        Expression::FixedPoint(_, _) => evaluate_fixed_point(expression),
        _ => panic!("not implemented")
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_anything_works() {
        assert!(true);
    }

    #[test]
    fn test_integer_value() {
        // arrange
        let expr = crate::Expression::Integer(42);
        // act
        let value = crate::evaluate(&expr);
        // assert
        assert_eq!(value, 42.0, "should be equal, 42 and 42.0");
    }

    #[test]
    fn test_fixed_point_value() {
        // arrange
        let expr = crate::Expression::FixedPoint(1,2);
        // act
        let value = crate::evaluate(&expr);
        // assert
        assert_eq!(value, 1.02, "should be 1.02");
    }

    #[test]
    fn test_simple_addition() {
        //arrange
        let expr = crate::Expression::Addition(vec![
            crate::Expression::Integer(2),
            crate::Expression::Integer(2)
        ]);
        //act
        let val = crate::evaluate(&expr);
        //assert
        assert_eq!(val, 4.0, "expressions not equal");
    }
}
