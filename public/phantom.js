window.solana_request = async function(method, params) {
    if (!window.solana || !window.solana.isPhantom) {
        throw new Error("Phantom wallet not found");
    }

    try {
        const connection = new solanaWeb3.Connection(solanaWeb3.clusterApiUrl('devnet'));
        const wallet = window.solana;
        
        switch (method) {
            case "createToken": {
                // Create mint account
                const mintAccount = solanaWeb3.Keypair.generate();
                const mintRent = await connection.getMinimumBalanceForRentExemption(
                    solanaWeb3.MINT_SIZE
                );

                // Create metadata account
                const [metadataAddress] = await solanaWeb3.PublicKey.findProgramAddress(
                    [
                        Buffer.from("metadata"),
                        mplTokenMetadata.PROGRAM_ID.toBuffer(),
                        mintAccount.publicKey.toBuffer(),
                    ],
                    mplTokenMetadata.PROGRAM_ID
                );

                const transaction = new solanaWeb3.Transaction();
                
                // Create mint account
                transaction.add(
                    solanaWeb3.SystemProgram.createAccount({
                        fromPubkey: wallet.publicKey,
                        newAccountPubkey: mintAccount.publicKey,
                        space: solanaWeb3.MINT_SIZE,
                        lamports: mintRent,
                        programId: splToken.TOKEN_PROGRAM_ID,
                    })
                );

                // Initialize mint
                transaction.add(
                    splToken.createInitializeMintInstruction(
                        mintAccount.publicKey,
                        params.decimals,
                        wallet.publicKey,
                        params.freeze_authority ? wallet.publicKey : null,
                        splToken.TOKEN_PROGRAM_ID
                    )
                );

                // Create metadata
                transaction.add(
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

                // Create ATA for initial supply
                const ata = await splToken.getAssociatedTokenAddress(
                    mintAccount.publicKey,
                    wallet.publicKey,
                    false,
                    splToken.TOKEN_PROGRAM_ID,
                    splToken.ASSOCIATED_TOKEN_PROGRAM_ID
                );

                // Create ATA
                transaction.add(
                    splToken.createAssociatedTokenAccountInstruction(
                        wallet.publicKey,
                        ata,
                        wallet.publicKey,
                        mintAccount.publicKey,
                        splToken.TOKEN_PROGRAM_ID,
                        splToken.ASSOCIATED_TOKEN_PROGRAM_ID
                    )
                );

                // Mint initial supply
                const initialSupply = params.initial_supply * Math.pow(10, params.decimals);
                transaction.add(
                    splToken.createMintToInstruction(
                        mintAccount.publicKey,
                        ata,
                        wallet.publicKey,
                        initialSupply,
                        [],
                        splToken.TOKEN_PROGRAM_ID
                    )
                );

                // Mint sample amount if different
                const sampleAmount = 1000 * Math.pow(10, params.decimals);
                if (sampleAmount !== initialSupply) {
                    transaction.add(
                        splToken.createMintToInstruction(
                            mintAccount.publicKey,
                            ata,
                            wallet.publicKey,
                            sampleAmount,
                            [],
                            splToken.TOKEN_PROGRAM_ID
                        )
                    );
                }

                // Sign with mint account
                transaction.partialSign(mintAccount);
                
                // Request wallet signature and send
                const signed = await wallet.signAndSendTransaction(transaction);
                const confirmation = await connection.confirmTransaction(signed.signature);
                
                if (confirmation.value.err) {
                    throw new Error(`Transaction failed: ${confirmation.value.err}`);
                }

                const result = {
                    signature: signed.signature,
                    mint: mintAccount.publicKey.toString(),
                    metadata: metadataAddress.toString(),
                    explorer_url: `https://solscan.io/token/${mintAccount.publicKey.toString()}?cluster=devnet`,
                };

                console.log("Token created successfully:", result);
                return result;
            }
            default:
                throw new Error(`Unknown method: ${method}`);
        }
    } catch (error) {
        console.error("Token creation error:", error);
        throw error;
    }
} 