use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(Eq)]
pub struct Todo {
    pub id: 132,
    pub title: String,
    pub due_date: String,
    pub is_done: bool,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub todos: Vec<Todo>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Add {todo: Todo},
    Remove {todo: Todo},
    Update {todo: Todo},
    Reset {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetTodosResponse)]
    GetTodos {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetTodosResponse {
    pub todos: Vec<Todo>,
}
