from guillotina import configure
from guillotina_grpc.grpc_helper import make_grpc_call

@configure.service(method='POST', name='@foobar',
                   permission='guillotina.AccessContent')
async def example_service(context, request):
    payload = {
        "tags": '1000',
        "creation_date": "2020-01-02T19:07:48.748922Z",
        "effective_date": "2020-01-02T19:07:48.748922Z",
        "expiration_date": "2020-01-02T19:07:48.748922Z",
        "creators": 'xyz',
    }
    response = make_grpc_call(payload)
    return {
        'foo': response
    }
