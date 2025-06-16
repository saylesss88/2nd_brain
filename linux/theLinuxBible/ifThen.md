---
id: ifThen
aliases: []
tags: []
---

```bash
VARIABLE=1
if [ $VARIABLE -eq 1 ]; then
echo "VARIABLE is equal to 1"
fi
```

- Instead of using `-eq` you can use `=`. The `=` works best for comparing
  string values, while `-eq` works best for comparing numeric values.

- Using the else statement, different words can be echoed if the criterion of the if
  statement isn’t met ($STRING = ″Friday″). Keep in mind that it’s good practice to put
  strings in double quotes.

```bash
STRING="Friday"
if [ $STRING = "Friday" ] ; then
echo "WhooHoo. Friday."
else
echo "Will Friday ever get here?"
fi
```

An example using `elif`:

```bash
filename="$HOME"
if [ -f "$filename" ] ; then
echo "$filename is a regular file"
elif [ -d "$filename" ] ; then
echo "$filename is a directory"
else
echo "I have no idea what $filename is"
fi
```

- The condition you are testing is placed between square brackets `[]`.

- When a test value is evaluated, it returns either a value of 0, meaning that
  it is true, or a 1, meaning that it is false.

- Notice that the echo lines are
  indented. The indentation is optional and done only to make the script more readable.

- To get a list of conditions that are testable you can use the `man test`
  command.

In the following example, the two pipes (||) indicate that if the
directory being tested for doesn’t exist (-d dirname), then make the directory (mkdir
$dirname):

```bash
# [ test ] || action
# Perform simple single command if test is false
dirname="/tmp/testdir"
[ -d "$dirname" ] || mkdir "$dirname"
```

- Instead of pipes, you can use the `&&` operator to test if something is
  true.In the following, a command is being tested to see if it includes at
  least three command-line arguments.

```bash
# [ test ] && {action}
# Perform simple single action if test is true
[ $# -ge 3 ] && echo "There are at least 3 command line arguments."
```

The following example tests that the directory represented by $dirname already exists. If
it does, a message says the directory already exists. If it doesn’t, the statement creates the
directory:

```bash
# dirname=mydirectory
[ -e $dirname ] && echo $dirname already exists || mkdir $dirname
```

[[theCaseCommand.md]]
