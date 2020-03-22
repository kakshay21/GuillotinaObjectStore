// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[macro_use]
extern crate log;

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
use transaction_grpc::Transaction;

#[derive(Clone)]
struct TransactionService;

impl Transaction for TransactionService {
    fn start_transaction(&mut self, ctx: ::grpcio::RpcContext, req: transaction::StartTransactionRequest, sink: ::grpcio::UnarySink<transaction::TxnId>) {
        let write = req.get_write();
        //  get transaction id from db
        let mut txn = transaction::TxnId::default();
        txn.set_tid(123);
        txn.set_part(456);
        let f = sink
            .success(txn)
            .map_err(move |e| error!("failed to reply {:?}: {:?}\n payload: {:?}", req, e, write));
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
