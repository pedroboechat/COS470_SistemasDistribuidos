"""Validator for T3 (DME) execution logs"""

def validate(filename: str):
    """Function to validate a DME execution log

    Args:
        filename (str): Name of the file to validate

    Raises:
        Exception: In case of invalid grants and releases sequence
    """
    class InvalidLogFileError(ValueError):
        def __init__(self, *args):
            super().__init__(args)
        
        def __str__(self):
            return "Invalid log file: invalid grants and releases sequence"

    with open(filename, "r") as log_file:
        lines = log_file.readlines()
        requests = []
        grants = []
        releases = []

        for line in lines:
            if "[R] Request" in line:
                requests.append(int(line.split("-")[2]))
                continue
            if "[S] Grant" in line:
                if len(grants) != len(releases):
                    print(line)
                    print(len(grants))
                    print(len(releases))
                    raise InvalidLogFileError()
                grants.append(int(line.split("-")[2]))
                continue
            if "[R] Release" in line:
                if len(releases) != len(grants) - 1:
                    raise InvalidLogFileError()
                releases.append(int(line.split("-")[2]))

        for i in range(len(requests)):
            if requests[i] != grants[i] or grants[i] != releases[i]:
                raise InvalidLogFileError()

        print("Log file was successfully validated")

if __name__ == "__main__":
    validate()
