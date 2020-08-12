#!/usr/bin/python3
import os

check_directory = os.path.dirname(os.path.abspath(__file__))

check_scripts = [check_script for check_script in os.listdir(check_directory) if not "all.py" in check_script]

# Run all checks and ruturn the last known error code.
# Otherwise exit with status code 0.
status_code = None
for check_script in check_scripts:
    status_code = os.system(check_directory + "/" + check_script)

exit(status_code)