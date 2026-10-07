# Lightyear port to the KyrioFox Bevy 0.20 development fork

This branch pins every Bevy crate used directly by the Lightyear workspace to KyrioFox Bevy commit `5cd69c13575d37a5a36988339d86814b2a459815`. The fork reports package version `0.20.0-dev` and Rust MSRV `1.97.1`; Lightyear's workspace MSRV is raised accordingly. Keep the git revision identical for every `bevy_*` workspace dependency, including the facade crate.

## Ported APIs

- Bevy moved curve APIs out of `bevy_math` into `bevy_curve`. Lightyear now depends on `bevy_curve`, uses its root exports, and enables the facade's `bevy_curve` feature. The removed `bevy_math/curve` feature is no longer requested.
- Observer lifecycle patterns now use one event-pattern type, for example `On<Add<Connected>>` and `On<Remove<Linked>>`. All Lightyear crates and examples have been converted from the Bevy 0.19 two-parameter form.
- The custom `SyncedLocalTimeline` system parameter now implements Bevy 0.20's `SystemParam::init_access` using `SystemAccess` and returns the access conflict result.
- Bevy's `Query::iter_many_unique{,_mut}` now yields per-entity results. The Lightyear call sites use `.matched()` to keep the previous behavior of skipping unavailable entities.
- The vendored Aeronet IO 0.22.0-rc.1 adapter and Bevy Replicon 0.44.1 adapter are pinned to the same Bevy 0.20-dev crate identities. Replicon's system parameter access and event trigger bounds are adapted to this fork.
- Replicon's server state transitions use Bevy 0.20's `NextState::set_if_different` API.
- Lightyear's custom prediction, interpolation, frame interpolation, correction, and deterministic checksum system parameters now implement the three-argument `SystemParam::init_access`. Component accesses that previously inspected the world from `init_access` are captured during `init_state`, matching Bevy's system initialization lifecycle.
- Converted a remaining prediction observer to `On<Add<(...)>>`.

## Verified profile

This native gameplay profile compiles against the fork:

```sh
cargo check -p lightyear --no-default-features --features "std,client,server,udp,netcode,replication,prediction,interpolation,input_native,deterministic"
```

This includes Lightyear client/server, UDP via Aeronet, netcode transport, replicated state via Bevy Replicon, prediction, snapshot and frame interpolation, deterministic rollback support, and native input.

## Validation boundary

The profile above is a compile gate, not a runtime multiplayer validation. In particular, it does not verify live UDP handshakes, prediction rollback behavior, or game-specific replicated components.

## Vendored provenance

`vendor/UPSTREAM.md` records the exact Aeronet and Replicon source commits and their licenses. They are local compatibility copies, not upstream releases; retain that provenance when updating them.
