## vapcore-clique

Implementation of the Clique PoA Engine.

File structure:
- mod.rs -> Provides the engine API implementation, with additional block state tracking
- block_state.rs -> Records the Clique state for given block.
- params.rs -> Contains the parameters for the Clique engine.
- step_service.rs -> An event loop to trigger sealing.
- util.rs -> Various standalone utility functions.
- tests.rs -> Consensus tests as defined in EIP-225.

How syncing works:

1. Client will call:
   - `Clique::verify_block_basic()`
   - `Clique::verify_block_unordered()`
   - `Clique::verify_block_family()`
2. Using `Clique::state()` we try and retrieve the parent state. If this isn't found
   we need to back-fill it from the last known checkpoint.
3. Once we have a good state, we can record it using `CliqueBlockState::apply()`.

How sealing works:

1. Set a signer using `Engine::set_signer()`. If a miner account was set up through
   a config file or CLI flag `MinerService::set_author()` will eventually set the signer
2. We check that the engine is ready for sealing through `Clique::sealing_state()`
   Note: This is always `SealingState::Ready` for Clique
3. Calling `Clique::new()` will spawn a `StepService` thread. This thread will call `Engine::step()`
   periodically. Internally, the Clique `step()` function calls `Client::update_sealing()`, which is
   what makes and seals a block.
4. `Clique::generate_seal()` will then be called by `miner`. This will return a `Seal` which
    is either a `Seal::None` or `Seal:Regular`. The following shows how a `Seal` variant is chosen:
      a. We return `Seal::None` if no signer is available or the signer is not authorized.
      b. If period == 0 and block has transactions, we return `Seal::Regular`, otherwise return `Seal::None`.
      c. If we're `INTURN`, wait for at least `period` since last block before trying to seal.
      d. If we're not `INTURN`, we wait for a random amount of time using the algorithm specified
         in EIP-225 before trying to seal again.
5. Miner will create new block, in process it will call several engine methods to do following:
  a. `Clique::open_block_header_timestamp()` must set timestamp correctly.
  b. `Clique::populate_from_parent()` must set difficulty to correct value.
      Note: `Clique::populate_from_parent()` is used in both the syncing and sealing code paths.
6. We call `Clique::on_seal_block()` which will allow us to modify the block header during seal generation.
7. Finally, `Clique::verify_local_seal()` is called. After this, the syncing code path will be followed
   in order to import the new block.
