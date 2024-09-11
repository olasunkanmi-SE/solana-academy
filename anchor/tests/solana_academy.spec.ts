import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { createAccount, createMint, mintTo } from '@solana/spl-token';
import { SolanaAcademy } from '../target/types/solana_academy';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';

describe('academy', () => {
  let provider: anchor.AnchorProvider;
  let program: anchor.Program<SolanaAcademy>;
  let admin: anchor.web3.Keypair;
  let user: anchor.web3.Keypair;
  let academy: anchor.web3.Keypair,
    student: anchor.web3.Keypair,
    name,
    enrollmentFee,
    academyAccount,
    studentNftMint,
    studentTokenAccount,
    course: anchor.web3.Keypair;

  beforeEach(async () => {
    provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    program = anchor.workspace.SolanaAcademy as Program<SolanaAcademy>;
    admin = anchor.web3.Keypair.generate();
    user = anchor.web3.Keypair.generate();
    academy = anchor.web3.Keypair.generate();
    student = anchor.web3.Keypair.generate();
    course = anchor.web3.Keypair.generate();
    name = 'Test Academy';
    enrollmentFee = new anchor.BN(1000000);

    const adminWalletAirDrop = await provider.connection.requestAirdrop(
      admin.publicKey,
      10000000
    );
    await provider.connection.confirmTransaction(adminWalletAirDrop);

    const userWalletAirDrop = await provider.connection.requestAirdrop(
      user.publicKey,
      10000000
    );
    await provider.connection.confirmTransaction(userWalletAirDrop);

    const StudentAirDrop = await provider.connection.requestAirdrop(
      student.publicKey,
      10000000
    );
    await provider.connection.confirmTransaction(StudentAirDrop);

    await program.methods
      .initializeAcademy(name, enrollmentFee)
      .accounts({
        academy: academy.publicKey,
        admin: admin.publicKey,
      })
      .signers([admin, academy])
      .rpc();
    academyAccount = await program.account.academy.fetch(academy.publicKey);

    studentNftMint = await createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      0
    );

    studentTokenAccount = await createAccount(
      provider.connection,
      user,
      studentNftMint,
      user.publicKey
    );
  });

  it('Should create an academy', async () => {
    expect(academyAccount.name).toBe(name);
    expect(academyAccount.enrollmentFee.toNumber()).toBe(
      enrollmentFee.toNumber()
    );
    expect(academyAccount.admin.toBase58()).toBe(admin.publicKey.toBase58());
    expect(academyAccount.courseCount.toNumber()).toBe(0);
  });

  it('Should enroll a student in the academy', async () => {
    const initialUserBalance = await provider.connection.getBalance(
      user.publicKey
    );
    const initialAdminBalance = await provider.connection.getBalance(
      admin.publicKey
    );
    await program.methods
      .enrollStudentInAcademy(enrollmentFee)
      .accounts({
        academy: academy.publicKey,
        student: student.publicKey,
        studentNftMint: studentNftMint,
        studentTokenAccount: studentTokenAccount,
        user: user.publicKey,
        admin: admin.publicKey,
      })
      .signers([user, admin, student])
      .rpc();

    const academyAccount = await program.account.academy.fetch(
      academy.publicKey
    );
    const studentAccount = await program.account.student.fetch(
      student.publicKey
    );
    const finalUserBalance = await provider.connection.getBalance(
      user.publicKey
    );
    const finalAdminBalance = await provider.connection.getBalance(
      admin.publicKey
    );

    const userBalanceDifference = initialUserBalance - finalUserBalance;
    const adminBalanceDifference = finalAdminBalance - initialAdminBalance;
    expect(userBalanceDifference).toBeGreaterThanOrEqual(
      parseInt(enrollmentFee.toString())
    );
    expect(finalAdminBalance).toBeGreaterThan(initialAdminBalance);
    expect(academyAccount.name).toBe(name);
    expect(parseInt(academyAccount.studentCounter)).toEqual(1);
    expect(studentAccount.studentNft).toEqual(studentNftMint);
    expect(adminBalanceDifference).toBe(enrollmentFee.toNumber());
  });

  it('Creates a course', async () => {
    // Generate a new keypair for the course account
    course = anchor.web3.Keypair.generate();

    // Prepare course data
    const courseData = {
      name: 'Introduction to Solana',
      description: 'Learn the basics of Solana blockchain development',
      startDate: new anchor.BN(Date.now() / 1000),
      endDate: new anchor.BN(Date.now() / 1000 + 30 * 24 * 60 * 60), // 30 days from now
      tuitionFee: new anchor.BN(1 * LAMPORTS_PER_SOL),
    };

    // Call the create_course instruction
    await program.methods
      .createCourse(courseData)
      .accounts({
        academy: academy.publicKey,
        course: course.publicKey,
        admin: admin.publicKey,
      })
      .signers([admin, course])
      .rpc();

    const courseAccount = await program.account.course.fetch(course.publicKey);

    const academyAccount = await program.account.academy.fetch(
      academy.publicKey
    );

    expect(courseAccount.id.toNumber()).toBe(0);
    expect(courseAccount.name).toBe(courseData.name);
    expect(courseAccount.description).toBe(courseData.description);
    expect(courseAccount.startDate.toNumber()).toBe(
      courseData.startDate.toNumber()
    );
    expect(courseAccount.endDate.toNumber()).toBe(
      courseData.endDate.toNumber()
    );
    expect(courseAccount.tuitionFee.toNumber()).toBe(
      courseData.tuitionFee.toNumber()
    );
    expect(courseAccount.enrollmentCount.toNumber()).toBe(0);

    expect(academyAccount.courseCount.toNumber()).toBe(1);

    expect(academyAccount.admin).toStrictEqual(admin.publicKey);
  });

  // it('Should enroll students in course', async () => {
  //   const [enrollmentPda, _] =
  //     await anchor.web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.from('enrollment'),
  //         course.publicKey.toBuffer(),
  //         student.publicKey.toBuffer(),
  //       ],
  //       program.programId
  //     );

  //   const intialStudentBalance = await provider.connection.getBalance(
  //     student.publicKey
  //   );
  //   const initialAdminBalance = await provider.connection.getBalance(
  //     admin.publicKey
  //   );

  //   await createAccount(
  //     provider.connection,
  //     admin,
  //     studentNftMint,
  //     student.publicKey
  //   );

  //   await program.methods
  //     .enrollInCourse(new anchor.BN(0))
  //     .accounts({
  //       academy: academy.publicKey,
  //       course: course.publicKey,
  //       enrollment: enrollmentPda,
  //       student: student.publicKey,
  //       studentNftMint: studentNftMint,
  //       admin: admin.publicKey,
  //     })
  //     .signers([admin])
  //     .rpc();

  //   const courseAccount = await program.account.course.fetch(course.publicKey);
  //   expect(courseAccount.enrollmentCount).toEqual(1);
  // });

  it('Should fail to enroll with insufficient balance', async () => {
    const user = anchor.web3.Keypair.generate();
    await provider.connection.requestAirdrop(
      user.publicKey,
      0.1 * LAMPORTS_PER_SOL
    );
    expect(
      program.methods
        .enrollStudentInAcademy(enrollmentFee)
        .accounts({
          academy: academy.publicKey,
          student: student.publicKey,
          studentNftMint: studentNftMint,
          studentTokenAccount: studentTokenAccount,
          user: user.publicKey,
          admin: admin.publicKey,
        })
        .signers([user, admin, student])
        .rpc()
    ).rejects.toThrow('Insufficient balance to pay school fees');
  });
});
