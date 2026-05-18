# Rc<T>, the Reference-Counted Smart Pointer

You have to enable multiple ownership explicitly by using the Rust type `Rc<T>`,
which is an abbreviation for _reference counting_. The `Rc<T>` type keeps track
of the number of references to a value to determine whether or not the value is
still in use. If there are zero references to a value, the value can be cleaned
up without any references becoming invalid.

We use the `Rc<T>` type when we want to allocate some data on the heap for
multiple parts of our program to read and we can’t determine at compile time
which part will finish using the data last. If we knew which part would finish
last, we could just make that part the data’s owner, and the normal ownership
rules enforced at compile time would take effect.

> [!NOTE] `Rc<T>` is only for use in single-threaded scenarios.

## Sharing Data


