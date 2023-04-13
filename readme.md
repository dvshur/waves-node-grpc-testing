# Waves Node Blockchain Updates test utility

## Usage

`cargo run --bin subscribe`

| Env             | Default               | Description                                                                                            |
| --------------- | --------------------- | ------------------------------------------------------------------------------------------------------ |
| URL             | http://localhost:6881 | blockchain updates URL                                                                                 |
| FROM_HEIGHT     | 1                     | Start streaming from a given height                                                                    |
| TO_HEIGHT       | 0                     | End streaming on a given height. Leave at 0 for no limit                                               |
| THROTTLE_MILLIS | 0                     | Intentionally slow down reading by sleeping T millis between each update streaming from a given height |
| SILENT          | false                 | Do not output received event height                                                                    |

`cargo run --bin get_block_update`

| Env    | Default               | Description            |
| ------ | --------------------- | ---------------------- |
| URL    | http://localhost:6881 | blockchain updates URL |
| HEIGHT | 1                     | Target block height    |
