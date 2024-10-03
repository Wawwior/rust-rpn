use std::iter;

use proc_macro::{Literal, TokenStream, TokenTree};

#[proc_macro]
pub fn rpn(stream: TokenStream) -> TokenStream {
    let mut stack: Vec<f32> = vec![];
    let mut tokens = stream.into_iter();

    while let Some(token) = tokens.next() {
        let is_valid: Result<(), &dyn Fn(TokenTree) -> String> = match &token {
            TokenTree::Literal(l) => match l.to_string().parse::<f32>() {
                Ok(f) => {
                    stack.push(f);
                    Ok(())
                }
                Err(_) => Err(&(|t| format!("expected number, got: {}", t))),
            },
            TokenTree::Punct(p) => match stack.pop() {
                Some(val1) => match stack.pop() {
                    Some(val2) => {
                        if let Some(f) = bin_op(val2, val1, p.to_string().as_str()) {
                            stack.push(f);
                            Ok(())
                        } else {
                            Err(&(|t| format!("Expected operator, got: {}", t)))
                        }
                    }
                    None => Err(&(|_| String::from("Only one number to compute!"))),
                },
                None => Err(&(|_| String::from("No numbers to compute!"))),
            },
            _ => Err(&(|t| format!("Expected number or operator, got: {}", t))),
        };
        if let Err(f) = is_valid {
            return compile_error_token(f(token));
        }
    }

    if stack.len() > 1 {
        return compile_error_token(String::from("Does not evaluate to a single number!"));
    }
    if stack.len() == 0 {
        return compile_error_token(String::from("rpn!(...) may not be empty!"));
    }
    return TokenStream::from_iter(iter::once(TokenTree::Literal(Literal::f32_unsuffixed(
        stack.pop().unwrap(),
    ))));
}

fn bin_op(a: f32, b: f32, op: &str) -> Option<f32> {
    match op {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),
        "/" => Some(a / b),
        _ => None,
    }
}

fn compile_error_token(str: String) -> TokenStream {
    format!("compile_error!(\"{}\");", str)
        .parse::<TokenStream>()
        .unwrap()
}
