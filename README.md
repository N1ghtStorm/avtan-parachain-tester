# Avtan Rooster Parachain Ko Ko Ko 

                        ~-.
                        ,,,;            ~-.~-.~-
                    (.../           ~-.~-.~-.~-.~-.
                < } O~`, ,        ~-.~-.~-.~-.~-.~-.
                    (/    T ,     ~-.~-.~-.~-.~-.~-.~-.
                        ;    T     ~-.~-.~-.~-.~-.~-.~-.
                      ;   {_.~-.~-.~-.~-.~-.~-.~
                    ;:  .-~`    ~-.~-.~-.~-.~-.
                    ;.: :'    ._   ~-.~-.~-.~-.~-
                    ;::`-.    '-._  ~-.~-.~-.~-
                    ;::. `-.    '-,~-.~-.~-.
                        ';::::.`''-.-'
                        ';::;;:,:'
                            '||T
                            / |
                          __   _

# Generate chainspec and wasm validation code:

./target/release/parachain-tester build-spec --disable-default-bootnode > rococo-local-parachain-plain.json
./target/release/parachain-tester build-spec --chain rococo-local-parachain-plain.json --raw --disable-default-bootnode > rococo-local-parachain-2000-raw.json

./target/release/parachain-tester  export-genesis-wasm --chain rococo-local-parachain-2000-raw.json > para-2000-wasm
./target/release/parachain-tester  export-genesis-state --chain rococo-local-parachain-2000-raw.json > para-2000-genesis

# Start Node

rm -rf /tmp/parachain/parachain-tester-alice

./target/release/parachain-tester \
--alice \
--collator \
--force-authoring \
--chain rococo-local-parachain-2000-raw.json \
--base-path /tmp/parachain/parachain-tester-alice \
--port 50534 \
--ws-port 18866 \
--pruning=archive --enable-offchain-indexing true \
-- \
--execution wasm \
--chain ../polkadot/rococo_local_raw.json \
--port 30355 \
--ws-port 19979