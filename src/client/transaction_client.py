from __future__ import print_function
import logging

import grpc

import transaction_pb2
import transaction_pb2_grpc


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
        stub = transaction_pb2_grpc.TransactionStub(channel)
        # Timeout in seconds.
        # Please refer gRPC Python documents for more detail. https://grpc.io/grpc/python/grpc.html
        response = stub.StartTransaction(
            transaction_pb2.StartTransactionRequest(write=True, user='akshay', path='/home'),
            timeout=10
        )
    print(response)


if __name__ == '__main__':
    logging.basicConfig()
    run()