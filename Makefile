METADATA_TMP_FILE := $(shell mktemp)

gasp-bindings:
	subxt metadata -f bytes -o $(METADATA_TMP_FILE) --url ws://127.0.0.1:9944
	echo "#[allow(non_snake_case)]" > bindings/src/lib.rs
	subxt codegen --attribute "#[allow(non_snake_case)]" --derive Clone --derive PartialEq --file $(METADATA_TMP_FILE) | rustfmt --edition=2018 --emit=stdout >> bindings/src/bindings.rs
