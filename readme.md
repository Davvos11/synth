# Synthesizer

A (currently) simple VST3 Synthesizer, written in Rust.
Mainly created to learn about writing Synthesizers.

## Features
- [x] Sine wave generation
- [x] Polyphonic midi input
- [x] ADSR
- [x] Visualisation
  - [x] Audio scope
  - [ ] ADSR graph
  - [ ] Parameter modulation
  - [ ] LFO scope
- [x] Other oscillator waves
- [x] Oscillator parameters
- [x] Multiple oscillators
- [ ] Modulation
  - [ ] Multiple ADSR curves
  - [ ] Modulate "any" parameter
  - [ ] LFOs
- [ ] Filters

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