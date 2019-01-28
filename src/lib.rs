pub mod parser;
pub mod equation;

#[cfg(test)]
mod tests {
    use super::equation;
    use super::parser;

    #[test]
    fn it_works() {
        let eq = equation::Equation::new("x - 2 * a + 4 ^ b");

        assert_eq!(eq.solve_with(vec![('x', 10.0), ('a', 4.5), ('b', 1.0)]).to_float().unwrap(), 5.0);
        assert_eq!(eq.solve_for(10.0, vec![('a', 4.5), ('b', 1.0)]).1, 15.0);
        assert_eq!(equation::Equation::new("4 ^ x * 3").solve_for(192.0, vec![]).1, 3.0);
    }

    #[test]
    fn test_to_string() {
        let expression = "x + y * 3 - z ^ 7";
        let component = parser::parse(expression);

        assert_eq!(expression, component.to_string());
    }
}
