use crate::{error::AcademyError, state::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use solana_program::system_instruction;

#[derive(Accounts)]
pub struct InitializeAcademy<'info> {
    #[account(init, payer = admin, space = Academy::LEN)]
    pub academy: Account<'info, Academy>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_academy(
    ctx: Context<InitializeAcademy>,
    name: String,
    enrolement_fee: u64,
) -> Result<()> {
    let academy = &mut ctx.accounts.academy;
    academy.name = name;
    academy.enrollment_fee = enrolement_fee;
    academy.admin = ctx.accounts.admin.key();
    academy.course_count = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct EnrollInAcademy<'info> {
    #[account(mut)]
    pub academy: Account<'info, Academy>,
    #[account(init, payer = user, space = 8 + 8 + 32 + 200)]
    pub student: Account<'info, Student>,
    #[account(mut, constraint = student_nft_mint.mint_authority.unwrap() == admin.key())]
    pub student_nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub student_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn enroll_student_in_academy(ctx: Context<EnrollInAcademy>, payment: u64) -> Result<()> {
    let from_account = &ctx.accounts.user;
    let to_account = &ctx.accounts.admin;
    let academy = &mut ctx.accounts.academy;

    if payment < academy.enrollment_fee {
        return Err(AcademyError::InsufficientBalance.into());
    }

    let user_balance = from_account.lamports();
    if user_balance < payment {
        return Err(AcademyError::InsufficientSchoolFee.into());
    }

    // Make payment

    let transfer_instruction =
        system_instruction::transfer(from_account.key, to_account.key, academy.enrollment_fee);

    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            from_account.to_account_info(),
            to_account.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    // Mint new student ID NFT
    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.student_nft_mint.to_account_info(),
        to: ctx.accounts.student_token_account.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, 1)?;

    // Create student account
    let student = &mut ctx.accounts.student;
    student.student_id = academy.student_counter;
    student.student_nft = ctx.accounts.student_nft_mint.key();
    academy.student_counter += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateCourse<'info> {
    // This means that the User account can only be modified by the admin account
    #[account(mut, has_one = admin)]
    pub academy: Account<'info, Academy>,
    #[account(init, payer = admin, space = Course::LEN)]
    pub course: Account<'info, Course>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_course(ctx: Context<CreateCourse>, course_data: CourseData) -> Result<()> {
    let academy = &mut ctx.accounts.academy;
    let course = &mut ctx.accounts.course;

    course.id = academy.course_count;
    course.name = course_data.name;
    course.description = course_data.description;
    course.start_date = course_data.start_date;
    course.end_date = course_data.end_date;
    course.tuition_fee = course_data.tuition_fee;
    course.enrollment_count = 0;

    academy.course_count += 1;

    Ok(())
}
