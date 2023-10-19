# Synthesizer

A (currently) simple VST3 Synthesizer, written in Rust.
Mainly created to learn about writing Synthesizers.

## Features
- [x] Sine wave generation
- [x] Polyphonic midi input
- [x] ADSR
- [x] Visualisation
- [ ] Other oscillator waves
- [ ] Oscillator parameters
- [ ] Multiple oscillators
- [ ] Filters
- [ ] Modulation

## Run
Run standalone using:
```bash
cargo run
```
Create VST3 plugin and standalone executable using:
```bash
cargo xtask bundle synth
```
The `target/bundled` directory now contains the plugin and executable.