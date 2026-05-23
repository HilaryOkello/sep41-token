-include .env

all: test

test: build
	cargo test

debug: build
	RUST_BACKTRACE=full cargo test

t: test
	RUST_BACKTRACE=1 cargo test -- --nocapture

build:
	stellar contract build
	@ls -l ../../target/wasm32v1-none/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean

upload: build
	@echo "Uploading contract..."
	@stellar contract install \
	  --network $(TESTNET) \
	  --source $(SOURCE_TESTNET) \
	  --wasm ../../target/wasm32v1-none/release/sep41_token.wasm > upload.log 2>&1 || (cat upload.log && exit 1)
	@grep -oE "[a-f0-9]{64}" upload.log | tail -n 1 > wasm_hash.txt
	@echo "Wasm hash saved to wasm_hash.txt"
	@cat wasm_hash.txt

deploy: upload
	@stellar contract deploy \
	  --wasm-hash $$(cat wasm_hash.txt) \
	  --network $(TESTNET) \
	  --source $(SOURCE_TESTNET) \
	  -- \
	  --admin $(ADMIN) \
	  --initial_supply $(INITIAL_SUPPLY) > deploy.log 2>&1 || (cat deploy.log && exit 1)
	@grep -oE "C[A-Z0-9]{55}" deploy.log | tail -n 1 > contract_id.txt
	@echo "Contract deployed:"
	@cat contract_id.txt

balance:
	stellar contract invoke \
		--id $$(cat contract_id.txt) \
		--source $(SOURCE_TESTNET) \
		--network $(TESTNET) \
		-- \
		balance \
		--id $(ADMIN)

interact:
	stellar contract invoke \
		--id $$(cat contract_id.txt) \
		--source $(SOURCE_TESTNET) \
		--network $(TESTNET) \
		-- \
		total_supply
