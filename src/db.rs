use crate::models::{BorrowedDB, Task};

use std::error::Error;

pub fn create_task(
    title: String,
    description: String,
    mut task_db: BorrowedDB,
) -> Result<Task, Box<dyn Error>> {
    if title.len() == 0 {
        return Err("Valid title is required".into());
    } else {
        let mut new_task: Task = Task {
            id: 0,
            title,
            description,
        };

        if task_db.len() > 0 {
            let last_index = task_db.len() - 1;

            let previous_task = task_db.get(last_index).unwrap();

            let new_id = previous_task.id + 1;

            new_task.id = new_id;
        }

        task_db.push(new_task.clone());

        Ok(new_task)
    }
}

pub fn delete_task(id: usize, mut task_db: BorrowedDB) -> Result<bool, Box<dyn Error>> {
    let mut task_index_to_be_deleted: Option<usize> = None;

    for (idx, task) in task_db.iter().enumerate() {
        if task.id == id {
            task_index_to_be_deleted = Some(idx);
            break;
        }
    }

    match task_index_to_be_deleted {
        Some(idx) => {
            task_db.remove(idx);

            Ok(true)
        }
        None => Err("The task could not be found".into()),
    }
}

pub fn update_task(
    id: usize,
    mut task_db: BorrowedDB,
    title: Option<String>,
    description: Option<String>,
) -> Result<Task, Box<dyn Error>> {
    let mut updated_task: Option<Task> = None;

    for task in task_db.iter_mut() {
        if task.id == id {
            match description {
                Some(ref udpated_desc) => task.description = udpated_desc.clone(),
                None => {}
            }

            match title {
                Some(ref updated_title) => task.title = updated_title.clone(),
                None => {}
            }

            updated_task = Some(Task {
                id: task.id,
                title: task.title.to_string(),
                description: task.description.to_string(),
            });
        }
    }

    match updated_task {
        Some(task) => {
            println!("Task updated successfully");
            return Ok(task);
        }
        None => return Err("The task with the id does not exist".into()),
    }
}

pub fn get_task(id: usize, task_db: BorrowedDB) -> Result<Task, Box<dyn Error>> {
    let task = task_db.iter().find(|task| task.id == id);

    match task {
        Some(retrieved_task) => {
            return Ok(retrieved_task.clone());
        }
        None => {
            return Err("The task with the provided id could not be found".into());
        }
    }
}

pub fn get_all_tasks(task_db: BorrowedDB) -> Vec<Task> {
    let result = task_db.clone();

    result
}
