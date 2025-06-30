.PHONY: install run versions

install:
	cargo binstall wash

run:
	wash app validate wadm.yaml
	wash up -d
	wash app deploy wadm.yaml
	wash ui

versions:
	wash --version
