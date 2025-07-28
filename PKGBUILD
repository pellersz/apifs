# Maintainer: Papp Gellert-Szabolcs <pappgellert2003@gmail.com>

#Before building it's likely you should add this line: `LTOFLAGS="-flto=auto -ffat-lto-objects"` to /etc/makepkg.conf.d/rust.conf
pkgname=apifs
pkgver=r29.29f4e55
pkgrel=1
pkgdesc="A package for creating and managing reminders and notes"
arch=('x86_64')
url="https://github.com/pellersz/apifs"
license=('MIT')
depends=('vlc' 'glib2' 'gtk4')
makedepends=('cargo' 'git')
source=("git+https://github.com/pellersz/apifs")
sha256sums=("SKIP")

pkgver() {
    cd "$pkgname"
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cd "$pkgname"
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
	cd "$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    cd "$pkgname"
    install -Dm755 ./target/release/apifs "$pkgdir/usr/bin/apifs"
    mkdir -p "$pkgdir/opt/apifs/scripts"
    install -Dm755 ./src/scripts/* "$pkgdir/opt/apifs/scripts/"

}
