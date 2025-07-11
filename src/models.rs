use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct PersonalInfo {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub email: String,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub summary: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub proficiency: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct WorkExperience {
    pub id: i32,
    pub company: String,
    pub position: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub achievements: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Education {
    pub id: i32,
    pub institution: String,
    pub degree: String,
    pub field_of_study: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub gpa: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub technologies: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub url: Option<String>,
    pub github_url: Option<String>,
    pub created_at: NaiveDateTime,
}

// 用于模板渲染的完整简历数据结构
#[derive(Serialize, Debug)]
pub struct ResumeData {
    pub personal_info: PersonalInfo,
    pub skills: Vec<Skill>,
    pub work_experiences: Vec<WorkExperience>,
    pub education: Vec<Education>,
    pub projects: Vec<Project>,
}
