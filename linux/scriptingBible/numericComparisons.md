---
id: numericComparisons
aliases:
  - Numeric Comparisons
tags: []
---

# Numeric Comparisons

`n1 -eq n2` Checks if n1 is equal to n2

`n1 -ge n2` Checks if n1 is greater than or equal to n2

`n1 -gt n2` Checks if n1 is greater than n2

`n1 -le n2` Checks if n1 is less than or equal to n2

`n1 -lt n2` Checks if n1 is less than n2

`n1 -ne n2` Checks if n1 is not equal to n2

The numeric test conditions can be used to evaluate both numbers and variables.
Here's an example of doing that:

```bash
#!/usr/bin/bash
# Using numeric test evaluations
#
value1=10
value2=11
#
if [ $value1 -gt 5 ]
then
    echo "The test value $value1 is greater than 5"
fi
#
if [ $value1 -eq $value2 ]
then
    echo "The values are equal"
else
    echo "The values are different"
fi
```

[[usingStringComparisons.md]]
