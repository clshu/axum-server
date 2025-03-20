// Mock store

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region:    --- Ticket Types

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}
// endregion: --- Ticket Types

// region:    --- Model Controller

#[derive(Clone)] // Clone is required for Arc
pub struct ModelConstroller {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelConstroller {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ticket_store: Arc::default(),
        })
    }
}

// CRUD operations
impl ModelConstroller {
    // Create a new ticket
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    // Read all tickets
    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    // Delete a ticket
    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}
// endregion: --- Model Controller
