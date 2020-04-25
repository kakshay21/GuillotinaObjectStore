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

const METHOD_TRANSACTION_START_TRANSACTION: ::grpcio::Method<super::transaction::StartTransactionRequest, super::transaction::TxnId> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gos.Transaction/StartTransaction",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TRANSACTION_GET_OID_STATE: ::grpcio::Method<super::transaction::GetOidTxn, super::transaction::State> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gos.Transaction/GetOidState",
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

    pub fn start_transaction_opt(&self, req: &super::transaction::StartTransactionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::transaction::TxnId> {
        self.client.unary_call(&METHOD_TRANSACTION_START_TRANSACTION, req, opt)
    }

    pub fn start_transaction(&self, req: &super::transaction::StartTransactionRequest) -> ::grpcio::Result<super::transaction::TxnId> {
        self.start_transaction_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_transaction_async_opt(&self, req: &super::transaction::StartTransactionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::transaction::TxnId>> {
        self.client.unary_call_async(&METHOD_TRANSACTION_START_TRANSACTION, req, opt)
    }

    pub fn start_transaction_async(&self, req: &super::transaction::StartTransactionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::transaction::TxnId>> {
        self.start_transaction_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_oid_state_opt(&self, req: &super::transaction::GetOidTxn, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::transaction::State> {
        self.client.unary_call(&METHOD_TRANSACTION_GET_OID_STATE, req, opt)
    }

    pub fn get_oid_state(&self, req: &super::transaction::GetOidTxn) -> ::grpcio::Result<super::transaction::State> {
        self.get_oid_state_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_oid_state_async_opt(&self, req: &super::transaction::GetOidTxn, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::transaction::State>> {
        self.client.unary_call_async(&METHOD_TRANSACTION_GET_OID_STATE, req, opt)
    }

    pub fn get_oid_state_async(&self, req: &super::transaction::GetOidTxn) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::transaction::State>> {
        self.get_oid_state_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Transaction {
    fn start_transaction(&mut self, ctx: ::grpcio::RpcContext, req: super::transaction::StartTransactionRequest, sink: ::grpcio::UnarySink<super::transaction::TxnId>);
    fn get_oid_state(&mut self, ctx: ::grpcio::RpcContext, req: super::transaction::GetOidTxn, sink: ::grpcio::UnarySink<super::transaction::State>);
}

pub fn create_transaction<S: Transaction + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TRANSACTION_START_TRANSACTION, move |ctx, req, resp| {
        instance.start_transaction(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_TRANSACTION_GET_OID_STATE, move |ctx, req, resp| {
        instance.get_oid_state(ctx, req, resp)
    });
    builder.build()
}
