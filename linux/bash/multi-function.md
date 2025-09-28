---
id: multi-function
aliases:
  - Testing the Multi-Function Bash Script
tags: []
---

# Testing the Multi-Function Bash Script

```bash
name=$(basename $0)

if [ $name = "addem" ]; then
  total=$[ $1 + $2 ]
else [ $name = "multem" ]; then
  total=$[ $1 * $2 ]
fi
echo
echo The calculated total is: $total
```

`cp test6.sh addem`
`chmod u+x addem`

`ln -s test6.sh multem`

`ls -l *em`

```bash
./addem 2 5
The calculated total is: 7
./multem 2 5
The calculated total is: 10
```
