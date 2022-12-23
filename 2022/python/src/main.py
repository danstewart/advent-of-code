import sys
import os
from importlib import import_module


def main():
    if len(sys.argv) != 3:
        print("Usage: python main.py <day> <part>")
        sys.exit(1)

    day = int(sys.argv[1])
    part = int(sys.argv[2])
    nice_day = str(day).zfill(2)
    file_name = os.path.join("data", f"day{nice_day}.txt")

    data = ""
    with open(file_name, "r") as f:
        data = f.read()

    # import the module for the day
    module = import_module(f"days.day{nice_day}")
    result = module.main(part, data)

    print(f"The answer for day {nice_day} is: {result}")


if __name__ == "__main__":
    main()
