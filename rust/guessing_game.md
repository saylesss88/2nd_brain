## Guessing Game

```rs
use std::io;

use rand::Rng;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..=100);

  println!("The secret number is: {secret_number}");

  println!("Please input your guess.");

  let mut guess = String::new();

  io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

  println!("You guessed: {guess}");
}
```

The `Rng` trait defines methods that random number generators implement, and
this trait must be in scope for us to use those methods.

Then we call the `rand::thread_rng` function that gives us the particular random
number generator we’re going to use: one that is local to the current thread of
execution and is seeded by the operating system.

Then we call the gen_range method on the random number generator. This method is
defined by the `Rng` trait that we brought into scope with the `use rand::Rng;`
statement.

The `gen_range` method takes a range expression as an argument and generates a
random number in the range. The kind of range expression we’re using here takes
the form `start..=end` and is inclusive on the lower and upper bounds, so we
need to specify `1..=100` to request a number between 1 and 100.

> ❗️ NOTE: You won't know which traits to use and which methods and functions to
> call from a crate, so each crate has documentation with instructions for using
> it. Use `cargo doc --open` to build documentation provided by all your
> dependencies locally and open it in your browser.

## Comparing the Guess to the Secret Number

```rs
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
  // --snip--

  println!("You guessed: {guess}");

  match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
  }
}
```

First we add another use statement, bringing a type called `std::cmp::Ordering`
into scope from the standard library. The Ordering type is another enum and has
the variants `Less`, `Greater`, and `Equal`. These are the three outcomes that
are possible when you compare two values.

The `cmp` method compares two values and can be called on anything that can be
compared. It takes a reference to whatever you want to compare with: here it’s
comparing `guess` to `secret_number`. Then it returns a variant of the
`Ordering` enum we brought into scope with the use statement. We use a match
expression to decide what to do next based on which variant of `Ordering` was
returned from the call to `cmp` with the values in guess and `secret_number`.
