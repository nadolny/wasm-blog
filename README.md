# WASM (with Rust) + Vite + Svelte Monorepo

## Quick Start

### Unix

HYO: test

Assuming a fresh install (no node, no rust)

1. Clone the repo.
2. `cd` into repo
3. Install [nvm](https://github.com/nvm-sh/nvm#installing-and-updating), close
   and reopen terminal after install.
4. Run

```bash
nvm install --lts
```

5. To verify install run

```bash
node -v
// should output something similar to
v16.13.0
```

```bash
$ node -v
v18.12.1
```

6. This monorepo uses [yarn workspaces](https://yarnpkg.com/features/workspaces)
   under the hood. Install `yarn` by running

```bash
npm i -g yarn
```

```bash
$ yarn --version
1.22.19
```

7. Install [rustup + rust](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
// follow installation prompts, close and reopen terminal after install.
```

```bash
$ rustc --version
rustc 1.78.0 (9b00956e5 2024-04-29)

$ cargo --version
cargo 1.78.0 (54d8815d0 2024-03-26)

$ wasm-pack --version
wasm-pack 0.12.1
```

8. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

```bash
$ wasm-pack --version
wasm-pack 0.12.1
```

9. If in Ubuntu or similar, you need to install a CC linker (you might already
   have it if you've run sudo apt-get update before), run

```bash
sudo apt-get update
sudo apt install build-essential
```

10. Yarn needs rust to be built at least once so it can cross reference
    dependencies in the monorepo. Run in the `packages/rust` directory

```bash
cd packages/rust
# build WebAssembly and Javascript wrappers using wasm-pack
# don't panic, this might take a few seconds
wasm-pack build --target web
...
Finished `release` profile [optimized] target(s) in 6.66s
[INFO]: Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 7.79s
[INFO]: :-) Your wasm pkg is ready to publish at ../wasm-blog/packages/rust/pkg.ls

cd packages/wasm-game-of-life
wasm-pack build --target web
...
    Finished `release` profile [optimized] target(s) in 12.85s
[INFO]: Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 13.20s
[INFO]: :-) Your wasm pkg is ready to publish at .../wasm-blog/packages/wasm-game-of-life/pkg.

# Then initialize yarn in the rust directory
yarn
# go back to the monorepo root directory
cd ../..
```

11. Install node dependencies, run

```bash
yarn
```

12. Install [cargo watch](https://crates.io/crates/cargo-watch)

```bash
cargo install cargo-watch
```

13. To start the development server, run

```
yarn dev
```

14. Enjoy! Got some feedback? Open an issue, or better yet, a PR. If you like
    this template, please star this repo.

## What's next

This needs to work with a regular CI (ie: vercel/netlify/github pages). Will
create a guide for this if there is enough demand for it.

If this gets enough attention, and there demand for it, I will create a template
for sveltekit, and potentially for vue and react.

## References

- [blog.md](https://gist.github.com/CJSmith-0141/c15d14924812a96bfb5af5c69fc3e1e1)
- [Rust + Svelte + WebAssembly + Vite](https://medium.com/@Tazato/rust-svelte-webassembly-vite-c9e1e085927b)
- [WASM (with Rust) + Vite + Svelte Monorepo](https://github.com/dsegovia90/wasm-vite-svelte-monorepo)
- [sample live](https://tazato.net/)
- [Game of Life in pure Rust/wasm: avoiding RefCell](https://users.rust-lang.org/t/game-of-life-in-pure-rust-wasm-avoiding-refcell/45836)
- [Tutorial: Conway's Game of Life in Rust and WebAssembly](https://www.reddit.com/r/rust/comments/8brxds/tutorial_conways_game_of_life_in_rust_and/)
- [Hello, World! - Rust and WebAssembly](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html)
- [wasm_game_of_life](https://github.com/rustwasm/wasm_game_of_life/tree/master?tab=readme-ov-file)
