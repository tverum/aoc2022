import re

FILENAME = "input/day5.txt"

PATTERN_1 = r"(([\s]{3}|\[\w\])\s{0,1})"
PATTERN_2 = r"move (\d+) from (\d+) to (\d+)"


def main():
    with open(FILENAME, "r") as file:
        lines = file.read()
    stacks_lines, instructions_lines = lines.split("\n\n")
    stacks = parse_stacks(stacks_lines)
    instructions = parse_instructions(instructions_lines)
    original_stacks = [[x for x in y] for y in stacks]
    print(process_instructions_1(stacks, instructions))
    print(process_instructions_2(original_stacks, instructions))


def parse_stacks(text: str):
    lines = text.splitlines()
    n_stacks = len(re.findall(PATTERN_1, lines[0]))
    stacks = [[] for i in range(n_stacks)]
    for line in lines[:-1]:
        for index, match in enumerate(re.findall(PATTERN_1, line)):
            if match[0].strip():
                stacks[index].insert(
                    0, match[0].strip().replace("[", "").replace("]", "")
                )
    return stacks


def parse_instructions(text: str):
    instructions = []
    instructions = re.findall(PATTERN_2, text)
    instructions = list(map(lambda x: (int(x[0]), int(x[1]), int(x[2])), instructions))
    return instructions


def process_instructions_1(stacks, instructions):
    for instruction in instructions:
        amount, s_index, destination = instruction
        to_move = stacks[s_index - 1][-amount:].copy()
        to_move.reverse()
        stacks[s_index - 1] = stacks[s_index - 1][:-amount]
        stacks[destination - 1].extend(to_move)
    return construct_output(stacks)


def process_instructions_2(stacks, instructions):
    for instruction in instructions:
        amount, s_index, destination = instruction
        to_move = stacks[s_index - 1][-amount:].copy()
        stacks[s_index - 1] = stacks[s_index - 1][:-amount]
        stacks[destination - 1].extend(to_move)
    return construct_output(stacks)


def construct_output(stacks):
    result = ""
    for stack in stacks:
        if stack:
            result += stack[-1]
    return result


if __name__ == "__main__":
    main()
