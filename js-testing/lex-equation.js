/*
5 + 4 {
    Branch - Addition<Number> - 2 {
        left: 5,
        right: 4
    }
}

2 + 4 * (2 / 2) {
    Branch - Addition<Number> - 2 {
        left: 2,
        right: Branch - Multiplication<Number> - 2 {
            left: 4,
            right: Branch - Division<Number> - 2 {
                left: 2,
                right: 2
            }
        }
    }
}
*/

const input = "2 + 4 * (2 / 2)";
const tokens = input.split(" ");
const operators = ["+", "-", "*", "/"];
const precedence = {
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2
};

const stack = [];
const root = {}
// Construct a nested JSON object from the input.
for (let i = 0; i < tokens.length; i++) {
    const token = tokens[i];
    if (operators.includes(token)) {
        const right = stack.pop();
        const left = stack.pop();
        const node = {
            left,
            right
        };
        stack.push(node);
    } else {
        stack.push(token);
    }
}

console.log(stack)