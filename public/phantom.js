// Initialize window functions immediately
if (typeof window !== 'undefined') {
    // Ensure function is defined before WASM loads
    window.update_status = function(status) {
        console.log("Status:", status);
        const statusElement = document.getElementById('creation-status');
        if (statusElement) {
            statusElement.textContent = status;
        }
        return true; // Return value for wasm binding
    };
}

window.solana_request = async function(method, params) {
    if (!window.solana || !window.solana.isPhantom) {
        throw new Error("Phantom wallet not found");
    }

    const MAX_RETRIES = 3;
    const RETRY_DELAY = 1000;
    let retries = 0;

    while (retries < MAX_RETRIES) {
        try {
            const connection = new solanaWeb3.Connection(
                solanaWeb3.clusterApiUrl('devnet'),
                'confirmed'
            );
            const wallet = window.solana;
            
            switch (method) {
                case "createToken": {
                    // Create mint account
                    window.update_status("Creating mint account...");
                    const mintAccount = solanaWeb3.Keypair.generate();
                    const mintRent = await connection.getMinimumBalanceForRentExemption(
                        solanaWeb3.MINT_SIZE
                    );

                    // First transaction: Create and initialize mint
                    const transaction1 = new solanaWeb3.Transaction();
                    
                    transaction1.add(
                        solanaWeb3.SystemProgram.createAccount({
                            fromPubkey: wallet.publicKey,
                            newAccountPubkey: mintAccount.publicKey,
                            space: solanaWeb3.MINT_SIZE,
                            lamports: mintRent,
                            programId: splToken.TOKEN_PROGRAM_ID,
                        }),
                        splToken.createInitializeMintInstruction(
                            mintAccount.publicKey,
                            params.decimals,
                            wallet.publicKey,
                            params.freeze_authority ? wallet.publicKey : null,
                            splToken.TOKEN_PROGRAM_ID
                        )
                    );

                    window.update_status("Creating token account...");
                    const latestBlockhash1 = await connection.getLatestBlockhash('confirmed');
                    transaction1.recentBlockhash = latestBlockhash1.blockhash;
                    transaction1.feePayer = wallet.publicKey;
                    transaction1.partialSign(mintAccount);
                    
                    const signed1 = await wallet.signAndSendTransaction(transaction1);
                    await connection.confirmTransaction({
                        signature: signed1.signature,
                        blockhash: latestBlockhash1.blockhash,
                        lastValidBlockHeight: latestBlockhash1.lastValidBlockHeight
                    });

                    // Second transaction: Create metadata
                    window.update_status("Creating metadata...");
                    const [metadataAddress] = await solanaWeb3.PublicKey.findProgramAddress(
                        [
                            Buffer.from("metadata"),
                            mplTokenMetadata.PROGRAM_ID.toBuffer(),
                            mintAccount.publicKey.toBuffer(),
                        ],
                        mplTokenMetadata.PROGRAM_ID
                    );

                    const transaction2 = new solanaWeb3.Transaction();
                    transaction2.add(
                        mplTokenMetadata.createCreateMetadataAccountV3Instruction(
                            {
                                metadata: metadataAddress,
                                mint: mintAccount.publicKey,
                                mintAuthority: wallet.publicKey,
                                payer: wallet.publicKey,
                                updateAuthority: wallet.publicKey,
                            },
                            {
                                data: {
                                    name: params.name,
                                    symbol: params.symbol,
                                    uri: params.metadata_uri,
                                    sellerFeeBasisPoints: 0,
                                    creators: null,
                                    collection: null,
                                    uses: null,
                                },
                                isMutable: params.is_mutable,
                                collectionDetails: null,
                            }
                        )
                    );

                    const latestBlockhash2 = await connection.getLatestBlockhash('confirmed');
                    transaction2.recentBlockhash = latestBlockhash2.blockhash;
                    transaction2.feePayer = wallet.publicKey;
                    
                    const signed2 = await wallet.signAndSendTransaction(transaction2);
                    await connection.confirmTransaction({
                        signature: signed2.signature,
                        blockhash: latestBlockhash2.blockhash,
                        lastValidBlockHeight: latestBlockhash2.lastValidBlockHeight
                    });

                    // Third transaction: Create ATA and mint tokens
                    window.update_status("Creating token account and minting...");
                    const ata = await splToken.getAssociatedTokenAddress(
                        mintAccount.publicKey,
                        wallet.publicKey
                    );

                    const transaction3 = new solanaWeb3.Transaction();
                    transaction3.add(
                        splToken.createAssociatedTokenAccountInstruction(
                            wallet.publicKey,
                            ata,
                            wallet.publicKey,
                            mintAccount.publicKey
                        ),
                        splToken.createMintToInstruction(
                            mintAccount.publicKey,
                            ata,
                            wallet.publicKey,
                            params.initial_supply * Math.pow(10, params.decimals),
                            []
                        )
                    );

                    const latestBlockhash3 = await connection.getLatestBlockhash('confirmed');
                    transaction3.recentBlockhash = latestBlockhash3.blockhash;
                    transaction3.feePayer = wallet.publicKey;
                    
                    const signed3 = await wallet.signAndSendTransaction(transaction3);
                    const confirmation = await connection.confirmTransaction({
                        signature: signed3.signature,
                        blockhash: latestBlockhash3.blockhash,
                        lastValidBlockHeight: latestBlockhash3.lastValidBlockHeight
                    });

                    if (confirmation.value.err) {
                        throw new Error(`Transaction failed: ${confirmation.value.err}`);
                    }

                    window.update_status("Verifying token...");
                    const tokenAccount = await connection.getAccountInfo(mintAccount.publicKey);
                    if (!tokenAccount) {
                        throw new Error("Token account not found after creation");
                    }

                    const result = {
                        signature: signed3.signature,
                        mint: mintAccount.publicKey.toString(),
                        metadata: metadataAddress.toString(),
                        explorer_url: `https://solscan.io/token/${mintAccount.publicKey.toString()}?cluster=devnet`,
                        status: "Token created successfully!"
                    };

                    console.log("Token created successfully:", result);
                    return result;
                }
                default:
                    throw new Error(`Unknown method: ${method}`);
            }
        } catch (error) {
            console.error("Token creation error:", error);
            if (error?.message?.includes('429') && retries < MAX_RETRIES - 1) {
                window.update_status(`Rate limit reached. Retrying (${retries + 1}/${MAX_RETRIES})...`);
                retries++;
                await new Promise(resolve => setTimeout(resolve, RETRY_DELAY * retries));
                continue;
            }
            throw error;
        }
    }
} 