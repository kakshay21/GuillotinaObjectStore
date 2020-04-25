import asyncio


async def test_install(guillotina_grpc_requester):  # noqa
    async with guillotina_grpc_requester as requester:
        response, _ = await requester('GET', '/db/guillotina/@addons')
        assert 'guillotina_grpc' in response['installed']
