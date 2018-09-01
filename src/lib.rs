#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub mod bindings;

pub fn sum_of_squares(input: &Vec<i32>) -> i32 {
    input.iter().map(|x| x*x).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
