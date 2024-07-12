import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
	PublicKey,
	sendAndConfirmTransaction,
	SystemProgram,
} from "@solana/web3.js";
import { Swap } from "../target/types/swap";
import {
	buyerKeypair,
	CORE_PROGRAM_ID,
	findAssetManagerAddress,
	findProtocolAddress,
	MINT_VAULT_ID,
} from "./utils";

describe("swap program", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const provider = anchor.getProvider();

	const program = anchor.workspace.Swap as Program<Swap>;

	it("Swaps tokens for NFTs!", async () => {
		const asset = new PublicKey(
			"CBMg87CRTWA1qenc7inasnnAQDLL5Tu5n8P7geFkiSkj"
		);
		const collection = new PublicKey(
			"EVhj14d1vKAP8ZAdgbkvCYqpcxtVAFYY2Z17sJhbM3k2"
		);

		const previousOwner = new PublicKey(
			"4kg8oh3jdNtn7j2wcS7TrUua31AgbLzDVkBZgTAe44aF"
		);

		const tx = await program.methods
			.swap()
			.accounts({
				payer: buyerKeypair.publicKey,
				buyer: buyerKeypair.publicKey,
				previousOwner,
				asset: asset,
				collection,
				assetManager: findAssetManagerAddress(),
				protocol: findProtocolAddress(),
				coreProgram: CORE_PROGRAM_ID,
				mintVaultProgram: MINT_VAULT_ID,
				systemProgram: SystemProgram.programId,
			})
			.transaction();

		let txHash = await sendAndConfirmTransaction(provider.connection, tx, [
			buyerKeypair,
		]);

		console.log(
			`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
		);
	});
});
