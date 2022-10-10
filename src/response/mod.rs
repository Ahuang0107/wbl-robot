use serde::{Deserialize, Serialize};
use std::error::Error;

#[allow(dead_code)]
async fn get_res_data<'a, T: Deserialize<'a>>(body: &'a str) -> Result<T, Box<dyn Error>> {
    let response: CustomResponse<T> = serde_json::from_str(body)?;
    let rest_operation_status_vox = response.rest_operation_status_vox;
    let status = rest_operation_status_vox.status;
    if status == "SUCCESS" {
        let data = rest_operation_status_vox.data;
        let rest_return_data = data.rest_return_data;
        return Ok(rest_return_data);
    } else {
        panic!("status not success")
    }
}

#[derive(Serialize, Deserialize)]
pub struct CustomResponse<T> {
    #[serde(rename = "restOperationStatusVOX")]
    pub(crate) rest_operation_status_vox: RestOperationStatusVOX<T>,
}

#[derive(Serialize, Deserialize)]
pub struct RestOperationStatusVOX<T> {
    pub(crate) operation: Option<String>,
    pub(crate) status: String,
    pub(crate) data: Data<T>,
    pub(crate) errors: Option<String>,
    pub(crate) warnings: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Data<T> {
    #[serde(rename = "REST_RETURN_DATA")]
    pub(crate) rest_return_data: T,
}

/// Iteration 接口返回的数据结构
#[derive(Serialize, Deserialize, Clone)]
pub struct IterationData {
    // has_assessment_iteration: bool,
    // selected_locale_id: String,
    #[serde(rename = "studentAssessmentIteration")]
    pub(crate) student_assessment_iteration: Option<StudentAssessmentIteration>,
    // exam_duration: Option<String>,
    // formatted_exam_duration: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StudentAssessmentIteration {
    // passing_grade: u8,
    #[serde(rename = "questionCount")]
    pub(crate) question_count: usize,
    pub(crate) questions: Vec<Question>,
    #[serde(rename = "studentAssessmentSysGUID")]
    pub(crate) student_assessment_sys_guid: String,
    #[serde(rename = "quizID")]
    pub(crate) quiz_id: String,
    #[serde(rename = "quizSysGUID")]
    pub(crate) quiz_sys_guid: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Question {
    #[serde(rename = "studentAssessmentQuestionSysGUID")]
    pub(crate) student_assessment_question_sys_guid: String,
}

/// Start 接口返回的数据结构
#[derive(Serialize, Deserialize)]
pub struct StartData {
    #[serde(rename = "questionID")]
    pub(crate) question_id: String,
    // #[serde(rename = "questionSysGUID")]
    // pub(crate) question_sys_guid: String,
    #[serde(rename = "answerChoices")]
    pub(crate) answer_choices: Vec<AnswerChoice>,
    pub(crate) format: QuestionFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum QuestionFormat {
    #[serde(rename = "MULTI_CHOICE_SINGLE_ANSWER")]
    MultiChoiceSingleAnswer,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnswerChoice {
    #[serde(rename = "answerSysGUID")]
    pub(crate) answer_sys_guid: String,
    #[serde(rename = "choiceValue")]
    pub(crate) choice_value: String,
}

/// submit 接口返回的数据结构
#[derive(Serialize, Deserialize)]
pub struct SubmitData {
    pub(crate) score: usize,
}
