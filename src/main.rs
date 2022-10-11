mod global;
mod request;
mod response;
mod url;

use crate::global::{GlobalData, MemoryStoreQuestion};
use crate::request::{iteration_request, save_request, start_request, submit_request, SaveBody};
use crate::response::{CustomResponse, IterationData, StartData};
use crate::url::{iteration_url, start_url};
use mongodb::{options::ClientOptions, Client};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut global_data = GlobalData::from_env();

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("wbl robot".to_string());
    let db_client = Client::with_options(client_options)?;
    let db = db_client.database("wbl_robot");

    let questions_collection = db.collection::<MemoryStoreQuestion>("questions");

    let client: reqwest::Client = reqwest::Client::new();

    loop {
        /*iteration*/
        let iteration_result = iteration_request(&client, &global_data).await?;
        let student_assessment_iteration = iteration_result
            .clone()
            .student_assessment_iteration
            .expect("unable to get [student_assessment_iteration]");
        global_data.set_student_guid(student_assessment_iteration.student_assessment_sys_guid);
        student_assessment_iteration.questions.iter().for_each(|q| {
            global_data.insert_question(q.student_assessment_question_sys_guid.clone());
        });
        println!("get {} question", global_data.question_count());

        /*start*/
        println!("request question choices");
        let start_result =
            start_request(&client, global_data.first_question(), &global_data).await?;
        global_data.set_useful_id(global_data.first_question(), start_result.question_id);
        global_data.set_format(global_data.first_question(), start_result.format);
        global_data.set_choices(global_data.first_question(), start_result.answer_choices);
        global_data
            .update_from_db(&questions_collection, global_data.first_question())
            .await?;

        /*save and next*/
        for i in 0..global_data.question_count() - 1 {
            let current = global_data.get_question_id(i);
            let next = global_data.get_question_id(i + 1);
            global_data.select_choices(current.clone());
            let body = SaveBody::from(global_data.chosen_choices(current.clone()), next.clone());
            println!("send a question answer");
            let save_result = save_request(&client, current.clone(), &global_data, &body)
                .await
                .expect("fail to get save request response");
            global_data.set_useful_id(next.clone(), save_result.question_id);
            global_data.set_format(next.clone(), save_result.format);
            global_data.set_choices(next.clone(), save_result.answer_choices);
            global_data
                .update_from_db(&questions_collection, next.clone())
                .await?;
        }

        /*submit*/
        let last_question_id = global_data.last_question();
        global_data.select_choices(last_question_id.clone());
        let body = SaveBody::from(
            global_data.chosen_choices(last_question_id.clone()),
            last_question_id.clone(),
        );
        println!("send a question answer");
        let submit_result =
            submit_request(&client, last_question_id.clone(), &global_data, &body).await?;
        println!(
            "get score({}), correct({}), already know correct({})",
            submit_result.score,
            submit_result.questions_correct,
            global_data.get_min_correct()
        );

        if global_data.get_min_correct() == submit_result.questions_correct {
            println!("try to remember error choices");
            global_data.remember_error();
        } else if submit_result.score == 100 {
            println!("try to remember correct choices");
            global_data.remember_correct();
        }

        global_data.store_into_db(&questions_collection).await?;

        global_data.clear_question();
    }
}
