use rust_demo::{cookies, env_get};
use std::collections::HashMap;

pub struct GlobalData {
    quiz_id: String,
    cookie: String,
    csrf_token: String,
    cookies: HashMap<String, String>,
    student_assessment_sys_guid: String,
}

impl GlobalData {
    pub fn from_env() -> GlobalData {
        let quiz_id = env_get("QUIZ_ID").expect("unable to find [QUIZ_ID] in .env file");
        let cookie = env_get("COOKIE").expect("unable to find [COOKIE] in .env file");
        let csrf_token = env_get("CSRF_TOKEN").expect("unable to find [CSRF_TOKEN] in .env file");
        GlobalData {
            quiz_id: quiz_id.clone(),
            cookie: cookie.clone(),
            csrf_token: csrf_token.clone(),
            cookies: cookies(cookie),
            student_assessment_sys_guid: "".to_string(),
        }
    }

    pub fn quiz_id(&self) -> &String {
        &self.quiz_id
    }

    pub fn cookie(&self) -> &String {
        &self.cookie
    }

    pub fn csrf_token(&self) -> &String {
        &self.csrf_token
    }

    pub fn cookie_value(&self, key: &str) -> &String {
        self.cookies
            .get(key)
            .expect("can not find [${key}] in coolies")
    }

    pub fn student_guid(&self) -> &String {
        &self.student_assessment_sys_guid
    }

    pub fn set_student_guid(&mut self, value: String) {
        self.student_assessment_sys_guid = value
    }
}
