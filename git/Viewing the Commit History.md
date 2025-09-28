---
id: Viewing the Commit History
aliases:
  - Viewing the Commit History
tags: []
---

# Viewing the Commit History

To look back and see what has happened in your commit history, the most basic
and powerful tool to do this is the `git log` command.

- By default, with no arguments, `git log` lists the commits made in that repo in
 reverse chronological order; that is, the most recent commit is listed first.
 - It lists each commit with its SHA-1 checksum, the authors name and email,
  the date written, and the commit message.

  - A huge number and variety of options to the `git log` command are available to
  show you exactly what you're looking for.
  - One of the more helpful options is `-p` or `--patch`, which will show you the 
   difference (the *patch* output) introduced in each commit.
- You can also limit the number of log entries displayed, such as the `-2` to show
only the last 2 entries.

```bash
git log -p -2
commit ca82a6dff817ec66f44342007202690a93763949
Author: Scott Chacon <schacon@gee-mail.com>
Date:
Mon Mar 17 21:52:11 2008 -0700
Change version number
diff --git a/Rakefile b/Rakefile
index a874b73..8f94139 100644
--- a/Rakefile
+++ b/Rakefile
@@ -5,7 +5,7 @@ require 'rake/gempackagetask'
spec = Gem::Specification.new do |s|
s.platform =
Gem::Platform::RUBY
s.name
=
"simplegit"
-
s.version
=
"0.1.0"
+
s.version
=
"0.1.1"
s.author
=
"Scott Chacon"
s.email
=
"schacon@gee-mail.com"
s.summary
=
"A simple gem for using Git in Ruby code."
commit 085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7
Author: Scott Chacon <schacon@gee-mail.com>
Date:
Sat Mar 15 16:40:33 2008 -0700
Remove unnecessary test
diff --git a/lib/simplegit.rb b/lib/simplegit.rb
index a0a60ae..47c6340 100644
--- a/lib/simplegit.rb
+++ b/lib/simplegit.rb
@@ -18,8 +18,3 @@ class SimpleGit
end
end
-
-if $0 == __FILE__
- git = SimpleGit.new
- puts git.show
-end
``````

This option displays the same info but with a diff directly following each
entry. This is very helpful for code review or to quickly brows what happened 
during a series of commits that a collaborator has added.

- You can also use a series of summarizing options with `git log`. For example,
if you want to see some abbreviated stats for each commit, you can use the
`--stat` option:
```bash
git log --stat
commit ca82a6dff817ec66f44342007202690a93763949
Author: Scott Chacon <schacon@gee-mail.com>
Date:
Mon Mar 17 21:52:11 2008 -0700
Change version number
Rakefile | 2 +-
1 file changed, 1 insertion(+), 1 deletion(-)
commit 085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7
Author: Scott Chacon <schacon@gee-mail.com>
Date:
Sat Mar 15 16:40:33 2008 -0700
Remove unnecessary test
lib/simplegit.rb | 5 -----
1 file changed, 5 deletions(-)
commit a11bef06a3f659402fe7563abf99ad00de2209e6
Author: Scott Chacon <schacon@gee-mail.com>
Date:
Sat Mar 15 10:31:28 2008 -0700
Initial commit
README
| 6 ++++++
Rakefile
| 23 +++++++++++++++++++++++
lib/simplegit.rb | 25 +++++++++++++++++++++++++
3 files changed, 54 insertions(+)
````

As you can see, the --stat option prints below each commit entry a list of
modified files, how many files were changed, and how many lines in those files
were added and removed. It also puts a summary of the information at the end.

- Another really useful option is `--pretty`. This option changes the output format
The `online` value for this option prints each commit on a single line, which is 
helpful if you're looking at a lot of commits.

```bash
git log --pretty=online
ca82a6dff817ec66f44342007202690a93763949 Change version number
085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7 Remove unnecessary test
a11bef06a3f659402fe7563abf99ad00de2209e6 Initial commit
```

| Specifier | Description of Output |
| -------------- | --------------- |
| `%H` | Commit hash |
| `%h` | Abbreviated commit hash |
| `%T` | Tree hash |
| `%t` | Abbreviated tree hash |
| `%P` | Parent hashes |
| `%an` |  author name |
| `%ae` |  author email |
| `%cn` |  Committer name |
| `%ad` | Date of commit |
| `%s` | Subject |
| `%b` | Commit message |

You may be wondering what the difference is between author and committer. The
author is the person who originally wrote the work, whereas the committer is the
person who last applied the work.

[[Undoing Things.md]]
