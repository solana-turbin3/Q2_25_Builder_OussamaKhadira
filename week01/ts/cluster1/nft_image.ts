import wallet from "../dev-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"
import { ruleSetToggle } from "@metaplex-foundation/mpl-token-metadata"

// Create a devnet connection
const umi = createUmi('https://turbine-solanad-4cde.devnet.rpcpool.com/168dd64f-ce5e-4e19-a836-f6482ad6b396');
// const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader({ address: "https://devnet.irys.xyz/" }));

umi.use(signerIdentity(signer));
// const filename ="generug.png";

(async () => {
    try {
        //1. Load image
        const image = await readFile("./generug.png" )
        //2. Convert image to generic file.
        const genericFile = await createGenericFile(image ,"rug.png" ,{
            contentType:"image/png",
        });
        //3. Upload image
        const [myUri] = await umi.uploader.upload([genericFile]);
        // const irysURI = myUri.replace(
        //     "https://arweave.net/",
        //     "https://devnet.irys.xyz/"
        //   );


        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
