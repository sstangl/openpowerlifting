.PHONY: npminit forcenpm builddir css scripts images slickgrid package-for-server rebuild-clean clean

BUILDDIR := build
PROJECTDATA := "../../project-data"
SERVERDIR := ".."

all: rebuild-clean css scripts html images package-for-server

# Node v8.7.0 fails on redundant `npm install` due to package error.
npminit:
	if [ ! -d "node_modules" ]; then npm install; fi

forcenpm:
	npm install

builddir:
	mkdir -p '${BUILDDIR}'
	mkdir -p '${BUILDDIR}/css'
	mkdir -p '${BUILDDIR}/images'
	mkdir -p '${BUILDDIR}/images/openipf'
	mkdir -p '${BUILDDIR}/scripts'

css: builddir npminit
	npm run sass
	npm run sass-openipf

images: builddir
	cp ${PROJECTDATA}/logo-on-white.svg "${BUILDDIR}/images/logo-compressed.svg"
	cp ${PROJECTDATA}/logo-on-black.svg "${BUILDDIR}/images/logo-on-black.svg"
	cp ${PROJECTDATA}/logo-gold-on-black.svg "${BUILDDIR}/images/logo-gold-on-black.svg"
	cp ${PROJECTDATA}/x-ipf-gold.svg "${BUILDDIR}/images/x-ipf-gold.svg"
	cp images/select_dropdown_red.png "${BUILDDIR}/images/"
	cp images/select_dropdown_gold.png "${BUILDDIR}/images/"
	cp images/mobile_filter.svg "${BUILDDIR}/images/"
	cp images/mobile_filter_white.svg "${BUILDDIR}/images/"
	cp images/search.png "${BUILDDIR}/images/"
	cp images/search_white.png "${BUILDDIR}/images/"
	cp images/close_icon.png "${BUILDDIR}/images/"
	cp images/close_icon_white.png "${BUILDDIR}/images/"
	cp images/burger_menu.png "${BUILDDIR}/images/"
	cp images/burger_menu_white.png "${BUILDDIR}/images/"
	cp ${PROJECTDATA}/favicon-32x32.ico "${BUILDDIR}/images/favicon.ico"
	cp ${PROJECTDATA}/apple-touch-icon.png "${BUILDDIR}/images/"
	cp ${PROJECTDATA}/favicon-32x32-gold.ico "${BUILDDIR}/images/openipf/favicon.ico"
	cp ${PROJECTDATA}/apple-touch-icon-gold.png "${BUILDDIR}/images/openipf/apple-touch-icon.png"

scripts: builddir npminit slickgrid
	npm run webpack

html: builddir
	cp html/* "${BUILDDIR}/"

SLICKGRID := "node_modules/slickgrid"
slickgrid: builddir
	cat "${SLICKGRID}/lib/jquery-1.11.2.min.js" \
	    "${SLICKGRID}/lib/jquery.event.drag-2.3.0.js" \
	    "${SLICKGRID}/lib/jquery.event.drop-2.3.0.js" \
	    "${SLICKGRID}/slick.core.js" \
	    "${SLICKGRID}/slick.grid.js" \
	    | ./try-uglify "${BUILDDIR}/scripts/slickgrid.min.js"

package-for-server:
	# Rewrite some filenames to contain their hash.
	./apply-md5
	# Gzip the contents. The hashed filename is not the hash of the gzip,
	# but the hash of the gzip's contents when decompressed.
	./apply-gzip
	# Copy asset map file to server templates folder (ignored by git).
	mkdir -p "${SERVERDIR}/templates/build"
	cp "${BUILDDIR}/static-asset-map.tera" "${SERVERDIR}/templates/build/"

# Cleans files that may cause `make` to be non-idempotent.
rebuild-clean:
	rm -rf build

# Fully cleans the working directory.
clean:
	rm -rf build node_modules
