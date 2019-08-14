import ssz

from ssz.sedes import (
    uint8,
    uint64,
    Vector,
    List,
    Serializable
)


class Account(Serializable):
    fields = [
        ('pubkey', Vector(uint8, 48)),
        ('nonce', uint64),
        ('value', uint64),
    ]

class State(Serializable):
    fields = [
        ('state', Vector(Account, 4))
    ]


state_pre = State(
    state=[
        Account(
            pubkey=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            nonce=1,
            value=0,
        ),
        Account(
            pubkey=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            nonce=0,
            value=0,
        ),
        Account(
            pubkey=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            nonce=0,
            value=0,
        ),
        Account(
            pubkey=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            nonce=0,
            value=0,
        )
    ]
)

print(state_pre.hash_tree_root.hex())
