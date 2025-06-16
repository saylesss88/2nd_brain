---
id: Install Source Code Pro
aliases:
  - Install Source Code Pro
tags: []
---

# Install Source Code Pro

1. Download the archive:

```bash
wget https://github.com/adobe-fonts/source-code-pro/archive/2.030R-ro/1.050R-it.zip
```

2. Unzip the archive:

```bash
unzip 1.050R-it.zip
```

3. Locate and ensure your fonts folder exists in your home directory, often in
   ~/.local/share/fonts folder or type the following:

```bash
fontpath="${XDG_DATA_HOME:-$HOME/.local/share}"/fonts

mkdir -p $fontpath
```
- If you already have that directory, don't worry.

4. Move the Open Type fonts (*.otf) to the new .fonts directory:

```bash
cp source-code-pro-*-it/OTF/*.otf $fontpath
```

5. Update the cache:

```bash
fc-cache -f -v
```

All in one script:

```bash
#!/bin/bash
set  -euo pipefail
I1FS=$'\n\t'
mkdir -p /tmp/adodefont
cd /tmp/adodefont
wget -q --show-progress -O source-code-pro.zip https://github.com/adobe-fonts/source-code-pro/archive/2.030R-ro/1.050R-it.zip
unzip -q source-code-pro.zip -d source-code-pro
fontpath="${XDG_DATA_HOME:-$HOME/.local/share}"/fonts
mkdir -p $fontpath
cp -v source-code-pro/*/OTF/*.otf $fontpath
fc-cache -f
rm -rf source-code-pro{,.zip}
```

If you want to install system wide instead of per user, copy the files to 
`/usr/local/share/fonts/` instead of `~/.local/share/fonts`
