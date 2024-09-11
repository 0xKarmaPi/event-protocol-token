import {
  percentAmount,
  generateSigner,
  signerIdentity,
  createSignerFromKeypair,
} from "@metaplex-foundation/umi";
import {
  TokenStandard,
  createAndMint,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { web3 } from "@coral-xyz/anchor";
import fs from "fs";
import { url } from "inspector";

// yarn add @metaplex-foundation/umi @metaplex-foundation/mpl-token-metadata @metaplex-foundation/umi-bundle-defaults
export function loadWalletKey(keyPairFile: string): web3.Keypair {
  const kp = web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(keyPairFile, "utf-8")))
  );
  return kp;
}

const umi = createUmi(
  "https://devnet.sonic.game/"
  //   "https://api.devnet.solana.com",
  // "http://localhost:8899"
);

const userWallet = umi.eddsa.createKeypairFromSecretKey(
  new Uint8Array(
    JSON.parse(
      fs.readFileSync("/Users/lainhathoang/.config/solana/id.json", "utf-8")
    )
  )
);
const userWalletSigner = createSignerFromKeypair(umi, userWallet);

const metadata = {
  name: "EVENT Token",
  symbol: "EVENT",
  description: "$EVENT token",
    uri: "https://event-protocol.s3.ap-southeast-1.amazonaws.com/event-token.json",
//   uri: "https://gist.githubusercontent.com/lainhathoang/adfad1e251ac190cc28098e6d6649e83/raw/062ea383e3f4ccc849eda06b47a47016229d5490/event_token.json",
};

const mint = generateSigner(umi);
umi.use(signerIdentity(userWalletSigner));
umi.use(mplTokenMetadata());

// const mint = publicKey("token_mint");

// CREATE & MINT
createAndMint(umi, {
  mint,
  authority: null,
  name: metadata.name,
  symbol: metadata.symbol,
  uri: metadata.uri,
  sellerFeeBasisPoints: percentAmount(0),
  decimals: 6,
  amount: 1_000_000_000_000000,
  tokenOwner: userWallet.publicKey,
  tokenStandard: TokenStandard.Fungible,
  isMutable: true,
})
  .sendAndConfirm(umi)
  .then(() => {
    console.log("Successfully minted 1 million tokens (", mint.publicKey, ")");
  })
  .catch((err) => {
    console.error("Error minting tokens:", err);
  });

// AFTER CRATE THE TOKEN MINT & MINT 1.000.000.001 TOKENs
// run the scripts below by the deployer's wallet
//
// spl-token authorize token_mint_addresss mint --disable
// spl-token authorize token_mint_address freeze --disable
//
// https://explorer.sonic.game/address/HEe2orwNWmRjarL6j356XEzgcD9WWc1PJ6MvTfMRnALw
