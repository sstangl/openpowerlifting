.PHONY: builddir binary data oldstatics config client clientstatics release clean

BUILDDIR := build
BUILDFILE := build.tar.bz2

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
	../scripts/csv-rmcol "${BUILDDIR}/opldb/openpowerlifting.csv" Name

config: builddir
	cp server-production.env "${BUILDDIR}/server.env"
	cp Rocket.toml "${BUILDDIR}/Rocket.toml"

client:
	$(MAKE) -C client

clientstatics: client
	mkdir -p "${BUILDDIR}/static"
	cp -r client/build/* "${BUILDDIR}/static"

release: binary data config oldstatics clientstatics
	touch "${BUILDDIR}/${BUILDFILE}"
	tar cvjf "${BUILDDIR}/${BUILDFILE}" --exclude="${BUILDFILE}" "${BUILDDIR}"
	echo "Good luck!"

clean:
	$(MAKE) -C client clean
	rm -rf ${BUILDDIR}
	rm -rf target
