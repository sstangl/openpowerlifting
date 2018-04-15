.PHONY: npminit builddir css js oldimages slickgrid

BUILDDIR := build
OLDWEB := "../../web"
PROJECTDATA := "../../project-data"

all: css js oldimages

# Node v8.7.0 fails on redundant `npm install` due to package error.
npminit:
	if [ ! -d "node_modules" ]; then npm install; fi

builddir:
	mkdir -p '${BUILDDIR}'

css: builddir npminit
	npm run sass

oldimages: builddir
	mkdir -p "${BUILDDIR}"
	mkdir -p "${BUILDDIR}/images"
	cp ${OLDWEB}/images/check_radio_sheet.png "${BUILDDIR}/images/"
	cp ${PROJECTDATA}/logo-compressed.svg "${BUILDDIR}/images/"
	cp ${OLDWEB}/images/patreon_navigation_logo_mini_orange.png "${BUILDDIR}/images/"
	cp ${OLDWEB}/images/select_dropdown.png "${BUILDDIR}/images/"

js: builddir slickgrid
	cp js/* ${BUILDDIR}/

# FIXME -- this is a temporary thing while I'm on a plane.
# FIXME -- the real thing should pull slickgrid from NPM or something.
slickgrid: builddir
	cp ../../web/scripts/SlickGrid-2.2.6/slick.grid.css ${BUILDDIR}/slickgrid.css
	cat ../../web/scripts/SlickGrid-2.2.6/lib/jquery-1.11.2.min.js \
            ../../web/scripts/SlickGrid-2.2.6/lib/jquery.event.drag-2.2.js \
            ../../web/scripts/SlickGrid-2.2.6/lib/jquery.event.drop-2.2.js \
            ../../web/scripts/SlickGrid-2.2.6/slick.core.js \
            ../../web/scripts/SlickGrid-2.2.6/slick.grid.js \
            | ../../web/try-uglify ${BUILDDIR}/slickgrid.min.js
