# bitclerk

A CLI tool to build unsigned PSBTs with OP_RETURNs.

# Usage

Clone and install the binary:
```console
git clone https://github.com/luisschwab/bitclerk
cd bitclerk
cargo install --locked --path .
```

Get a hold of a public descriptor and sync to get a list of available UTXOs:
```console
bitclerk sync \
    --desc "tr([abcf4bd2/86h/0h/0h]xpub6DWT5EhrhJX3LLxKDhuCyDEd9GZsB7Xqzawa4V3V1kj41DafvNgBzhjyYzsehTGWPVVz87sncxd5rVQUY4TpYcTttMaiegdTAvCcuNxVADJ/0/*)#yd7caawn"

Synchronizing wallet with descriptor
 tr([abcf4bd2/86h/0h/0h]xpub6DWT5EhrhJX3LLxKDhuCyDEd9GZsB7Xqzawa4V3V1kj41DafvNgBzhjyYzsehTGWPVVz87sncxd5rVQUY4TpYcTttMaiegdTAvCcuNxVADJ/0/*)#yd7caawn
from
 https://mempool.emzy.de/api...

>AVAILABLE UTXOs
+-------+---------------------------+--------------------------------------------------------------------+
| INDEX | AMOUNT                    | OUTPOINT                                                           |
+-------+---------------------------+--------------------------------------------------------------------+
| 0     | 20,580 sats               | afe7405726f738f99306f81e7497a9ab8ded44e08d7e0e730800ac36451feec1:0 |
+-------+---------------------------+--------------------------------------------------------------------+
```

Then create a transaction with the UTXO selected by it's index:
```console
bitclerk create \
    --desc "tr([abcf4bd2/86h/0h/0h]xpub6DWT5EhrhJX3LLxKDhuCyDEd9GZsB7Xqzawa4V3V1kj41DafvNgBzhjyYzsehTGWPVVz87sncxd5rVQUY4TpYcTttMaiegdTAvCcuNxVADJ/0/*)#yd7caawn"
    --utxo 0 \
    --msg "bitclerk"

Synchronizing wallet with descriptor
 tr([abcf4bd2/86'/0'/0']xpub6DWT5EhrhJX3LLxKDhuCyDEd9GZsB7Xqzawa4V3V1kj41DafvNgBzhjyYzsehTGWPVVz87sncxd5rVQUY4TpYcTttMaiegdTAvCcuNxVADJ/0/*)#hup888es
from
 https://mempool.emzy.de/api...

SERIALIZED UNSIGNED PSBT (SIGN AND BROADCAST IT YOURSELF)
70736274ff0100710200000001c1ee1f4536ac0008730e7e8de044ed8daba997741ef80693f938f7265740e7af0000000000fdffffff02c04e0000000000002251207dc6b7a623a22784b6a85e4fc4992f24be8234335695124ca11d81901c4d8deb00000000000000000a6a08626974636c65726b1b1e0e000001012b6450000000000000225120782699c4fe6fcd63b4ae6a41f2d317cef818f77812667b19fe06de3867ac0f8a21165310a80376647007a9ccbb2e2e9239b5b9a321802ad301f193194dde5094bd571900abcf4bd256000080000000800000008000000000010000000117205310a80376647007a9ccbb2e2e9239b5b9a321802ad301f193194dde5094bd57000105208155ca180da5961a6e003a604790d077796b7674f2d1f36f01af127f933e6b4121078155ca180da5961a6e003a604790d077796b7674f2d1f36f01af127f933e6b411900abcf4bd256000080000000800000008000000000020000000000
```

Finally, copy the unsigned PSBT, sign it as you please and broadcast it.
