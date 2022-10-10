use crate::response::{AnswerChoice, QuestionFormat};
use rust_demo::{cookies, env_get};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct GlobalData {
    quiz_id: String,
    cookie: String,
    csrf_token: String,
    cookies: HashMap<String, String>,
    student_assessment_sys_guid: String,
    questions: Vec<MemoryStoreQuestion>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MemoryStoreQuestion {
    pub useful_id: String,
    pub temp_id: String,
    format: QuestionFormat,
    choices: Vec<AnswerChoice>,
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
            questions: Vec::new(),
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

    pub fn insert_question(&mut self, temp_id: String) {
        self.questions.push(MemoryStoreQuestion {
            useful_id: "".to_string(),
            temp_id,
            format: QuestionFormat::MultiChoiceSingleAnswer,
            choices: Vec::new(),
        })
    }

    pub fn set_useful_id(&mut self, temp_id: String, useful_id: String) {
        self.questions
            .iter_mut()
            .find(|q| q.temp_id == temp_id)
            .expect("fail to find")
            .useful_id = useful_id;
    }

    pub fn set_format(&mut self, temp_id: String, format: QuestionFormat) {
        self.questions
            .iter_mut()
            .find(|q| q.temp_id == temp_id)
            .expect("fail to find")
            .format = format;
    }

    pub fn set_choices(&mut self, temp_id: String, choices: Vec<AnswerChoice>) {
        self.questions
            .iter_mut()
            .find(|q| q.temp_id == temp_id)
            .expect("fail to find")
            .choices = choices;
    }

    pub fn choice(&self, temp_id: String) -> Vec<AnswerChoice> {
        vec![self
            .questions
            .iter()
            .find(|q| q.temp_id == temp_id)
            .expect("fail to find")
            .choices
            .first()
            .expect("")
            .clone()]
    }

    pub fn question_count(&self) -> usize {
        self.questions.len()
    }

    pub fn clear_question(&mut self) {
        self.questions.clear()
    }

    pub fn first_question(&self) -> String {
        self.questions.first().expect("fail").temp_id.clone()
    }

    pub fn get_question_id(&self, index: usize) -> String {
        self.get_question(index).temp_id.clone()
    }

    pub fn get_question(&self, index: usize) -> &MemoryStoreQuestion {
        &self.questions.get(index).expect("fail to get")
    }

    pub fn last_question(&self) -> String {
        self.questions.last().expect("fail").temp_id.clone()
    }
}
