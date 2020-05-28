.PHONY: test
test:
	cargo test

.PHONY: build
build:
	cargo build

.PHONY: docs
docs: docs/index.html

check:
	@mkdir -p target/docs
	@(which -s pandoc) || (echo "Need pandoc(1) installed"; exit 1)
	@(which -s ruby) || (echo "Need ruby(1) installed"; exit 1)
	@(which -s md-toc) || (echo "Need md-toc(1): cargo install markdown-toc"; exit 1)

docs/index.html: check target/docs/toc.html docs/manual.tpl docs/MANUAL.md
	@echo "building docs/index.html..."
	@pandoc docs/MANUAL.md -o target/docs/manual.html
	@ruby -e 'File.write("docs/index.html", File.read("docs/manual.tpl").sub("{body}", File.read("target/docs/manual.html")).sub("{toc}", File.read("target/docs/toc.html")))'
	@echo "DONE"

target/docs/toc.html: check docs/MANUAL.md docs/manual.tpl
	@echo "building target/docs/toc.html..."
	@md-toc --header '### Core' --min-depth 1 --max-depth 2 --bullet - docs/MANUAL.md  > target/docs/toc.md
	@ruby -e 'toc = File.read("target/docs/toc.md"); idx = toc.index("- [Templates]"); toc.insert(idx, "\n### Bonus Features\n"); File.write("target/docs/toc.md", toc)'
	@pandoc target/docs/toc.md -o target/docs/toc.html
	@echo "DONE"

clean:
	-cargo clean