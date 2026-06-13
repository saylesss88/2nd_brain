# The Option Enum

The `Option` type encodes the very common scenario in which a value could be
something, or it could be nothing.

Rust does not have nulls, but it does have an enum that can encode the concept
of a value being present or absent. This enum is `Option<T`, and it is defined
by the standard library as follows:

```rs
enum Option<T> {
  None,
  Some(T),
}
```

- The `Option<T>` enum is just a regular enum, and `Some(T)` and `None` are its
  variants.

- `<T>` means that the `Some` variant of the `Option` enum can hold one piece of
  data of any type, and that each concrete type that gets used in place of `T`
  makes the overall `Option<T>` type a different type.

```rs
let some_number = Some(5);
let some_char = Some('c');

let absent_number: Option<i32> = None;
```

The type of `some_number` is `Option<i32>`. The type of `some_char` is
`Option<char>`, which is a different type.

Rust can infer these types because we've specified a value inside the `Some`
variant.

For `absent_number`, Rust requires us to annotate the overall `Option` type: The
compiler can't infer the type that the corresponding `Some` variant will hold by
looking only at a `None` value.

`Option<T>`, and `T` are different types, the compiler won't let us use an
`Option<T>` value as if it were definitely a valid value. The following code
won't compile because it's trying to add an `i8` to an `Option<i8>`:

```rs
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
```

Rust doesn't understand how to add an `i8` and an `Option<i8>`,because they're
different types. When we have an `Option<i8>`, the compiler will make sure we
handle the all cases before using the value.

In other words, we have to convert an `Option<T>` to a `T` before you can
perform `T` operations with it.

In order to have a value that can possibly be null, you must explicitly opt in
by making the type of that value `Option<T>`. Then, when you use that value, you
are required to explicitly handle the case when the value is null. Everywhere
that a value has a type that isn’t an `Option<T>`, you can safely assume that
the value isn’t null.

So how do you get the `T` value out of a `Some` variant when you have a value of
type `Option<T>` so that you can use that value? The `Option<T>` enum has a
large number of methods that are useful in a variety of situations; you can
check them out in
[its documentation](https://doc.rust-lang.org/std/option/enum.Option.html).
Becoming familiar with the methods on `Option<T>` will be extremely useful in
your journey with Rust.

## Unwrapping Options

1. The Safe Way: Provide a Default (`unwrap_or`)

If the option is `None`, you can provide a fallback value. This is usually the
best approach because it never crashes your program.

```rs
let liquid: Option<&str> = None;

// If it's None, it falls back to "water"
let dring = liquid.unwrap_or("water");
```

If calculating the default value takes a lot of CPU power or memory, use
unwrap_or_else. It takes a closure (an anonymous function) that only runs if it
absolutely has to:

```rs
let user_input: Option<String> = None;
let data = user_input.unwrap_or_else(|| expensive_database_call());
```

2. The Idiomatic Way: Pattern Matching (`if let` or `match`)

If you only want to run code when the value actually exists, use `if let`. This
safely extracts the value into a new variable inside that block.

```rs
let secret_code: Option<i32> = Some(42);

if let Some(code) = secret_code {
  println!("The code is: {}", code); // 'code' is a normal i32 here
}
```

If you need to handle both the `Some` and `None` cases explicitly, use a full
`match` statement:

```rs
let survival_rating = match secret_code {
  Some(code) => code * 2,
  None => 0,
};
```

3. The Early Return Way: The `?` Operator or `let-else`

If you're inside a function and want to exit early if the value is `None`, you
have two great choices.

The `?` operator (unwraps the value, or returns `None` from the whole function):

```rs
fn get_username() -> Option<String> {
  let raw_input: Option<String> = Some("Ferris".to_string());

  // If raw_input is None, the function stops here and returns None
  let name = raw_input?;

  Some(name.to_uppercase())
}
```

The `let-else` statement (lets you write an explicit `return`, `break`, or
`panic` if it fails):

```rs
fn process(input: Option<i32>) {
  let Some(value) = input else {
    return;  // Muse exit the current block
  };

  // 'value' is now a normal i32 for the rest of this function
  println!("Processing {value}");
}
```

4. The Dangerous Way: Direct `unwrap`

You can force Rust to give you the value using `.unwrap()` or `.expect()`.

> ⚠️ Warning: If the option is None, your entire program will panic (crash).

```rs
let gold: Option<&str> = Some("shiny");
let item = gold.unwrap(); // Works fine, item is "shiny"

let empty: Option<&str> = None;
let item2 = empty.unwrap(); // 💥 CRASH! Program panics.
```

If you must use this because you are 100% sure it's impossible for the value to
be `None`, prefer `.expect("your custom error message")` so it's easier to debug
when it crashes.

## Resources

- [std::option Enum Option](https://doc.rust-lang.org/std/option/enum.Option.html)

- [Option Enum module level docs](https://doc.rust-lang.org/std/option/index.html)

- [The book Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
