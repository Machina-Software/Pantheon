use crate::SharedState;
use rocket::serde::json::Json;
use std::collections::HashMap;
use talaria::api::*;

/// Retrieves all agents
#[get("/agents")]
pub async fn get_agents(state: &rocket::State<SharedState>) -> Json<HashMap<u64, Agent>> {
    Json(state.read().await.agents.clone())
}

/// Retrieves arbitrary amount of network history
/// for specified agent
#[get("/<agent_id>/network_history?<count>")]
pub async fn get_agent_history(
    state: &rocket::State<SharedState>,
    agent_id: u64,
    count: usize,
) -> Option<Json<Vec<NetworkHistoryEntry>>> {
    let agents = state.read().await.agents.clone();

    // FIXME: slow and evil
    match agents.get(&agent_id) {
        Some(agent) => Some(Json(
            agent
                .network_history
                .clone()
                .iter()
                .rev()
                .take(count)
                .map(|x| x.clone())
                .collect::<Vec<NetworkHistoryEntry>>(),
        )),
        None => None,
    }
}

// Retrieves basic info about agent
#[get("/list_agents")]
pub async fn list_agents(state: &rocket::State<SharedState>) -> Json<Vec<AgentInfo>> {
    let agents: HashMap<u64, Agent> = state.read().await.agents.clone();
    let mut agent_info: Vec<AgentInfo> = vec![];

    for (_, agent) in agents {
        agent_info.push(AgentInfo {
            name: agent.nickname.unwrap_or("No Name".to_string()),
            id: agent.id,
            ip: agent.ip.to_string(),
            status: true,
            ping: agent.last_packet_send - agent.last_packet_recv, // FIXME: unsafe,
        });
    }

    Json(agent_info)
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_agents, list_agents, get_agent_history]
}
