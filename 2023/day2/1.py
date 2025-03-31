from dataclasses import dataclass


@dataclass
class Game:
    gid: int
    d: dict

    @classmethod
    def read_game(cls, line):
        parts = line.split(": ")
        p = parts[0]
        if not p.startswith("Game "):
            raise ValueError("Wrong header")
        gid = int(p[5:])
        set_strings = parts[1].split(";")
        sets = [cls.parse_set(s) for s in set_strings]
        d = cls.merge_sets(sets)
        return cls(gid=gid, d=d)

    @classmethod
    def merge_sets(cls, sets) -> dict:
        d = {"red": 0, "green": 0, "blue": 0}
        for s in sets:
            for key, value in s.items():
                num = int(value)
                old_num = int(d.get(key, 0))
                if old_num <= num:
                    d[key] = num
        return d

    @classmethod
    def parse_set(cls, s) -> list[dict]:
        """
        >>> Game.parse_set("3 blue, 4 red")
        {'blue': 3, 'red': 4}
        >>> Game.parse_set("1 red, 2 green, 6 blue")
        {'red': 1, 'green': 2, 'blue': 6}
        """
        d = {}
        for item in s.strip().split(","):
            n, color = item.strip().split(" ")
            n = int(n)
            d[color] = n
        return d

    def can_be(self, d) -> bool:
        for key, value in d.items():
            if self.d[key] > value:
                return False
        return True


def get_games():
    with open("input") as f:
        for l in f:
            yield Game.read_game(l)


def main():
    import doctest

    doctest.testmod()
    goal = {
        "red": 12,
        "green": 13,
        "blue": 14,
    }
    total = 0
    for game in get_games():
        if game.can_be(goal):
            print(game.gid)
            total += game.gid
    print(total)


if __name__ == "__main__":
    main()
