from setuptools import find_packages
from setuptools import setup


try:
    README = open('README.rst').read()
except IOError:
    README = None

setup(
    name='guillotina_grpc',
    version="1.0.0",
    description='Guillotina gRPC server',
    long_description=README,
    install_requires=[
        'guillotina'
    ],
    author='Kumar Akshay',
    author_email='k.akshay9721@gmail.com',
    url='',
    packages=find_packages(exclude=['demo']),
    include_package_data=True,
    tests_require=[
        'pytest',
    ],
    extras_require={
        'test': [
            'pytest'
        ]
    },
    classifiers=[],
    entry_points={
    }
)
