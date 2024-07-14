import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solvote } from "../target/types/solvote";
import { assert } from "chai";

export function findProposalPDA(
  daoPublicKey: anchor.web3.PublicKey,
  proposalId: number,
  programId: anchor.web3.PublicKey
): [anchor.web3.PublicKey, number] {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      daoPublicKey.toBuffer(),
      new anchor.BN(proposalId).toArrayLike(Buffer, "le", 8),
    ],
    programId
  );
}

export function findUserDaoPDA(
  daoPublicKey: anchor.web3.PublicKey,
  userPublicKey: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): [anchor.web3.PublicKey, number] {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [daoPublicKey.toBuffer(), userPublicKey.toBuffer()],
    programId
  );
}

export function findUserVoterPDA(
  proposalPublicKey: anchor.web3.PublicKey,
  userPublicKey: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): [anchor.web3.PublicKey, number] {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [proposalPublicKey.toBuffer(), userPublicKey.toBuffer()],
    programId
  );
}

describe("solvote", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solvote as Program<Solvote>;
  const provider = anchor.AnchorProvider.env();

  // Accounts
  const daoAccount = anchor.web3.Keypair.generate();
  const user = provider.wallet;

  it("Is initialized!", async () => {
    // Initialize the DAO.
    const tx = await program.methods
      .initialize()
      .accounts({
        dao: daoAccount.publicKey,
        user: user.publicKey,
      })
      .signers([daoAccount])
      .rpc();

    const dao = await program.account.dao.fetch(daoAccount.publicKey);
    assert.ok(dao.totalProposals.toNumber() === 0);
  });

  it("Creates a proposal!", async () => {
    const title = "Test Proposal";
    const desc = "This is a test proposal.";
    const options = ["Option 1", "Option 2"];

    // Create a proposal.
    const tx = await program.methods
      .createProposal(title, desc, options)
      .accounts({
        dao: daoAccount.publicKey,
        user: user.publicKey,
      })
      .rpc();

    const proposalId = 0;
    const [proposalPDA] = findProposalPDA(
      daoAccount.publicKey,
      proposalId,
      program.programId
    );
    const proposal = await program.account.proposal.fetch(proposalPDA);
    assert.ok(proposal.title === title);
    assert.ok(proposal.desc === desc);
    assert.ok(proposal.options.length === options.length);
    assert.ok(proposal.options[0].name === options[0]);
    assert.ok(proposal.options[1].name === options[1]);

    const dao = await program.account.dao.fetch(daoAccount.publicKey);
    assert.ok(dao.totalProposals.toNumber() === 1);
  });

  it("Votes on a proposal!", async () => {
    const proposalId = 0; // Assuming this is the first proposal
    const optionIndex = 0; // Voting for "Option 1"

    // Cast a vote.
    const tx = await program.methods
      .vote(new anchor.BN(proposalId), new anchor.BN(optionIndex))
      .accounts({
        dao: daoAccount.publicKey,
        user: user.publicKey,
      })
      .rpc();
    const [proposalPDA] = findProposalPDA(
      daoAccount.publicKey,
      proposalId,
      program.programId
    );
    const proposal = await program.account.proposal.fetch(proposalPDA);
    assert.ok(proposal.options[optionIndex].count.toNumber() === 1);

    const [userDaoPDA] = findUserDaoPDA(
      daoAccount.publicKey,
      user.publicKey,
      program.programId
    );
    const userDao = await program.account.userDao.fetch(userDaoPDA);
    assert.ok(userDao.points.toNumber() === 1);

    const [userVotePDA] = findUserVoterPDA(
      proposalPDA,
      user.publicKey,
      program.programId
    );
    const userVote = await program.account.userVote.fetch(userVotePDA);
    assert.ok(userVote.vote.toNumber() === optionIndex);
  });
});
