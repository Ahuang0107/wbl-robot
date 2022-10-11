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
    #[serde(rename = "MULTI_CHOICE_MULTIPLE_ANSWER")]
    MultiChoiceMultipleAnswer,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnswerChoice {
    #[serde(rename = "answerSysGUID")]
    answer_sys_guid: String,
    #[serde(rename = "groupNumber")]
    group_number: Option<usize>,
    #[serde(rename = "isArchived")]
    is_archived: Option<bool>,
    #[serde(rename = "isCorrect")]
    is_correct: Option<bool>,
    #[serde(rename = "isPrimary")]
    is_primary: Option<bool>,
    #[serde(rename = "lastUpdateUser")]
    last_update_user: Option<String>,
    #[serde(rename = "lastUpdateTimestamp")]
    last_update_timestamp: Option<usize>,
    number: Option<usize>,
    #[serde(rename = "questionVariantSysGUID")]
    question_variant_sys_guid: Option<String>,
    #[serde(rename = "defaultImageID")]
    default_image_id: Option<String>,
    #[serde(rename = "answerI18nDetailSysGUID")]
    answer_i18n_detail_sys_guid: Option<String>,
    #[serde(rename = "choiceValue")]
    pub(crate) choice_value: String,
    #[serde(rename = "defaultImageFileName")]
    default_image_file_name: Option<String>,
    #[serde(rename = "defaultImageSize")]
    default_image_size: Option<String>,
    #[serde(rename = "i18nDetailLastUpdateUser")]
    i18n_detail_last_update_user: Option<String>,
    #[serde(rename = "i18nDetailLastUpdateTimestamp")]
    i18n_detail_last_update_timestamp: Option<usize>,
    #[serde(rename = "imageFileName")]
    image_file_name: Option<String>,
    #[serde(rename = "imageSize")]
    image_size: Option<String>,
    #[serde(rename = "localeID")]
    locale_id: Option<String>,
    #[serde(rename = "questionVersionSysGUID")]
    question_version_sys_guid: Option<String>,
    #[serde(rename = "imageID")]
    image_id: Option<String>,
    #[serde(rename = "imageAltText")]
    image_alt_text: Option<String>,
    #[serde(rename = "answerI18nSysGUID")]
    answer_i18n_sys_guid: Option<String>,
    #[serde(rename = "isSelected")]
    pub(crate) is_selected: bool,
    order: Option<String>,
    #[serde(rename = "studentAssessmentAnswerChoiceSysGUID")]
    student_assessment_answer_choice_sys_guid: Option<String>,
    #[serde(rename = "studentAssessmentQuestionSysGUID")]
    student_assessment_question_sys_guid: Option<String>,
    value: Option<String>,
}

impl AnswerChoice {
    #[allow(dead_code)]
    pub(crate) fn from_mock(v: &str) -> AnswerChoice {
        AnswerChoice {
            answer_sys_guid: String::from("default mock"),
            group_number: None,
            is_archived: None,
            is_correct: None,
            is_primary: None,
            last_update_user: None,
            last_update_timestamp: None,
            number: None,
            question_variant_sys_guid: None,
            default_image_id: None,
            answer_i18n_detail_sys_guid: None,
            choice_value: String::from(v),
            default_image_file_name: None,
            default_image_size: None,
            i18n_detail_last_update_user: None,
            i18n_detail_last_update_timestamp: None,
            image_file_name: None,
            image_size: None,
            locale_id: None,
            question_version_sys_guid: None,
            image_id: None,
            image_alt_text: None,
            answer_i18n_sys_guid: None,
            is_selected: false,
            order: None,
            student_assessment_answer_choice_sys_guid: None,
            student_assessment_question_sys_guid: None,
            value: None,
        }
    }
}

/// submit 接口返回的数据结构
#[derive(Serialize, Deserialize)]
pub struct SubmitData {
    pub(crate) score: usize,
}
