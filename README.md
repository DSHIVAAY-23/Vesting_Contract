# Vesting_Contract
COSMWASM  CONTRACT
instantiate is to set up the data like constructor Execute - is to write the data Query - is the to read the data These are basics entrypoint for simple contract other then that We have Migrate - to migrate contract data from contract if you change versions Reply - to wait for rply Pseudo - used in governance contracts allow governance to interact with the contract

.commands to run the wasmdd node.

./wasmd init wasmnode --chain-id=wasm_904-3

./wasmd keys add bob --keyring-backend test

./wasmd genesis add-genesis-account bob 10000000000000000000000000000000000token,10000000000000000000000000000000000stake --keyring-backend test

./wasmd genesis gentx bob 100000000000000000stake --keyring-backend test

./wasmd genesis collect-gentxs

To compile the contract

cargo build --target wasm32-unknown-unknown --release

To store the tx–
wasmd tx wasm store /data/cw-contracts/contracts/empty-contract/target/wasm32-unknown-unknown/release/voting.wasm --from bob --gas auto --gas-prices="1905stake" --gas-adjustment=1.2 --output json --yes --keyring-backend test | jq

From here we get the code id and further paste it in instantiate command and  we give admin address to

To initiate the tranx



wasmd tx wasm instantiate 1 '{"admin_address":"wasm1f79zr5q4k0tmrrm4lqg4dv3eyq5z4aq75693ff"}' --from bob --keyring-backend test --label contract --gas auto --gas-adjustment=1.2 --gas-prices=1905stake --output json --no-admin --yes | jq


We get contract address from her 
Ex- wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d
And we use it in execute mssg


To execute the transaction
wasmd tx wasm execute wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d '{"create_poll":"Is 2+2=5 ?"}' --from bob --keyring-backend test  --
output json  --yes | jq




We will check the state of contract from here 
http://localhost:26657/


//localhost:26657/tx?hash=_&prove=_
//localhost:26657/tx_search?query=_&prove=_&page=_&per_page=_&order_by=_

http://localhost:26657/tx?hash=0x12673FB5EDE2E78D72001D96C8E53492E42C8B2FAB9E3EF828D8659BB44F1F2D
