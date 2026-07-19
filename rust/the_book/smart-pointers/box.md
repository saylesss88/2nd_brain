# Box<T>

Boxes allow you to store data on the heap rather than the stack. What remains
on the stack is the pointer to the heap data.

You'll use a Box most often in these situatiions:

- When you have a type whose size cannot be known at compile time and you want to use a value of that type in a context that requires an exact size.

- When you have a large amount of data and you want to transfer ownership but ensure that the data won't be copied when you do

- When you want to own a value and you care only that it is a value of a particular type rather than a specific type.

## Storing Data on the Heap

```rs
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```

`b` is a pointer to the data on the heap (`5`), and when `b` goes out of scope, the memory will be automatically cleaned up. The deallocation happens both for the box (stored on the
stack) and the data it points to (stored on the heap).

## Enabling Recursive Types

A value of a recursive type has to be stored on the heap because the compiler needs to know how much space a type takes up. If a type is recursive, it would take up an infinite amount of space. To get around this, we can use a `Box<T>` to store the recursive part of the type on the heap.


### Cons List

A _cons list_ is made up of nested pairs, where the first element of the pair is the value and the second element is a pointer to the next pair. The last pair in the list points to `Nil`, which indicates that there are no more elements in the list.

psudocode representation of a cons list containing the list `1, 2, 3`:

```
(1, (2, (3, Nil)))
```

Each item in a cons list contains two elements: the value and a pointer to the next item in the list. The last item in the list points to `Nil`, which indicates that there are no more items in the list.

```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```
