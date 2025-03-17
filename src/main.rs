use regex::Regex;

fn main() {
    // Extract lexemes from data:

    // Removed whitespace from infix
    let mut infix: String = "(3+ 4545 / 3) * {64} / 1 + [2] (33 33 - 2)".to_string();    
    infix = infix.split(' ').collect();

    // Filtered by lexeme
    let re = Regex::new(r"\d+|/|\+|-|\*|\^|√|\(|\)|\[|\]|\{|\}").unwrap();
    let matches: Vec<_> = re.find_iter(&infix).map(|m| m.as_str()).collect();

    println!("{:?}", matches);

}


/*
entregar programa en eq:
cuando el usuario le ponga una expresion infija
1. evaluar si la expresion esta correctamente escrita
2. si esta correctamente escrita, debe de mostrarme su notacion (dar en prefija postfija infija)
3.- evaluar la expresion (en postfija obvio)
4.- debe permitir agrupadores (parentesis, corchete, llave) [DONE]
5.- permitir suma resta division multiplicacion potencia raiz
6.- permitir numeros de N digitos
7.- debe dar igual si tienen o no espacios [DONE]

usar re.findall [DONE]
*/