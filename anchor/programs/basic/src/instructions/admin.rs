use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeAcademy<'info> {
    #[account(init, payer = admin, space = Academy::LEN)]
    pub academy: Account<'info, Academy>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_academy(ctx: Context<InitializeAcademy>, name: String) -> Result<()> {
    let academy = &mut ctx.accounts.academy;
    academy.name = name;
    academy.admin = ctx.accounts.admin.key();
    academy.course_count = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateCourse<'info> {
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

// Add other admin instructions here
