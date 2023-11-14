# astar.rs

A blazingly-fast [A-Star](https://en.wikipedia.org/wiki/A*_search_algorithm) search algorithm and visualisation written in Rust. GUI powered by [egui](https://www.egui.rs/). 

Allows for dynamic grid sizes and user-chosen heuristics. Runs natively on MacOS, Linux and the web using WASM!

## Demo
The WASM-based demo of the app is hosted [here](https:/nikhilhenry.github.io/astar.rs).

## Installation

Prebuilt binaries for MacOS can be found on the releases page. 

For other platforms, clone the repo and run:

```bash
cargo run --release
```
...with Rust installed ofc ;)

## Changelog

### Version v0.0.1 [14-11-2023]
- Initial release
- Support for up to 50x50 grids
- Supports up to 3 heuristics
- Builds for both WASM and x86_64

Icon credits - [Václav Vančura](https://www.figma.com/community/file/857303226040719059)
