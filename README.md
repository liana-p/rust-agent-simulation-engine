# Rust Agent Simulation

A library for doing agent and systems based simulations in Rust, with an example simulation.

**(Work in progress, not functional yet)**

## Installation

Clone the repo and use Cargo

```bash
git@github.com:nialna/rust-agent-simulation-engine.git
```

The package isn't functional yet so builds will fail

```bash
cargo build
```

## Usage

```bash
cargo run
```

## Architecture

The architecture is somewhat similar to an [ECS](https://en.wikipedia.org/wiki/Entity_component_system) but with only agents (equivalent to being an entity and its only component at the same time) and systems (which run on agents)

A simulation is composed of the following:

- A world in which agents are added
- Agents which have a state containing data
- Systems which are run on agents individually to perform whatever they're supposed to do

Agents have a dynamic list of systems in their data, this allows the user to change which systems a particular agent uses at runtime).

## Current Issues

The simulation engine doesn't currently work due to issues with figuring out how to store dynamic user-provided systems and entities properly without making the compiler sad.

## License

[MIT](https://choosealicense.com/licenses/mit/)
