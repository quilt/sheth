# sheth

![client tests build status](https://github.com/lightclient/sheth/workflows/sheth-bench/badge.svg)
![sheth-tests build status](https://github.com/lightclient/sheth/workflows/sheth-tests/badge.svg)
[![Apache License](https://img.shields.io/badge/license-Apache--2.0-blue)](https://github.com/lightclient/sheth#license)

`sheth` [ ˈshēth ] is an [execution
environment](https://hackmd.io/UzysWse1Th240HELswKqVA?view#Execution-Environment-EE)
(EE) for Ethereum 2.0 that facilitates the movement of ether within a shard and
provides mechanisms to move ether between a shard and the beacon chain.

## Quick Start

First, setup your environment:
```console
rustup target install wasm32-unknown-unknown
cargo install chisel
make setup
```

Then simulate execution using [Scout](https://github.com/ewasm/scout):
```console
make scout
```

Or run on your local architecture (useful for tracking down bugs):
```console
make test
```

#### Recommended Reading
The design space for EEs is broad and builds on many different Ethereum 2.0
related concepts. If you're lost, here are a few places to get started:

* [Phase 2 Wiki](https://hackmd.io/UzysWse1Th240HELswKqVA)
* [Phase 2 Proposal](https://notes.ethereum.org/w1Pn2iMmSTqCmVUTGV4T5A?view#Implementing-in-shard-ETH-transfers)
* [Eth EE Proposal](https://ethresear.ch/t/eth-execution-environment-proposal/5507)

Ping me [@matt_garnett](https://twitter.com/matt_garnett) for any
other questions / concerns.

## Motivation
* Understanding the developer experience regarding EEs could influence design
  decisions of the protocol and EE runtime.
* Efficiently authenticating and updating merkle multi-proofs is critical to the
  success of stateless execution environments.
* In order to develop strong tooling for EE development, it's important to
  determine a lower bound for execution time and binary size to target. 
* Provide a framework on which others can develop and experiment.



## Architecture
At a high level, `sheth` provides a single state transition function:

```rust
pub fn main(pre_state: &[u8; 32], data: &[u8]) -> [u8; 32];
```

The main function essentially takes in the latest merkle root of the state as
`pre_state` and some amount of `data`. It deserializes the data into the
transactions that will be executed and the merkle multi-proof which is used
authenticate the transactions. Due to some of the semantics of WebAssembly, it
isn't quite this simple (see the [FFI interface](src/lib.rs)) -- but the general
idea remains intact.

`sheth`'s design is heavily influenced by Vitalik's sample EE in his [phase 2
proposal](https://notes.ethereum.org/w1Pn2iMmSTqCmVUTGV4T5A?view#Implementing-in-shard-ETH-transfers).

### State
The `state` can be thought of abstractly as an array `[Account, 2**256]`, where
an account's index in the array is equal to `Sha256(account.pubkey)`. This is
far too large to fit into memory all at once, so operations are done on specific
elements within a merkle multi-proof.

The sparse merkle tree for `sheth`'s `state` can be roughly visualized as
follows:

```
FL = first leaf node = 2**256
LL = last leaf node = 2**257 - 1
        
              +---------- 1 ----------+             
             /                         \
        +-- 2 --+                   +-- 3 --+       
       /         \                 /         \
     ...         ...             ...         ...   
    /   \       /   \           /   \       /   \
  FL+0 FL+1   FL+2 FL+3  ...  LL-3 LL-2   LL-1 LL-0 
```

Each leaf node is the root of the corresponding `account`. An `account`'s merkle
tree structure is as follows:

```
              +--- account ---+
             /                 \
      pubkey_root           sub_root    
        /     \              /    \     
  pk[0..32] pk[32..48]    nonce  value
```

### Merkle Multi-Proof 
A merkle multi-proof is a data structure which stores multiple branches proving
various items within the `state`. For a formal definition, see the
[specification](https://github.com/ethereum/eth2.0-specs/blob/dev/specs/light_client/merkle_proofs.md#merkle-multiproofs).

#### Example
Imagine a merkle tree of this shape (e.g. an `account`):

```
     1
   /   \
  2     3
 / \   / \
4   5 6   7
```

In order to prove `6` is included in the root at `1`, the nodes [2, 6, 7] would
be needed since `1` and `3` can be calculated from that set.

```
     1
   /   \
 [2]    3
 / \   / \
4   5[6] [7]
```

## Roadmap
- [x] Support intra-shard transfers
- [ ] Consume beacon chain withdrawal receipts
- [ ] Allow shard ether to be deposited to the beacon chain
- [ ] Validate transaction signature against BLS pubkey
- [x] Verify transaction nonce against account
- [ ] Implement `merge` functionality for multiple packages
- [ ] Minimize binary size
- [ ] Minimize execution time
- [ ] Minimize multi-proof size

## License
Licensed under Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
