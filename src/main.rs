// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[macro_use]
extern crate log;
extern crate rocksdb;

#[path = "log_util.rs"]
mod log_util;
mod transaction;
mod transaction_grpc;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{ChannelBuilder, Environment, ResourceQuota, ServerBuilder};
use rocksdb::{DB};
use transaction_grpc::Transaction;

#[derive(Clone)]
struct TransactionService;

fn intialize_db(path: &str) -> DB {
    return DB::open_default(&path).unwrap();
}

fn db_write(db: &DB, key: &str, value: &str) -> bool {
    let val = db.put(&key.as_bytes(), &value.as_bytes());
    return val.is_ok();
}

fn db_delete(db: &DB, key: &str) -> bool {
    let val = db.delete(key.as_bytes());
    return val.is_ok();
}

fn db_get_value(db: &DB, key: &str) -> String {
    let value = db.get(key.as_bytes()).unwrap().unwrap();
    return String::from_utf8(value).unwrap();
}

impl Transaction for TransactionService {
    fn start_transaction(&mut self, ctx: ::grpcio::RpcContext, req: transaction::StartTransactionRequest, sink: ::grpcio::UnarySink<transaction::TxnId>) {
        let write = req.get_user();
        let db: DB = intialize_db("localdb");
        db_write(&db, write, "1234");
        let value = db_get_value(&db, write);
        let tid: u64 = value.parse().unwrap();
        let mut txn = transaction::TxnId::default();
        txn.set_tid(tid);
        txn.set_part(456);
        db_delete(&db, write);
        let f = sink
            .success(txn)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    let _guard = log_util::init_log(None);
    let env = Arc::new(Environment::new(1));
    let service = transaction_grpc::create_transaction(TransactionService);

    let quota = ResourceQuota::new(Some("HelloServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", 50_051)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server.start();
    for (host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
