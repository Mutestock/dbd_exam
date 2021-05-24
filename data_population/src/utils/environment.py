import os
from dotenv import load_dotenv
from .aliases import ROOT_DIR

ENVIRONMENT_VARIABLE_FILE = ROOT_DIR + "/.env.development"

# Switches depending on whether or not the project is containerized.
# This is for manual execution versus containerized execution.

if not os.getenv("PRODUCTION_VARIABLES"):
    load_dotenv(ENVIRONMENT_VARIABLE_FILE)
