install_rust_toolchain:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

run_receiver:
	@printf "running receiver with default settings\n" && \
	printf "listening_port: 9999\n" && \
	printf "window_size: 1\n" && \
	printf "outfile: outfile.txt\n" && \
	cargo run --bin acp_receiver -- -p 9999 -w 1 -f outfile.txt

run_sender:
	@printf "running sender with default settings\n" && \
	printf "destination_name: 127.0.0.1\n" && \
	printf "destination_port: 9999\n" && \
	printf "window_size: 1\n" && \
	printf "infile: infile.txt\n" && \
	cargo run --bin acp_sender -- -d 127.0.0.1 -p 9999 -w 1 -f infile.txt
