use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockWriteGuard, RwLockReadGuard},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use anyhow::Context;
use validator::Validate;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound id is {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn creat(&self, payload: CreatTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreatTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    text: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default(),
        }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDatas> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<TodoDatas> {
        self.store.read().unwrap()
    }
}


impl TodoRepository for TodoRepositoryForMemory {
    fn creat(&self, payload: CreatTodo) -> Todo {
        let mut store = self.write_store_ref();
        // idは保存済みの長さ + 1　で管理する
        let id = (store.len() + 1) as i32; 
        let todo = Todo::new(id, payload.text.clone());
        store.insert(id, todo.clone());

        todo
    }

    fn find(&self, id: i32) -> Option<Todo> {
        let store = self.read_store_ref();
        store.get(&id).map(|todo| todo.clone())
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.read_store_ref();
        Vec::from_iter(store.values().map(|todo| todo.clone()))
    }

    // 本当はBoxで返した方がいいかも
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        let mut store = self.write_store_ref();
        let todo = store
            .get(&id)
            .context(RepositoryError::NotFound(id))?;

        // UpdateTodoにtextがなかったらstoreのクローンを返す
        let text= if payload.text.is_empty() {
            todo.text.clone()
        } else {
            payload.text
        };

        // unwrap()を使いたかったがなぜかコンパイルエラーになってしまった。
        // あまりいい書き方ではない
        let completed = if payload.completed{
            true
        } else {
            false
        };


        let todo = Todo {
            id,
            text,
            completed,
        };
        store.insert(id, todo.clone());
        Ok(todo)
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn todo_crud_scenario() {
        let text = String::from("todo text");
        let id = 1;
        let expected = Todo::new(id ,text.clone());
        
        // creat
        let repository = TodoRepositoryForMemory::new();
        let todo = repository.creat(CreatTodo { text });
        assert_eq!(expected, todo);

        // find
        let todo = repository.find(todo.id).unwrap();
        assert_eq!(expected, todo);

        // all
        let todo = repository.all();
        assert_eq!(vec![expected], todo);

        // update
        let text = String::from("update todo text");
        let todo = repository.update(
            1,
            UpdateTodo {
                text: text.clone(),
                completed: true,
            }
         ).expect("failed update todo");

         assert_eq!(
            Todo {
                id,
                text,
                completed: true,
            },
            todo,
         );

        //  delete
        let res = repository.delete(id);
        assert!(res.is_ok());

    }
}