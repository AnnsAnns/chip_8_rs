web:
	# Create www
	mkdir -p www

	# Compile
	cargo build --target wasm32-unknown-unknown --lib
	tsc src_web/*.ts

	# Move files
	cp target/wasm32-unknown-unknown/debug/chip_8_rs.wasm www/chip_8_rs.wasm
	cp src_web/index.html www/index.html
	cp src_web/handler.js www/handler.js
	cp src_web/server.py www/server.py

	# Start webserver
	cd www && python server.py

clean:
	rm -rf www
	rm src_web/handler.js

debug:
	cargo build

release:
	cargo build --release