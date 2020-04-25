from __future__ import print_function
from google.protobuf import json_format
from guillotina.behaviors.dublincore import IDublinCore
from guillotina.json.deserialize_value import schema_compatible

import grpc
import jsonschema
import logging
import string
import time

import guillotina_grpc.transaction_pb2 as protobuff
import guillotina_grpc.transaction_pb2_grpc as grpc_server

TEST_PAYLOAD = {
    "tags": '1000',
    "creation_date": "2020-01-02T19:07:48.748922Z",
    "effective_date": "2020-01-02T19:07:48.748922Z",
    "expiration_date": "2020-01-02T19:07:48.748922Z",
    "creators": 'xyz',
}

def run():
    # NOTE(gRPC Python Team): .close() is possible on a channel and should be
    # used in circumstances in which the with statement does not fit the needs
    # of the code.
    #
    # For more channel options, please see https://grpc.io/grpc/core/group__grpc__arg__keys.html
    with grpc.insecure_channel(target='localhost:50051',
                               options=[('grpc.lb_policy_name', 'pick_first'),
                                        ('grpc.enable_retries', 0),
                                        ('grpc.keepalive_timeout_ms', 10000)
                                       ]) as channel:
        stub = grpc_server.TransactionStub(channel)
        # Timeout in seconds.
        # Please refer gRPC Python documents for more detail. https://grpc.io/grpc/python/grpc.html

        response = stub.StartTransaction(
            protobuff.StartTransactionRequest(write=True, user='akshay', path='/home'),
            timeout=10
        )

        response2 = stub.GetOidState(
            protobuff.GetOidTxn(tid=10, oid='akshay', update=True),
            timeout=10
        )

        response3 = stub.SaveDublinCore(
            json_format.ParseDict(TEST_PAYLOAD, protobuff.DublinCore()),
            timeout=10
        )
    print(response3)
    print(response2)
    print(response)


if __name__ == '__main__':
    logging.basicConfig()
    run()