.PHONY: builddir binary data oldstatics config client clientstatics release rebuild-clean clean

BUILDDIR := build
BUILDFILE := build.tar.bz2

all: release

# Test using --release to get the CSV crate to parse more quickly.
# That way, it's no problem to test over actual data.
check: client
	cargo test --release

builddir:
	mkdir -p "${BUILDDIR}"

binary: builddir
	cargo build --release
	cp target/release/server "${BUILDDIR}/server"

# Documents everything, not just the public-facing APIs.
devdocs:
	cargo rustdoc --open -- --no-defaults --passes collapse-docs --passes unindent-comments --passes strip-priv-imports

data: builddir
	mkdir -p "${BUILDDIR}/templates"
	cp -r templates/* "${BUILDDIR}/templates/"
	mkdir -p "${BUILDDIR}/translations"
	cp -r translations/* "${BUILDDIR}/translations/"
	mkdir -p "${BUILDDIR}/opldb"
	cp ../build/lifters.csv "${BUILDDIR}/opldb/lifters.csv"
	cp ../build/meets.csv "${BUILDDIR}/opldb/meets.csv"
	cp ../build/entries.csv "${BUILDDIR}/opldb/entries.csv"
	../scripts/csv-rmcol "${BUILDDIR}/opldb/entries.csv" Name

config: builddir
	cp server-production.env "${BUILDDIR}/server.env"
	cp Rocket.toml "${BUILDDIR}/Rocket.toml"

client:
	$(MAKE) -C client

clientstatics: client
	mkdir -p "${BUILDDIR}/static"
	cp -r client/build/* "${BUILDDIR}/static"
	rm "${BUILDDIR}/templates/static-asset-map.tera"
	cp client/build/static-asset-map.tera "${BUILDDIR}/templates/"

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
