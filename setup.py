"""
Setup script for eYcel - legacy fallback for older pip versions.
Modern packaging is handled by pyproject.toml.
"""
from setuptools import setup, find_packages

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

with open("requirements.txt", "r", encoding="utf-8") as fh:
    requirements = [line.strip() for line in fh if line.strip() and not line.startswith("#")]

setup(
    name="eYcel",
    version="0.1.0",
    author="Voicu Tomut",
    author_email="",
    description="Excel Data Anonymization & Encryption Tool",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/VoicuTomut/eYcel",
    package_dir={"": "src"},
    packages=find_packages(where="src"),
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
    ],
    python_requires=">=3.9",
    install_requires=[
        "openpyxl>=3.1.0",
        "PyYAML>=6.0",
    ],
    extras_require={
        "dev": [
            "pytest>=7.0.0",
            "pytest-cov>=4.0.0",
            "memory-profiler>=0.60.0",
        ],
        "cli": [
            "tqdm>=4.65.0",
        ],
    },
    entry_points={
        "console_scripts": [
            "eYcel=eYcel_cli:main",
        ],
    },
)
