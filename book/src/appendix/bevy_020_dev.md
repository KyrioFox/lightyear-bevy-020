# Lightyear port to the KyrioFox Bevy 0.20 development fork

This branch pins every Bevy crate used directly by the Lightyear workspace to KyrioFox Bevy commit `5cd69c13575d37a5a36988339d86814b2a459815`. The fork reports package version `0.20.0-dev` and Rust MSRV `1.97.1`; Lightyear's workspace MSRV is raised accordingly. Keep the git revision identical for every `bevy_*` workspace dependency, including the facade crate.

## Ported APIs

- Bevy moved curve APIs out of `bevy_math` into `bevy_curve`. Lightyear now depends on `bevy_curve`, uses its root exports, and enables the facade's `bevy_curve` feature. The removed `bevy_math/curve` feature is no longer requested.
- Observer lifecycle patterns now use one event-pattern type, for example `On<Add<Connected>>` and `On<Remove<Linked>>`. All Lightyear crates and examples have been converted from the Bevy 0.19 two-parameter form.
- The custom `SyncedLocalTimeline` system parameter now implements Bevy 0.20's `SystemParam::init_access` using `SystemAccess` and returns the access conflict result.
- Bevy's `Query::iter_many_unique{,_mut}` now yields per-entity results. The Lightyear call sites use `.matched()` to keep the previous behavior of skipping unavailable entities.
- The vendored Aeronet IO 0.22.0-rc.1 adapter and Bevy Replicon 0.44.1 adapter are pinned to the same Bevy 0.20-dev crate identities. Replicon's system parameter access and event trigger bounds are adapted to this fork.
- Replicon's server state transitions use Bevy 0.20's `NextState::set_if_different` API.

## Verified profile

This native UDP and state-replication profile compiles against the fork:

```sh
cargo check -p lightyear --no-default-features --features "std,client,server,udp,netcode,replication"
```

This includes Lightyear client/server, UDP via Aeronet, netcode transport, and replicated state via Bevy Replicon. It does not establish prediction/interpolation compatibility.

## Prediction boundary

Enabling `prediction` currently fails in Lightyear's custom interpolation `SystemParam` implementations. Bevy 0.20 changed `SystemParam::init_access` from four arguments (including `World`) to three, with a `SystemAccess` result. Several Lightyear parameters inspect runtime registries through the removed `World` argument, so this is a structural migration rather than a dependency-version issue. Keep prediction disabled for this port slice until those access declarations are moved into parameter state and validated against the fork.

## Vendored provenance

`vendor/UPSTREAM.md` records the exact Aeronet and Replicon source commits and their licenses. They are local compatibility copies, not upstream releases; retain that provenance when updating them.
