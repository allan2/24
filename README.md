# 24

A solver for the 24 card game. https://ayz.ai/24

The objective of the game is to find a way to combine four positive integers using arithmetic operations to obtain a target (the number 24).
Exponentiation is supported.

## Things to improve:

- more deduplication of solutions
- fix excessive cloning of `Ops` in `Card`
- better serialization from `solve24` across WebAssembly ABI
- suppport factorials

## Usage

To compile twentyfour-wasm for the frontend, run the following from the project root:

```
cargo build -p twentyfour-wasm --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/twentyfour_wasm.wasm --out-dir frontend/pkg

cd frontend
pnpm build
```

## Helpful Resources

The solver itself was based on [dbkaplun/solve24-rs](https://github.com/dbkaplun/solve24-rs).

- http://24solver.us-west-2.elasticbeanstalk.com/
- https://theconfused.me/get24/
- https://w3.cs.jmu.edu/spragunr/CS159/pas/twenty_four/twenty_four.shtml
- https://theconfused.me/blog/solving-the-24-game/
- https://leetcode.com/problems/24-game/
- https://math.stackexchange.com/questions/2785407/proof-of-solutions-to-24-operations-game
- https://github.com/gsingh93/24-solver
- https://github.com/mhfan/inrust
- https://github.com/DM-Earth/Solver024
- https://frank-deng.github.io/24game-solver/index.html

## Contributing

PRs welcome!
