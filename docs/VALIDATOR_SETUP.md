# Network Validator Setup

To begin, let's setup the keys to run a validator node. These are the required session keys:

```
stash = sr25519
controller = sr25519
grandpa = ed25519
babe = sr25519
imonline = sr25519
authority_discovery = sr25519
```

Please make sure to have subkey installed and generate the keys for 5 validators by running:

```
subkey -n substrate --sr25519 generate > stash.keys
cat stash.keys | grep phrase | cut -d"\`" -f2 > stash.seed &&
subkey -n substrate --sr25519 generate > controller.keys
cat controller.keys | grep phrase | cut -d"\`" -f2 > controller.seed &&
subkey -n substrate --ed25519 generate > grandpa.keys
cat grandpa.keys | grep phrase | cut -d"\`" -f2 > grandpa.seed &&
subkey -n substrate --sr25519 generate > babe.keys
cat babe.keys | grep phrase | cut -d"\`" -f2 > babe.seed &&
subkey -n substrate --sr25519 generate > imonline.keys
cat imonline.keys | grep phrase | cut -d"\`" -f2 > imonline.seed &&
subkey -n substrate --sr25519 generate > authority_discovery.keys
cat authority_discovery.keys | grep phrase | cut -d"\`" -f2 > authority_discovery.seed
```

Next we will generate the string to update the chain spec with these keys set as the initial validators with (remember to remove the 0x):

```
cat stash.keys | grep Address | cut -d":" -f2 &&
cat stash.keys | grep Public | cut -d":" -f2 &&
cat controller.keys | grep Address | cut -d":" -f2 &&
cat controller.keys | grep Public | cut -d":" -f2 &&
cat grandpa.keys | grep Address | cut -d":" -f2 &&
cat grandpa.keys | grep Public | cut -d":" -f2 &&
cat babe.keys | grep Address | cut -d":" -f2 &&
cat babe.keys | grep Public | cut -d":" -f2 &&
cat imonline.keys | grep Address | cut -d":" -f2 &&
cat imonline.keys | grep Public | cut -d":" -f2 &&
cat authority_discovery.keys | grep Address | cut -d":" -f2 &&
cat authority_discovery.keys | grep Public | cut -d":" -f2
```

The next step is to go to the Polkadot UI and upload these keys or do this via RPC with:

```
> curl http://localhost:9993 -H 'Content-Type:application/json;charset=utf-8' -d "{\"jsonrpc\":\"2.0\",\"id\":1, \"method\":\"author_hasSessionKeys\", \"params\": [\"$ROTATE_KEY\"]}"
> {"jsonrpc":"2.0","result":true,"id":1}
```
