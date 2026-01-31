# Values

|                     | Types                          | Literals               |
| ------------------- | ------------------------------ | ---------------------- |
| Signed ints         | i8, i16, i32, i64, i128, isize | -10, 0, 1_000, 123_i64 |
| Unsigned ints       | u8, u16, u32, u64, u128, usize | 0, 123, 10_u16         |
| Floating point      | f32, f64                       | 3.14, -10.0e20, 2_f32  |
| Unicode scalar vals | char                           | 'a', 'α', '∞'          |
| Booleans            | bool                           | true, false            |

The types have widths as follows:

- `iN`, `uN`, and `Fn` are _N_ bits wide,
- `isize` and `usize` are the width of a pointer
- `char` is 32 bits wide,
- `bool` is 8 bits wide.
