---
id: theCaseCommand
aliases:
  - theCaseCommand
tags: []
---

# theCaseCommand

Similar to a switch statement in
programming languages, this can take the place of several nested if statements. The fol-
lowing is the general form of the case statement:

```bash
case "VAR" in
Result1)
{ body };;
Result2)
{ body };;
*)
{ body };;
esac
```

- You can use the `case` command to help with your backups. The following case
  statement tests for the first three letters of the current day (case 'date+%a' in)
  Then, depending on the day, a particular backup directory (BACKUP) and tape
  drive (TAPE) are set:

```bash
# Our VAR doesn't have to be a variable,
# it can be the output of a command as well
# Perform action based on day of week
case `date +%a` in
"Mon")
      BACKUP=/home/myproject/data0
      TAPE=/dev/rft0
# Note the use of the double semi-colon to end each option
      ;;
# Note the use of the "|" to mean "or"
"Tue" | "Thu")
      BACKUP=/home/myproject/data1
      TAPE=/dev/rft1
      ;;
"Wed" | "Fri")
      BACKUP=/home/myproject/data2
      TAPE=/dev/rft2
      ;;
# Don't do backups on the weekend.
*)

BACKUP="none"
      TAPE=/dev/null
      ;;
esac
```

- The asterisk (\*) is used as a catchall, similar to the default keyword in the C program-
  ming language. In this example, if none of the other entries are matched on the way down
  the loop, the asterisk is matched and the value of BACKUP becomes none. Note the use of
  esac, or case spelled backwards, to end the case statement.

[[forDoLoop.md]]
