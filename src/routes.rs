use rocket::{get, routes, Route};
use rocket_dyn_templates::{context, Template};
use crate::database::ResumeDb;
use crate::repository;
use std::collections::HashMap;

#[get("/")]
pub async fn index(db: ResumeDb) -> Result<Template, String> {
    match repository::get_complete_resume_data(&db).await {
        Ok(resume_data) => {
            // 将技能按分类分组
            let mut skills_by_category = HashMap::new();
            for skill in &resume_data.skills {
                skills_by_category
                    .entry(skill.category.clone())
                    .or_insert_with(Vec::new)
                    .push(skill);
            }
            
            Ok(Template::render("index", context! {
                personal_info: &resume_data.personal_info,
                skills_by_category: &skills_by_category,
                work_experiences: &resume_data.work_experiences,
                education: &resume_data.education,
                projects: &resume_data.projects,
                page_title: "个人简历"
            }))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err("数据库查询失败".to_string())
        }
    }
}

#[get("/api/resume")]
pub async fn api_resume(db: ResumeDb) -> Result<rocket::serde::json::Json<crate::models::ResumeData>, String> {
    match repository::get_complete_resume_data(&db).await {
        Ok(resume_data) => Ok(rocket::serde::json::Json(resume_data)),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err("数据库查询失败".to_string())
        }
    }
}

#[get("/skills")]
pub async fn skills(db: ResumeDb) -> Result<Template, String> {
    match repository::get_skills_by_category(&db).await {
        Ok(skills_by_category) => {
            Ok(Template::render("skills", context! {
                skills_by_category: &skills_by_category,
                page_title: "技能详情"
            }))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err("数据库查询失败".to_string())
        }
    }
}

#[get("/experience")]
pub async fn experience(db: ResumeDb) -> Result<Template, String> {
    match repository::get_work_experiences(&db).await {
        Ok(work_experiences) => {
            Ok(Template::render("experience", context! {
                work_experiences: &work_experiences,
                page_title: "工作经验"
            }))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err("数据库查询失败".to_string())
        }
    }
}

#[get("/projects")]
pub async fn projects(db: ResumeDb) -> Result<Template, String> {
    match repository::get_projects(&db).await {
        Ok(projects) => {
            Ok(Template::render("projects", context! {
                projects: &projects,
                page_title: "项目经验"
            }))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err("数据库查询失败".to_string())
        }
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![index, api_resume, skills, experience, projects]
}
