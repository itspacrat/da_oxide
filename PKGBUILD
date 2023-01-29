# Maintainer: parkcitymedia <blakemichaelgaynor@gmail.com>
pkgname=duo_alert_oxide
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')

build() {
    return 0
}

package() {
	git clone https://github.com/parkcitymedia/duo_alert_oxide tmp
	cd tmp && cargo install --path "."
	cd .. && rm -r tmp
}
