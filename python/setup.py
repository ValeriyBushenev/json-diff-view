from setuptools import setup
from setuptools_rust import Binding, RustExtension
import os
import sys

# Версия для удобного обновления
VERSION = "0.1.2"
PACKAGE_NAME = "json-diff-view"

# Определяем путь к корню проекта (относительно этого файла)
ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# Чтение README.md для long_description
readme_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "README.md")
if not os.path.exists(readme_path):
    # Если нет локального README.md, используем из корня проекта
    readme_path = os.path.join(ROOT_DIR, "README.md")

long_description = "A tool for visually displaying differences between JSON files in a human-readable format"
if os.path.exists(readme_path):
    with open(readme_path, "r", encoding="utf-8") as f:
        long_description = f.read()

setup(
    name=PACKAGE_NAME,
    version=VERSION,
    packages=[],
    description="A tool for visually displaying differences between JSON files in a human-readable format",
    long_description=long_description,
    long_description_content_type="text/markdown",
    author="Your Name",
    author_email="your.email@example.com",
    url="https://github.com/your-username/json-diff-view",
    rust_extensions=[
        RustExtension(
            "json_diff_view", 
            path=os.path.join(ROOT_DIR, "Cargo.toml"),
            binding=Binding.PyO3,
            features=["python-bindings"]
        )
    ],
    setup_requires=["setuptools-rust>=1.0.0"],
    install_requires=[],
    python_requires=">=3.7",
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Rust",
        "Topic :: Software Development :: Libraries",
    ],
    zip_safe=False,
)
