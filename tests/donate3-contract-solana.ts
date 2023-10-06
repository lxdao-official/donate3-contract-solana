import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import {Donate3ContractSolana} from "../target/types/donate3_contract_solana";
import {Metaplex} from "@metaplex-foundation/js"
import {getAssociatedTokenAddressSync} from "@solana/spl-token";

const user = anchor.web3.Keypair.generate()
describe("donate3-contract-solana", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Donate3ContractSolana as Program<Donate3ContractSolana>;

    // metaplex token metadata program ID
    const TOKEN_METADATA_PROGRAM_ID = new web3.PublicKey(
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );

    // metaplex setup
    const connection = program.provider.connection
    const metaplex = Metaplex.make(connection);


    it("Is initialized!", async () => {

        await connection.requestAirdrop(
            user.publicKey,
            web3.LAMPORTS_PER_SOL,
        );

        const [rewardTokenMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("collection")],
            program.programId
        );

        const rewardTokenMintMetadataPDA = metaplex
            .nfts()
            .pdas()
            .metadata({mint: rewardTokenMintPDA});


        const rewardTokenMintMasterPDA = metaplex
            .nfts()
            .pdas()
            .masterEdition({mint: rewardTokenMintPDA});

        const TokenAccount = getAssociatedTokenAddressSync(
            rewardTokenMintPDA,
            user.publicKey
        );

        // Add your test here.
        const tx = await program.methods.initialize().accounts({
            signer: user.publicKey,
            collectionMint: rewardTokenMintPDA,
            tokenAccount: TokenAccount,
            metadataAccount: rewardTokenMintMetadataPDA,
            masterEdition: rewardTokenMintMasterPDA,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
            associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        }).rpc();

        console.log("Your transaction signature", tx);
    });

});















