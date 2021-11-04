import os
from glob import glob

directories = ["data/files/*", "data/keys/[!test_*]*"]


def main():
    for directory in directories:
        for file in glob(directory):
            print(f"Removed: {file}")
            os.unlink(file)


if __name__ == "__main__":
    main()
