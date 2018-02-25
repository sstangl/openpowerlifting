.PHONY: builddir binary data statics config release

BUILDDIR := build

all: release

builddir:
	mkdir -p "${BUILDDIR}"

binary: builddir
	cargo build --release
	cp target/release/server "${BUILDDIR}/server"

data: builddir
	mkdir -p "${BUILDDIR}/templates"
	cp -r templates/* "${BUILDDIR}/templates/"
	mkdir -p "${BUILDDIR}/translations"
	cp -r translations/* "${BUILDDIR}/translations/"
	mkdir -p "${BUILDDIR}/opldb"
	cp ../build/lifters.csv "${BUILDDIR}/opldb/lifters.csv"
	cp ../build/meets.csv "${BUILDDIR}/opldb/meets.csv"
	cp ../build/openpowerlifting.csv "${BUILDDIR}/opldb/openpowerlifting.csv"

statics: builddir
	mkdir -p "${BUILDDIR}/static"
	cp -r -L static/* "${BUILDDIR}/static/"
	rm "${BUILDDIR}/static/data/meets.csv"
	rm "${BUILDDIR}/static/data/openpowerlifting.csv"
	rmdir "${BUILDDIR}/static/data"

config: builddir
	cp server-production.env "${BUILDDIR}/server.env"
	cp Rocket.toml "${BUILDDIR}/Rocket.toml"

release: binary data config statics
	echo "Good luck!"

clean:
	rm -rf ${BUILDDIR}
	rm -rf target
