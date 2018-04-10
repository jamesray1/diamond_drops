use collation::header;
use collation::collation;
use message;

use indextree;

use std::sync::mpsc;
use std::collections::HashMap;

pub struct Notary {
    selected: bool,
    shard_id: usize,
    collation_trees: HashMap<usize, indextree::Arena<collation::Collation>>,
    listener: mpsc::Receiver<message::Message>,
}

impl Notary {
    pub fn new(listener: mpsc::Receiver<message::Message>) -> Notary {
        Notary {
            selected: false,
            shard_id: 0,
            listener,
            collation_trees: HashMap::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            // Get message from the SMC listener
            let msg_result = self.listener.recv();
            let msg: message::Message;

            match msg_result {
                Ok(m) => { msg = m }
                Err(e) => { eprintln!("Error receiving message from SMC listener: {}", e); continue }
            }

            // Respond to SMC listener message
            match msg {
                message::Message::Selected { value } => { self.selected = value; }
                message::Message::Shard { value } => { self.shard_id = value; },
                message::Message::Header { value } => { self.store_collation_header(value); },
                message::Message::Collation { value } => { self.store_collation(value); },
                message::Message::Proposal { value } => { self.store_proposal(value); }
            }

            if self.selected {
                self.get_availability();
                self.submit_vote();
            }
        }
    }

    fn store_collation_header(&self, header: header::Header) {}

    fn store_collation(&self, collation: collation::Collation) {}

    fn store_proposal(&self, collation: collation::Collation) {}

    fn get_availability(&self) {}

    fn submit_vote(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_stores_collation_header() {
        assert!(false);
    }

    #[test]
    fn it_downloads_collations() {
        assert!(false);
    }

    #[test]
    fn it_collects_proposals() {
        assert!(false);
    }

    #[test]
    fn it_selects_vote() {
        assert!(false);
    }

    #[test]
    fn it_submits_vote() {
        assert!(false);
    }
}