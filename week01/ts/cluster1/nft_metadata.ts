import wallet from "../dev-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader({ address: "https://devnet.irys.xyz/" }));
umi.use(signerIdentity(signer));

const isrysURI ="https://arweave.net/9CLAzaWFaL6CT9G45yUyGRsYUHrmSNrzD6aShNTRcbrs".replace( 
    "https://arweave.net/" , 
    "https://devnet.irys.xyz/"
) ;
console.log("your img uri :" , isrysURI);

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = isrysURI;
        const metadata = {
            name: "Jeff the Meat Lord",
            symbol: "BEEFMAX",
            description: "Jeff,,,,",
            image: image,
            external_url: "",
            attributes: [
                { trait_type: "colors", value: "20" },
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    }
                ],
                creators: [
                    {
                        address: keypair.publicKey.toString(),
                        share: 100
                    }
                ]
            }
        };
        const myUri = await umi.uploader.uploadJson(metadata);
          console.log("✅ Your metadata URI: ", myUri);
            }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
