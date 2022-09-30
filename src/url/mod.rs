use crate::GlobalData;
use std::collections::HashMap;

pub fn iteration_url(global_data: &GlobalData) -> String {
    let base_url = "https://eygsl.plateau.com/learning/user/api/v1/current-user/quiz";
    let mut queries: HashMap<String, String> = HashMap::new();
    queries.insert(
        "studentComponentID".to_string(),
        global_data.cookie_value("PSA_STUD_CPNT_ID").clone(),
    );
    queries.insert(
        "moduleID".to_string(),
        global_data.cookie_value("PSA_STUD_CPNT_MOD_ID").clone(),
    );
    queries.insert("generateIteration".to_string(), "true".to_string());
    base_url.to_string()
        + "/"
        + global_data.quiz_id()
        + "/iteration/English"
        + "?"
        + &*url_queries(queries)
}

pub fn start_url(global_data: &GlobalData, question_guid: String) -> String {
    let base_url = "https://eygsl.plateau.com/learning/user/api/v1/current-user/quiz";
    base_url.to_string()
        + "/"
        + global_data.quiz_id()
        + "/iteration/"
        + global_data.student_guid()
        + "/question/"
        + &*question_guid
}

fn url_queries(queries: HashMap<String, String>) -> String {
    let a = queries
        .iter()
        .map(|(key, value)| key.to_string() + "=" + value);
    let mut extended = Vec::new();
    extended.extend(a);

    extended.join("&")
}
