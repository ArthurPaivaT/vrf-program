# Solana VRF Program

This repository contains the source code to a Solana Random Oracle program, which works together with
the pseudorandom number generator server, available at [Vrf-Server](https://github.com/ArthurPaivaT/vrf-server)

Whenever creating a program to interact with the vrf_program, make sure that you have at least two transactions. One is to Commit and ask for a pseudorandom number, other is to use the number generated.
An integration example is available in the [cpi_example](https://github.com/ArthurPaivaT/vrf-program/tree/main/programs/cpi_example) contract.

The latest version of the vrf_program is deployed in the devnet:

```
vrf-program: 7dsYrsf7cjdMjZBBVv2DzK17Y2Lh8ie16fe48yG1Sn1t
```

## Dependencies

In order to deploy your own version of the vrf_program, you will need to install:

- [Anchor](https://www.anchor-lang.com/docs/installation)
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana-Cli](https://docs.solana.com/cli/install-solana-cli-tools)

## Deployment

**The deployment costs around 4 sol on devnet**

- Make sure you have a keypair configured, and at least 4 sol on devnet beforehand
- Update path to your keypair in `Anchor.toml` that begins with `wallet =`
- Run `anchor build` to build the programs
- We need to update the program IDs:
  - Run `solana-keygen pubkey ./target/deploy/vrf_program-keypair.json` - update the program ID in the following locations:
    - `./Anchor.toml`
    - `./programs/vrf_program/src/lib.rs`
  - If you also want to deploy the cpi_example program, run `solana-keygen pubkey ./target/deploy/cpi_example-keypair.json` - update the program ID in the following locations:
    - `./Anchor.toml`
    - `./programs/cpi_example/src/lib.rs`
- Run `anchor build` to build one more time with the updated program ID.
- Run `anchor deploy --provider.cluster devnet` to deploy to devnet

## License

MIT
