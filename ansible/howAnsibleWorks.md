---
title: How Ansible Works
date: 2024-10-09
tags: [ansible, tag2]
---

# How Ansible Works

Ansible uses the concepts of control and managed nodes. It connects from the
`control node`, any machine with Ansible installed, to the `managed nodes`
sending commands and instructions to them.

- The units of code that Ansible executes on the managed nodes are called
  `modules`.

- Each `module` is invoked by a `task`, and an orderd list of tasks together
  forms a `playbook`. Users write playbooks with tasks and modules to define the
  desired state of the system.
