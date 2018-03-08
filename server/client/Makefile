.PHONY: npminit builddir css oldimages

BUILDDIR := build
OLDWEB := "../../web"
PROJECTDATA := "../../project-data"

all: css oldimages

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
