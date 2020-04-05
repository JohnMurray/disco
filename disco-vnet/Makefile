.PHONY: run debug

DEBUG=

run:
	cargo build
	sudo setcap cap_net_raw=ep target/debug/disco-vnet
	$(DEBUG) ./target/debug/disco-vnet

debug: DEBUG=gdb
debug: run
	@echo '' >/dev/null