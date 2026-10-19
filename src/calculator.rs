//! Small, bounded arithmetic parser. No code evaluation, shell or GTK dependencies.
//! Grammar: sum -> product -> unary -> power -> percentage -> number/parentheses.
#[derive(Debug, PartialEq)]
pub enum Evaluation {
    NotExpression,
    Incomplete,
    Result(String),
    Error(&'static str),
}

#[derive(Debug, PartialEq)]
enum ParseError {
    Incomplete,
    Invalid,
    DivisionByZero,
    OutOfRange,
    TooComplex,
}

type Parsed = Result<f64, ParseError>;

pub fn evaluate(input: &str) -> Evaluation {
    let input = input.trim();
    let explicit = input.starts_with('=');
    let expression = input.strip_prefix('=').unwrap_or(input).trim();
    // Names such as "7-Zip" or "1Password" still go through application search.
    let allowed = expression
        .chars()
        .all(|c| c.is_ascii_digit() || c.is_ascii_whitespace() || "+-*/^%().,eE×÷−".contains(c));
    let arithmetic = expression.chars().any(|c| "+-*/^%().,eE×÷−".contains(c));
    let starts_as_number =
        expression.starts_with(|c: char| c.is_ascii_digit() || "+-(.,−".contains(c));
    if !explicit && (!allowed || !arithmetic || !starts_as_number) {
        return Evaluation::NotExpression;
    }
    if expression.len() > 1024 {
        return Evaluation::Error("La expresión es demasiado larga");
    }
    if !allowed {
        return Evaluation::Error("Revisá la expresión matemática");
    }
    let expression = expression
        .replace('×', "*")
        .replace('÷', "/")
        .replace('−', "-")
        .replace(',', ".");
    let mut parser = Parser {
        input: expression.as_bytes(),
        position: 0,
    };
    let result = parser.sum(0).and_then(|value| {
        parser.spaces();
        if parser.position == parser.input.len() {
            Ok(value)
        } else {
            Err(ParseError::Invalid)
        }
    });
    match result {
        Ok(value) => Evaluation::Result(format_result(value)),
        Err(ParseError::Incomplete) => Evaluation::Incomplete,
        Err(ParseError::Invalid) => Evaluation::Error("Revisá la expresión matemática"),
        Err(ParseError::DivisionByZero) => Evaluation::Error("No se puede dividir por cero"),
        Err(ParseError::OutOfRange) => {
            Evaluation::Error("El resultado no es un número real finito")
        }
        Err(ParseError::TooComplex) => Evaluation::Error("La expresión es demasiado compleja"),
    }
}

struct Parser<'a> {
    input: &'a [u8],
    position: usize,
}

impl Parser<'_> {
    fn spaces(&mut self) {
        while self
            .input
            .get(self.position)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.position += 1;
        }
    }

    fn consume(&mut self, token: u8) -> bool {
        self.spaces();
        if self.input.get(self.position) == Some(&token) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn sum(&mut self, depth: usize) -> Parsed {
        let mut value = self.product(depth)?;
        loop {
            if self.consume(b'+') {
                value = finite(value + self.product(depth)?)?;
            } else if self.consume(b'-') {
                value = finite(value - self.product(depth)?)?;
            } else {
                return Ok(value);
            }
        }
    }

    fn product(&mut self, depth: usize) -> Parsed {
        let mut value = self.unary(depth)?;
        loop {
            if self.consume(b'*') {
                value = finite(value * self.unary(depth)?)?;
            } else if self.consume(b'/') {
                let divisor = self.unary(depth)?;
                if divisor == 0.0 {
                    return Err(ParseError::DivisionByZero);
                }
                value = finite(value / divisor)?;
            } else {
                return Ok(value);
            }
        }
    }

    fn unary(&mut self, depth: usize) -> Parsed {
        if depth > 64 {
            return Err(ParseError::TooComplex);
        }
        if self.consume(b'+') {
            self.unary(depth + 1)
        } else if self.consume(b'-') {
            Ok(-self.unary(depth + 1)?)
        } else {
            self.power(depth)
        }
    }

    fn power(&mut self, depth: usize) -> Parsed {
        let mut value = self.primary(depth)?;
        if self.consume(b'%') {
            value /= 100.0;
        }
        if self.consume(b'^') {
            value = finite(value.powf(self.unary(depth + 1)?))?;
        }
        Ok(value)
    }

    fn primary(&mut self, depth: usize) -> Parsed {
        if self.consume(b'(') {
            let value = self.sum(depth + 1)?;
            if !self.consume(b')') {
                return Err(self.expected());
            }
            return Ok(value);
        }
        self.spaces();
        let start = self.position;
        self.digits();
        let mut digits = self.position - start;
        if self.input.get(self.position) == Some(&b'.') {
            self.position += 1;
            let fraction = self.position;
            self.digits();
            digits += self.position - fraction;
        }
        if digits == 0 {
            return Err(self.expected());
        }
        if matches!(self.input.get(self.position), Some(b'e' | b'E')) {
            self.position += 1;
            if matches!(self.input.get(self.position), Some(b'+' | b'-')) {
                self.position += 1;
            }
            let exponent = self.position;
            self.digits();
            if self.position == exponent {
                return Err(self.expected());
            }
        }
        let number = std::str::from_utf8(&self.input[start..self.position])
            .map_err(|_| ParseError::Invalid)?
            .parse()
            .map_err(|_| ParseError::Invalid)?;
        finite(number)
    }

    fn digits(&mut self) {
        while self
            .input
            .get(self.position)
            .is_some_and(u8::is_ascii_digit)
        {
            self.position += 1;
        }
    }

    fn expected(&self) -> ParseError {
        if self.position == self.input.len() {
            ParseError::Incomplete
        } else {
            ParseError::Invalid
        }
    }
}

