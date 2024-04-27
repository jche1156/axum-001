buildrun:
	tailwindcss -i input.css -o assets/app.css
	cargo run

watch:
	watchexec -r just buildrun

fmt:
	cargo fmt
	rustywind --write .
