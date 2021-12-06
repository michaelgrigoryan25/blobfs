import os
from glob import glob

DIRECTORIES = ["data/files/*"]


def main():
    for directory in DIRECTORIES:
        for file in glob(directory):
            print(f"Removed: {file}")
            os.unlink(file)


if __name__ == "__main__":
    main()
