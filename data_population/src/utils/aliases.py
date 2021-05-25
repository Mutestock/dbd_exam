import os

ROOT_DIR = os.path.dirname(os.path.abspath(__file__))
ROOT_DIR = os.path.dirname(ROOT_DIR)
ROOT_DIR = os.path.dirname(ROOT_DIR)

RESOURCES_DIR = os.path.dirname(ROOT_DIR) + "/resources"
DATASETS_DIR = RESOURCES_DIR + "/datasets"
ANIME_DIR = DATASETS_DIR + "/anime_faces"

DATASETS = {
    "universities": DATASETS_DIR + "/universities.csv",
    "first_names": DATASETS_DIR + "/first_names_formatted.csv",
    "last_names": DATASETS_DIR + "/surnames_formatted.csv",
    "random_words_for_email_generation": DATASETS_DIR
    + "/random_words_for_email_generation.csv",
}
