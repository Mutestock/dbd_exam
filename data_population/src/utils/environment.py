import os
from dotenv import load_dotenv
from .aliases import ROOT_DIR

ENVIRONMENT_VARIABLE_FILE = ROOT_DIR + "/.env.development"

if not os.getenv("PRODUCTION_VARIABLES"):
    load_dotenv(ENVIRONMENT_VARIABLE_FILE)
