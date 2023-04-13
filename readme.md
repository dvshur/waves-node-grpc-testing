# Waves Node Blockchain Updates test utility

## Environment

`subscribe` binary (default, and only one available in Docker)

| Env             | Default               | Description                                                                |
| --------------- | --------------------- | -------------------------------------------------------------------------- |
| URL             | http://localhost:6881 | blockchain updates URL                                                     |
| FROM_HEIGHT     | 1                     | Start streaming from a given height                                        |
| TO_HEIGHT       | 0                     | End streaming on a given height. Leave at 0 for no limit                   |
| THROTTLE_MILLIS | 0                     | Intentionally slow down streaming by sleeping T millis between each update |
| SILENT          | false                 | Do not output received event height                                        |

`get_block_update` binary

| Env    | Default               | Description            |
| ------ | --------------------- | ---------------------- |
| URL    | http://localhost:6881 | blockchain updates URL |
| HEIGHT | 1                     | Target block height    |

## Usage

### Docker

Provided `Dockerfile` only includes `subscribe` binary.
Usage is straightforward: `docker build`, then `docker run`.

### Cargo

`cargo run --bin subscribe`

`cargo run --bin get_block_update`

### Performance benchmarking

For performance benchmarks, one might want to disable logging by setting env `SILENT=true` and running `cargo` in `--release` mode.

Docker always runs in release mode.
