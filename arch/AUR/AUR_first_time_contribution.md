# Arch PKGBUILD

- [Rust package guidelines](https://wiki.archlinux.org/title/Rust_package_guidelines)
  gives you a template to follow.

```bash
# Maintainer: Your Name <your_email at domain dot tld>
pkgname=mdbook-rss-feed
pkgver=1.3.1  # Update to current version
pkgrel=1
pkgdesc='mdbook preprocessor that generates a full-content RSS 2.0, Atom, and JSON feeds'
url='https://github.com/saylesss88/mdbook-rss-feed'
license=('Apache-2.0')
makedepends=('cargo')
depends=('gcc-libs' 'glibc')
arch=('x86_64')
source=("$pkgname-$pkgver.tar.gz::https://static.crates.io/crates/$pkgname/$pkgname-$pkgver.crate")
b2sums=('SKIP')  # Generate with updpkgsums after first download

prepare() {
  export RUSTUP_TOOLCHAIN=stable
  cd "$pkgname-$pkgver"
  cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cd "$pkgname-$pkgver"
  cargo build --frozen --release --all-features
}

check() {
  export RUSTUP_TOOLCHAIN=stable
  cd "$pkgname-$pkgver"
  cargo test --frozen --all-features
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
```

### Building and Testing Locally

Place the `PKGBUILD` file in your crate repositories
root.(`~/mdbook-rss-feed/PKGBUILD`)

```bash
sudo pacman -S base-devel

makepkg -si
```

Once it builds successfully, generate the corrected checksums:

```bash
updpkgsums
```

---

# First time contribution to AUR

1. Generate a ssh keypair specifically for the AUR:

```bash
ssh-keygen -t ed25519 -f ~/.ssh/aur -C "your_email@example.com"
```

Update `~/.ssh/config`:

```text
Host aur.archlinux.org
    IdentityFile ~/.ssh/aur
    User aur
    IdentitiesOnly yes
    AddKeysToAgent yes
```

2. **Register** at `https://aur.archlinux.org/register`

You'll need to copy the public key we generated in step 1:

```bash
cat ~/.ssh/aur.pub
```

Paste it into the AUR account profile.

3. Clone the Empty AUR Repository

The AUR uses its own Git hosting at `aur.archlinux.org`, this is not your GitHub
repo:

```bash
git clone ssh://aur@aur.archlinux.org/mdbook-rss-feed.git
```

4. Add the Files

```bash
cd mdbook-rss-feed

# Copy your PKGBUILD here
cp /path/to/your/PKGBUILD .

# Generate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# Add maintainer comment to PKGBUILD
# Edit PKGBUILD and add this at the top:
# Maintainer: Your Name <email at domain dot tld>
```

5. Commit and Push

```bash
git add PKGBUILD .SRCINFO
git commit -m "Initial commit: mdbook-rss-feed 1.3.1"
git push
```

### Resources

- [PKGBUILD](https://wiki.archlinux.org/title/PKGBUILD)

- [Rust package guidelines](https://wiki.archlinux.org/title/Rust_package_guidelines)

- [Arch package guidelines](https://wiki.archlinux.org/title/Arch_package_guidelines)
