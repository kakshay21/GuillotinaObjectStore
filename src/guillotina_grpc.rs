// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_TRANSACTION_START_TRANSACTION: ::grpcio::Method<super::guillotina::StartTransactionRequest, super::guillotina::TxnId> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gos.Transaction/StartTransaction",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct TransactionClient {
    client: ::grpcio::Client,
}

impl TransactionClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        TransactionClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn start_transaction_opt(&self, req: &super::guillotina::StartTransactionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::guillotina::TxnId> {
        self.client.unary_call(&METHOD_TRANSACTION_START_TRANSACTION, req, opt)
    }

    pub fn start_transaction(&self, req: &super::guillotina::StartTransactionRequest) -> ::grpcio::Result<super::guillotina::TxnId> {
        self.start_transaction_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_transaction_async_opt(&self, req: &super::guillotina::StartTransactionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::guillotina::TxnId>> {
        self.client.unary_call_async(&METHOD_TRANSACTION_START_TRANSACTION, req, opt)
    }

    pub fn start_transaction_async(&self, req: &super::guillotina::StartTransactionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::guillotina::TxnId>> {
        self.start_transaction_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Transaction {
    fn start_transaction(&mut self, ctx: ::grpcio::RpcContext, req: super::guillotina::StartTransactionRequest, sink: ::grpcio::UnarySink<super::guillotina::TxnId>);
}

pub fn create_transaction<S: Transaction + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_TRANSACTION_START_TRANSACTION, move |ctx, req, resp| {
        instance.start_transaction(ctx, req, resp)
    });
    builder.build()
}
