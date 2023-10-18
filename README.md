# Member Protocol

Onchain membership based experience

## Deployment

Install terrain globally then run

```sh
terrain task:run build_and_optimize
```

Now copy .wasm files to each contract's own artifacts directory cause terrain has issue (or my setup wrong) detecting cargo workspace when storing code.

NOTE: Please use [celatone](https://terra.celat.one) or write your own script with feather.js to store code and instantiate contract. Terrain is broken (or my setup is wrong) on signing tx.

```sh
terrain task:run store_code_and_instantiate
```
