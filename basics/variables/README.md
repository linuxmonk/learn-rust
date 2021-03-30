## Variables

In Rust by default all variables are immutable. This gives Rust the advantage to provide -

* Easy concurrency
* Safety

Assigning over an immutable variable prints such an error by the compiler -

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> vars.rs:5:5
  |
3 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: make this binding mutable: `mut x`
4 |     println!("The value of x is {}", x);
5 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0384`.
```

### Mutable vs. Immutable ?

There are multiple tradeoffs to consider in addition to prevention of bugs. For example in cases of large data structures having the datastructure immutable causes performance overheads because the large datastructure needs to be copied in order to be modified. But in smaller / basic data types these can be left immutable and create new values. Here a small penalty leads to better readability.

## Constants

### Difference between constants and variables

* In Rust variables are immutable by default. 
* Constants are always immutable. You cannot use `mut` with constants.
* You use `const` keyword to define constants.


Example -

```
const MAX_ITEMS: u32 = 100_000;
```

*NOTE* 
* Constants can be declared in any scope, local to function or global etc. 
* In Rust constants are by convention named with all upper case alphabets and underscore to separate the name.
* Constants must always be annotated with a type (as above).

## Shadowing

In Rust, you can shadow a previously declared variable. Example -


[Playpen link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=08ea74cb1591b7808bf509d33730f6fc) -
```
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x + 2;
    println!("x = {}", x);
}
```

This prints - 

```
x = 8
```

### Difference between `mut` and Shadowing

The difference between `mut` and shadowing is - 

If we specify `let mut x = 5;` subsequent operations can modify `x` without the use of `let`. In case of shadowing each use of `let` creates a new variable letting us use earlier value of `x` on the expression's rhs. Since we are creating a new variable with `let` it also let's us modify the type of the variable. For example -

[Playpen link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=d9959c3e8ad2291aa5819d6c6d0023c2)
```
fn main() {
    let spaces = "     "; // inferred type String
    println!("Spaces: [{}]", spaces);
    // spaces on rhs is the String spaces. spaces on lhs is u32 and shadows the earlier one.
    let spaces: usize = spaces.len(); 
    println!("# of spaces: {}", spaces);
}
```

This helps us from not having to declare variables like spaces_str, spaces_len etc.
