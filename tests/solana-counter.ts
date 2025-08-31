import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaCounter } from "../target/types/solana_counter";
import { ProgramB } from "../target/types/program_b";
import { Keypair } from "@solana/web3.js";
import { assert } from "chai";

describe("solana-counter", () => {
  // 配置本地 provider
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

    // SolanaCounter对应Anchor.toml里programs.localnet.solana_counter的PascalCase ！！！
    const program = anchor.workspace.SolanaCounter as Program<SolanaCounter>;
    const programB = anchor.workspace.programB as Program<ProgramB>;

  // 计数器账户 keypair
  const counter = Keypair.generate();

  it("初始化计数器", async () => {
    await program.methods.initialize()
        .accounts({
          counter: counter.publicKey,
          signer: provider.wallet.publicKey,
        })
        .signers([counter]) // counter 是新建的账户，必须签名
        .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    assert.equal(account.count.toNumber(), 0);
    console.log("初始化完成，count =", account.count.toNumber());
  });

  it("执行 increment (+1)", async () => {
    await program.methods.increment()
        .accounts({
          counter: counter.publicKey,
        })
        .rpc();

    const account = await program.account.counter.fetch(counter.publicKey);
    assert.equal(account.count.toNumber(), 1);
    console.log("执行 increment 后，count =", account.count.toNumber());
  });

  it("调用 program-b 执行 increment (+1)", async () => {
      await programB.methods.increase()
          .accounts({
              counter: counter.publicKey,
          })
          .rpc();

      const account = await program.account.counter.fetch(counter.publicKey);
      assert.equal(account.count.toNumber(), 2);
      console.log("执行 programB increase 后，count =", account.count.toNumber());
  })

});

// output:
//   solana-counter
// 初始化完成，count = 0
//     ✔ 初始化计数器 (528ms)
// 执行 increment 后，count = 1
//     ✔ 执行 increment (+1) (480ms)
//
//
//   2 passing (1s)
//
// ✨  Done in 2.18s.