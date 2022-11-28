use std::collections::HashMap;
use std::error::Error;

use mongodb::bson::doc;
use mongodb::Collection;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

use wbl_robot::{cookies, env_get};

use crate::response::{AnswerChoice, QuestionFormat};

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
    // 已知的错误选项
    error_choices: Vec<AnswerChoice>,
    // 已知的多选题的错误选项
    multiple_error_choices: Vec<Vec<AnswerChoice>>,
    // 本次选择的选项
    chosen_choices: Vec<AnswerChoice>,
}

impl MemoryStoreQuestion {
    /// 排除error_choices后剩余的选项
    #[allow(dead_code)]
    fn excluded_errors(&self) -> Vec<AnswerChoice> {
        let errors = self
            .error_choices
            .iter()
            .map(|c| c.choice_value.clone())
            .collect::<Vec<String>>();
        let exclude_errors = self
            .choices
            .clone()
            .into_iter()
            .filter(|q| !errors.contains(&q.choice_value))
            .collect::<Vec<AnswerChoice>>();
        exclude_errors
    }
    /// 随机从所有choices里返回一个（已经排除error_choices）
    #[allow(dead_code)]
    fn random_choice(&self) -> AnswerChoice {
        self.excluded_errors()
            .choose(&mut rand::thread_rng())
            .expect("process error, have not choices left")
            .clone()
    }
    /// 随机从所有choices里返回若干个
    #[allow(dead_code)]
    fn random_choices(&self) -> Vec<AnswerChoice> {
        let len = self.choices.len();
        let mut result = self
            .choices
            .clone()
            .choose_multiple(&mut rand::thread_rng(), get_random(len))
            .cloned()
            .collect::<Vec<AnswerChoice>>();
        let mut same = false;
        self.multiple_error_choices.iter().for_each(|m| {
            if !same {
                if compare_multiple_choices(m, &result) {
                    same = true;
                }
            }
        });

        while same {
            same = false;
            result = self
                .choices
                .clone()
                .choose_multiple(&mut rand::thread_rng(), get_random(len))
                .cloned()
                .collect::<Vec<AnswerChoice>>();
            self.multiple_error_choices.iter().for_each(|m| {
                if !same {
                    if compare_multiple_choices(m, &result) {
                        same = true;
                    }
                }
            });
        }

        result
    }
}

/// 比较两个多选题选择的选项是否相同
#[allow(dead_code)]
fn compare_multiple_choices(a: &Vec<AnswerChoice>, b: &Vec<AnswerChoice>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let b_values = b
        .iter()
        .map(|c| c.choice_value.clone())
        .collect::<Vec<String>>();
    let mut same = true;
    a.iter().for_each(|c| {
        if !b_values.contains(&c.choice_value) {
            same = false;
        }
    });
    same
}

/// 得到1-max之间随机的一个整数，包含1和max
#[allow(dead_code)]
fn get_random(max: usize) -> usize {
    let mut rng = rand::thread_rng();
    let index = rng.gen::<usize>();
    let mut result = index % max;
    if result == 0 {
        result = max;
    }
    result
}

