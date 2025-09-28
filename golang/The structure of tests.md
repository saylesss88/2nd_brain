---
id: The structure of tests
aliases:
  - The structure of tests
tags: []
---

# The structure of tests

All tests follow more or less the same basic structure:
• Set up the inputs for the test, and the expected results
• Call the function under test and pass it the appropriate input
• Check that the result matches the expectation, and if it doesn’t, fail the test with a
message explaining what happened And we’ve also practiced a specific workflow, or sequence
of actions, that we can use when developing any piece of Go code:

## The development process

1. Start by writing a test for the function (even though it doesn’t exist yet)
2. See the test fail with a compilation error because the function doesn’t exist yet.
3. Write the minimum code necessary to make the test compile and fail.
4. Make sure that the test fails for the reason we expected, and check that the failure
message is accurate and informative.
5. Write the minimum code necessary to make the test pass.
6. Optionally, tweak and improve the code, preferably without breaking the test.
7. Commit!
This is sometimes referred to as the “red, green, refactor” cycle. The “failing test” stage
is “red”, the “passing test” stage is “green”, and beautifying the code is “refactor”. In
practice, we may go through this loop a few times with any particular test or function.
Any function with reasonably complicated behaviour is probably too hard to test and
implement from scratch, so try to break its behaviour down into smaller chunks. These
might actually be separate functions, or maybe just sub‐behaviours that we can write
individual tests for. Once we have most of the parts we need, we can use them to build
up the real function, step by step.
