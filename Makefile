# Makefile for Rust Lambdas
# NOTE: This file _must_ be indented using tabs or make will not parse it

# Build an AWS resources (will match build-ResourceName from template.yaml)
build-%:
	rustup target add x86_64-unknown-linux-musl
	AWS_RESOURCE_NAME=`echo $@ | rev | cut -d- -f1 | rev` TEMPLATE_YAML=`cat template.yaml` cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap $(ARTIFACTS_DIR)
