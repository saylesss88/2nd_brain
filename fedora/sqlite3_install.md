# SQLite Installation
#SQLite

To install basic SQLite:
```bash
sudo dnf install sqlite
```
This package provides the basic library and the command-line client `sqlite`. In order to access SQLite databases from various programming languages, the lang bindings need to be installed separately:

```bash
sudo dnf install sqlite-devel sqlite-tcl
```

## SQLite Browser
```bash
sudo dnf install sqlitebrowser
```
#### Making a database file

```bash
sqlite3 hello-world.db
```

