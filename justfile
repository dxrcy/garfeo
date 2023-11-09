# Runs `watch` and `listen`
serve port='4000':
	netstat -tuln | grep -q {{port}} || \
	( just watch & just listen {{port}} ) \
	&& echo 'port {{port}} is already in use'

# Watch and compile for development
watch:
	cargo watch -c -x 'run -- local' -i build -w . -w static

# Open a local http server
listen port='4000':
	basic-http-server build -a 127.0.0.1:{{port}}

# Install all dependencies
install:
	cargo install cargo-watch basic-http-server

# Build for production
build:
	cargo run

