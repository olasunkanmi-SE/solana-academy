import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { SolanaAcademy } from '../target/types/solana_academy';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';


const { SystemProgram } = anchor.web3;

describe('Solana Academy', () => {
 
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(anchor.AnchorProvider.local());

  const academyName: String = "My test academy";
  const courseName: String = "My academy course";
  /* const courseFee: Number  = 1 * anchor.web3.LAMPORTS_PER_SOL; */
  const courseFee: Number  = 1000;
  

  const program = anchor.workspace.SolanaAcademy as Program<SolanaAcademy>;
  const admin = anchor.web3.Keypair.generate();
  const academy = anchor.web3.Keypair.generate();
  
  //TODO: what about turning courses into PDAs?
  const course = anchor.web3.Keypair.generate();


  beforeAll(async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        admin.publicKey,
        10 * LAMPORTS_PER_SOL
      )
    );
  });

  it('Initializes the Academy', async () => {

    const tx = await program.methods
      .initializeAcademy(academyName)
      .accounts({
        academy: academy.publicKey,
        admin: admin.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin, academy])
      .rpc();

    console.log("Init Academy Tx signature:", tx);

    let academyState = await program.account.academy.fetch(academy.publicKey); 
    console.log("academy data structure", academyState);

    expect(academyState.name).toBe(academyName);
    expect(academyState.admin.toString()).toBe(admin.publicKey.toString());
    expect(academyState.courseCount.toNumber()).toBe(0);
  });

  it('Creates a course', async () => {
    
    
    const courseData = {
      name: courseName,
      description: "Sol dev course",
      start_date: new anchor.BN(1000),  
      end_date: new anchor.BN((Date.now() / 1000) + (42 * 24 * 60 * 60)),  
      tuition_fee: courseFee,  
    };

    const tx = await program.methods
      .createCourse(courseData)
      .accounts({
        academy: academy.publicKey,
        course: course.publicKey,
        admin: admin.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin, course])
      .rpc();

    console.log("Create course Tx signature:", tx);

    let courseState = await program.account.course.fetch(course.publicKey); 

    console.log("Course onchain representation", courseState); 

    const academyState = await program.account.academy.fetch(academy.publicKey);
    
    //FIXME TODO: figure out why the numbers are not serializing/converting correctly
    expect(courseState.id.toNumber()).toBe(academyState.courseCount.toNumber() - 1)
    expect(courseState.name).toBe(courseName);
    expect(courseState.description).toBe(courseData.description);
    /* expect(courseState.startDate.toNumber()).toBe(startDate) */
    /* expect(courseState.endDate.toNumber()).toBe(startDate) */
    /* expect(courseState.tuitionFee.toNumber()).toBe(courseFee); */
    expect(academyState.courseCount.toNumber()).toBe(1);
  });
});
