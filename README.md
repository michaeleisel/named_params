## named\_params

#### Simple, fast named parameters for Rust functions

Named function parameters have been a much-requested feature for Rust, with various approaches for it. This crate is a proc macro crate that streamlines one particular approach, that of replacing the parameters of a function with a struct that has each parameter as a member (note that optional parameters are not currently supported, although they could be if people want it).

#### Motivation

Although there are many potential reasons for wanting to use named parameters, this section will focus on one specific example: avoiding misorderings of variables. For some Rust functions, it can be tricky to get the order of parameters right, e.g. if a function takes a large number of parameters that are all of the same type. For example, for the function `fn create_rect(x: usize, y: usize, width: usize, height: usize) { ... }`, where all 4 parameters have the same type, the type system won't catch misorderings of the parameters. It may be safer to change it to something like the following:
```
struct CreateRectArgs {
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

// Call this function with, e.g., create_rect(CreateRectArgs { x: 1, y: 2, width: 10, height: 20 });
fn create_rect(CreateRectArgs { x, y, width, height }: CreateRectArgs) {
    // ...
}
```

Now whenever someone calls it, they can see very clearly which arguments they pass in will be interpreted as which parameters. The downside of that approach is that it requires a lot of boilerplate. This crate, on the other hand, can do the above with far less boilerplate:
```
// Gets expanded to match the previous code block
#[named_params]
fn create_rect(x: usize, y: usize, width: usize, height: usize) { ... }
```

We see that the struct definition can be completely omitted. The crate also supports references, by tying the lifetime of the reference to that of the args struct:

```
#[named_params]
fn some_fn(some_str: &str, some_slice: &[u8]) { ... }
```
gets expanded to:
```
struct SomeFnArgs<'a> {
    some_str: &'a str,
    some_slice: &'a [u8]
}

fn some_fn(SomeFnArgs { some_str, some_slice }: SomeFnArgs) { ... }
```


#### License

This crate is licensed under the MIT license.
