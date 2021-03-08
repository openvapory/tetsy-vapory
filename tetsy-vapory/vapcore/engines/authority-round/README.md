## authority-round

A blockchain engine that supports a non-instant BFT proof-of-authority.

It is recommended to use the `two_thirds_majority_transition` option, to defend against the ["Attack of the Clones"](https://arxiv.org/pdf/1902.10244.pdf).  Newly started networks can set this option to `0`, to use a 2/3 quorum from the beginning.

To support on-chain governance, the [ValidatorSet] is pluggable: Aura supports simple constant lists of validators as well as smart contract-based dynamic validator sets.  Misbehavior is reported to the [ValidatorSet] as well, so that e.g. governance contracts can penalize or ban attacker's nodes.

* "Benign" misbehavior are faults that can happen in normal operation, like failing to propose a block in your slot, which could be due to a temporary network outage, or wrong timestamps (due to out-of-sync clocks).

* "Malicious" reports are made only if the sender misbehaved deliberately (or due to a software bug), e.g. if they proposed multiple blocks with the same step number.
