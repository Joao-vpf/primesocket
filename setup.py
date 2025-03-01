from setuptools import setup, find_packages
import platform

install_requires = [
    'maturin>=1.8.2',
    'wheel>=0.44.0',
]

if platform.system() == "Windows":
    install_requires.append(
        'tabu_structure @ https://github.com/Joao-vpf/primesocket/releases/download/v0.1.0/primesocket_core-0.1.0-cp38-abi3-win_amd64.whl'
    )
elif platform.system() == "Linux":
    install_requires.append(
        'tabu_structure @ https://github.com/Joao-vpf/primesocket/releases/download/v0.1.0/primesocket_core-0.1.0-cp38-abi3-manylinux_2_34_x86_64.whl'
    )


setup(
    name="primesocket",
    version="0.1.0",
    packages=find_packages(),
    install_requires=[],
    include_package_data=True,
    entry_points={
        "console_scripts": [
            "primesocket-server=primesocket.server:main",
            "primesocket-client=primesocket.client:main",
        ],
    },
    author="Joao-vpf",
    description="A performance UDP client-server library for prime number computation",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/Joao-vpf/primesocket",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.8",
)