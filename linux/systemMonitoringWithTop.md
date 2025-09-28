---
id: systemMonitoringWithTop
aliases:
  - The essential guide to system monitoring with `top`, `htop`, and `vmstat`
tags: []
---

# The essential guide to system monitoring with `top`, `htop`, and `vmstat`

1. Filtering: You can filter top and htop to show specific processes or system
   resources. For example, to show only the processes owned by a specific user,
   you can use the following command:

```bash
top -U username
```

To show only processes using a certain amount of CPU, you can use the following
command:

```bash
tom -o %CPU
```

2. Sorting: You can sort the output of top and htop based on various criteria
   such as CPU usage, memory usage, and process ID. To sort top by CPU usage,
   you can press the `P` key. To sort htop by memory usage, you can press the
   "F6" key and select the "MEM%" option.

3. Color-coded output: By default, processes using more than 75% of CPU or
  memory are displayed in red. You can customize this behavior by using the "F2"
  key and selecing the "Display options" menu.

4. Continuous monitoring: You can use the `vmstat` command to continuously
   monitor system performance. You can use the `-n` option to set the interval
   between updates. For example, to monitor system performance every 5 seconds,
   you can use the following command:

```bash
vmstat -n 5
```

5. Redirecting output: You can redirect the output of top and htop to a file or
   another command for further analysis. For example, to save the output of top
   to a file you can use:

```bash
top -n 1 > top_output.txt
```

Similarly, to pipe the output of vmstat to another command:

```bash
vmstat 1 | awk '{print $1}'
```
  - This command will print the first column of the output of vmstat.

