.PHONY: builddir csvfile check

BUILDDIR = build
SCRIPTDIR = ../scripts

PLFILE = ../build/openpowerlifting.csv
PLFILEJS = openpowerlifting.js
MEETFILE = ../build/meets.csv
MEETFILEJS = meets.js

all: database scripts css html images

builddir:
	mkdir -p '${BUILDDIR}'
	mkdir -p '${BUILDDIR}/scripts'
	mkdir -p '${BUILDDIR}/images'

# Depends on the main project already being built.
database: builddir
	${SCRIPTDIR}/csv-to-js "${PLFILE}" opldb > "${BUILDDIR}/scripts/${PLFILEJS}"
	${SCRIPTDIR}/csv-to-js "${MEETFILE}" meetdb > "${BUILDDIR}/scripts/${MEETFILEJS}"

scripts: builddir
	cp scripts/common.js ${BUILDDIR}/scripts
	cp scripts/database.js ${BUILDDIR}/scripts
	cp scripts/main-index.js ${BUILDDIR}/scripts
	cp scripts/main-lifters.js ${BUILDDIR}/scripts
	cp scripts/main-meet.js ${BUILDDIR}/scripts
	cat scripts/SlickGrid-2.2.6/lib/jquery-1.11.2.min.js \
	    scripts/SlickGrid-2.2.6/lib/jquery.event.drag-2.2.js \
	    scripts/SlickGrid-2.2.6/lib/jquery.event.drop-2.2.js \
	    scripts/SlickGrid-2.2.6/slick.core.js \
	    scripts/SlickGrid-2.2.6/slick.grid.js \
	    | uglifyjs -o ${BUILDDIR}/scripts/jquery-slickgrid-2.2.6.min.js

css: builddir
	cp css/style.css ${BUILDDIR}/style.css
	cp scripts/SlickGrid-2.2.6/slick.grid.css ${BUILDDIR}/slick.grid.css

html: builddir
	./preprocess-html templates/index.html > ${BUILDDIR}/index.html
	./preprocess-html templates/lifters.html > ${BUILDDIR}/lifters.html
	./preprocess-html templates/meet.html > ${BUILDDIR}/meet.html
	./preprocess-html templates/meetlist.html > ${BUILDDIR}/meetlist.html

images: builddir
	cp images/check_radio_sheet.png ${BUILDDIR}/images
	cp images/select_dropdown.png ${BUILDDIR}/images

check:
	echo "Nothing to check for the moment."

clean:
	rm -rf '${BUILDDIR}'
	rm -rf __pycache__
