import importlib.metadata as metadata

# constants
PACKAGE_NAME = "diem-sdk"


class Metadata:
    DIEM_HEADER = "x-diem-client"

    @staticmethod
    def get_diem_header_val():
        version = metadata.version(PACKAGE_NAME)
        return f"diem-python-sdk/{version}"
