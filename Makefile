.PHONY: install

INSTALL_PATH := /opt/fan-ctrl-rs

build:
	cargo build --release

install:
	mkdir -p ${INSTALL_PATH}
	# cargo install --path . --root ${INSTALL_PATH}
	install target/release/fan-ctrl-rs ${INSTALL_PATH}
	install fan-ctrl-rs.service ${INSTALL_PATH}

enable:
	systemctl enable --now /opt/fan-ctrl-rs/fan-ctrl-rs.service
