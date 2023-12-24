# Synthesizer

A (currently) simple VST3 Synthesizer, written in Rust.
Mainly created to learn about writing Synthesizers.

![Screenshot](https://github.com/Davvos11/synth/assets/20478740/c884fdcb-4e5f-4272-b73c-b4e0b95a6b51)

### Note:
The current version suffers from bad performance (because of my bad code design when implementing some features), and a few bugs.  
Check out the `release` branch for a better performing version (with less features). 

## Features
- [x] Sine wave generation
- [x] Polyphonic midi input
- [x] ADSR
- [x] Visualisation
  - [x] Audio scope
  - [x] ADSR graph
  - [ ] Parameter modulation
  - [ ] LFO scope
- [x] Other oscillator waves
- [x] Oscillator parameters
- [x] Multiple oscillators
- [x] Modulation
  - [x] Multiple ADSR curves
  - [ ] Modulate "any" parameter
  - [ ] LFOs
- [ ] Advanced midi input
  - [ ] Sustain
  - [ ] CC Modulation
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