/// 得到有v个选项时，选项组合的数量
#[allow(dead_code)]
fn get_combination_count(v: usize) -> usize {
    let result = 0;
    for _ in 1..=v {
        todo!("计算Cv_的结果加总")
    }
    result
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
            multiple_error_choices: Vec::new(),
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

    /// 得到本次答题理论最少能答对的题数，及已经知道答案的题目数量
    /// 当本次答题答对题数与已知答案的题数相同，则可知其他题目均答错
    pub fn get_min_correct(&self) -> usize {
        let mut result: usize = 0;
        self.questions
            .iter()
            .filter(|q| !q.correct_choices.is_empty())
            .for_each(|_| result += 1);
        result
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
                question.multiple_error_choices = q.multiple_error_choices;
            }
            None => {}
        }
        Ok(())
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
                    question.chosen_choices.push(question.random_choice());
                }
                QuestionFormat::MultiChoiceMultipleAnswer => {
                    question.chosen_choices = question.random_choices();
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
                        if q.excluded_errors().len() == 1 {
                            q.correct_choices = q.excluded_errors();
                            // todo 统计得到全部可能的选项数量，当error的记录已经跟全部可能只差一时，则可以得到剩下就是正确选项
                        }
                    }
                    QuestionFormat::MultiChoiceMultipleAnswer => {
                        q.multiple_error_choices.push(q.chosen_choices.clone());
                    }
                }
            }
        });
    }

    pub fn remember_correct(&mut self) {
        self.questions.iter_mut().for_each(|q| {
            q.correct_choices = q.chosen_choices.clone();
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

#[cfg(test)]
mod tests {
    use crate::global::{compare_multiple_choices, get_combination_count, get_random};
    use crate::response::{AnswerChoice, QuestionFormat};
    use crate::MemoryStoreQuestion;

    #[test]
    fn random_choice_test() {
        let mut question = MemoryStoreQuestion {
            useful_id: String::from(""),
            temp_id: String::from(""),
            format: QuestionFormat::MultiChoiceSingleAnswer,
            choices: vec![
                AnswerChoice::from_mock("1"),
                AnswerChoice::from_mock("2"),
                AnswerChoice::from_mock("3"),
                AnswerChoice::from_mock("4"),
            ],
            correct_choices: Vec::new(),
            error_choices: Vec::new(),
            multiple_error_choices: Vec::new(),
            chosen_choices: Vec::new(),
        };
        let random_choice = question.random_choice().choice_value;
        assert_eq!(
            random_choice == String::from("1")
                || random_choice == String::from("2")
                || random_choice == String::from("3")
                || random_choice == String::from("4"),
            true
        );
        question.error_choices.push(AnswerChoice::from_mock("1"));
        let random_choice = question.random_choice().choice_value;
        assert_eq!(
            random_choice == String::from("2")
                || random_choice == String::from("3")
                || random_choice == String::from("4"),
            true
        );
        question.error_choices.push(AnswerChoice::from_mock("2"));
        let random_choice = question.random_choice().choice_value;
        assert_eq!(
            random_choice == String::from("3") || random_choice == String::from("4"),
            true
        );
        question.error_choices.push(AnswerChoice::from_mock("3"));
        let random_choice = question.random_choice().choice_value;
        assert_eq!(random_choice == String::from("4"), true);
    }

    #[test]
    fn get_random_test() {
        for _ in 0..50 {
            let random = get_random(5);
            assert_eq!(
                random == 1 || random == 2 || random == 3 || random == 4 || random == 5,
                true
            );
        }
    }

    #[test]
    fn compare_multiple_choices_test() {
        let mut a = vec![AnswerChoice::from_mock("1")];
        let mut b = vec![AnswerChoice::from_mock("1")];
        assert_eq!(compare_multiple_choices(&a, &b), true);
        a.clear();
        assert_eq!(compare_multiple_choices(&a, &b), false);
        b.clear();
        assert_eq!(compare_multiple_choices(&a, &b), true);
        a.push(AnswerChoice::from_mock("1"));
        a.push(AnswerChoice::from_mock("2"));
        b.push(AnswerChoice::from_mock("1"));
        assert_eq!(compare_multiple_choices(&a, &b), false);
        b.push(AnswerChoice::from_mock("2"));
        assert_eq!(compare_multiple_choices(&a, &b), true);
    }

    #[test]
    fn random_choices_test() {
        let mut question = MemoryStoreQuestion {
            useful_id: String::from(""),
            temp_id: String::from(""),
            format: QuestionFormat::MultiChoiceSingleAnswer,
            choices: vec![
                AnswerChoice::from_mock("1"),
                AnswerChoice::from_mock("2"),
                AnswerChoice::from_mock("3"),
                AnswerChoice::from_mock("4"),
            ],
            correct_choices: Vec::new(),
            error_choices: Vec::new(),
            multiple_error_choices: Vec::new(),
            chosen_choices: Vec::new(),
        };
        for _ in 0..50 {
            let random_choices = question.random_choices();
            assert_eq!(random_choices.len() > 0 && random_choices.len() < 5, true);
        }
        question
            .multiple_error_choices
            .push(vec![AnswerChoice::from_mock("1")]);
        question
            .multiple_error_choices
            .push(vec![AnswerChoice::from_mock("2")]);
        question
            .multiple_error_choices
            .push(vec![AnswerChoice::from_mock("3")]);
        question
            .multiple_error_choices
            .push(vec![AnswerChoice::from_mock("4")]);
        for _ in 0..50 {
            let random_choices = question.random_choices();
            assert_eq!(random_choices.len() > 1 && random_choices.len() < 5, true);
        }
        question.multiple_error_choices.push(vec![
            AnswerChoice::from_mock("1"),
            AnswerChoice::from_mock("2"),
        ]);
        question.multiple_error_choices.push(vec![
            AnswerChoice::from_mock("1"),
            AnswerChoice::from_mock("3"),
        ]);
        question.multiple_error_choices.push(vec![
            AnswerChoice::from_mock("1"),
            AnswerChoice::from_mock("4"),
        ]);
        question.multiple_error_choices.push(vec![
            AnswerChoice::from_mock("2"),
            AnswerChoice::from_mock("3"),
        ]);
        question.multiple_error_choices.push(vec![
            AnswerChoice::from_mock("2"),
            AnswerChoice::from_mock("4"),
        ]);
        question.multiple_error_choices.push(vec![
            AnswerChoice::from_mock("3"),
            AnswerChoice::from_mock("4"),
        ]);
        for _ in 0..50 {
            let random_choices = question.random_choices();
            assert_eq!(random_choices.len() > 2 && random_choices.len() < 5, true);
        }
        question.multiple_error_choices.push(vec![
            AnswerChoice::from_mock("1"),
            AnswerChoice::from_mock("2"),
            AnswerChoice::from_mock("3"),
        ]);
        question.multiple_error_choices.push(vec![
            AnswerChoice::from_mock("1"),
            AnswerChoice::from_mock("2"),
            AnswerChoice::from_mock("4"),
        ]);
        question.multiple_error_choices.push(vec![
            AnswerChoice::from_mock("1"),
            AnswerChoice::from_mock("3"),
            AnswerChoice::from_mock("4"),
        ]);
        question.multiple_error_choices.push(vec![
            AnswerChoice::from_mock("2"),
            AnswerChoice::from_mock("3"),
            AnswerChoice::from_mock("4"),
        ]);
        for _ in 0..50 {
            let random_choices = question.random_choices();
            assert_eq!(random_choices.len() == 4, true);
        }
    }

    #[test]
    fn get_combination_count_test() {
        // assert_eq!(get_combination_count(1), 1);
        // assert_eq!(get_combination_count(2), 3);
        // assert_eq!(get_combination_count(3), 6);
        // assert_eq!(get_combination_count(4), 16);
    }
}
