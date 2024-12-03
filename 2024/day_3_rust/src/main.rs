use regex::{Captures, Regex};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Debug)]
enum MulCreationError {
    Captures,
    NumericConverion,
}

#[derive(Debug)]
struct Mul {
    a: u32,
    b: u32,
}

impl Mul {
    fn new_from_cap(cap: Captures<'_>) -> Result<Self, MulCreationError> {
        if let (Some(a_raw), Some(b_raw)) = (cap.get(1), cap.get(2)) {
            if let (Ok(a), Ok(b)) = (a_raw.as_str().parse(), b_raw.as_str().parse()) {
                return Ok(Self { a, b });
            }
            return Err(MulCreationError::NumericConverion);
        }
        Err(MulCreationError::Captures)
    }

    fn calculate(&self) -> u32 {
        self.a * self.b
    }
}

fn parse_line_to_mul_list(s: &str) -> Vec<Mul> {
    let mut result = Vec::new();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for cap in re.captures_iter(s) {
        if let Ok(mul) = Mul::new_from_cap(cap) {
            result.push(mul)
        }
    }

    result
}

fn parse_line_to_mul_list_v2(s: &str) -> Vec<Mul> {
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don\'t\(\))").unwrap();
    let mut new_s: String = String::new();

    let mut enabled = true;
    for cap in re.captures_iter(s) {
        let c = &cap[0];
        if enabled && c.starts_with("mul(") {
            new_s.push_str(c)
        }
        if c == "do()" {
            enabled = true;
        }
        if c == "don't()" {
            enabled = false;
        }
    }

    parse_line_to_mul_list(&new_s)
}

fn sum_mul_list_results(mul_list: &Vec<Mul>) -> u32 {
    let mut sum = 0;
    for mul in mul_list {
        sum += mul.calculate();
    }
    sum
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut program: String = String::new();
    for line in reader.lines().filter_map(Result::ok) {
        program.push_str(&line);
    }

    let mul_list = parse_line_to_mul_list(&program);
    let mul_list_v2 = parse_line_to_mul_list_v2(&program);

    let sum = sum_mul_list_results(&mul_list);
    let sum_v2 = sum_mul_list_results(&mul_list_v2);
    println!("Sum 1: {}", sum);
    println!("Sum 2: {}", sum_v2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_to_mul_list_test() {
        assert_eq!(
            parse_line_to_mul_list(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            )
            .len(),
            4
        );
    }

    #[test]
    fn parse_line_to_mul_list_v2_test() {
        assert_eq!(
            parse_line_to_mul_list_v2(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            )
            .len(),
            2
        );
        assert_eq!(
            parse_line_to_mul_list_v2(
                "()who(537,994)/(?]mul(257,635)^when()don't()!what()}where()#:when()>'where()mul(360,222),when()@&<^mul(268,245){{)%:from()<#mul(936,776)$select()!mul(474,825)how()}~mul(484,39)!?:@[*<mul(357,805)how()mul(261,810) {$>mul(306,422)$when()when()/@$$!mul(944,563)<!%!(from()mul(47,642)#^(why()(}:$mul(403,781)mul(382,778)$%$-)mul(48,400)@{?select()-/%when())}mul(114,537)^$&{&select()+why(){/mul(688,466) ):mul(950,333)when()}what()when(){>'!%+mul(974,802)what(291,78)*mul(394,250)why()<;mul(271,377)how()%**@who()*from()mul(569,753))who()*^mul(83,470)#-{from()when()-;mul(678,845)where()><;]$do()>what()(#)how() mul(958,48)select()from()<%mul(965,566)!};<where()mul(926,836)>*when()?)%do()}mul(465,920)>$%$when()mul(905,944)#$+~>mul(738,782)how()]&'{~from()-@-mul(671,73)#@'+when()(*mul(981,305)(],%~why()mul(943,403);*(& f",
            )
            .len(),
            10
        );
    }

    #[test]
    fn sum_mul_test() {
        assert_eq!(
            sum_mul_list_results(&parse_line_to_mul_list(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            )),
            161
        );
        assert_eq!(
            sum_mul_list_results(&parse_line_to_mul_list_v2(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            )),
            48
        );
    }
}
