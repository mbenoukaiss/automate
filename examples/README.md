# Examples
This folder contains a few examples that demonstrate the main features provided by Automate.

- [Basic](examples/basic.rs): Basic example with a few stateless listeners listening to messages and reactions and that responds by sending messages and creating invites
- [Counter](examples/counter.rs): Simple per guild and per user message counter, also shows two stateless listeners (invite generator and simple use of reactions).
- [Levels](examples/levels.rs): Example demonstrating the custom storage feature by creating a `Count` storage and using the provided ones.
- [Sharded](examples/sharding.rs): Shows how the sharding API works