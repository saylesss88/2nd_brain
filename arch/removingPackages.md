---
id: How to remove packages
aliases:
  - How to remove packages
tags: []
---

# How to remove packages

## Pamac - the easy GUI way

Using Pamac is most likely the easiest way of dealing with packages you want to
get rid of unneeded packages if you aren't familiar with the terminal. Keep in
mind Pamac is not installed in every edition out of the box - KDE for example
uses Octopi which sadly does not provide a list of explicitly installed packages
which means you could potentially uninstall an important package and break your
system this way.

    Open Pamac using the start menu
    Head over to the “Installed” tab and select “Explicitly Installed”
    Select any application you want to remove - since these are explicitly
    installed the risk of uninstalling a needed dependency is quite low
    Don't forget to check the list of packages, which are going to be removed
    before confirming the transaction! ⚠️
    Apply the changes

The terminal way

The terminal way is a bit harder to complete but serves as excellent training
to get more familiar with our beloved terminal. Another advantage is that we
don't rely on a third-party application to manage our packages rather than using
Pacman directly. Here is the procedure:

- As you might have assumed already, open up a terminal emulator (eg. Konsole
  or Alacritty) using the start menu
- Type `pacman -Qe` to list all explicitly installed packages and search for
  packages you want to remove

- Eventually, check unknown packages by typing pacman -Ss somepackage to find
  out its purpose

- Add all of those packages to the following command: `sudo pacman -Rns package1
package2 package3` - this will remove all packages and their respective
  dependencies which aren't required anymore from the system
