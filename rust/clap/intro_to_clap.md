# Intro to Clap

- [builder tutorial](https://docs.rs/clap/latest/clap/_tutorial/index.html)

```rs
use clap::{command, Arg, ArgMatches};

fn main() {
    let match_result: ArgMatches = command!()
        .arg(Arg::new("firstname"))
        .arg(Arg::new("lastname"))
        .get_matches();
}
```

These are called positional arguments because they don't have names (no short
name or long name).

We can add short and long flags for this with:

```rs
use clap::{command, Arg, ArgMatches};

fn main() {
    let match_result: ArgMatches = command!()
        .arg(Arg::new("firstname").short('f').long("first-name"))
        .arg(Arg::new("lastname").short('l').long("last-name"))
        .arg(Arg::new("fluffy").long("fluffy"))
        .get_matches();
}
```

## Hidden Aliases

```rs
use clap::{command, Arg, ArgMatches};

fn main() {
    let match_result: ArgMatches = command!()
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first-name")
                .aliases(["fname", "firstname"]),
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last-name")
                .aliases(["lname", "lastname"]),
        )
        .arg(Arg::new("fluffy").long("fluffy"))
        .get_matches();
}
```

## Mandatory arguments

```rs
use clap::{command, Arg, ArgMatches};

fn main() {
    let match_result: ArgMatches = command!()
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first-name")
                .aliases(["fname", "firstname"])
                .required(true),
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last-name")
                .aliases(["lname", "lastname"])
                .required(true),
        )
        .arg(Arg::new("fluffy").long("fluffy"))
        .get_matches();
}
```

## Built in Documentation

```rs
use clap::{Arg, ArgMatches, command};

fn main() {
    let match_result: ArgMatches = command!()
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first-name")
                .aliases(["fname", "firstname"])
                .required(true)
                .help("The person's first name"),
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last-name")
                .aliases(["lname", "lastname"])
                .required(true)
                .help("This argument takes the person's last name"),
        )
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
                .help("Is the person wearing a fluffy hat or not"),
        )
        .get_matches();
}
```

## Conflicting Arguments

```rs
.conflcts_with("lastname")
```

```sh
cr -- -f T -l S
error: the argument '--first-name <firstname>' cannot be used with '--last-name <lastname>'
```

## Adding About

```rs
use clap::{Arg, ArgMatches, command};

fn main() {
    let match_result: ArgMatches = command!()
        .about("This app registers people with their doctors office.")
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first-name")
                .aliases(["fname", "firstname"])
                .required(true)
                .help("The person's first name")
                .conflicts_with("lastname"),
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last-name")
                .aliases(["lname", "lastname"])
                .required(true)
                .help("This argument takes the person's last name"),
        )
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
                .help("Is the person wearing a fluffy hat or not"),
        )
        .get_matches();
}
```

## Grouping Parameters


```rs
use clap::{command, Arg, ArgGroup, ArgMatches};

fn main() {
    let match_result: ArgMatches = command!()
        .about("This app registers people with their doctors office.")
        .group(
            ArgGroup::new("person-register")
                .arg("firstname")
                .arg("lastname")
                .multiple(true)
                .required(true),
        )
        .group(ArgGroup::new("dog-register").arg("pet-name"))
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first-name")
                .aliases(["fname", "firstname"])
                .help("The person's first name"),
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last-name")
                .aliases(["lname", "lastname"])
                .help("This argument takes the person's last name"),
        )
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
                .help("Is the person wearing a fluffy hat or not"),
        )
        .arg(
            Arg::new("pet-name")
                .short('p')
                .long("pet-name")
                .aliases(["pname", "petname"])
                .help("The person's dogs name"),
        )
        .get_matches();
}
```

## SubCommands

```rs
use clap::{command, Arg, ArgMatches, Command};

fn main() {
    let match_result: ArgMatches = command!()
        .subcommand(
            Command::new("register-person")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .aliases(["fname", "firstname"])
                        .required(true)
                        .help("The person's first name"),
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .aliases(["lname", "lastname"])
                        .required(true)
                        .help("This argument takes the person's last name"),
                ),
        )
        .subcommand(
            Command::new("register-pet").arg(
                Arg::new("pet-name")
                    .short('p')
                    .long("pet-name")
                    .aliases(["pname", "petname"])
                    .required(true)
                    // .conflicts_with("firstname")
                    // .conflicts_with("lastname")
                    .help("The person's dogs name"),
            ),
        )
        .about("This app registers people with their doctors office.")
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
                .help("Is the person wearing a fluffy hat or not"),
        )
        .get_matches();
}
```

```sh
cr -- --help
This app registers people with their doctors office.

Usage: clap-cli [OPTIONS] [COMMAND]

Commands:
  register-person  
  register-pet     
  help             Print this message or the help of the given subcommand(s)

Options:
      --fluffy <fluffy>  Is the person wearing a fluffy hat or not
  -h, --help             Print help
  -V, --version          Print version
```

```sh
cr -- register-pet
error: the following required arguments were not provided:
  --pet-name <pet-name>

Usage: clap-cli register-pet --pet-name <pet-name>

For more information, try '--help'.
```

```sh
cr -- register-person -f Tom -l Sawyer --fluffy 
error: unexpected argument '--fluffy' found
```

We have to call fluffy first because it's defined in the global scope.

This will work fine:

```sh
cr -- --fluffy true register-person -f t -l s
```

