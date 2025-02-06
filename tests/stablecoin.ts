import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";
import { BN } from "bn.js";
import { Stablecoin } from "../target/types/stablecoin";

describe("stablecoin", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.Stablecoin as Program<Stablecoin>;

  const pythSolanaReceiver = new PythSolanaReceiver({connection,wallet})

  const SOL_PRICE_FEED_ID = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

  const solUsdPriceFeedAccount = pythSolanaReceiver
  .getPriceFeedAccountAddress(0,SOL_PRICE_FEED_ID).toBase58();

  console.log(solUsdPriceFeedAccount);

  const [collateralAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("collateral"),wallet.publicKey.toBuffer()],
    program.programId
  );



  it("Is initialized!", async () => {
    const tx = await program.methods
    .initializeConfig()
    .accounts({})
    .rpc({skipPreflight: true,commitment:"confirmed"});
    console.log("Initialize Config tx", tx);
  });

  it("Deposit Collateral And Mint USDC",async () => {
    const amount_to_collateral = 1_000_000_000;
    const amount_to_mint = 1_000_000_000;

    const tx = await program.methods.depositCollateralAndMintToken(
      new BN(amount_to_collateral),
      new BN(amount_to_mint)
    ).accounts({
      priceUpdate : solUsdPriceFeedAccount,
    })
    .rpc({skipPreflight: true,commitment:"confirmed"});

    console.log("Deposit Collateral And Mint USDC TX",tx);
  });

  it("Redeem Collateral And Burn USDC",async () => {
    const amount_to_collateral = 500_000_000;
    const amount_to_burn = 500_000_000;

    const tx = await program.methods.redeemCollateralAndBurnTokens(
      new BN(amount_to_collateral),
      new BN(amount_to_burn)
    ).accounts({
      priceUpdate : solUsdPriceFeedAccount,
    })
    .rpc({skipPreflight: true,commitment:"confirmed"});

    console.log("Redeem Collateral And Burn USDC TX",tx);
  });

  it("Update Config",async () => {

    const tx = await program.methods.updateConfig(
      new BN(1000)
    ).accounts({})
    .rpc({skipPreflight: true,commitment:"confirmed"});

    console.log("Update Config TX",tx);
  });

  it("liquidate",async () => {
    const amount_to_burn = 500_000_000;
    const tx = await program.methods.liquidate(
      new BN(amount_to_burn)
    ).accounts({collateralAccount,priceUpdate:solUsdPriceFeedAccount})
    .rpc({skipPreflight: true,commitment:"confirmed"});

    console.log("Liquidate TX",tx);
  });

  it("Update Config Back",async () => {

    const tx = await program.methods.updateConfig(
      new BN(1)
    ).accounts({})
    .rpc({skipPreflight: true,commitment:"confirmed"});

    console.log("Update Config Back TX",tx);
  });


});
