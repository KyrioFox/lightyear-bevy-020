# Upstream provenance and local adaptation notes

This directory vendors a minimal adapter subset while Lightyear is being ported to KyrioFox Bevy 0.20-dev.

- `aeronet_io` source and README: https://github.com/aecsocket/aeronet/tree/82b2090e6f63d2f3a321a8fdcab2d823b2eee002/crates/aeronet_io
- `aeronet_io` upstream commit: `82b2090e6f63d2f3a321a8fdcab2d823b2eee002` (Aeronet 0.22.0-rc.1)
- `bevy_replicon` source and README: https://github.com/andriyDev/bevy_replicon/tree/6d4fda8b2e811d87e65cc8d407903933604b63c9
- Replicon upstream fork commit: `6d4fda8b2e811d87e65cc8d407903933604b63c9` (Bevy 0.20.0-rc.1 adapter fork used by Aeronet)
- Both crates retain their upstream MIT and Apache-2.0 license texts in their directories.

The manifests resolve Bevy dependencies from the exact KyrioFox Bevy commit pinned in the root workspace. Replicon observer patterns are migrated to the Bevy 0.20 single-event `On<Add<Component>>` form. Keep the upstream commits and local API adaptations explicit when refreshing this vendored code; do not present these copies as upstream releases.