// fn is_reserved_symbol(symbol: String) {}

// pub fn tokenize(text: String) {
//     let mut multi = String::new();
//
//     for c in text.chars() {
//         match c {
//             '+'|'-'|'('|')'|'*'|'^' => println!("Reserved_symbol: {}", c),
//             x if x.is_digit(10) => multi.push(x),
//             _ => continue,
//         }
//     }
// }

pub fn tokenize(text: String) {
    let mut tab = String::new();
    for c in text.chars() {
        match c {
            '+' | '-' | '*' | '/' | '(' | ')' | '"' => match tab.is_empty() {
                true => println!("Reserved symbol: {}", c),
                false => {
                    if let Ok(_test) = tab.parse::<f64>() {
                        println!("Double: {}", tab);
                        tab = "".to_string();
                        println!("Reserved symbol: {}", c);
                    } else {
                        tab.push(c);
                    }
                }
            },
            x if x.is_digit(10) => tab.push(x),
            '|' => match tab.is_empty() {
                true => {
                    tab.push(c);
                    println!("Tab: {}", tab);
                    println!("is_empty: {}", tab.is_empty());
                }
                false => {
                    if let Ok(_test) = tab.parse::<f64>() {
                        println!("Double: {}", tab);
                        tab = "".to_string();
                    }
                    tab.push(c);
                    println!("String: {}", tab);
                    tab = "".to_string();
                }
            },
            '.' => tab.push('.'),
            y => match tab.is_empty() {
                false => {
                    if let Ok(_test) = tab.parse::<f64>() {
                        println!("Double: {}", tab);
                        tab = "".to_string();
                    }
                    tab.push(y);
                }
                true => println!("Variable: {}", y),
            },
        }
    }
}
