# Defining an Enum

Where structs give you a way of grouping together related fields and data, enums
give you a way of saying a value is one of a possible set of values.

Any IP address can be either a version four or a version six address, but not
both at the same time. That property of IP addresses makes the enum data
structure appropriate because an enum value can only be one of its variants.
Both version four and version six addresses are still fundamentally IP
addresses, so they should be treated as the same type when the code is handling
situations that apply to any kind of IP address.

We can express this concept in code by defining an IpAddrKind enumeration and
listing the possible kinds an IP address can be, V4 and V6. These are the
variants of the enum:

```rs
enum IpAddrKind {
  V4,
  V6,
}
```

`IpAddrKind` is now a custom data type that we can use elsewhere in our code.

## Enum Values

We can create instances of each of the two variants of `IpAddrKind` like this:

```rs
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

The variants of the enum are namespaced under its identifier, and we use a
double colon to separate the two. This is useful because now both values
`IpAddrKind::V4` and `IpAddrKind::V6` are of the same type: `IpAddrKind`. We can
then define a function that takes any `IpAddrKind`:

```rs
fn route(ip_kind: IpAddrKind) {}
```

And we call this function with either variant:

```rs
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Currently, we don't have a way to store the actual IP address _data_; we only
know what kind it is.

We could solve this with structs:

```rs
  enum IpAddrKind {
          V4,
          V6,
      }

  struct IpAddr {
      kind: IpAddrKind,
      address: String,
  }

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };
```

We could solve this with enums in a more concise way:

```rs
enum IpAddr {
  V4(String),
  V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

We attach data to each variant of the enum directly, so there is no need for an
extra struct.

We can also see that the name of each enum variant we defined also becomes a
function that constructs an instance of the enum. `IpAddr::V4()` is a function
call that takes a `String` argument and returns an instance of the `IpAddr`
type.

Each variant can have different types and amounts of associated data. Version
four IP addresses will always have four numeric components that will have values
between 0 and 255.

We could do this instead:

```rs
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

- [stdlib Enum IpAddr](https://doc.rust-lang.org/std/net/enum.IpAddr.html)

The standard library's implementation is similar to this:

```rs
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

## Wide Variety of Types in an Enum

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

This is similar to the following struct definitions, with the main difference
being all of the enums variants grouped together under the same `Message` type.

```rs
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

But if we used the different structs, each of which has its own type, we
couldn’t as easily define a function to take any of these kinds of messages as
we could with the `Message` enum.
