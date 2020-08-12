#!/usr/bin/python3.7
import os

check_directory = os.path.dirname(os.path.abspath(__file__))

check_scripts = [check_script for check_script in os.listdir(check_directory) if not "all.py" in check_script]

for check_script in check_scripts:
    check_error_code = os.system(check_directory + "/" + check_script)
    if error_code:
        exit
