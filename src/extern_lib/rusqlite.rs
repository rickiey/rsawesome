#![feature(once_cell)]

use rusqlite::{params, Connection, Result};
use std::sync::Mutex;
use std::lazy::SyncLazy;

static CONN : SyncLazy<Mutex<Connection>> = SyncLazy::new(|| {
    let s = Mutex::new(Connection::open("penalty_msg.db").unwrap());
    println!("create conn");
    let _ = s.lock().unwrap().execute("CREATE TABLE if not exists  `penalty_msgs` (`to_addr` text,`from_addr` text,`height` integer,
                    `amount` text,`time_at` datetime,`call_function` text,`sub_cause` text);", params![]);
    s
});

pub fn use_sqlite() {
    let mut conn = CONN.lock().unwrap();
    let _ = conn.execute("INSERT INTO penalty_msgs (to_addr,from_addr,height,amount,time_at,call_function,sub_cause) VALUES (?1,?2,?3,?4,?5,?6,?7)",
        params![
            "to_addr",
            "from_addr",
            "height",
            "amount",
            "time_at",
            "call_function",
            "sub_cause"
        ]);
}