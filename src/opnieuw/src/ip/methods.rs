use crate::{
    ip::models::{
        IpData,
        NewTrafficType,
        ProtectedReferences,
        Token,
        IP,
    },
    utils::{
        counter::Counter,
        cycle::Cycle,
    },
    IPS,
};
use std::{
    net::Ipv4Addr,
    ops::Deref,
    str::FromStr,
    sync::{
        Arc,
        RwLock,
    },
};
use tokio::task::AbortHandle;

impl IP {
    // Create a new IP address or get it
    pub fn get(ip: &str) -> Arc<IP> {
        match IPS.get(ip) {
            Some(ip) => ip.clone(),
            None => {
                let obj = Arc::new(IP {
                    ip: ip.to_string(),
                    points: Counter::new(),
                    cycle: Cycle::new(),
                    protected: RwLock::new(ProtectedReferences {
                        tokens: Vec::new(),
                        handles: Vec::new(),
                    }),
                    socket_addr: Ipv4Addr::from_str(ip).unwrap(),
                    data: RwLock::new(None),
                });

                IPS.insert(ip.to_string(), obj.clone());

                obj
            }
        }
    }

    // Allow a request to be made
    pub fn allow(&self, traffic_type: NewTrafficType) -> bool {
        if self.points.get() > 2000 {
            return false;
        }

        match traffic_type {
            NewTrafficType::Stream => self.points.inc_by(69).get() < 2000,
            NewTrafficType::Request => self.points.inc_by(2).get() < 2000,
            NewTrafficType::Token => {
                if self.points.inc_by(50).get() > 2000 {
                    return false;
                }

                true
            }
        }
    }

    // Add a token to the IP address
    pub fn add_token(&self, token: Token) {
        let mut write_lock = self.protected.write().unwrap();

        write_lock.tokens.push((token, Cycle::new()));
    }

    // Remove old tokens from the IP address
    pub fn remove_old_tokens(&self) {
        let mut write_lock = self.protected.write().unwrap();

        write_lock.tokens.retain(|(_, cycle)| cycle.diff() < 100);
    }

    // Add a handle to the IP address
    pub fn add_handle(&self, handle: tokio::task::AbortHandle) {
        let mut write_lock = self.protected.write().unwrap();

        write_lock.handles.push(handle);
    }

    // Remove old handles from the IP address
    pub fn remove_old_handles(&self) {
        let mut write_lock = self.protected.write().unwrap();

        // Remove handles that are finished
        write_lock.handles.retain(|handle| !handle.is_finished());
    }

    // Get handles from the IP address
    pub fn get_handles_len(&self) -> usize {
        let read_lock = self.protected.read().unwrap();

        read_lock.handles.len()
    }

    // Abort!!!!
    pub fn abort_handles(&self) {
        let read_lock = self.protected.read().unwrap();

        for i in read_lock.handles.iter() {
            i.abort();
        }
    }

    // Ban the IP address, to be called after banning at the network level
    pub fn ban(&self) {
        // Abort handles
        let write_lock = self.protected.write().unwrap();

        for handle in write_lock.handles.iter() {
            // schedule the task to be aborted
            handle.abort();
        }

        IPS.remove(&self.ip);
    }

    // Ip address information

    // Get IP address data
    pub fn get_data(&self) -> Option<IpData> {
        let read_lock = self.data.read().unwrap();

        (*read_lock).clone()
    }

    // Set IP address data
    pub fn set_data(&self, data: IpData) {
        let mut write_lock = self.data.write().unwrap();

        *write_lock = Some(data);
    }
}