fn finite(value: f64) -> Parsed {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(ParseError::OutOfRange)
    }
}

// Twelve significant digits keep common floating-point artifacts out of the UI.
fn format_result(value: f64) -> String {
    if value == 0.0 {
        return "0".into();
    }
    let magnitude = value.abs().log10().floor() as i32;
    if !(-9..12).contains(&magnitude) {
        let scientific = format!("{value:.11e}");
        let (mantissa, exponent) = scientific.split_once('e').expect("formatted exponent");
        format!(
            "{}e{exponent}",
            mantissa.trim_end_matches('0').trim_end_matches('.')
        )
    } else {
        let decimals = (11 - magnitude).max(0) as usize;
        let text = format!("{value:.decimals$}");
        if text.contains('.') {
            text.trim_end_matches('0').trim_end_matches('.').into()
        } else {
            text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arithmetic_and_precedence() {
        for (input, expected) in [
            ("125 * 1.22", "152.5"),
            ("2+3*4", "14"),
            ("(2+3)*4", "20"),
            ("-2^2", "-4"),
            ("(-2)^2", "4"),
            ("2^3^2", "512"),
            ("2^-2", "0.25"),
            ("200*15%", "30"),
            ("1,5 + .5", "2"),
            ("8÷2×3", "12"),
            ("8-3-2", "3"),
            ("8/2/2", "2"),
            ("= 42", "42"),
            ("0.1+0.2", "0.3"),
            ("1e-12", "1e-12"),
            ("1e20", "1e20"),
            ("−0+0", "0"),
        ] {
            assert_eq!(
                evaluate(input),
                Evaluation::Result(expected.into()),
                "{input}"
            );
        }
    }
    #[test]
    fn app_names_are_not_calculations() {
        for input in [
            "",
            " ",
            "Firefox",
            "7-Zip",
            "1Password",
            "2048",
            "e",
            "code",
            "foo/bar",
        ] {
            assert_eq!(evaluate(input), Evaluation::NotExpression, "{input}");
        }
    }
    #[test]
    fn incomplete_input_is_distinct_from_errors() {
        for input in ["=", "1+", "(2+3", "2^", "1e-", "-", "2*("] {
            assert_eq!(evaluate(input), Evaluation::Incomplete, "{input}");
        }
        for input in [
            "1..2", "2**3", "2(3)", "1/0", "0/0", "1e999", "(-1)^.5", "=hello",
        ] {
            assert!(matches!(evaluate(input), Evaluation::Error(_)), "{input}");
        }
    }
    #[test]
    fn bounds_work_on_adversarial_input() {
        for input in [
            format!("{}1", "(".repeat(70)),
            format!("{}1", "-".repeat(70)),
            "2^".repeat(70),
            "1+".repeat(600),
        ] {
            assert!(matches!(evaluate(&input), Evaluation::Error(_)));
        }
    }
}
