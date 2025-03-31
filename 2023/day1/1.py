def get_number(line: str) -> int:
    digits = [c for c in line if c.isnumeric()]
    return int(''.join((digits[0], digits[-1])))

def main():
    with open("input") as f:
        numbers = [get_number(line) for line in f]
        print(sum(numbers))

if __name__ == "__main__":
    main()
