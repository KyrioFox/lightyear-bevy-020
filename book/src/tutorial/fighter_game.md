# Building a rollback-friendly fighter

This guide maps Lightyear's existing input and prediction APIs onto a native 1v1 fighting game. It is an integration blueprint: the game owns its combat rules, content, and presentation, while Lightyear transports tick-indexed inputs and replicates authoritative state.

## Pin the engine boundary first

Upstream Lightyear `0.30` targets Bevy `0.19` and Rust `1.95` (see the README compatibility table). This port branch pins its direct Bevy dependencies to KyrioFox Bevy `0.20.0-dev`; the verified client/server core profile and remaining adapter blockers are listed in the [port status](../appendix/bevy_020_dev.md). For a game crate, use the exact same Bevy revision as Lightyear for every Bevy-facing dependency. Mixing upstream Bevy and a distinct fork in one ECS world is not supported. Record the exact Bevy revision and Lightyear revision together in the game workspace lockfile.

For an initial native build, use the ordinary client/server topology. It leaves match validation and hit resolution on the server. Consider deterministic peer simulation only after the entire combat simulation is deterministic across supported CPUs and the design accepts peers simulating the full match. Lightyear's `simple_box` example demonstrates both topologies; its P2P path deliberately runs the same fixed roster and simulation on every peer.

## Define the wire input as player intent

Represent one player's input for one simulation tick with a small, versionable value. Send directions, button edges, and quantized analog values; do not send an animation name, local entity ID, move result, or predicted hit result.

```rust
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, PartialEq, Eq, Clone, bevy::prelude::Reflect)]
pub struct FighterInput {
    // Signed 8-bit axes keep the wire representation compact and deterministic.
    pub horizontal: i8,
    pub vertical: i8,
    // Rising-edge intent is interpreted by the authoritative simulation.
    pub buttons_pressed: u16,
    pub buttons_held: u16,
}
```

Keep a stable bit assignment for guard, light/heavy attack, jump, dash, and support actions. Treat a missing input for a tick as missing data; an explicit all-zero input means the player supplied neutral intent. Add a protocol version before changing field meaning or encoding.

Register the input type with Lightyear's native input plugin as `simple_box` does with `input::native::InputPlugin::<Inputs>::default()`. Read local devices and write the action state in `FixedPreUpdate` / `InputSystems::WriteClientInputs`; the sample's `buffer_input` system shows the required timing. Run shared fighter simulation in the fixed schedule, using integer frame counters for startup, active, recovery, hitstop, and stun windows. Keep rendering, audio, camera shake, and particles outside rollback state so replaying simulation does not duplicate effects.

## Separate simulation state from presentation

A minimal rollback state should contain only data that changes the rules: fighter slot, quantized position and velocity, facing, grounded state, current move identifier, move frame, health, stun, and resource meters. Move definitions should be immutable, versioned game data shared by client and server. Resolve hitboxes and damage from that data during the simulation tick; derive skeletal animation and effects afterward from the resulting state.

Use fixed-point or carefully bounded integer arithmetic for movement and combat if the same simulation will run on heterogeneous clients. Do not assume that Bevy's floating-point transforms or physics components are bitwise deterministic. In server-authoritative mode, the server remains the source of truth for hit confirmation and match results; client prediction is a responsiveness aid and must be correctable.

## Start with prediction, then choose correction

Lightyear's `simple_box` protocol marks a position component as `.replicate().predict().add_linear_interpolation()` and registers input buffering. For a fighter, predict the locally controlled fighter's movement and immediately visible action state; replicate confirmed combat state from the server. Avoid interpolating discrete combat transitions such as hit, guard-break, or round-end as though they were positions.

Use Lightyear's client input timeline and prediction configuration to tune input delay and correction. Begin with conservative server authority and measure corrections under packet loss and latency before enabling broader client-side rollback. The examples under `examples/simple_box/src/{protocol,client,shared}.rs` are the concrete API references for protocol registration, input capture, shared simulation, and predicted entities.

## Suggested integration sequence

1. Pin the Bevy fork revision, Lightyear revision, Rust toolchain, and protocol version in the game workspace.
2. Add a headless server and two native clients using the `simple_box` client/server setup.
3. Replace movement-only input with `FighterInput`; keep it tick-indexed and have the server validate button transitions and legal move states.
4. Implement one deterministic move and one hit interaction before adding character data or effects.
5. Replicate confirmed health, round state, and discrete combat outcomes; keep local presentation derived from those outcomes.
6. Profile packet size, correction frequency, and server tick cost before adding rollback of additional state or a WebAssembly client.

Relevant references: [`simple_box` protocol](https://github.com/cBournhonesque/lightyear/blob/main/examples/simple_box/src/protocol.rs), [`simple_box` input capture](https://github.com/cBournhonesque/lightyear/blob/main/examples/simple_box/src/client.rs), [`simple_box` shared simulation](https://github.com/cBournhonesque/lightyear/blob/main/examples/simple_box/src/shared.rs), and [`simple_box` deterministic P2P setup](https://github.com/cBournhonesque/lightyear/blob/main/examples/simple_box/src/p2p.rs).
