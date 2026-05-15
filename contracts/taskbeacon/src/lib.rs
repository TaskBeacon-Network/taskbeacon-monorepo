#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[contract]
pub struct TaskBeaconContract;

#[contractimpl]
impl TaskBeaconContract {
    pub fn create_task(env: Env, id: u64, title: String) {
        let mut tasks: Vec<Task> = env
            .storage()
            .persistent()
            .get(&Symbol::new(&env, "tasks"))
            .unwrap_or(Vec::new(&env));
        tasks.push_back(Task { id, title, completed: false });
        env.storage().persistent().set(&Symbol::new(&env, "tasks"), &tasks);
    }

    pub fn complete_task(env: Env, id: u64) {
        let mut tasks: Vec<Task> = env
            .storage()
            .persistent()
            .get(&Symbol::new(&env, "tasks"))
            .unwrap_or(Vec::new(&env));
        let updated: Vec<Task> = tasks
            .iter()
            .map(|t| if t.id == id { Task { completed: true, ..t } } else { t })
            .collect();
        env.storage().persistent().set(&Symbol::new(&env, "tasks"), &updated);
    }

    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage()
            .persistent()
            .get(&Symbol::new(&env, "tasks"))
            .unwrap_or(Vec::new(&env))
    }
}
