mod global;
mod response;
mod url;

use crate::global::GlobalData;
use crate::response::{CustomResponse, IterationData, StartData};
use crate::url::{iteration_url, start_url};
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut global_data = GlobalData::from_env();

    let client = reqwest::Client::new();
    let res = client
        .get(iteration_url(&global_data))
        .header("COOKIE", global_data.cookie())
        .header("owasp_csrftoken", global_data.csrf_token())
        .send()
        .await?;

    if res.status().is_success() {
        let body = res.text().await?;
        let mut file =
            File::create("temp/iteration.json").expect("unable to create file temp/iteration.json");
        file.write(body.as_bytes())
            .expect("unable to write file temp/iteration.json");
        let response: CustomResponse<IterationData> = serde_json::from_str(&*body)?;
        let rest_operation_status_vox = response.rest_operation_status_vox;
        let status = rest_operation_status_vox.status;
        if status == "SUCCESS" {
            let data = rest_operation_status_vox.data;
            let rest_return_data = data.rest_return_data;
            let student_assessment_iteration = rest_return_data
                .student_assessment_iteration
                .expect("unable to get [student_assessment_iteration]");
            let question_count = student_assessment_iteration.question_count;
            println!("question_count:{question_count}");
            let questions = student_assessment_iteration.questions;
            questions.iter().for_each(|question| {
                println!(
                    "question_sys_guid:{}",
                    question.student_assessment_question_sys_guid
                );
            });
            global_data.set_student_guid(student_assessment_iteration.student_assessment_sys_guid);

            let first = questions
                .first()
                .unwrap()
                .student_assessment_question_sys_guid
                .clone();
            let res = client
                .get(start_url(&global_data, first))
                .header("COOKIE", global_data.cookie())
                .header("owasp_csrftoken", global_data.csrf_token())
                .send()
                .await?;

            if res.status().is_success() {
                let body = res.text().await?;
                let mut file = File::create("temp/start.json")
                    .expect("unable to create file temp/iteration.json");
                file.write(body.as_bytes())
                    .expect("unable to write file temp/iteration.json");
                let response: CustomResponse<StartData> = serde_json::from_str(&*body)?;
                let rest_operation_status_vox = response.rest_operation_status_vox;
                let status = rest_operation_status_vox.status;
                if status == "SUCCESS" {
                    let data = rest_operation_status_vox.data;
                    let rest_return_data = data.rest_return_data;
                    println!("{}", rest_return_data.question_id);
                }
            }
        }
    }

    Ok(())
}
