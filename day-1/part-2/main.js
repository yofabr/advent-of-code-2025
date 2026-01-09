const fs = require('node:fs/promises'); // Import the promise-based fs module

function parse(input) {
    return input.split("\n").map(x => {
        let [, direction, count] = x.match(/(L|R)(\d+)/);
        return { direction: direction === "L" ? -1 : 1, count: +count };
    });
}


async function readMyFile(filePath) {
  try {
    const data = await fs.readFile(filePath, 'utf8');
    return data;
  } catch (error) {
    console.error('Error reading file:', error);
  }
}

function part2(input) {
    let password = 0;
    parse(input).reduce((prev, curr) => {
        for (let i = 0; i < curr.count; i++) {
            prev += curr.direction;
            if (prev === -1) prev = 99;
            if (prev === 100) prev = 0;
            if (prev === 0) password++;
        }
        return prev;
    }, 50);
    return password;
}



async function main() {
    const content = await readMyFile('input.txt');
    let pass = part2(content)
    console.log("Pass:", pass);
}

main();

