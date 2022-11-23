# AR LavaFlow

## Temporarily Offline
This repository was migrated from another GitHub repository (18 november 2022), where it was originally hosted through GitHub Pages. However, we will most likely not be using GitHub Pages for hosting from PRIME-TU-Delft. Other hosting possibilities are currently being investigated.

At least until this decision has been made, *this repository must remain private*.

## Repository structure
The production-ready part of this project is split into two folders: `wasm` and `svelte-app`. The `wasm` folder contains the model generation and smoothing code, which was written in Rust for performance reasons. `svelte-app` is our frontend, which contains the image processing, and the Augmented Reality visualization of the generated model.

Other than that, in the beginning of the project, we created a lot of smaller demos. These proof-of-concepts can be found in `poc`.


## Development setup
To run this project, the following dependencies need to be installed on your system:
- [Rust](https://www.rust-lang.org/tools/install) toolchain with `wasm32-unknown-unknown` target
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- [NodeJS](https://nodejs.org/) (v16 LTS or newer recommended) with [yarn](https://yarnpkg.com/getting-started/install) package manager

Then to run a local development server, run the following commands:
```bash
cd svelte-app
wasm-pack -v build ../wasm --target web
yarn
yarn dev
```

Note that you will need to manually rerun the `wasm-pack` and `yarn dev` commands whenever you change the Rust code. If this gets annoying, you could use a tool such as [`watchexec`](https://watchexec.github.io/):
```bash
cd svelte-app (if you were not already)
watchexec -cr -w wasm/src/ -- "wasm-pack -v build ../wasm --target web && yarn dev"
```


## Deploying to production
This repository is currently being mirrored to GitHub, where a pipeline is run to deploy it to GitHub pages. But if this changes in the future, you can run follow the instructions for setting up a development server, but run `yarn build` instead of `yarn dev`.

This will build a static website in `svelte-app/build/`, which can be deloyed on a webserver of choice. Though keep in mind that GitHub pages requires some further magic, such as adding a `.nojekyll` file to make sure folders starting with an underscore are served normally.

---

AR LavaFlow (c) by PRIME, TU Delft

AR LavaFlow is licensed under a
Creative Commons Attribution 4.0 International License.

You should have received a copy of the license along with this
work. If not, see <http://creativecommons.org/licenses/by/4.0/>.
