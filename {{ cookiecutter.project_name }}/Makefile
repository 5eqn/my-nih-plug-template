./target/bundled/{{ cookiecutter.plugin_name }}.vst3:
	cargo xtask bundle {{ cookiecutter.project_name }} --release

.PHONY: vst3
vst3: ./target/bundled/{{ cookiecutter.plugin_name }}.vst3
	cp "./target/bundled/{{ cookiecutter.plugin_name }}.vst3" ~/.vst3 -r
