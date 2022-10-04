#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn calc(vec: &mut Vec<i32>, oper: &CalculatorInput) -> Result<i32, String> {
    let opt_snd: Option<i32> = vec.pop();
    let opt_fst: Option<i32> = vec.pop();

    if opt_fst.is_none() || opt_snd.is_none() {
        return Err(String::from("The stack is empty"));
    }

    let fst: i32 = opt_fst.unwrap();
    let snd: i32 = opt_snd.unwrap();

    println!("fst: {fst}, snd: {snd} ");    
    match oper {
        CalculatorInput::Add => { return Ok(fst+snd); },
        CalculatorInput::Subtract => { return Ok(fst - snd); },
        CalculatorInput::Multiply => { return Ok(fst * snd); },
        CalculatorInput::Divide => {
            if snd == 0 {
                return Err(String::from("Cannot be divided by zero"));
            } else {
                return Ok(fst / snd);
            }
        }
        _ => {
            return Err(String::from("Input Error"));
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut num_vec: Vec<i32> = Vec::new();
    
    for val in inputs.iter() {
        match val {
            CalculatorInput::Value(v) => {
                num_vec.push(*v);
            }
            match_oper @ (CalculatorInput::Add |
            CalculatorInput::Subtract |
            CalculatorInput::Multiply |
            CalculatorInput::Divide) => {
                let ret: Result<i32, String> = calc(&mut num_vec, match_oper);
                match ret {
                    Ok(v) => {
                        num_vec.push(v);
                    },
                    Err(s) => {
                        println!("{s}");
                        return None;
                    }
                }
            }
        }
    };

    if num_vec.len() != 1 {
        return None;
    }

    return Some(num_vec[0]);
}

#[cfg(test)]
fn calculator_input(s: &str) -> Vec<CalculatorInput> {
    s.split_whitespace()
        .map(|s| match s {
            "+" => CalculatorInput::Add,
            "-" => CalculatorInput::Subtract,
            "*" => CalculatorInput::Multiply,
            "/" => CalculatorInput::Divide,
            n => CalculatorInput::Value(n.parse().unwrap()),
        })
        .collect()
}

#[test]
fn test_empty_input_returns_none() {
    let input = calculator_input("");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_simple_value() {
    let input = calculator_input("10");
    assert_eq!(evaluate(&input), Some(10));
}

#[test]
fn test_simple_addition() {
    let input = calculator_input("2 2 +");
    assert_eq!(evaluate(&input), Some(4));
}

#[test]
fn test_simple_subtraction() {
    let input = calculator_input("7 11 -");
    assert_eq!(evaluate(&input), Some(-4));
}

#[test]
fn test_simple_multiplication() {
    let input = calculator_input("6 9 *");
    assert_eq!(evaluate(&input), Some(54));
}

#[test]
fn test_simple_division() {
    let input = calculator_input("57 19 /");
    assert_eq!(evaluate(&input), Some(3));
}

#[test]
fn test_complex_operation() {
    let input = calculator_input("4 8 + 7 5 - /");
    assert_eq!(evaluate(&input), Some(6));
}

#[test]
fn test_too_few_operands_returns_none() {
    let input = calculator_input("2 +");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_too_many_operands_returns_none() {
    let input = calculator_input("2 2");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_zero_operands_returns_none() {
    let input = calculator_input("+");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_intermediate_error_returns_none() {
    let input = calculator_input("+ 2 2 *");
    assert_eq!(evaluate(&input), None);
}
