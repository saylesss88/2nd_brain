---
id: parameterExpansion
aliases:
  - Parameter Expansion
tags: []
---

# Parameter Expansion

The expansion of a parameter is achieved by prefixing that parameter with a `$`
sign. In certain situations, additional curly braces around the parameter's name
are required:

```bash
echo "'$USER', '$USERs', '${USER}s'"
'jr', '','jrs'
```

- Since there's no way Bash could know we want a literal `s` to be appended to
  the parameter's value, you need to use curly braces to mark the beginning and
  end of the parameter name.

```bash
for file in *.JPG *.jpg
do mv -- "$file" "${file%.*}.jpg"
done
```

- This will rename all JPEG files with a `.JPG` extension to have a `.jpg`
  extension.

- The expression `${file%.*}` cuts off everything from the end starting with the
  last period. Then, in the same quotes, a new extension is appended to the
  expansion result.

Parameter Expansion tricks:

`${parameter:-word}`: Use Default Value. If `parameter` is unset or null, `word`
is substituted. Otherwise, the value of `parameter` is substituted.

`${parameter:=word}`: Use Default Value. If `parameter` is unset or null, `word`
is assigned to `parameter`. The value of `parameter` is then substituted.

`${parameter:+word}`: Use Alternate Value. If `parameter` is unset or null,
nothing is substituted, otherwise `word` is substituted.

```bash
file="$HOME/.secrets/007"; \
echo "File location: $file"; \
echo "Filename: ${file##*/}"; \
echo "Directory of file: ${file%/*}"; \
echo "Non-secret file: ${file/secrets/not_secret}"; \
echo; \
echo "Other file location: ${other:-There is no other file}"; \
echo "Using file if there is no other file: ${other:=$file}"; \
echo "Other filename: ${other##*/}"; \
echo "Other file location length: ${#other}"
File location: /home/lhunath/.secrets/007
Filename: 007
Directory of file: /home/lhunath/.secrets
Non-secret file: /home/lhunath/.not_secret/007
```

Output:

```bash
File location: /home/jr/.secrets/007
Filename: 007
Directory of file: /home/jr/.secrets
Non-secret file: /home/jr/.not_secret/007

Other file location: There is no other file
Using file if there is no other file: /home/jr/.secrets/007
Other filename: 007
Other file location length: 21
./secrets: line 11: File: command not found
./secrets: line 12: Filename:: command not found
./secrets: line 13: Directory: command not found
./secrets: line 14: Non-secret: command not found
```

Remember the difference between `${v#p}` and `${v##p}`. The doubling of the `#`
character means patterns will become greedy. The same goes for `%`:

```bash
version=1.5.9; echo "MAJOR: ${version%%.*}, MINOR: ${version#*.}."
MAJOR: 1, MINOR: 5.9.
$ echo "Dash: ${version/./-}, Dashes: ${version//./-}."
Dash: 1-5.9, Dashes: 1-5-9.
```

#### Brace Expansion

Globs only expand to actual filenames, but brace expansion will expand to any
possible permutation of their contents. Here's how they work:

```bash
$ echo th{e,a}n
then than
$ echo {/home/*,/root}/.*profile
/home/axxo/.bash_profile /home/lhunath/.profile /root/.bash_profile /root/.profile
$ echo {1..9}
1 2 3 4 5 6 7 8 9
$ echo {0,1}{0..9}
00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19
```

- The brace expansion is replaced by a list of words, just like a glob is.
  However, these words aren't necessarily filenames, and they are not sorted(`than` would have come before `then` if they were).

Brace expansion happens before filename expansion:

```bash
echo /home/*/.*profile /root/.*profile
/home/jr/.bash_profile /home/jr/.profile /root/.*profile
```

- After the brace expansion, the glob is expanded, and we get the filenames as
  the final result.

- Brace expansion can only be used to generate lists of words. They cannot be
  used for pattern matching.
