<img alt="Maikor" src="https://github.com/MaikorAppPublic/.github/raw/main/profile/controller_with_logo_blue.png?raw=true" width="200">

>⚠️ Work in progress
>
> Links may be broken, features missing, etc

*Cross-platform 16-bit game system*

See more at [maikor.app](https://maikor.app) and the [project homepage](https://github.com/MaikorAppPublic)

### Play

[Android](https://github.com/MaikorAppPublic/android-app)

[iOS](https://github.com/MaikorAppPublic/ios-app)

[Windows, macOS and Linux](https://github.com/MaikorAppPublic/desktop-app)

### Make

[iOS IDE](https://github.com/MaikorAppPublic/ios-app)

[Desktop IDE](https://github.com/MaikorAppPublic/desktop-ide)

[Build tools](https://github.com/MaikorAppPublic/build-tools)

[REPL](https://play.vm.maikor.app)

[Docs](https://docs.maikor.app)

## maikor-platform

- constants
  - Graphical constants and limits for the platform
- graphics
  - Sprite bit masks
- input
  - Bits used for inputs
- mem
  - sizes
    - Size in byte of sections
  - addresses
    - Address in bytes of sections
- names
  - op
    - Short names, eg. `CPY.B`
  - full
    - OP + args, eg. `CPY.B (R,N)`
- op_params
  - Registers are encoded in one byte, the first four bits are Pre/Post Inc/Dec and the last four bits are the register
- ops
  - Codes for ops
- registers
  - offset
    - Address of register
  - ids
    - ID of register
  - flags
    - Bits for flag register