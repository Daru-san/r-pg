use std::mem::take;
use std::path::{Path, PathBuf};

use image::DynamicImage;

use super::error::TaskError;

#[derive(Debug, Default, PartialEq, Clone)]
pub enum TaskState {
    #[default]
    Pending,
    Decoded,
    Processed,
    Working,
    Failed(TaskError),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ImageTask {
    pub id: u32,
    pub image: Option<DynamicImage>,
    pub image_path: PathBuf,
    pub out_path: PathBuf,
    pub state: TaskState,
}

impl ImageTask {
    pub fn new(id: u32, path: &Path) -> Self {
        ImageTask {
            id,
            image: None,
            image_path: path.to_path_buf(),
            out_path: path.to_path_buf(),
            state: TaskState::Pending,
        }
    }
}

pub struct TaskQueue {
    tasks: Vec<Option<ImageTask>>,
    next_id: u32,
}

impl TaskQueue {
    pub fn new() -> TaskQueue {
        TaskQueue {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn new_task(&mut self, path: &Path) -> u32 {
        let task = ImageTask::new(self.next_id, path);
        self.tasks.push(Some(task));
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn task_by_id_mut(&mut self, task_id: u32) -> Option<&mut ImageTask> {
        self.tasks
            .iter_mut()
            .flatten()
            .find(|task| task.id == task_id)
    }

    pub fn working_task(&mut self, task_id: u32) {
        for task in self.tasks.iter_mut().flatten() {
            if task.id == task_id {
                task.state = TaskState::Working;
                break;
            }
        }
    }

    pub fn completed_task(&mut self, task_id: u32) {
        for task in self.tasks.iter_mut() {
            if let Some(this_task) = task {
                if this_task.id == task_id {
                    *task = None;
                    break;
                }
            }
        }
    }

    pub fn decoded_task(&mut self, decoded_image: &mut Option<DynamicImage>, task_id: u32) {
        for task in self.tasks.iter_mut().flatten() {
            if task.id == task_id {
                task.state = TaskState::Decoded;
                task.image = decoded_image.take();
                break;
            }
        }
    }

    pub fn fail_task(&mut self, task_id: u32, task_error: &str) {
        for task in self.tasks.iter_mut().flatten() {
            if task.id == task_id {
                task.state = TaskState::Failed(TaskError::SingleError(task_error.to_string()));
                break;
            }
        }
    }

    pub fn processed_task(&mut self, processed_image: &mut Option<DynamicImage>, task_id: u32) {
        for task in self.tasks.iter_mut().flatten() {
            if task.id == task_id {
                task.state = TaskState::Processed;
                task.image = processed_image.take();
                break;
            }
        }
    }

    pub fn has_failures(&self) -> bool {
        self.tasks
            .iter()
            .flatten()
            .any(|task| matches!(task.state, TaskState::Failed(_)))
    }

    pub fn count_failures(&self) -> usize {
        self.failed_tasks().len()
    }

    pub fn failed_tasks(&self) -> Vec<&ImageTask> {
        self.tasks
            .iter()
            .flatten()
            .filter(|task| matches!(task.state, TaskState::Failed(_)))
            .collect()
    }

    pub fn processed_tasks(&self) -> Vec<&ImageTask> {
        self.tasks
            .iter()
            .flatten()
            .filter(|task| matches!(task.state, TaskState::Processed))
            .collect()
    }

    pub fn decoded_tasks(&self) -> Vec<&ImageTask> {
        self.tasks
            .iter()
            .flatten()
            .filter(|task| matches!(task.state, TaskState::Decoded))
            .collect()
    }

    pub fn decoded_task_ids(&self) -> Vec<u32> {
        self.tasks
            .iter()
            .flatten()
            .filter(|task| matches!(task.state, TaskState::Decoded))
            .map(|task| task.id)
            .collect()
    }

    pub fn set_task_out_path(&mut self, task_id: u32, path: &mut PathBuf) {
        for task in self.tasks.iter_mut().flatten() {
            if task.id == task_id {
                task.out_path = take(path);
                break;
            }
        }
    }
}
