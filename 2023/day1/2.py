from typing import Generator

digits = {
    "zero": "0",
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}


def get_next_digit(line: str) -> str | None:
    """
    >>> print(get_next_digit("mone93"))
    1
    """
    if not line:
        return None
    if line[0].isnumeric():
        return line[0]
    for word in digits:
        if line.startswith(word):
            return digits[word]
    return get_next_digit(line[1:])


def get_all_digits(line: str) -> Generator[int, None, None]:
    l = line
    while l:
        d = get_next_digit(l)
        if d:
            l = l[len(d) :]
            if d.isnumeric():
                yield d
            else:
                yield d
        else:
            l = l[1:]


def get_number(line: str) -> int:
    digits = list(get_all_digits(line))
    digits = [digits[0], digits[-1]]
    return int("".join(digits))


def main():
    import doctest
    doctest.testmod()
    with open("input") as f:
        numbers = [get_number(line) for line in f]
        print(sum(numbers))


if __name__ == "__main__":
    main()
