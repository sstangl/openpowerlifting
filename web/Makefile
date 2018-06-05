.PHONY: builddir csvfile check

BUILDDIR := build
SCRIPTDIR := ../scripts

PLFILE := ../build/openpowerlifting.csv
PLFILEJS := openpowerlifting.js
MEETFILE := ../build/meets.csv
MEETFILEJS := meets.js
LOGO := ../project-data/logo-compressed.svg
FAVICON := ../project-data/favicon-32x32.ico

all: database scripts css html images

# Node v8.7.0 fails on redundant `npm install` due to package error.
npminit:
	if [ ! -d "node_modules" ]; then npm install; fi

builddir:
	mkdir -p '${BUILDDIR}'
	mkdir -p '${BUILDDIR}/static'
	mkdir -p '${BUILDDIR}/static/scripts'
	mkdir -p '${BUILDDIR}/static/images'

# Copies the OPL database, and formats it for the Web.
# A number of columns are omitted to save data.
data: builddir
	rm -f "${BUILDDIR}/static/openpowerlifting.zip"
	zip --junk-paths "${BUILDDIR}/static/openpowerlifting" "${PLFILE}" "${MEETFILE}"

database: data
	./make-openpowerlifting-js > ${BUILDDIR}/static/scripts/${PLFILEJS}
	${SCRIPTDIR}/csvtojs.py "${MEETFILE}" meetdb > "${BUILDDIR}/static/scripts/${MEETFILEJS}"
	./make-social-media-js > "${BUILDDIR}/static/scripts/social-media.js"

scripts: builddir npminit
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
	./preprocess-html templates/faq.html > ${BUILDDIR}/faq.html

images: builddir
	cp images/check_radio_sheet.png ${BUILDDIR}/static/images
	cp images/select_dropdown.png ${BUILDDIR}/static/images
	cp images/patreon_navigation_logo_mini_orange.png "${BUILDDIR}/static/images"
	cp "${LOGO}" "${BUILDDIR}/static/images/"
	cp "${FAVICON}" "${BUILDDIR}/favicon.ico"

check:
	echo "Nothing to check for the moment."

clean:
	rm -rf '${BUILDDIR}'
	rm -rf __pycache__
	rm -rf node_modules
