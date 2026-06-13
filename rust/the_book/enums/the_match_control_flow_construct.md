# The match Control Flow Construct

`match` lets you compare a value against a series of patterns and then execute
code based on which pattern matches.

The power of `match` comes from the expressiveness of the patterns and the fact
that the compiler confirms that all possible cases are handled. The following
function takes an unknown US coin and, determines which coin it is and returns
its value in cents:

```rs
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```

We list the `match` keyword followed by an expression, which in this case is the
value `coin`. This seems similar to an `if` expression, but with `if`, the
condition needs to evaluate to a Boolean value, here it can be any type.

Next are the `match` arms. An arm has two parts: a pattern and some code. The
first arm has a pattern that is the value `Coin::Penny` and the `=>` operator
that separates the pattern and the code to run. The code in this case is just
the value `1`. Each arm is separated from the next with a comma.

When the match expression executes, it compares the resultant value against the
pattern of each arm, in order. If a pattern matches the value, the code
associated with that pattern is executed. If that pattern doesn’t match the
value, execution continues to the next arm, much as in a coin-sorting machine.

The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the
value that gets returned for the entire `match` expression.

You can also add curly brackets after the `=>` operator to
include multiple lines of code:

```rs
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Patterns That Bind to Values

Another useful feature of match arms is that they can bind to
the parts of the values that match the pattern. This is how
we extract values out of enum variants.

Let's change one of our enum variants to hold data inside it.

Only Quarters got designs of the states between 1999 through 2008. We can add this info to our `enum` by changing our `Quarter` variant to include a `UsState` value stored inside it:
```rs
#[derive(Debug)]  // so we can inspect the state shortly
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}
```


Let’s imagine that a friend is trying to collect all 50 state quarters. While we sort our loose change by coin type, we’ll also call out the name of the state associated with each quarter so that if it’s one our friend doesn’t have, they can add it to their collection.

In the match expression for this code, we add a variable called state to the pattern that matches values of the variant Coin::Quarter. When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state. Then, we can use state in the code for that arm, like so:

```rs
fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {state:?}!");
      25
    }
  }
}
```

A semi-workingn program:

```rs
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let quarter = Coin::Quarter(UsState::Alaska);

    value_in_cents(quarter);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

Better example: Say we wanted to give a special bonus for your
home state. If you wanted to look _inside_ the `UsState` enum,
you can destructure it at the same exact time.

You do this by replacing the generic `state` variable with the
specific `UsState` variants inside the match arm:

```rs
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        
        // Look closely here: we are destructuring BOTH enums at once!
        Coin::Quarter(UsState::Alaska) => {
            println!("Wow, an Alaska quarter! That's worth extra to me.");
            30 // Bonus cents!
        },
        
        // This catches all other states if it wasn't Alaska
        Coin::Quarter(any_other_state) => {
            println!("A regular state quarter from {any_other_state:?}.");
            25
        }
    }
}
```
