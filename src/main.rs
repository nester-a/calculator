fn main() {
    let first = 1.1;
    let mut second = 2.2;
    println!("first: {}, second: {}", first, second);

    let mut opp = Operation::Add;
    println!("current operation: {:?}", opp);
    let res = calculate(first, second, &opp);
    match res {
        Ok(result) => println!("{result}"),
        Err(error) => println!("{:?}", error)
    }

    opp = Operation::Subtract;
    println!("current operation: {:?}", opp);
    let res = calculate(first, second, &opp);
    match res {
        Ok(result) => println!("{result}"),
        Err(error) => println!("{:?}", error)
    }

    opp = Operation::Multiply;
    println!("current operation: {:?}", opp);
    let res = calculate(first, second, &opp);
    match res {
        Ok(result) => println!("{result}"),
        Err(error) => println!("{:?}", error)
    }

    opp = Operation::Divide;
    println!("current operation: {:?}", opp);
    let res = calculate(first, second, &opp);
    match res {
        Ok(result) => println!("{result}"),
        Err(error) => println!("{:?}", error)
    }

    println!("error demonstartion");
    second = 0.0;
    println!("first: {}, second: {}", first, second);
    println!("current operation: {:?}", opp);
    let res = calculate(first, second, &opp);
    match res {
        Ok(result) => println!("{result}"),
        Err(error) => println!("{:?}", error)
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum CalculationError {
    DivisionByZero,
    InvalidInput,
}

fn calculate(first: f64, second: f64, operation: &Operation) -> Result<f64, CalculationError> {
    match operation {
        Operation::Add => Ok(first + second),
        Operation::Subtract => Ok(first - second),
        Operation::Multiply => Ok(first * second),
        Operation::Divide => {
            if second == 0.0 {
                Err(CalculationError::DivisionByZero)
            } else {
                Ok(first / second)
            }
        }
    }
}
