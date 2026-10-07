# Lightyear port to the KyrioFox Bevy 0.20 development fork

This branch pins every Bevy crate used directly by the Lightyear workspace to KyrioFox Bevy commit `5cd69c13575d37a5a36988339d86814b2a459815`. The fork reports package version `0.20.0-dev` and Rust MSRV `1.97.1`; Lightyear's workspace MSRV is raised accordingly. Keep the git revision identical for every `bevy_*` workspace dependency, including the facade crate.

## Ported APIs

- Bevy moved curve APIs out of `bevy_math` into `bevy_curve`. Lightyear now depends on `bevy_curve`, uses its root exports, and enables the facade's `bevy_curve` feature. The removed `bevy_math/curve` feature is no longer requested.
- Observer lifecycle patterns now use one event-pattern type, for example `On<Add<Connected>>` and `On<Remove<Linked>>`. All Lightyear crates and examples have been converted from the Bevy 0.19 two-parameter form.
- The custom `SyncedLocalTimeline` system parameter now implements Bevy 0.20's `SystemParam::init_access` using `SystemAccess` and returns the access conflict result.
- Bevy's `Query::iter_many_unique{,_mut}` now yields per-entity results. The Lightyear call sites use `.matched()` to keep the previous behavior of skipping unavailable entities.

## Verified profile

This subset compiles with the fork:

```sh
cargo check -p lightyear --no-default-features --features "std,client,server"
```

It includes the Lightyear client/server plugin surface, link and connection layers, message transport, synchronization, and the app facade dependencies. It does not enable an IO backend or replication.

## Remaining dependency boundary

The current Lightyear dependency graph still contains Bevy 0.19 types through external crates. `aeronet_io 0.21.0`, used by `lightyear_udp`, and `bevy_replicon 0.44.3`, used by replication, resolve `bevy_ecs 0.19.1` / `bevy_app 0.19.1` from crates.io. These are distinct Rust crate identities from KyrioFox Bevy's git-pinned `bevy_ecs 0.20.0-dev` and cannot be mixed in the same ECS `App`.

Consequently the UDP/netcode profile currently fails to compile, and replication, prediction, and native input are not yet a usable game integration. Upgrade or fork those external adapters against the same Bevy revision before claiming full Lightyear compatibility. The command above is the intentionally narrow compile gate for this port slice, not the full workspace gate.
