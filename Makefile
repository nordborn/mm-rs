run_cargo:
	cargo run --bin singlepairbot --  -b exmo_nb_single

run_cargo_hotpath:
	cargo run --features hotpath,hotpath-alloc --bin singlepairbot --  -b exmo_nb_single

run_dev:
	target/debug/singlepairbot -b test_single