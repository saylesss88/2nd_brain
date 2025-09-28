---
id: Traits: Defining Shared Behavior
aliases: [generics]
---

# Traits

A _trait_ defines the functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use _trait bounds_ to specify that a generic type can be any type that has certain behavior.

## Defining a Trait

A type's behavior consists of the methods we can call on that type.

We want to make a media aggregator library crate named `aggregator` that can display summaries of data that might be stored in a `NewsArticle` or `Tweet` instance.

To do this, we need a summary from each type, and we'll request that summary by calling a `summarize` method on an instance.

- This shows an implementation of the `Summary` trait on the `NewsArticle` struct
  that uses the headline, the author, and the location to create the return value
  of `summarize`.

- For the `Tweet` struct, we define `summarize` as the username followed by the
  entire text of the tweet, assuming that the tweet content is already limited
  to 280 characters.

```rust src/lib.rs
pub trait Summary {
  fn summarize(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}
```

- The difference in implementing a trait on a type is that after `impl`, we put the trait name we want to implement, then use the `for` keyword, and then specify the name of the type we want to implement the trait for.

  - Inside the `impl` block, we put the method signatures that the trait definition has defined.

  - Instead of adding a semicolon after each signature, we use curly brackets and fill in the method body with the specific behavior that we want the methods of the trait to have for the particular type.

Now that the library has implemented the `Summary` trait on `NewsArticle` and `Tweet`, users of the crate can call the trait methods on instances on them in the same way we call regular methods.

The only difference is that you need to bring the trait into scope as well as the types. Here's an example of how a binary crate could use our `aggregator` library crate.

```rust src/main.rs
use aggregator::{Summary, Tweet};

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
      "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());
}
```

- One restriction is that we can implement a trait on a type only if either the trait or the type, or both, are local to our crate.

  - This restriction is part of a property called _coherence_, and more specifically the _orphan rule_, so named because the parent type is not present. It ensures other peoples code can't break your code and vice versa.

  - Without the rule, two crates could implement the same trait for the same type, and Rust wouldn't know which implementation to use.

## Default Implementations

Sometimes it's useful to have default behavior for some or all of the methods in a trait instead of requiring Implementations for all the methods on every type. Then, as we implement the trait on a particular type, we can keep or override each method's default behavior.

Here, we specify a default string for the `summarize` method of the `Summary` trait instead of only defining the method signature:

```rust src/lib.rs
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}
```

- To use a default implementation to summarize instances of `NewsArticle`, we specify an empty `impl` block with `impl Summary for NewsArticle {}`.

- We can still call `summarize` on an instance of `NewsArticle`, like this:

```rust
use aggregator::{self, NewsArticle, Summary};

fn main() {
  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best \
      hocky team in the NHL.",
    ),
  };

  println!("New article available! {}", article.summarize());
}
```

Output:

`New article available! (Read more...)`

Default implementations can call other methods in the same trait, even if those other methods don't have an implementation.

For example, we could define the `Summary` trait to have a `summarize_author` method whose implementation is required, and then define a `summarize` method that has a default implementation that calls the `summarize_author` method:

```rust
pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}
```

To use this version of `Summary`, we only need to define `summarize_author` we implement the trait on a type:

```rust
impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}
```

Because we’ve implemented summarize_author, the Summary trait has given us the behavior of the summarize method without requiring us to write any more code. Here’s what that looks like:

```rust
use aggregator::{self, Summary, Tweet};

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
      "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());
}
```

### Traits as Parameters

We can use traits to define functions that accept many different types.

We can add this to the end of our program with `NewsArticle` and `Tweet`:

```rust
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
```
- This parameter accepts any type that implements the specified trait, such as `summarize`. We can call `notify` and pass in any instance of `NewsArticle` or `Tweet`.
- Code that calls the function with any other type won't compile because those types don't implement `Summary`

[[trait_bound_syntax.md]]
