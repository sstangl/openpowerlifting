.PHONY: npminit builddir css scripts oldimages slickgrid gzip clean

BUILDDIR := build
OLDWEB := "../../web"
PROJECTDATA := "../../project-data"

all: css scripts oldimages gzip

# Node v8.7.0 fails on redundant `npm install` due to package error.
npminit:
	if [ ! -d "node_modules" ]; then npm install; fi

builddir:
	mkdir -p '${BUILDDIR}'
	mkdir -p '${BUILDDIR}/css'
	mkdir -p '${BUILDDIR}/images'
	mkdir -p '${BUILDDIR}/scripts'

css: builddir npminit
	npm run sass

oldimages: builddir
	cp ${OLDWEB}/images/check_radio_sheet.png "${BUILDDIR}/images/"
	cp ${PROJECTDATA}/logo-compressed.svg "${BUILDDIR}/images/"
	cp ${OLDWEB}/images/patreon_navigation_logo_mini_orange.png "${BUILDDIR}/images/"
	cp ${OLDWEB}/images/select_dropdown.png "${BUILDDIR}/images/"
	cp ${PROJECTDATA}/favicon-32x32.ico "${BUILDDIR}/images/favicon.ico"

scripts: builddir npminit slickgrid
	npm run webpack

SLICKGRID := "node_modules/slickgrid"
slickgrid: builddir
	cat "${SLICKGRID}/lib/jquery-1.11.2.min.js" \
	    "${SLICKGRID}/lib/jquery.event.drag-2.3.0.js" \
	    "${SLICKGRID}/lib/jquery.event.drop-2.3.0.js" \
	    "${SLICKGRID}/slick.core.js" \
	    "${SLICKGRID}/slick.grid.js" \
	    | "${OLDWEB}/try-uglify" "${BUILDDIR}/scripts/slickgrid.min.js"

gzip:
	./apply-gzip

clean:
	rm -rf build node_modules
