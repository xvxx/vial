FEATURES = horror,state,cookies,sessions,default

.PHONY: test
test:
	cargo test --features $(FEATURES)
	cargo test --features json_serde --test json_test
	cargo test --features json_nano --test json_test

.PHONY: build
build:
	cargo build

.PHONY: docs
docs: docs/index.html
	cargo doc --features $(FEATURES)
	cargo doc --features json_serde
	cargo doc --features json_nano

# The Vial docs were originally built using:
# pandoc 2.8
# Compiled with pandoc-types 1.20, texmath 0.12, skylighting 0.8.2.3
deps: check
	@mkdir -p target/docs
	@(which pandoc) > /dev/null || (echo "Need pandoc(1) installed"; exit 1)
	@(which ruby) > /dev/null || (echo "Need ruby(1) installed"; exit 1)
	@(which md-toc) > /dev/null || (echo "Need md-toc(1): cargo install markdown-toc"; exit 1)

docs/index.html: deps target/docs/toc.html docs/manual.tpl docs/MANUAL.md
	@echo "building docs/index.html..."
	@pandoc docs/MANUAL.md -o target/docs/manual.html
	@ruby -e 'File.write("docs/index.html", File.read("docs/manual.tpl").sub("{body}", File.read("target/docs/manual.html")).sub("{toc}", File.read("target/docs/toc.html")))'
	@echo "DONE"

target/docs/toc.html: deps docs/MANUAL.md docs/manual.tpl
	@echo "building target/docs/toc.html..."
	@md-toc --header '### Sections' --min-depth 1 --max-depth 2 --bullet - docs/MANUAL.md  > target/docs/toc.md
	@ruby -e 'toc = File.read("target/docs/toc.md"); idx = toc.index("- [Templates]"); toc.insert(idx, "\n### Bonus Features\n"); File.write("target/docs/toc.md", toc)'
	@pandoc target/docs/toc.md -o target/docs/toc.html
	@pandoc docs/links.md >> target/docs/toc.html
	@echo "DONE"

.PHONY: clean
clean:
	-cargo clean

.PHONY: clippy
clippy:
	cargo clippy --features $(FEATURES)

.PHONY: check
check:
	cargo check --features $(FEATURES)
	cargo check --features json_serde
	cargo check --features json_nano