.PHONY: builddir binary data oldstatics config client clientstatics release clean

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

oldstatics: builddir
	mkdir -p "${BUILDDIR}/static"
	mkdir -p "${BUILDDIR}/static/images"
	cp static/images/check_radio_sheet.png "${BUILDDIR}/static/images/"
	cp static/images/logo-compressed.svg "${BUILDDIR}/static/images/"
	cp static/images/patreon_navigation_logo_mini_orange.png "${BUILDDIR}/static/images/"
	cp static/images/select_dropdown.png "${BUILDDIR}/static/images/"

config: builddir
	cp server-production.env "${BUILDDIR}/server.env"
	cp Rocket.toml "${BUILDDIR}/Rocket.toml"

client:
	$(MAKE) -C client

clientstatics: client
	mkdir -p "${BUILDDIR}/static"
	cp -r client/build/* "${BUILDDIR}/static"

release: binary data config oldstatics clientstatics
	echo "Good luck!"

clean:
	rm -rf ${BUILDDIR}
	rm -rf target
