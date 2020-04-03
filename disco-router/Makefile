.PHONY: run debug

DEBUG=

run:
	cargo build
	sudo setcap cap_net_raw=ep target/debug/disco
	$(DEBUG) ./target/debug/disco

debug: DEBUG=gdb
debug: run
	@echo '' >/dev/null