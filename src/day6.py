FILENAME = "input/day6.txt"
from itertools import islice


def window(seq, n=2):
    it = iter(seq)
    result = tuple(islice(it, n))
    if len(result) == n:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result


def process_singal(text, size):
    for index, seq in enumerate(window(text, size)):
        if len(list(set(seq))) == size:
            return index + size


def main():
    with open(FILENAME, "r") as file:
        text = file.read()
    print(process_singal(text, 4))
    print(process_singal(text, 14))


if __name__ == "__main__":
    main()
