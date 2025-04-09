import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../dev-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("DXh59unrRhdDrsUWNAeH9VkMWBvrXRxgyineW1EsSA3H");

// Recipient address
const to = new PublicKey("<receiver address>");

(async () => {
    try {
        const ata_sender = await getOrCreateAssociatedTokenAccount(
                    connection,
                    keypair,
        
                    mint,
                    keypair.publicKey
        
        
                );
                console.log(`Your ata is: ${ata_sender.address.toBase58()}`);

                const ata_reciever = await getOrCreateAssociatedTokenAccount(
                    connection,
                    keypair,
        
                    mint,
                    to
        
        
                );
                console.log(`Your ata is: ${ata_reciever.address.toBase58()}`);

                const tx = await transfer(
                    connection,
                    keypair,
                    ata_sender.address,
                    ata_reciever.address,
                    keypair,
                    1e9
                );
                console.log(`Your tx is: ${tx}`);

        
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();