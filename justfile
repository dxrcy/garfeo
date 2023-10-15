serve:
	cargo watch -x 'run -- local' -i build &\
	basic-http-server build

install:
	cargo install cargo-watch basic-http-server

