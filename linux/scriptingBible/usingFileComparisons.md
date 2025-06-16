---
id: usingFileComparisons
aliases:
  - Using File Comparisons
tags:
  - bash
---

# Using File Comparisons

This is possibly the most powerful and most used comparisons in shell scripting.
This allows you to test the status of files and directories on the Linux
filesystem.

`-d file` Checks if file exists and is a directory

`-e file` Checks if file exists

`-f file` Checks if file exists and is a regular file

`-r file` Checks if file exists and is readable

`-s file` Checks if file exists and has a size greater than zero

`-w file` Checks if file exists and is writable

`-x file` Checks if file exists and is executable

`-O file` Checks if file exists and is owned by the current user

`-G file` Checks if file exists and is owned by the current group

`file1 -nt file2` Checks if file1 is newer than file2

`file1 -ot file2` Checks if file1 is older than file2

## Checking directories

The `-d` test checks to see if a specified directory exists on the system. This
is usually good to do if you're trying to write a file to a directory or before
you try to change a directory location:

```bash
#!/usr/bin/bash test11.sh
# Look before you leap
#
jump_directory=/home/jr
#
if [ -d $jump_directory ]
then
  echo "The $jump_directory exists"
  cd $jump_directory
  ls
else
  echo "The $jump_directory does not exist"
fi
```

- The `-d` test condition checks to see if the `jump_directory` variable's
  directory exists. If it does, it proceeds to use the `cd` command to change to
  the current directory and performs a directory listing.

### Checking whether an object exists

The `-e` test checks if either a file or directory object exists before you
attempt to use it in your script:

```bash
#!/usr/bin/bash test11.sh
# Check if either a directory or file exists
#
location=$HOME
file_name="sentinel"
#
if [ -e $location ]
then #Directory does exist
    echo "OK on the $location directory."
    echo "Now checking on the file, $file_name."
    #
    if [ -e $location/$file_name ]
    then #File does exist
        echo "OK on the filename"
        echo "Updating Current Date..."
        date >> $location/$file_name
    #
    else #File does not exist
        echo "File does not exist"
        echo "Nothing to update"
    fi
#
else
#Directory does not exist
    echo "The $location directory does not exist."
    echo "Nothing to update"
fi
```
