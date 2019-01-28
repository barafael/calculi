use std::collections::HashMap;

use super::parser;
use super::parser::Component;

#[derive(Debug)]
pub struct Equation {
    text: String,
    expression: Component
}

impl Equation {
    pub fn new<T: Into<String>>(text: T) -> Equation {
        let text = text.into();
        let expression = parser::parse(&text);
        Equation { text, expression }
    }

    // Attempt to solve component with given variables
    fn solve_component(vars: &HashMap<char, f32>, component: &Component) -> Component {
        match component {
            // Attempt to retrieve variable value
            Component::Variable(c) => {
                if vars.contains_key(c) {
                    Component::Number(vars[c])
                } else {
                    Component::Variable(*c)
                }
            },

            Component::Number(f) => Component::Number(*f),

            // Attempt to solve binary component
            Component::Binary { operator, left, right } => {
                // Retrieve value of left and right component
                let left = Self::solve_component(vars, left);
                let right = Self::solve_component(vars, right);

                // Apply operator to components if they are both numbers
                if let (Component::Number(f1), Component::Number(f2)) = (&left, &right) {
                    return match operator {
                        '+' => Component::Number(f1 + f2),
                        '-' => Component::Number(f1 - f2),
                        '*' => Component::Number(f1 * f2),
                        '/' => Component::Number(f1 / f2),
                        '%' => Component::Number(f1 % f2),
                        '^' => Component::Number(f1.powf(*f2)),
                        _ => Component::End
                    };
                }

                // Attempt to simplify current binary component that still contains unknown variables
                else if let (Component::Binary { operator: op, left: l, right: r }, Component::Number(f)) = (&left, &right) {
                    if (*op == '+' || *op == '-') && (*operator == '+' || *operator == '-') {
                        let mut sum = 0.0;
                        let mut var = ' ';
                        let mut pos_left = false;

                        // Apply number to external number
                        if let (Component::Number(f1), Component::Variable(c)) = (&**l, &**r) {
                            var = *c;
                            match (op, operator) {
                                ('+', '-') => sum = f1 - f,
                                ('-', '-') => sum = -f1 + f,
                                ('+', '+') => sum = f1 + f,
                                ('-', '+') => sum = -f1 - f,
                                _ => ()
                            }
                        } else if let (Component::Variable(c), Component::Number(f1)) = (&**l, &**r) {
                            var = *c;
                            pos_left = true;
                            match (op, operator) {
                                ('+', '-') => sum = f1 - f,
                                ('-', '-') => sum = -f1 - f,
                                ('+', '+') => sum = f1 + f,
                                ('-', '+') => sum = -f1 + f,
                                _ => ()
                            }
                        }

                        // Return the correct syntax in a binary component
                        return if sum == 0.0 && var != ' ' {
                            Component::Variable(var)
                        } else if pos_left {
                            Component::Binary { operator: if sum < 0.0 { '-' } else { '+' }, left: Box::new(Component::Variable(var)), right: Box::new(Component::Number(sum.abs())) }
                        } else {
                            Component::Binary { operator: if sum < 0.0 { '-' } else { '+' }, left: Box::new(Component::Number(sum.abs())), right: Box::new(Component::Variable(var)) }
                        }
                    }
                }
                
                // Return original binary component if simplifying failed
                Component::Binary { operator: *operator, left: Box::new(left), right: Box::new(right) }
            },
            _ => Component::End
        }
    }

    // Solve component with an unknown variable for given outcome algebraically
    fn solve(expr: Component, outcome: f32) -> (Component, f32) {
        match expr {
            Component::Variable(c) => (Component::Variable(c), outcome),
            Component::Number(f) => (Component::Number(f), outcome),

            // Attempt to apply algebraic rules to binary component if it contains a number
            Component::Binary { operator, left, right } => {
                let mut maybe_num = None;
                let mut pos_left = false;

                // Retrieve possible number from binary component
                if let Component::Number(f) = *left {
                    maybe_num = Some(f);
                    pos_left = true;
                } else if let Component::Number(f) = *right {
                    maybe_num = Some(f);
                }

                // Inverts operator so the unknown component gets closer to a solution
                // Example:
                // 5 * x - 3 = 7        : unsolved
                // 5 * x = 7 + 3 = 10   : solve 1
                // x = 10 / 5 = 2       : solve 2
                if let Some(f) = maybe_num {

                    // Get unkown component
                    (if pos_left { *right } else { *left }, 
                    
                    // Get new outcome
                    match operator {
                        '+' => outcome - f,
                        '-' => {
                            if pos_left {
                                f - outcome
                            } else {
                                outcome + f
                            }
                        },
                        '*' => outcome / f,
                        '/' => {
                            if pos_left {
                                f / outcome
                            } else {
                                outcome * f
                            }
                        },
                        '^' => {
                            if pos_left {
                                outcome.log(f)
                            } else {
                                outcome.powf(1.0 / f)
                            }
                        }
                        _ => outcome
                    })
                } else {
                    (Component::Binary { operator, left: Box::new(*left), right: Box::new(*right) }, outcome)
                }
            },
            _ => (Component::End, 0.0)
        }
    }

    // Get output of function with given variables
    pub fn solve_with(&self, vars_raw: Vec<(char, f32)>) -> Component {
        let mut vars = HashMap::new();

        // Collect variables into a HashMap
        vars_raw.iter().for_each(|(c, f)| { vars.insert(*c, *f); });
        Self::solve_component(&vars, &self.expression)
    }

    // Solve equation with an unknown variable for given outcome algebraically
    pub fn solve_for(&self, outcome: f32, vars: Vec<(char, f32)>) -> (Component, f32) {
        let mut expr = Self::solve(self.solve_with(vars), outcome);

        // Attempt to apply algebra while a binary component appears
        while let Component::Binary { .. } = &expr.0 {
            expr = Self::solve(expr.0, expr.1);
        }

        expr
    }
}