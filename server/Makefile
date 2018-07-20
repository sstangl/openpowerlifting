.PHONY: builddir binary data oldstatics config forceclientnpm client clientstatics release rebuild-clean clean

BUILDDIR := build
BUILDFILE := build.tar.bz2

all: release

# Test using --release to get the CSV crate to parse more quickly.
# That way, it's no problem to test over actual data.
check: forceclientnpm client
	cargo test --release

builddir:
	mkdir -p "${BUILDDIR}"
	mkdir -p "${BUILDDIR}/data"

binary: builddir
	cargo build --release
	cp target/release/server "${BUILDDIR}/server"

# Documents everything, not just the public-facing APIs.
devdocs:
	cargo rustdoc --open -- --no-defaults --passes collapse-docs --passes unindent-comments --passes strip-priv-imports

data: builddir
	mkdir -p "${BUILDDIR}/data/templates"
	cp -r templates/* "${BUILDDIR}/data/templates/"
	mkdir -p "${BUILDDIR}/data/translations"
	cp -r translations/* "${BUILDDIR}/data/translations/"
	mkdir -p "${BUILDDIR}/data/opldb"
	cp ../build/lifters.csv "${BUILDDIR}/data/opldb/lifters.csv"
	cp ../build/meets.csv "${BUILDDIR}/data/opldb/meets.csv"
	cp ../build/entries.csv "${BUILDDIR}/data/opldb/entries.csv"

config: builddir
	cp server-production.env "${BUILDDIR}/data/server.env"

forceclientnpm:
	$(MAKE) -C client forcenpm

client:
	$(MAKE) -C client

clientstatics: client
	mkdir -p "${BUILDDIR}/data/static"
	cp -r client/build/* "${BUILDDIR}/data/static"
	rm "${BUILDDIR}/data/templates/static-asset-map.tera"
	cp client/build/static-asset-map.tera "${BUILDDIR}/data/templates/"

release: rebuild-clean binary data config oldstatics clientstatics
	touch "${BUILDDIR}/${BUILDFILE}"
	tar cvjf "${BUILDDIR}/${BUILDFILE}" --exclude="${BUILDFILE}" "${BUILDDIR}"
	echo "Good luck!"

# Cleans files that may cause `make` to be non-idempotent.
rebuild-clean:
	$(MAKE) -C client rebuild-clean
	rm -rf ${BUILDDIR}

clean:
	$(MAKE) -C client clean
	rm -rf ${BUILDDIR}
	rm -rf target
