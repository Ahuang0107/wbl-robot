use rust_demo::{cookies, env_get};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let quiz_id = env_get("QUIZ_ID").expect("unable to find [QUIZ_ID] in .env file");
    let cookie = env_get("COOKIE").expect("unable to find [COOKIE] in .env file");
    let csrf_token = env_get("CSRF_TOKEN").expect("unable to find [CSRF_TOKEN] in .env file");

    let cookies = cookies(cookie);
    let student_component_id = cookies
        .get("PSA_STUD_CPNT_ID")
        .expect("can not find [PSA_STUD_CPNT_ID] in coolies");
    let module_id = cookies
        .get("PSA_STUD_CPNT_MOD_ID")
        .expect("can not find [PSA_STUD_CPNT_MOD_ID] in coolies");
    let example = "https://eygsl.plateau.com/learning/user/api/v1/current-user/quiz/940fa94e-da49-4efa-9b9f-751fa346f357/iteration?studentComponentID=32729100&moduleID=669142&generateIteration=true";
    let base_url = "https://eygsl.plateau.com/learning/user/api/v1/current-user/quiz";
    let url = base_url.to_string()
        + "/"
        + quiz_id
        + "/"
        + "iteration"
        + "?"
        + "studentComponentID"
        + "="
        + student_component_id
        + "&"
        + "moduleID"
        + "="
        + module_id
        + "&"
        + "generateIteration"
        + "="
        + "true";
    assert_eq!(example, url);

    let client = reqwest::Client::new();
    let res = client
        .get(example)
        .header("COOKIE", cookie)
        .header("owasp_csrftoken", csrf_token)
        .send()
        .await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
