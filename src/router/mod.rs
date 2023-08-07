use rocket::{
    http::Status,
    serde::json::{json, Json, Value},
    State,
};

use super::database::DatabaseRepository;
use super::models::Tasks;

#[get("/")]
pub fn get_tasks(db: &State<DatabaseRepository>) -> Result<Value, Status> {
    match db.get_tasks() {
        Ok(res) => Ok(json!(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/", data = "<new_task>")]
pub fn add_task(db: &State<DatabaseRepository>, new_task: Json<Tasks>) -> Result<Value, Status> {
    let to_insert = Tasks {
        id: None,
        title: new_task.title.to_owned(),
        description: new_task.description.to_owned(),
        is_done: new_task.is_done,
    };

    match db.insert_task(&to_insert) {
        Ok(res) => {
            Ok(json!(res))
        }
        Err(_) => Err(Status::InternalServerError)
    }
}


#[delete("/<id>")]
pub fn delete_task(db: &State<DatabaseRepository>, id: String) -> Result<Value, Status> {
    match db.remove_task(&id) {
        Ok(res) => {
            Ok(json!(res))
        }
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/<id>/done")]
pub fn mark_as_done(db: &State<DatabaseRepository>, id: String) -> Result<Value, Status> {
    match db.mark_as_done(&id) {
        Ok(res) => {
            Ok(json!(res))
        }
        Err(_) => Err(Status::InternalServerError)
    }
}