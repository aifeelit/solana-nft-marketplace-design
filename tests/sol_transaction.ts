import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SolTransaction } from '../target/types/sol_transaction';

describe('sol_transaction', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolTransaction as Program<SolTransaction>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
