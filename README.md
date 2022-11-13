# fetch-block
a simple program to fetch the bytes of a specific block (identified by slot and block hash) from a cardano network, useful for debugging the on-chain serialisation of blocks and transactions

### Usage

Fetch the bytes of the block `33f1f63f10e82cf72e137b92e6d307be85e8a3ddd5b9131220734b82cd2974ae` which was minted at slot `1363359` on the `preview` network:
```console
$ cargo run -- --network preview --slot 1363359 --hash 33f1f63f10e82cf72e137b92e6d307be85e8a3ddd5b9131220734b82cd2974ae

820685828a19f45f1a0014cd9f582057b77bbddd4a8109decb6c1eec99103ac8110423b3380c22fd1a8974174135fb582023bb0a21009d...
```

Print a diagnostic representation of the CBOR bytes of the first transaction in the block:
```console
$ cargo run -- --network preview --slot 1363359 --hash 33f1f63f10e82cf72e137b92e6d307be85e8a3ddd5b9131220734b82cd2974ae --tx-at 0 --diag

84                                                       # array(4)
   a5                                                    #   map(5)
      00                                                 #     unsigned(0)
      82                                                 #     array(2)
         82                                              #       array(2)
            58 20                                        #         bytes(32)
               17ee45f3e5e24b375a6b6ffc02aff2fa          #           "\x17\xeeE\xf3\xe5\xe2K7Zko\xfc\x02\xaf\xf2\xfa"
               96be716e2762f10b45c584d8c308a048          #           "\x96\xbeqn\'b\xf1\x0bE\xc5\x84\xd8\xc3\x08\xa0H"
            00                                           #         unsigned(0)
         82                                              #       array(2)
         ...
```

```console
$ cargo run -- --help

Usage: fetch-block [OPTIONS] --network <NETWORK> --slot <BLOCK_SLOT> --hash <BLOCK_HASH_HEX>

Options:
      --network <NETWORK>      The Cardano network from which to fetch the block from [possible values: preview, preprod, mainnet]
      --slot <BLOCK_SLOT>      The slot of the block to fetch
      --hash <BLOCK_HASH_HEX>  The block hash of the block to fetch
      --tx-at <TX_INDEX>       (Optional) Return only the transaction at this index in the block
      --diag                   Print a diagnostic representation of the CBOR
  -h, --help                   Print help information
  -V, --version                Print version information
```