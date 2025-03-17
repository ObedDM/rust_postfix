use std::{collections::HashMap, io::stdin};

use regex::Regex;

fn main() {

    //Read user input:

    let mut infix: String = String::new();
    println!("Enter your expression (infix):");
    stdin().read_line(&mut infix).expect("Unable to read expression");
    
    // Extract lexemes from data:

    // Removed whitespace from infix
    let mut infix: String = "(3+ 4545 / 3) * {64} / 1 + [2] (33 33 - 2)".to_string(); // Placeholder expression
    infix = infix.split(' ').collect();

    // Filtered by lexeme
    let re = Regex::new(r"\d+|/|\+|-|\*|\^|√|\(|\)|\[|\]|\{|\}").unwrap();
    let matches: Vec<&str> = re.find_iter(&infix).map(|m| m.as_str()).collect();

    println!("{:?}", matches);

}
struct PermittedDelimiters {
    start_del: &'static str,
    start_counter: u8,
    
    end_del: &'static str,
    end_counter: u8,

    is_match: bool
}

fn is_right(expression: Vec<&str>) -> bool {

    let mut parenthesis = PermittedDelimiters{ start_del: "(", end_del: ")", start_counter: 0, end_counter: 0, is_match: false };
    let mut brackets = PermittedDelimiters{ start_del: "[", end_del: "]", start_counter: 0, end_counter: 0, is_match: false };
    let mut braces = PermittedDelimiters{ start_del: "{", end_del: "}", start_counter: 0, end_counter: 0, is_match: false };

    let mut delimiters: [&mut PermittedDelimiters;3] = [&mut parenthesis, &mut brackets, &mut braces];

    for lexeme in expression {
        for del in delimiters.iter_mut() {
            if lexeme == del.start_del {
                del.start_counter += 1;
            }

            if lexeme == del.end_del {
                del.end_counter += 1;
            }

            if del.start_counter == del.end_counter {
                del.is_match = true;
            }
        }
    }

    if parenthesis.is_match && brackets.is_match && braces.is_match {
        return true;
    }

    return false   
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