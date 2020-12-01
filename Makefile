
all:
	rm -rf dist/
	mkdir dist/
	cp index.html dist/
	cp -r css dist/
	wasm-pack --target web --out-name package
	cp -r pkg/ dist/
