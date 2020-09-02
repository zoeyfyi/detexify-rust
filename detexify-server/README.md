# Detexify Server

Rust port of the [detexify backend](https://github.com/kirel/detexify-hs-backend). Note, this only includes the `/classify` route (using the classifier snapshot found [here](https://github.com/kirel/detexify-hs-backend/blob/master/snapshot.json)), not the training or snapshot routes.

## Run

```
ROCKET_PORT=3000 cargo run
```
