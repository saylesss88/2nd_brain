# Futures and the Async Syntax

The key elements of asynchronous programming in Rust are _futures_ and Rust's
`async` and `await` keywords.

A _future_ is a value that may not be ready now but will become ready at some
point in the future.

You can apply the async keyword to blocks and functions to specify that they can
be interrupted and resumed. Within an async block or async function, you can use
the await keyword to await a future (that is, wait for it to become ready). Any
point where you await a future within an async block or function is a potential
spot for that block or function to pause and resume. The process of checking
with a future to see if its value is available yet is called polling.
