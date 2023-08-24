---
title: "Python SDK"
slug: "python-sdk"
---

# Diem Python SDK

Diem provides a lightly maintained official Python SDK. It is available on [PyPi](https://pypi.org/project/diem-sdk/) with the source code in the [Diem-core GitHub repository](https://github.com/aptos-labs/diem-core/tree/main/ecosystem/python/sdk). Much of the functionality of the Python SDK mirrors the [Typescript SDK](./ts-sdk/index.md). The primary purpose of the Python SDK is to help Python developers to quickly become familiar with Diem and as an accompaniment to Diem tutorials.

## Installing Python SDK

The Python SDK can either be installed via `pip`, from source, or embedded:

### Install with pip

To install via `pip`:

```bash
pip3 install diem-sdk
```

The `diem-sdk` will be installed in the local site packages directory. For example, on macOS, you will find the `diem-sdk` in the `~/Library/Python/3.8/lib/python/site-packages/diem_sdk` directory.

### Install from the source code

To install from source:

```bash
git clone https://github.com/aptos-labs/diem-core
cd diem-core/ecosystem/python/sdk
python3 setup.py install --user
```

### Install by embedding

To embed the Python SDK into your existing Python project:

```
cd /path/to/python/project
cp -r /path/to/diem-core/ecosystem/python/sdk/diem-sdk diem-sdk
```

## Using the Python SDK

See the [Developer Tutorials](../tutorials/index.md) for code examples showing how to use the Python SDK.