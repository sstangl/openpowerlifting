.PHONY: builddir binary release

BUILDDIR := build

all: release

builddir:
	mkdir -p "${BUILDDIR}"

binary: builddir
	cargo build --release
	cp target/release/server "${BUILDDIR}/server"
	strip "${BUILDDIR}/server"

database: builddir
	cp ../build/openpowerlifting.sqlite3 "${BUILDDIR}/openpowerlifting.sqlite3"

statics: builddir
	mkdir -p "${BUILDDIR}/htmltmp"
	cp htmltmp/*.html "${BUILDDIR}/htmltmp/"
	cp -r -L static "${BUILDDIR}/static"
	cp -r -L templates "${BUILDDIR}/templates"

config: builddir
	cp server-production.env "${BUILDDIR}/server.env"
	cp Rocket.toml "${BUILDDIR}/Rocket.toml"

release: binary database statics config
	echo "Good luck!"

clean:
	rm -rf ${BUILDDIR}
	rm -rf target
