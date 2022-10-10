use crate::response::{AnswerChoice, SubmitData};
use crate::url::{save_url, submit_url};
use crate::{iteration_url, start_url, CustomResponse, GlobalData, IterationData, StartData};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;

async fn base_get(
    client: &Client,
    url: String,
    cookie: &String,
    token: &String,
    file_name: &String,
) -> Result<String, Box<dyn Error>> {
    let rb: RequestBuilder = client.get(url);
    let res = rb
        .header("COOKIE", cookie)
        .header("owasp_csrftoken", token)
        .send()
        .await?;
    if res.status().is_success() {
        let body = res.text().await?;
        let file_path = "temp/".to_string() + file_name + ".json";
        let msg = "unable to write file ".to_string() + &*file_path;
        let mut file = File::create(file_path).expect("unable to create file temp/iteration.json");
        file.write(body.as_bytes()).expect(&*msg);
        Ok(body)
    } else {
        panic!("request return error")
    }
}

async fn base_put<T: Serialize>(
    client: &Client,
    url: String,
    cookie: &String,
    token: &String,
    json: &T,
    file_name: &String,
) -> Result<String, Box<dyn Error>> {
    let rb: RequestBuilder = client.put(url);
    let res = rb
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("COOKIE", cookie)
        .header("owasp_csrftoken", token)
        .header(
            "x-requested-with",
            "XMLHttpRequest, OWASP CSRFGuard Project",
        )
        .json(json.clone())
        .send()
        .await?;
    if res.status().is_success() {
        let body = res.text().await?;
        let file_path = "temp/".to_string() + file_name + ".json";
        let msg = "unable to write file ".to_string() + &*file_path;
        let mut file = File::create(file_path).expect("unable to create file temp/iteration.json");
        file.write(body.as_bytes()).expect(&*msg);
        Ok(body)
    } else {
        panic!(
            "request return error {}",
            serde_json::to_string(json.clone()).expect("fail to string json")
        )
    }
}

pub(crate) async fn iteration_request(
    client: &Client,
    global_data: &GlobalData,
) -> Result<IterationData, Box<dyn Error>> {
    let body = base_get(
        client,
        iteration_url(&global_data),
        global_data.cookie(),
        global_data.csrf_token(),
        &"iteration".to_string(),
    )
    .await?;
    let response: CustomResponse<IterationData> = serde_json::from_str(&*body)?;
    let rest_operation_status_vox = response.rest_operation_status_vox;
    let status = rest_operation_status_vox.status;
    if status == "SUCCESS" {
        let data = rest_operation_status_vox.data;
        let rest_return_data = data.rest_return_data;
        Ok(rest_return_data.clone())
    } else {
        panic!("")
    }
}

pub(crate) async fn start_request(
    client: &Client,
    first_question_guid: String,
    global_data: &GlobalData,
) -> Result<StartData, Box<dyn Error>> {
    let label = "start".to_string() + &*first_question_guid;
    let body = base_get(
        client,
        start_url(&global_data, first_question_guid),
        global_data.cookie(),
        global_data.csrf_token(),
        &label,
    )
    .await?;
    let response: CustomResponse<StartData> = serde_json::from_str(&*body)?;
    let rest_operation_status_vox = response.rest_operation_status_vox;
    let status = rest_operation_status_vox.status;
    if status == "SUCCESS" {
        let data = rest_operation_status_vox.data;
        let rest_return_data = data.rest_return_data;
        Ok(rest_return_data)
    } else {
        panic!("")
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SaveBody {
    score: bool,
    #[serde(rename = "studentAnswers")]
    student_answers: Vec<AnswerChoice>,
    #[serde(rename = "nextStudentAssessmentQuestionSysGUID")]
    next: String,
}

impl SaveBody {
    pub(crate) fn from(answers: Vec<AnswerChoice>, next: String) -> SaveBody {
        SaveBody {
            score: true,
            student_answers: answers,
            next,
        }
    }
}

pub(crate) async fn save_request(
    client: &Client,
    question_guid: String,
    global_data: &GlobalData,
    save_body: &SaveBody,
) -> Result<StartData, Box<dyn Error>> {
    let label = "save".to_string() + &*question_guid;
    let body = base_put(
        client,
        save_url(&global_data, question_guid.clone()),
        global_data.cookie(),
        global_data.csrf_token(),
        save_body,
        &label,
    )
    .await?;
    let response: CustomResponse<StartData> =
        serde_json::from_str(&*body).expect("fail to serialize SaveAndNext response data");
    let rest_operation_status_vox = response.rest_operation_status_vox;
    let status = rest_operation_status_vox.status;
    if status == "SUCCESS" {
        let data = rest_operation_status_vox.data;
        let rest_return_data = data.rest_return_data;
        Ok(rest_return_data)
    } else {
        panic!("")
    }
}

pub(crate) async fn submit_request(
    client: &Client,
    first_question_guid: String,
    global_data: &GlobalData,
    save_body: &SaveBody,
) -> Result<SubmitData, Box<dyn Error>> {
    let body = base_put(
        client,
        submit_url(&global_data, first_question_guid),
        global_data.cookie(),
        global_data.csrf_token(),
        save_body,
        &"submit".to_string(),
    )
    .await?;
    let response: CustomResponse<SubmitData> =
        serde_json::from_str(&*body).expect("fail to serialize SaveAndNext response data");
    let rest_operation_status_vox = response.rest_operation_status_vox;
    let status = rest_operation_status_vox.status;
    if status == "SUCCESS" {
        let data = rest_operation_status_vox.data;
        let rest_return_data = data.rest_return_data;
        Ok(rest_return_data)
    } else {
        panic!("")
    }
}
