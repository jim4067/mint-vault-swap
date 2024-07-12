# Mint, Vault and Swap

## Project intro

This project, contains two programs to Mint Metaplex Core Collections and assets, lock them inside a vault using the mint-vault program and swap the assets for SOL using the swap Program

The mint-swap charges are as follows

1. A flat fee of 1 SOL for locking asset in our vault.
2. A flat fee of 2 SOL when swapping out an asset inside the vault.

## Technical

When developing change your solana dev environment to match the following

```bash
solana-install init 1.18.8
# solana-cli 1.18.8 (src:e2d34d37; feat:3469865029, client:SolanaLabs)

avm use 0.29.0
# anchor-cli 0.29.0
```

To build the projects

1. Install required yarn dependencies

```bash
yarn install
```

2. Build the project to generate types, idl and deployable sbf program

```bash
anchor build
```

3. The test are split into two

-   the [mint-vault tests](./tests/mint-vault.ts) which mint the asset and lock the asset in the program vault
-   the [swap tests](./tests/swap.ts) for swapping the assets to SOL

Due to public rate limits of the public solana RPC endpoint the tests are run individually one after another

Update the `uploadAssetFiles` method in [utils.ts](./tests/utils.ts) path with your file path for the image

Call the available instructions one by one update the variables appropriately with the required values for the public keys. These might be `collection` `asset` or `previous owner`

Available instructions are

1. MintVault program

-   `init` - **only call this if you deploy a new program**. Initializes the program protocol state and asset manager accounts
-   `create_collection` - creates a collection for the assets that will be minted
-   `mint_asset` - mints asset from previously created collection
-   `lock_in_vault` - locks an asset in the program vault. Protocol takes a flat fee of 1 SOl for locking the asset
-   `purchase` - purchase an asset locked in program vault for a flat fee of 2 SOL

2. Swap program

-   swap. CPI's into mint-vault program and calls purchase IX to exchange an asset locked in vault for SOL

### Transactions

-   [init](https://explorer.solana.com/tx/cGZWoTWW2iihzAEBG2g2m3gdbkHU1NEidva2gsnJ79DtHjq4P8ND9J4etzQJeu7ptvGd4HxkZaBPHN19QjVuwjC?cluster=devnet)

-   [create collection](https://explorer.solana.com/tx/2wgL1f5k2YtBjDxYbJdFwceZKt18BUNK42XjWP1u3rWiLHT78yfvRMeQEGogozwJnSx2B8MNrUeJUJk9zPrmChQW?cluster=devnet)

    [created collection - EVhj14d1vKAP8ZAdgbkvCYqpcxtVAFYY2Z17sJhbM3k2 ](https://mpl-core-ui.vercel.app/explorer/collection/EVhj14d1vKAP8ZAdgbkvCYqpcxtVAFYY2Z17sJhbM3k2?env=devnet)

-   [mint asset](https://explorer.solana.com/tx/4G3PukKuFZQZim5dgDcGmvqXxWSLwyEqV9f9JnXqKerDsAX6y6C2mRLW2WX4rQ7PjptwX6qGCiXPJ58TRHX541jD?cluster=devnet)

    [minted asset - CBMg87CRTWA1qenc7inasnnAQDLL5Tu5n8P7geFkiSkj](https://mpl-core-ui.vercel.app/explorer/CBMg87CRTWA1qenc7inasnnAQDLL5Tu5n8P7geFkiSkj?env=devnet)

-   [lock in vault](https://explorer.solana.com/tx/p4jBwenbZXP7e16FC1q5MTgEqfWv2fAbKDCCq4mxdbfh1y6xQu8vymxe1LyTo9pWAuLWV2wk7M8LrabcVX4Rtam?cluster=devnet)

-   [swap](https://explorer.solana.com/tx/57F2V6mFSRMvMDzWTtomPK3XzgoX5TSnpywqnFVp88e4fRnrsCeTWupBbeweFEtJUbwfw9RfyxgoFLfBb3pPQuXN?cluster=devnet)
