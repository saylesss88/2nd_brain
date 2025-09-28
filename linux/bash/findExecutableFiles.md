---
id: findExecutableFiles
aliases: []
tags: []
---

The first step is to create a `for` loop to iterate through the folders stored
in the `PATH` environment variable. Don't forget the `IFS` separator character:

```bash
#!/usr/bin/bash
# finding files in the PATH

IFS=:
for folder in $PATH; do
  echo "$folder:"
  for file in $folder/*; do
    if [ -x "$file" ]; then
      echo "   $file"
    fi
  done
done
```
