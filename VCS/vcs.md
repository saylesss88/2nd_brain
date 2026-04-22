I have to say, my Git skills could be much better. I have been developing quite
a few small rust projects for fun and want the next person that has to read my
history for it to:

1. Make sense for them

&

2. Make developing easier for me

For your Git history to make sense, it's best to use atomic commits with a
linear history and that actually helps with # 1 and # 2.

1. Atomic Commits An atomic commit is a single unit of work that cannot be
   broken down further without losing its meaning.

- The Rule: One commit should do one thing.

- The Test: If you had to "undo" that commit later, would it break unrelated
  features? If "Yes," it's not atomic.

- Why it helps: If you find a bug, you can pinpoint the exact 10 lines of code
  that caused it. In `jj`, the split command we used earlier is the ultimate
  tool for "atomizing" a messy afternoon of coding.
