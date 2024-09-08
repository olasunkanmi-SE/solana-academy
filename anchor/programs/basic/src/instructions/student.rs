use crate::error::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct EnrollInCourse<'info> {
    #[account(mut)]
    pub academy: Account<'info, Academy>,
    #[account(mut)]
    pub course: Account<'info, Course>,
    #[account(
        init,
        payer = student,
        space = Enrollment::LEN,
        seeds = [b"enrollment", course.key().as_ref(), student.key().as_ref()],
        bump
    )]
    pub enrollment: Account<'info, Enrollment>,
    #[account(mut)]
    pub student: Account<'info, Student>,
    #[account(mut)]
    pub student_nft_mint: Account<'info, Mint>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// TODO refactor code
pub fn enroll_in_course(ctx: Context<EnrollInCourse>, course_id: u64) -> Result<()> {
    let course = &mut ctx.accounts.course;
    let enrollment = &mut ctx.accounts.enrollment;
    let student = &mut ctx.accounts.student;
    let admin = &ctx.accounts.admin;

    require!(course.id == course_id, AcademyError::InvalidCourseId);

    let mint_authority = ctx.accounts.student_nft_mint.mint_authority.unwrap();

    if mint_authority != ctx.accounts.admin.key() {
        return Err(AcademyError::InvalidNFTAuthority.into());
    }

    if student.student_nft != ctx.accounts.student_nft_mint.key() {
        return Err(AcademyError::InvalidStudentNFT.into());
    }

    //Make Payment

    let transfer_instruction =
        system_instruction::transfer(&ctx.accounts.student.key(), admin.key, course.tuition_fee);

    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            ctx.accounts.student.to_account_info(),
            admin.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    enrollment.student = ctx.accounts.student.key();
    enrollment.course = course.key();
    enrollment.enrolled_at = Clock::get()?.unix_timestamp;
    enrollment.completed = false;

    course.enrollment_count += 1;

    msg!("A student has enrolled!");

    Ok(())
}
