# Member Protocol

Onchain membership based experience

## Deployment

Install dependency.

```sh
npm i
```

Build and optimize wasm bytecode.

```sh
npm run build-and-optimize
```

NOTE: Please use [celatone](https://terra.celat.one) or write your own script with feather.js to store code and instantiate contract. Terrain is broken (or my setup is wrong) on signing tx.

Create your own `.env`.

```sh
cp .example.env .env
```

Store code onchain.

```sh
npm run store-code
```

Init all contracts. You need to update `.env` accordingly as distribution and thread both depends on member for their instantiate msgs.

```sh
npm run init-member
npm run init-distribution
npm run init-thread
```
