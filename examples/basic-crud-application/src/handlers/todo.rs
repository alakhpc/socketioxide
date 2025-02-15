use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};
use socketioxide::extract::{AckSender, Data, SocketRef, State};
use tracing::info;
use uuid::Uuid;

use crate::handlers::events::Response;

use super::events::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    id: Uuid,
    #[serde(flatten)]
    inner: PartialTodo,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialTodo {
    completed: bool,
    title: String,
}

#[derive(Clone, Default)]
pub struct Todos(Arc<RwLock<HashMap<Uuid, Todo>>>);
impl Todos {
    fn insert(&self, id: Uuid, todo: Todo) {
        self.0.write().unwrap().insert(id, todo);
    }
    fn get(&self, id: &Uuid) -> Option<Todo> {
        self.0.read().unwrap().get(id).cloned()
    }
    fn get_mut(&self, id: &Uuid) -> Option<Todo> {
        self.0.write().unwrap().get_mut(id).cloned()
    }
    fn remove(&self, id: &Uuid) -> Option<Todo> {
        self.0.write().unwrap().remove(id)
    }
    fn get_all(&self) -> Vec<Todo> {
        self.0.read().unwrap().values().cloned().collect()
    }
}

pub async fn create(
    s: SocketRef,
    Data(data): Data<PartialTodo>,
    ack: AckSender,
    todos: State<Todos>,
) {
    let id = Uuid::new_v4();
    let todo = Todo { id, inner: data };

    todos.insert(id, todo.clone());

    let res: Response<_> = id.into();
    ack.send(&res).ok();

    s.broadcast().emit("todo:created", &todo).await.ok();
}

pub async fn read(Data(id): Data<Uuid>, ack: AckSender, todos: State<Todos>) {
    let todo = todos.get(&id).ok_or(Error::NotFound);
    ack.send(&todo).ok();
}

pub async fn update(s: SocketRef, Data(data): Data<Todo>, ack: AckSender, todos: State<Todos>) {
    let res = match todos.get_mut(&data.id) {
        Some(mut todo) => {
            todo.inner = data.inner.clone();
            s.broadcast().emit("todo:updated", &data).await.ok();
            Ok(())
        }
        None => Err(Error::NotFound),
    };

    ack.send(&res).ok();
}

pub async fn delete(s: SocketRef, Data(id): Data<Uuid>, ack: AckSender, todos: State<Todos>) {
    let res = match todos.remove(&id) {
        Some(_) => {
            s.broadcast().emit("todo:deleted", &id).await.ok();
            Ok(())
        }
        None => Err(Error::NotFound),
    };

    ack.send(&res).ok();
}

pub async fn list(ack: AckSender, todos: State<Todos>) {
    let res: Response<_> = todos.get_all().into();
    info!("Sending todos: {:?}", res);
    ack.send(&res).ok();
}
