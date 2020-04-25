# from aiohttp import web
# from guillotina import configure
# from guillotina import content
# from guillotina import schema
# from guillotina.factory import make_app
# from zope import interface


# class IMyType(interface.Interface):
#     foobar = schema.TextLine()

# @configure.contenttype(
#     type_name="MyType",
#     schema=IMyType,
#     behaviors=["guillotina.behaviors.dublincore.IDublinCore"])
# class MyType(content.Resource): pass

# @configure.service(
#     context=IMyType, method='GET', permission='guillotina.ViewContent', name='@foobar')
# async def foobar_service(context, request):
#     return {
#         "foobar": context.foobar
#     }

# if __name__ == '__main__':
#     app = make_app(settings={ "applications": ["__main__"] })
#     web.run_app(app)