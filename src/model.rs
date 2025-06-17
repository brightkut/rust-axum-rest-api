use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTicketDto {
    title: String,
}

#[derive(Clone)]
pub struct TicketController {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl TicketController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ticket_store: Arc::default(),
        })
    }

    pub async fn create_ticket(&self, dto: CreateTicketDto) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();

        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            title: dto.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn find_all_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketNotFound { id })
    }
}
