from operator import attrgetter
from collections import deque
from typing import Callable
from dataclasses import dataclass
from devtools import debug


@dataclass
class Monkey:
    items: deque[int]
    test: Callable[[int], bool]
    operation: Callable[[int], int]
    on_true: int
    on_false: int
    items_inspected: int

    @classmethod
    def from_string(cls, data: str) -> "Monkey":
        lines = data.split("\n")

        # Parse starting items
        items = lines[1].split(":")[-1].strip()
        items = items.split(",")
        items = [int(item.strip()) for item in items]

        # Parse operation
        operation = lines[2].split(":")[-1].strip()
        operation = operation.split(" ")
        operation_type = operation[3]
        operation_value = operation[4]
        if operation_type == "*":
            if operation_value == "old":
                op = lambda num: num**2
            else:
                operation_value = int(operation_value)
                op = lambda num: num * operation_value
        else:
            if operation_value == "old":
                op = lambda num: num + num
            else:
                operation_value = int(operation_value)
                op = lambda num: num + operation_value

        # Parse test
        test_value = lines[3].split(":")[-1]
        test_value = int(test_value.split(" ")[-1])
        test = lambda value: value % test_value == 0

        # Parse targets
        true_target = int(lines[4].split(":")[-1].split(" ")[-1])
        false_target = int(lines[5].split(":")[-1].split(" ")[-1])

        return cls(
            items=deque(items),
            operation=op,
            test=test,
            on_true=true_target,
            on_false=false_target,
            items_inspected=0,
        )

    def inspect_and_get_next_item(self, worry_modifier: Callable[[int], int]) -> int:
        item = self.items.popleft()
        item = self.operation(item)
        item = worry_modifier(item)

        self.items_inspected += 1

        return item

    def throw_item(self, item, others: list["Monkey"]):
        if self.test(item):
            others[self.on_true].items.append(item)
        else:
            others[self.on_false].items.append(item)


def main(part: int, data: str):
    if part == 1:
        return part1(data)
    elif part == 2:
        return part2(data)
    raise ValueError("Invalid part number")


def part1(data: str) -> int:
    rounds = 20
    monkeys: list[Monkey] = []

    for chunk in data.split("\n\n"):
        monkeys.append(Monkey.from_string(chunk))

    for _ in range(rounds):
        for monkey in monkeys:
            while monkey.items:
                item = monkey.inspect_and_get_next_item(lambda item: item // 3)
                monkey.throw_item(item, monkeys)

    monkeys = sorted(monkeys, key=attrgetter("items_inspected"), reverse=True)
    return monkeys[0].items_inspected * monkeys[1].items_inspected


def part2(data: str) -> int:
    return 0
