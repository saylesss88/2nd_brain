---
id: limitingProcessesWithCgroups
aliases:
  - limitingProcessesWithCgroups
tags: []
---

# limitingProcessesWithCgroups

You can use a feature like "nice" to give a single process more or less access
to CPU time.

- Setting the nice value for one process, however, doesn’t apply to child processes that a pro-
  cess might start up or any other related processes that are part of a larger service. In other
  words, “nice” doesn’t limit the total amount of resources a particular user or application
  can consume from a Linux system.

- As cloud computing takes hold, many Linux systems will be used more as hypervisors than
  as general-purpose computers. Their memory, processing power, and access to storage will
  become commodities to be shared by many users. In that model, more needs to be done to
  control the amount of system resources to which a particular user, application, container,
  or virtual machine running on a Linux system has access.
  That’s where _cgroups_ come in.

Cgroups can be used to identify a process as a task, belonging to a particular
control group.

- Tasks can be set up in a hierarchy where, there may be a task called daemons
  that sets default limitations for all daemon server processes, then subtasks
  that may set specific limits on a web server daemon (httpd) for FTP service
  daemon (vsftpd), and so on.

- As a task launches a process, the child process inherits the same limitations
  set for the parent process.

- Creating and managing cgroups is generall not a job for new administrators.

- As a Linux admin, you will probably run into these features from controllers
  that manage your cloud infrastructure. You will be able to set rules like
  "Allow the Marketing department's virtual machines to consume up to 40% of
  the available memory" or "Pin the database application to a particular CPU
  and memory set".
