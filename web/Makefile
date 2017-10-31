.PHONY: builddir csvfile check

BUILDDIR := build
SCRIPTDIR := ../scripts

PLFILE := ../build/openpowerlifting.csv
PLFILEJS := openpowerlifting.js
MEETFILE := ../build/meets.csv
MEETFILEJS := meets.js

all: database npminit scripts css html images

npminit:
	npm install

builddir:
	mkdir -p '${BUILDDIR}'
	mkdir -p '${BUILDDIR}/static'
	mkdir -p '${BUILDDIR}/static/scripts'
	mkdir -p '${BUILDDIR}/static/images'
	mkdir -p '${BUILDDIR}/static/data'

# Copies the OPL database, and formats it for the Web.
# A number of columns are omitted to save data.
data: builddir
	cp ${PLFILE} ${BUILDDIR}/static/data
	${SCRIPTDIR}/csv-rmcol "${BUILDDIR}/static/data/openpowerlifting.csv" Squat1Kg Squat2Kg Squat3Kg Bench1Kg Bench2Kg Bench3Kg Deadlift1Kg Deadlift2Kg Deadlift3Kg McCulloch Event
	cp ${MEETFILE} ${BUILDDIR}/static/data

database: data
	${SCRIPTDIR}/csv-to-js "${BUILDDIR}/static/data/openpowerlifting.csv" opldb > "${BUILDDIR}/static/scripts/${PLFILEJS}"
	sed -i -e 's/,"M",/,0,/g' "${BUILDDIR}/static/scripts/${PLFILEJS}"
	sed -i -e 's/,"F",/,1,/g' "${BUILDDIR}/static/scripts/${PLFILEJS}"
	sed -i -e 's/,"Raw",/,0,/g' "${BUILDDIR}/static/scripts/${PLFILEJS}"
	sed -i -e 's/,"Wraps",/,1,/g' "${BUILDDIR}/static/scripts/${PLFILEJS}"
	sed -i -e 's/,"Single-ply",/,2,/g' "${BUILDDIR}/static/scripts/${PLFILEJS}"
	sed -i -e 's/,"Multi-ply",/,3,/g' "${BUILDDIR}/static/scripts/${PLFILEJS}"
	sed -i -e 's/,"Straps",/,4,/g' "${BUILDDIR}/static/scripts/${PLFILEJS}"
	${SCRIPTDIR}/csv-to-js "${MEETFILE}" meetdb > "${BUILDDIR}/static/scripts/${MEETFILEJS}"
	./make-social-media-js > "${BUILDDIR}/static/scripts/social-media.js"

scripts: builddir
	npm run webpack
	cat scripts/SlickGrid-2.2.6/lib/jquery-1.11.2.min.js \
	    scripts/SlickGrid-2.2.6/lib/jquery.event.drag-2.2.js \
	    scripts/SlickGrid-2.2.6/lib/jquery.event.drop-2.2.js \
	    scripts/SlickGrid-2.2.6/slick.core.js \
	    scripts/SlickGrid-2.2.6/slick.grid.js \
	    | ./try-uglify ${BUILDDIR}/static/scripts/jquery-slickgrid-2.2.6.min.js

css: builddir
	npm run sass
	./gen-meettable-css > ${BUILDDIR}/static/meettable.css
	cp scripts/SlickGrid-2.2.6/slick.grid.css ${BUILDDIR}/static/slick.grid.css

html: builddir data
	./preprocess-html templates/index.html > ${BUILDDIR}/index.html
	./preprocess-html templates/lifters.html > ${BUILDDIR}/lifters.html
	./preprocess-html templates/meet.html > ${BUILDDIR}/meet.html
	./preprocess-html templates/meetlist.html > ${BUILDDIR}/meetlist.html
	./preprocess-html templates/data.html > ${BUILDDIR}/data.html
	./preprocess-html templates/contact.html > ${BUILDDIR}/contact.html

images: builddir
	cp images/check_radio_sheet.png ${BUILDDIR}/static/images
	cp images/select_dropdown.png ${BUILDDIR}/static/images
	cp images/patreon_navigation_logo_mini_orange.png "${BUILDDIR}/static/images"

check:
	echo "Nothing to check for the moment."

clean:
	rm -rf '${BUILDDIR}'
	rm -rf __pycache__
