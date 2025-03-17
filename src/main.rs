use std::{collections::HashMap, f32::consts::E, io::stdin};

use regex::Regex;

fn main() {

    let mut input_flag: bool = false;

    while input_flag == false {

        //Read user input:  
        let mut infix: String = String::new();
        println!("Enter your expression (infix):");
        stdin().read_line(&mut infix).expect("Unable to read expression");

        // Extract lexemes from data:

        // Removed whitespace from infix
        let mut infix: String = "(3+ 4545 / 4 ) * { 2√6 * 2√64 } / 1 + [2 ] ( 33 33 - 2)".to_string(); // Placeholder expression
        infix = infix.split(' ').collect();

        // Filtered by lexeme
        let re = Regex::new(r"\w+|\d+|/|\+|-|\*|\^|√|\(|\)|\[|\]|\{|\}").unwrap(); //Accepts characters so it can warn about their usage at is_right()
        let matches: Vec<&str> = re.find_iter(&infix).map(|m| m.as_str()).collect();

        println!("{:?}", matches);

        match is_right(&matches) {
            Ok(true) => {
                input_flag = true;
                println!("Expression matches!")
            }

            Err(e) => {
                println!("{}", format!("{e}. Try again."));
            }

            Ok(false) => { /*Nothing here*/ }
        };
    }

}
struct PermittedDelimiters {
    start_del: &'static str,
    start_counter: u8,
    
    end_del: &'static str,
    end_counter: u8,

    is_match: bool,

    name: &'static str
}

// Checks if a given expression is correctly written (must be a vector of lexemes without whitespaces)
fn is_right(expression: &[&str]) -> Result<bool, String> {

    let mut parenthesis = PermittedDelimiters{ start_del: "(", end_del: ")", start_counter: 0, end_counter: 0, is_match: false, name: "parenthesis" };
    let mut brackets = PermittedDelimiters{ start_del: "[", end_del: "]", start_counter: 0, end_counter: 0, is_match: false, name: "brackets" };
    let mut braces = PermittedDelimiters{ start_del: "{", end_del: "}", start_counter: 0, end_counter: 0, is_match: false, name: "braces" };

    let expression_size = expression.len();

    let mut delimiters: [&mut PermittedDelimiters; 3] = [&mut parenthesis, &mut brackets, &mut braces];

    for del in delimiters.iter_mut() {
        for (index, lexeme) in expression.iter().enumerate() {

            // Checks for unclosed delimiters
            if *lexeme == del.start_del {
                del.start_counter += 1;
            }
    
            if *lexeme == del.end_del {
                del.end_counter += 1;
            }

            if del.start_counter == del.end_counter {
                del.is_match = true;
            }

            else {
                del.is_match = false;
            }

            // Checks for missing operand and repeated operators:

            // Checks if the current lexeme is an operator
            if ["/", "+", "-", "*", "^", "√"].contains(&lexeme) {
                // If the next lexeme is also an operator, throws an Error (avoids next-lexeme higher than slice size)  
                if (index + 1) <= (expression_size) && ["/", "+", "-", "*", "^", "√"].contains(&expression[index + 1]) {
                    
                    let mut sqrt_case: &str = "";

                    // If the next lexeme after an operand is √, it means that it is being ambiguous about 2√x
                    if *lexeme == "√" {
                        sqrt_case = ". Please use n√x notation, where n, x ∈ ℕ. 2√x for square root.\n"
                    }
                    
                    return Err(format!("missing operand, or duplicated operator, at {}", expression[index+1]) + sqrt_case)  
                }
            }
        }
    }

    // Only has to check for delimiters matching because operand/operators return case is already handled as Err()
    if parenthesis.is_match && brackets.is_match && braces.is_match {
        return Ok(true)
    }

    return Err("unclosed delimiter".to_string())   
}

// Checks if a given &str is numeric
fn is_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}
    
/*
entregar programa en eq:
cuando el usuario le ponga una expresion infija
1. evaluar si la expresion esta correctamente escrita
2. si esta correctamente escrita, debe de mostrarme su notacion (dar en prefija postfija infija)
3.- evaluar la expresion (en postfija obvio)
4.- debe permitir agrupadores (parentesis, corchete, llave) [DONE]
5.- permitir suma resta division multiplicacion potencia raiz [DONE]
6.- permitir numeros de N digitos [DONE]
7.- debe dar igual si tienen o no espacios [DONE]

usar re.findall [DONE]
*/

