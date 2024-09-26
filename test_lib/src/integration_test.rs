use named_params::named_params;

#[named_params]
fn some_fn(foo: usize, bar: isize) -> usize {
    foo + (bar as usize)
}

#[test]
fn test_everything() {
}
