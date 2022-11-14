DEADNAME="Jack"
NEWNAME="Jill"
REPO="./repos/a/"
.PHONY: clean run

default: run

clean:
	echo $(TODO) 

run:
	cargo build && \
	./target/debug/chowndn version && \
	./target/debug/chowndn scan -- ${DEADNAME} ${REPO} && \
	./target/debug/chowndn replace -- ${DEADNAME} ${NEWNAME} ${REPO}

build:
	cargo fmt && \
	cargo build

test:
	cargo test
