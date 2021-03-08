## vvmbin

VVM implementation for Tetsy.

### Usage

```
VVM implementation for Tetsy.
  Copyright 2015-2020 Parity Technologies (UK) Ltd.

Usage:
    tetsy-vvm state-test <file> [--json --std-json --std-dump-json --only NAME --chain CHAIN --std-out-only --std-err-only]
    tetsy-vvm stats [options]
    tetsy-vvm stats-jsontests-vm <file>
    tetsy-vvm [options]
    tetsy-vvm [-h | --help]

Commands:
    state-test         Run a state test from a json file.
    stats              Execute VVM runtime code and return the statistics.
    stats-jsontests-vm Execute standard json-tests format VMTests and return
                       timing statistics in tsv format.

Transaction options:
    --code CODE        Contract code as hex (without 0x).
    --to ADDRESS       Recipient address (without 0x).
    --from ADDRESS     Sender address (without 0x).
    --input DATA       Input data as hex (without 0x).
    --gas GAS          Supplied gas as hex (without 0x).
    --gas-price WEI    Supplied gas price as hex (without 0x).

State test options:
    --only NAME        Runs only a single state test matching the name.
    --chain CHAIN      Run only tests from specific chain.

General options:
    --json             Display verbose results in JSON.
    --std-json         Display results in standardized JSON format.
    --std-err-only     With --std-json redirect to err output only.
    --std-out-only     With --std-json redirect to out output only.
    --std-dump-json    Display results in standardized JSON format
                       with additional state dump.
Display result state dump in standardized JSON format.
    --chain CHAIN      Chain spec file path.
    -h, --help         Display this message and exit.
```

## Tetsy Vapory toolchain
_This project is a part of the Tetsy Vapory toolchain._

- [vvmbin](https://github.com/openvapory/tetsy-vapory/blob/master/vvmbin/) - VVM implementation for Tetsy Vapory.
- [vapabi](https://github.com/tetcoin/vapabi) - Tetsy Vapory function calls encoding.
- [vapstore](https://github.com/openvapory/tetsy-vapory/blob/master/accounts/vapstore) - Tetsy Vapory key management.
- [vapkey](https://github.com/openvapory/tetsy-vapory/blob/master/accounts/vapkey) - Tetsy Vapory keys generator.
