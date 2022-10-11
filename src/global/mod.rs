use crate::response::{AnswerChoice, QuestionFormat};
use mongodb::bson::doc;
use mongodb::Collection;
use rust_demo::{cookies, env_get};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

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
    // 已知的正确选项
    correct_choices: Vec<AnswerChoice>,
    // 已知的错误选线
    error_choices: Vec<AnswerChoice>,
    // 本次选择的选项
    chosen_choices: Vec<AnswerChoice>,
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
            correct_choices: Vec::new(),
            error_choices: Vec::new(),
            chosen_choices: Vec::new(),
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
        let question = self
            .questions
            .iter_mut()
            .find(|q| q.temp_id == temp_id)
            .expect("fail to find");
        question.choices = choices;
        question.choices.iter_mut().for_each(|q| {
            q.is_selected = true;
        });
    }

    pub async fn update_from_db(
        &mut self,
        coll: &Collection<MemoryStoreQuestion>,
        temp_id: String,
    ) -> Result<(), Box<dyn Error>> {
        let mut question = self
            .questions
            .iter_mut()
            .find(|q| q.temp_id == temp_id)
            .expect("fail to find");
        let db = coll
            .find_one(doc! {"useful_id":question.useful_id.clone()}, None)
            .await?;
        match db {
            Some(q) => {
                question.correct_choices = q.correct_choices;
                question.error_choices = q.error_choices;
            }
            None => {}
        }
        Ok(())
    }

    pub fn filter_choices(&mut self, temp_id: String) {
        let question = self
            .questions
            .iter_mut()
            .find(|q| q.temp_id == temp_id)
            .expect("fail to find");
        if !question.error_choices.is_empty() {
            let mut error_ids: Vec<String> = Vec::new();
            question.error_choices.iter().for_each(|q| {
                error_ids.push(q.choice_value.clone());
            });
            question.choices = question
                .choices
                .clone()
                .into_iter()
                .filter(|q| error_ids.contains(&q.choice_value))
                .collect::<Vec<AnswerChoice>>();
        }
    }

    pub fn select_choices(&mut self, temp_id: String) {
        let question = self
            .questions
            .iter_mut()
            .find(|q| q.temp_id == temp_id)
            .expect("fail to find");
        if !question.correct_choices.is_empty() {
            question.chosen_choices = question.correct_choices.clone();
        } else {
            match question.format {
                QuestionFormat::MultiChoiceSingleAnswer => {
                    question.chosen_choices.clear();
                    question.chosen_choices.push(
                        question
                            .choices
                            .first()
                            .expect("have no choices can chose")
                            .clone(),
                    );
                }
                QuestionFormat::MultiChoiceMultipleAnswer => {
                    todo!("处理多选题的逻辑")
                }
            }
        }
    }

    pub fn chosen_choices(&self, temp_id: String) -> Vec<AnswerChoice> {
        self.questions
            .iter()
            .find(|q| q.temp_id == temp_id)
            .expect("fail to find")
            .chosen_choices
            .clone()
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

    pub fn remember_error(&mut self) {
        self.questions.iter_mut().for_each(|q| {
            if q.correct_choices.is_empty() {
                match q.format {
                    QuestionFormat::MultiChoiceSingleAnswer => {
                        q.chosen_choices
                            .iter()
                            .for_each(|cq| q.error_choices.push(cq.clone()));
                    }
                    QuestionFormat::MultiChoiceMultipleAnswer => {
                        todo!("处理多选题的逻辑")
                    }
                }
            }
        });
    }

    pub async fn store_into_db(
        &self,
        coll: &Collection<MemoryStoreQuestion>,
    ) -> Result<(), Box<dyn Error>> {
        for i in 0..self.question_count() {
            let current = self.get_question(i);
            let db = coll
                .find_one(doc! {"useful_id":current.useful_id.clone()}, None)
                .await?;
            match db {
                Some(_) => {
                    coll.delete_one(doc! {"useful_id":current.useful_id.clone()}, None)
                        .await?;
                }
                None => {}
            }
            coll.insert_one(current.clone(), None).await?;
        }
        Ok(())
    }
}