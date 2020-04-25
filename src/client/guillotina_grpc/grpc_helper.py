from google.protobuf import json_format
from google.protobuf.json_format import MessageToDict

import grpc
import guillotina_grpc.transaction_pb2 as protobuff
import guillotina_grpc.transaction_pb2_grpc as grpc_server

URL = 'localhost:50051'
CONFIG_OPTIONS = [('grpc.lb_policy_name', 'pick_first'),
    ('grpc.enable_retries', 0),
    ('grpc.keepalive_timeout_ms', 10000)
]

def make_grpc_call(payload):
    with grpc.insecure_channel(target=URL, options=CONFIG_OPTIONS) as channel:
        stub = grpc_server.TransactionStub(channel)
        response = stub.SaveDublinCore(
            json_format.ParseDict(payload, protobuff.DublinCore()),
            timeout=10
        )
        return MessageToDict(response)
