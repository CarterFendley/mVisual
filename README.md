# mVisual

## Setup

First step is to [install rust](https://www.rust-lang.org/) and to install the webassembly target with the command below.

```
rustup target add wasm32-unknown-unknown
```

Then you can [install yarn](https://yarnpkg.com/getting-started/install) and this project and its dependencies.

```
yarn
```

Then launch the electron.

```
yarn start
```

## Notes current package setup

### Electron

Referenced the [quick start](https://www.electronjs.org/docs/latest/tutorial/quick-start) tutorial and electron's [typescript example](https://github.com/electron/electron-quick-start-typescript).

### Webpack

Webpack is used primarily for the ease of use with the `WasmPackPlugin` plugin. Although [electron-webpack](https://webpack.electron.build/) exists to make webpack easier to use with electron, the initial testing of their [quick start repo](https://github.com/electron-userland/electron-webpack-quick-start) resulted in installation errors.

The setup is based on [this article](https://www.sitepen.com/blog/getting-started-with-electron-typescript-react-and-webpack) and [this example](https://github.com/lgoodcode/secure-electron-boilerplate) repository. It uses [electronmon](https://github.com/catdad/electronmon) to reload electron.

The `startDev.ts` script is a little inefficient, due calling both `yarn build` and `yarn watch`. The call to `yarn build` is expected to exit before electron is started through `electronmon`. This is to make sure you have compiled to `./dist` so electron does not crash. After this happens `yarn watch` will be run to keep the files in `./dist` up to date. This does a initial re-compile when launched but `electronmon` does not seem to notice it until the files are truly updated.