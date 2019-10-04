# Hacking `sheth`

## Overview

`sheth`'s motivations list various performance metrics -- however, it is
unlikely that `sheth` will ever be maximally performant. This reasons for this
is long, but idiomatic Rust does create an overhead when compiling to
WebAssembly. In some cases, idiomatic Rust is chosen over performance to create
an inviting codebase for new researchers & developers.

## Getting Started

It's easier to think of `sheth` as a rough framework in which transaction-based
execution environments may be developed. In that vein, `sheth` is composed of
several interfaces that can be thought of as plug-n-play compatible.

+------------------------+---------------------------+------------------------+
|                        |                           |                        |
|        transfer        |         deposit           |         withdraw       |
|                        |                           |                        |
+------------------------+---------------------------+------------------------+
|                                                                             |
|                               tx interpreter                                |
|                                                                             |
+-----------+-----------+-----------+-------------+-------------+-------------+
|           |           |           |             |             |             |
|    root   |   value   |   nonce   |  add_value  |  sub_value  |  inc_nonce  |
|           |           |           |             |             |             |
+-----------+-----------+-----------+-------------+-------------+-------------+
|                                                                             |
|                            multiproof db (imp)                              |
|                                                                             |
+-----------------------------------------------------------------------------+
|                                                                             |
|                                   sheth                                     |
|                                                                             |
+-----------------------------------------------------------------------------+

In the diagram above, `sheth` is the core interface. Essentially everything on
top of it can be added to or swapped out with alternative implementations.


## Multiproof database

In accordance with Ethereum 2.0 specification, `sheth` adheres to the stateless
paradigm. This means that at runtime the only state provided by the protocol is
a 32 byte hash. Any other information that an EE wants to authenticate must be
authenticated against that hash. As of now, `sheth` uses the `Imp` merkle proof
format. This can be replaced with any type of backend, so long as it implements
the `State` trait.

## Extending the `State` trait

The `State` trait defines an interface for accessing data from the backend.
There is nothing particularly special about the methods defined in the trait,
other than the fact that they are needed for `sheth` to operate.

A good place to add additional functionality to `sheth` or transform `sheth`
into something completely different is the `State` trait. Treat the state trait
similarly to how contract variables are treated in Solidity. The main difference
there is that Solidity already defines how to retrieve certain data types from
their definitions alone. `sheth` is still primitive in this respect, so the
`State` trait requires that a retrieval algorithm be defined alongside 
the definition of the member variables.

## New Transaction Types

The transaction interpreter is really just a pretentious term for the match
statement which deals with certain transaction types accordingly. `sheth` comes
with the concept of three types of transactions: `transfer`, `deposit`, and
`withdraw`. Adding additional transaction types should be as simple as defining
its structure, processing it in the transaction processor, and unmarshalling it
from the input data.
