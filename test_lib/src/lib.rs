use named_params::named_params;

#[named_params]
fn add_numbers(foo: usize, bar: usize) -> usize {
    foo + bar
}

#[named_params]
fn append_elements(string: &str, slice: &[usize]) -> String {
    let mut result = string.to_string();
    for number in slice {
        result = format!("{}, {}", result, number);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_everything() {
        assert_eq!(add_numbers(AddNumbersArgs { foo: 2, bar: 3 }), 5);
        assert_eq!(append_elements(AppendElementsArgs { string: "foo", slice: &[1, 2, 3] }), "foo, 1, 2, 3");
    }
}
