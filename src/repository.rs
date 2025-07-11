use crate::database::{ResumeDb, *};
use crate::models::*;
use rocket::serde::json::Json;
use std::collections::HashMap;

pub async fn get_personal_info(db: &ResumeDb) -> Result<PersonalInfo, diesel::result::Error> {
    db.run(|conn| {
        personal_info::table
            .first::<PersonalInfo>(conn)
    }).await
}

pub async fn get_all_skills(db: &ResumeDb) -> Result<Vec<Skill>, diesel::result::Error> {
    db.run(|conn| {
        skills::table
            .order(skills::category.asc())
            .load::<Skill>(conn)
    }).await
}

pub async fn get_skills_by_category(db: &ResumeDb) -> Result<HashMap<String, Vec<Skill>>, diesel::result::Error> {
    let all_skills = get_all_skills(db).await?;
    let mut skills_by_category = HashMap::new();
    
    for skill in all_skills {
        skills_by_category
            .entry(skill.category.clone())
            .or_insert_with(Vec::new)
            .push(skill);
    }
    
    Ok(skills_by_category)
}

pub async fn get_work_experiences(db: &ResumeDb) -> Result<Vec<WorkExperience>, diesel::result::Error> {
    db.run(|conn| {
        work_experiences::table
            .order(work_experiences::start_date.desc())
            .load::<WorkExperience>(conn)
    }).await
}

pub async fn get_education(db: &ResumeDb) -> Result<Vec<Education>, diesel::result::Error> {
    db.run(|conn| {
        education::table
            .order(education::start_date.desc())
            .load::<Education>(conn)
    }).await
}

pub async fn get_projects(db: &ResumeDb) -> Result<Vec<Project>, diesel::result::Error> {
    db.run(|conn| {
        projects::table
            .order(projects::start_date.desc())
            .load::<Project>(conn)
    }).await
}

pub async fn get_complete_resume_data(db: &ResumeDb) -> Result<ResumeData, Box<dyn std::error::Error + Send + Sync>> {
    let personal_info = get_personal_info(db).await?;
    let skills = get_all_skills(db).await?;
    let work_experiences = get_work_experiences(db).await?;
    let education = get_education(db).await?;
    let projects = get_projects(db).await?;
    
    Ok(ResumeData {
        personal_info,
        skills,
        work_experiences,
        education,
        projects,
    })
}
