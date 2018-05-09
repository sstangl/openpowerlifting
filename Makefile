.PHONY: builddir csvfile benchdata check probe

DATADIR := meet-data
BUILDDIR := build

PLFILE := openpowerlifting.csv
PLFILEJS := openpowerlifting.js
MEETFILE := meets.csv
MEETFILEJS := meets.js

all: csvfile web

builddir:
	mkdir -p '${BUILDDIR}'

# Cram all the data into a single, huge CSV file.
csvfile: builddir
	scripts/compile "${BUILDDIR}" "${DATADIR}" "lifter-data"
	scripts/csv-bodyweight "${BUILDDIR}/${PLFILE}"
	scripts/csv-wilks "${BUILDDIR}/${PLFILE}"

# Generate a large amount of test data: entries.csv with > 10million entries.
benchdata: builddir
	mkdir -p "${BUILDDIR}/bench-data"
	scripts/compile-for-benchmarking "${BUILDDIR}/bench-data" "${DATADIR}" "lifter-data"
	scripts/csv-bodyweight "${BUILDDIR}/bench-data/${PLFILE}"
	scripts/csv-wilks "${BUILDDIR}/bench-data/${PLFILE}"

# Optionally build an SQLite3 version of the database.
sqlite: csvfile
	scripts/prepare-for-sqlite
	scripts/compile-sqlite

web: csvfile
	$(MAKE) -C web

# Make sure that all the fields in the CSV files are in expected formats.
check:
	tests/check-entries-csv
	tests/check-meet-csv
	tests/check-sex-consistency
	tests/check-lifter-data
	tests/check-duplicates
	tests/check-python-style

# List of probes for federations that should be fully up-to-date,
# or at least are quick to read and not filled with noise.
# Data showing up here should be immediately actionable.
probe-quick:
	${DATADIR}/aau/aau-probe --quick || true
	${DATADIR}/aep/aep-probe --quick || true
	${DATADIR}/apa/apa-probe || true
	${DATADIR}/apc/apc-probe --quick || true
	${DATADIR}/apf/apf-probe --quick || true
	${DATADIR}/apu/apu-probe --quick || true
	${DATADIR}/bb/bb-probe || true
	${DATADIR}/bp/bp-probe --quick || true
	${DATADIR}/bp/bpu-probe --quick || true
	${DATADIR}/capo/capo-probe --quick || true
	${DATADIR}/commonwealthpf/commonwealthpf-probe || true
	${DATADIR}/cpf/cpf-probe --quick || true
	${DATADIR}/cpl/cpl-probe --quick || true
	${DATADIR}/femepo/femepo-probe --quick || true
	${DATADIR}/fesupo/fesupo-probe --quick || true
	${DATADIR}/gpc-aus/gpc-aus-probe --quick || true
	${DATADIR}/herc/herc-probe || true
	${DATADIR}/ipa/ipa-probe --quick || true
	${DATADIR}/irishpf/irishpf-probe || true
	${DATADIR}/irishpo/irishpo-probe --quick || true
	${DATADIR}/nasa/nasa-probe --quick || true
	${DATADIR}/nipf/nipf-probe || true
	${DATADIR}/nzpf/nzpf-probe --quick || true
	${DATADIR}/oceaniapf/oceaniapf-probe --quick || true
	${DATADIR}/pa/pa-probe --quick || true
	${DATADIR}/plzs/plzs-probe --quick || true
	${DATADIR}/rps/rps-probe || true
	${DATADIR}/rupc/rupc-probe || true
	${DATADIR}/scottishpl/scottishpl-probe --quick || true
	${DATADIR}/spf/spf-probe || true
	${DATADIR}/upa/upa-probe --quick || true
	${DATADIR}/usapl/usapl-probe || true
	${DATADIR}/usapl-archive/usapl-archive-probe --quick || true
	${DATADIR}/uspa/uspa-probe || true
	${DATADIR}/xpc/xpc-probe || true
	${DATADIR}/wrpf-can/wrpf-can-probe --quick || true

# List of all probes.
probe:
	${DATADIR}/aau/aau-probe || true
	${DATADIR}/aep/aep-probe || true
	${DATADIR}/apa/apa-probe || true
	${DATADIR}/apc/apc-probe || true
	${DATADIR}/apf/apf-probe || true
	${DATADIR}/apu/apu-probe || true
	${DATADIR}/bb/bb-probe || true
	${DATADIR}/bp/bp-probe || true
	${DATADIR}/bp/bpu-probe || true
	${DATADIR}/capo/capo-probe || true
	${DATADIR}/commonwealthpf/commonwealthpf-probe || true
	${DATADIR}/cpf/cpf-probe || true
	${DATADIR}/cpl/cpl-probe || true
	${DATADIR}/epf/epf-probe || true
	${DATADIR}/femepo/femepo-probe || true
	${DATADIR}/fesupo/fesupo-probe || true
	${DATADIR}/fpo/fpo-probe || true
	${DATADIR}/gpc-aus/gpc-aus-probe || true
	${DATADIR}/herc/herc-probe || true
	${DATADIR}/ipa/ipa-probe || true
	${DATADIR}/ipf/ipf-probe || true
	${DATADIR}/irishpf/irishpf-probe || true
	${DATADIR}/irishpo/irishpo-probe || true
	${DATADIR}/napf/napf-probe || true
	${DATADIR}/nasa/nasa-probe || true
	${DATADIR}/nipf/nipf-probe || true
	${DATADIR}/nsf/nsf-probe || true
	${DATADIR}/nzpf/nzpf-probe || true
	${DATADIR}/oceaniapf/oceaniapf-probe || true
	${DATADIR}/pa/pa-probe || true
	${DATADIR}/plzs/plzs-probe || true
	${DATADIR}/raw/raw-probe || true
	${DATADIR}/rps/rps-probe || true
	${DATADIR}/rupc/rupc-probe || true
	${DATADIR}/scottishpl/scottishpl-probe || true
	${DATADIR}/spf/spf-probe || true
	${DATADIR}/thspa/thspa-probe || true
	${DATADIR}/upa/upa-probe || true
	${DATADIR}/usapl/usapl-probe || true
	${DATADIR}/usapl-archive/usapl-archive-probe || true
	${DATADIR}/uspa/uspa-probe || true
	${DATADIR}/wrpf/wrpf-probe || true
	${DATADIR}/wrpf-can/wrpf-can-probe || true
	${DATADIR}/xpc/xpc-probe || true

clean:
	rm -rf '${BUILDDIR}'
	rm -rf 'scripts/__pycache__'
	rm -rf 'tests/__pycache__'
	rm -rf '${DATADIR}/apf/__pycache__'
	rm -rf '${DATADIR}/cpu/__pycache__'
	rm -rf '${DATADIR}/ipf/__pycache__'
	rm -rf '${DATADIR}/nasa/__pycache__'
	rm -rf '${DATADIR}/nipf/__pycache__'
	rm -rf '${DATADIR}/nsf/__pycache__'
	rm -rf '${DATADIR}/pa/__pycache__'
	rm -rf '${DATADIR}/rps/__pycache__'
	rm -rf '${DATADIR}/spf/__pycache__'
	rm -rf '${DATADIR}/thspa/__pycache__'
	rm -rf '${DATADIR}/usapl/__pycache__'
	rm -rf '${DATADIR}/wrpf/__pycache__'
	$(MAKE) -C server clean
	$(MAKE) -C web clean
