use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
pub enum Component {
    Variable(char),
    Number(f32),
    Binary {
        operator: char,
        left: Box<Component>,
        right: Box<Component>
    },
    End
}

impl Component {
    // Converts component to a readable text
    pub fn to_text(&self) -> String {
        match self {
            Component::Variable(c) => c.to_string(),
            Component::Number(f) => f.to_string(),
            Component::Binary { operator, left, right } => {
                format!("{} {} {}", left.to_text(), operator, right.to_text())
            },
            _ => String::from("")
        }
    }

    // Tries to convert the component to a float if it is a number
    pub fn to_float(&self) -> Option<f32> {
        match self {
            Component::Number(f) => Some(*f),
            _ => None
        }
    }
}

fn is_operator(c: &char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '%' | '^' | '(' | ')' => true,
        _ => false
    }
}

fn is_digit(c: &char) -> bool {
    (*c >= '0' && *c <= '9') || *c == '.'
}

fn get_precedence(c: Option<&char>) -> i8 {
    match c {
        Some(c) => match c {
            '+' => 1,
            '-' => 1,
            '*' => 3,
            '/' => 3,
            '%' => 3,
            '^' => 5,
            _ => -1
        },
        None => -1
    }
}

fn parse_component(chars: &mut Peekable<Chars>) -> Component {
    let mut maybe_num = String::new();
    
    while let Some(c) = chars.peek() {
        if is_digit(c) { maybe_num.push(*c); }
        else if !maybe_num.is_empty() { break; }
        else if !is_operator(c) { return Component::Variable(*c); }
        chars.next();
    }

    if !maybe_num.is_empty() { return Component::Number(maybe_num.parse::<f32>().unwrap()); }
    Component::End
}

fn parse_binary(chars: &mut Peekable<Chars>, prev_prec: i8, left: Component) -> Component {
    let mut left = left;
    loop {
        while let Some(c) = chars.peek() {
            if is_operator(c) { break; }
            chars.next();
        }
        let c = chars.peek();
        let prec = get_precedence(c);

        if prec < prev_prec { return left; }

        let c = *c.unwrap();
        let mut right = parse_component(chars);

        let new_prec = get_precedence(chars.peek());
        if prec < new_prec {
            right = parse_binary(chars, prec + 1, right);
            if let Component::End = right { return Component::End }
        }

        left = Component::Binary { operator: c, left: Box::new(left), right: Box::new(right) };
    }
}

pub fn parse(chars_raw: &mut Peekable<Chars>) -> Component {
    let string = chars_raw.filter(|x| x != &' ').collect::<String>();
    let mut chars = string.chars().peekable();
    let left = parse_component(&mut chars);
    parse_binary(&mut chars, 0, left)
}