---
id: usingStringComparisons
aliases:
  - Using String Comparisons
tags:
  - bash strings
---

# Using String Comparisons

`str1 = str2` Checks if str1 is equal to str2

`str1 != str2` Checks if str1 is not equal to str2

`str1 < str2` Checks if str1 is less than str2

`str1 > str2` Checks if str1 is greater than str2

`-n str1` Checks if str1 has a length greater than zero

`-z str1` Checks if str1 has a length of zero

**Looking at string equality**

```bash
#!/usr/bin/bash
# mis-using string comparisons
#
val1=baseball
val2=hockey
#
if [ $val1 \> $val2 ]
then
    echo "$val1 is greater than $val2"
else
    echo "$val1 is less than $val2"
fi
$
$ ./test9.sh
baseball is less than hockey
```

- Notice the escape character before the greater than sign. If you don't escape
  the greater than sign, it will be interpreted as a redirection.

- The second got ya is the way that the `sort` command handles uppercase letters
  opposite to the way the `test` conditions consider them:

```bash
###!/usr/bin/bash:
# testing string sort order
val1=Testing
val2=testing
#
if [ $val1 \> $val2 ]
then
    echo "$val1 is greater than $val2"
else
    echo "$val1 is less than $val2"
fi
$
$ ./test9b.sh
Testing is less than testing
$
$ sort testfile
testing
Testing
```

- Capitalized letters are treated as less than lowercase letters in test
  comparisons. However, the `sort` command does the opposite.

[[usingFileComparisons.md]]
