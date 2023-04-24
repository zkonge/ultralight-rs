# ultralight-rs

[Ultralight](https://ultralig.ht/) API binding for Rust.

Focusing on using the lightweight headless browser as renderer.

For giving simple page[^1], It takes about 30MB of RAM and less than 10ms to render page into PNG[^2].

## Build

1. Get the `v1.3.0` SDK from [ultralight GitHub Page](https://github.com/ultralight-ux/ultralight#getting-the-latest-sdk)
2. Set `ULTRALIGHT_SDK_PATH` environment variable to the SDK path
3. Compile and ...
4. do not forget to set `LD_LIBRARY_PATH` to the SDK bin path, or copy dynamic library to target path (both Linux and Windows).
5. copy `resouces` to the filesystem root (specified in code).

## Example

see `ultralight/examples`

## License

### for the binding SDK
AGPL for now, but I'm considering to change to in the future.

### for the ultralight itself
see [ultralight GitHub Page](https://github.com/ultralight-ux/ultralight#licensing)

**NOTICE**: Ultralight has a commercial license.

[^1]: In `ultralight/examples`.

[^2]: On my Windows laptop.
