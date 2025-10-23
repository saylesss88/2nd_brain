## Password Entropy

Password entropy measures how hard it is to guess a password, expressed in bits.
It essentially represents the amount of randomness and unpredictability in the
password.

How to Calculate Password Entropy You need two key pieces of information:

L: The length of the password (number of characters).

R: The size of the character set from which the password characters are drawn.

The formula to calculate entropy (E) in bits is:

```text
E = L x log2(R)
```

- Here `Log2(R)` is the number of bits of entropy per character, and multiplying
  by L gives the total bits for the entire password.

## Determining Character Set Size (R)

You identify which character groups your password uses and sum their sizes:

## |Character Type | Number of Characters |

| Numbers (0-9)         | 10                          |
| --------------------- | --------------------------- |
| Lowercase letters     | 26                          |
| --------------------- | --------------------------  |
| Uppercase letters     | 26                          |
| --------------------- | --------------------------- |
| Special symbols       | ~32 (varies by keyboard)    |

Example: If your password contains lowercase and uppercase letters plus digits,
your character set size `R = 26 + 26 + 10 = 62`
