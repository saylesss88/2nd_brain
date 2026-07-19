# References

There are two kinds of references:

- Shared reference: `&`
- Mutable reference: `&mut`

## Aliasing

Variables and pointers _alias_ if they refer to overlapping regions of memory.

Wont compile:

```rs
let mut data = vec![1, 2, 3];
let x = &data[0];
data.push(4);
println!("{}", x);
```

You can't have a shared reference at line 2, and a mutable reference at the same
time at line 3. This would create an aliased mutable reference.

The fix:

```rs
let mut data = vec![1, 2, 3];
let x = &data[0];
println!("{}", x);
// This is OK, x is no longer needed
data.push(4);
```

However, if the value has a destructor, the destructor is run at the end of the
scope. And running the destructor is a use, the last one. So this won't compile:

```rs
#[derive(Debug)]
struct X<'a>(&'a i32);

impl Drop for X<'_> {
    fn drop(&mut self) {}
}

let mut data = vec![1, 2, 3];
let x = X(&data[0]);
println!("{:?}", x);
data.push(4);
// Here, the destructor is run and therefore this'll fail to compile.
```
