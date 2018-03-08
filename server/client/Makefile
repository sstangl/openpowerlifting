.PHONY: npminit builddir css

BUILDDIR := build

all: css

# Node v8.7.0 fails on redundant `npm install` due to package error.
npminit:
	if [ ! -d "node_modules" ]; then npm install; fi

builddir:
	mkdir -p '${BUILDDIR}'

css: builddir npminit
	npm run sass
