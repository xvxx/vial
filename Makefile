.PHONY: test
test:
	cargo test

.PHONY: build
build:
	cargo build

.PHONY: docs
docs: docs/manual.html

check:
	@(which -s pandoc) || (echo "Need pandoc(1) installed"; exit 1)
	@(which -s ruby) || (echo "Need ruby(1) installed"; exit 1)

docs/manual.html: check docs/manual.tpl
	pandoc docs/MANUAL.md -o rendered.html
	@ruby -e 'File.open("docs/manual.html", "w") { |f| f.write(File.read("docs/manual.tpl").sub("{{body}}", File.read("rendered.html"))) }'
	@-rm rendered.html
	@echo "built docs/manual.html"

clean:
	-rm docs/manual.html