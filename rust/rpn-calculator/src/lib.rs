#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![0i32; 0];
    for input in inputs.iter() {
        match input {
            CalculatorInput::Value(n) => stack.push(*n),
            CalculatorInput::Add => {
                if let (Some(r), Some(l)) = (stack.pop(), stack.pop()) {
                    stack.push(l + r)
                } else {
                    return None;
                }
            }
            CalculatorInput::Subtract => {
                if let (Some(r), Some(l)) = (stack.pop(), stack.pop()) {
                    stack.push(l - r)
                } else {
                    return None;
                }
            }
            CalculatorInput::Multiply => {
                if let (Some(r), Some(l)) = (stack.pop(), stack.pop()) {
                    stack.push(l * r)
                } else {
                    return None;
                }
            }
            CalculatorInput::Divide => {
                if let (Some(r), Some(l)) = (stack.pop(), stack.pop()) {
                    stack.push(l / r)
                } else {
                    return None;
                }
            }
        }
    }
    if stack.len() != 1 {
        return None
    }
    stack.pop()
}
