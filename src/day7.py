FILENAME = "input/day7.txt"
TRESHOLD = 100000
TOTAL_SPACE = 70000000
TRESHOLD_2 = 30000000


class FileTreeItem(object):
    def __init__(self, name, parent, size: int = 0):
        self.name = name
        self.parent = parent
        self.children = []
        self.size = size

    def add_size(self, size):
        self.size += size
        if self.parent:
            self.parent.add_size(size)

    def __str__(self):
        return f"{self.name} children: {[child.name for child in self.children]} -- size: {self.size}"


def main():
    with open(FILENAME, "r") as file:
        text = file.read()
    root = create_filetree(text)
    print(f"Solution part 1: {part_1(root)}")
    print(f"Solution part 2: {part_2(root)}")


def create_filetree(text):
    commands = [command for command in text.split("$ ") if command]
    root = FileTreeItem("root", None)
    working_dir = root

    for command in commands:
        lines = command.splitlines()
        working_dir = process_command(root, working_dir, lines)
    return root


def process_command(root, working_dir, lines):
    match lines[0].split():
        case ["ls"]:
            output = lines[1:]
            for o_line in output:
                match o_line.split():
                    case ["dir", name]:
                        if name not in working_dir.children:
                            new_item = FileTreeItem(name, working_dir, 0)
                            working_dir.children.append(new_item)
                    case [size, filename]:
                        if filename not in working_dir.children:
                            new_item = FileTreeItem(filename, working_dir, int(size))
                            working_dir.children.append(new_item)
                            working_dir.add_size(int(size))
        case ["cd", ".."]:
            working_dir = working_dir.parent
        case ["cd", "/"]:
            working_dir = root
        case ["cd", directory]:
            if directory in [child.name for child in working_dir.children]:
                working_dir = [
                    child for child in working_dir.children if child.name == directory
                ][0]
    return working_dir


def part_1(root):
    return tree_walkthrough_rec_1(root)


def tree_walkthrough_rec_1(item):
    result = 0
    if item.size < TRESHOLD and item.children:
        result += item.size
    for child in item.children:
        result += tree_walkthrough_rec_1(child)
    return result


def part_2(root):
    unused_space = TOTAL_SPACE - root.size
    eligible_sizes = find_recursive([root], TRESHOLD_2 - unused_space)
    return min(eligible_sizes)


def find_recursive(items, treshold):
    result = []
    for item in items:
        if item.size > treshold:
            result.append(item.size)
            result.extend(find_recursive(item.children, treshold))
    return result


if __name__ == "__main__":
    main()
