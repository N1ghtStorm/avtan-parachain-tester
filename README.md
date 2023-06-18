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

<!-- ./target/release/avtan-node build-spec --disable-default-bootnode > rococo-local-parachain-plain.json
./target/release/avtan-node build-spec --chain rococo-local-parachain-plain.json --raw --disable-default-bootnode > rococo-local-parachain-2666-raw.json

./target/release/avtan-node  export-genesis-wasm --chain rococo-local-parachain-2666-raw.json > para-2666-wasm
./target/release/avtan-node  export-genesis-state --chain rococo-local-parachain-2666-raw.json > para-2666-genesis -->

# Start Node

<!-- rm -rf /tmp/parachain/avtan-parachain-alice

./target/release/avtan-node \
--alice \
--collator \
--force-authoring \
--chain rococo-local-parachain-2666-raw.json \
--base-path /tmp/parachain/avtan-parachain-alice \
--port 50534 \
--ws-port 18866 \
--pruning=archive --enable-offchain-indexing true \
-- \
--execution wasm \
--chain ../polkadot/rococo_local_raw.json \
--port 30355 \
--ws-port 19979 -->