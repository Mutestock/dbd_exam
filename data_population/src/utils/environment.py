import os
from dotenv import load_dotenv
from .aliases import ROOT_DIR

ENVIRONMENT_VARIABLE_FILE = ROOT_DIR + "/.env.development"

# Switches depending on whether or not the project is containerized.
# This is for manual execution versus containerized execution.

_message_issued = False

if not os.getenv("PRODUCTION_VARIABLES"):
    if not _message_issued:
        print(
            "PRODUCTION_VARIABLES environment variable not found, Loading .env.development..."
        )
        print(
            "If this happened inside the container, please add a PRODUCTION_VARIABLES environment variable in the docker compose file"
        )
        _message_issued = True
    load_dotenv(ENVIRONMENT_VARIABLE_FILE)
else:
    if not _message_issued:
        print("Production mode is on")


