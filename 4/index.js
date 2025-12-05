const fs = require('fs')

const lines = fs.readFileSync('input.txt', 'utf8').split('\n')

let total = 0
let total2 = 0

for (let line of lines) {
    const elves = line.split(',').map((range) => {
        const [start, end] = range.split('-').map(Number)
        return { start, end }
    })

    //silver star - elft 1 contains elf 2
    if (elves[1].start >= elves[0].start && elves[1].end <= elves[0].end) {
        total++
    } else if (elves[0].start >= elves[1].start && elves[0].end <= elves[1].end) {
        total++
    }

    //gold star
    if (
        (elves[1].start >= elves[0].start && elves[1].start <= elves[0].end) ||
        (elves[1].end >= elves[0].start && elves[1].end <= elves[0].end)
    ) {
        total2++
    } else if (
        (elves[0].start >= elves[1].start && elves[0].start <= elves[1].end) ||
        (elves[0].end >= elves[1].start && elves[0].end <= elves[1].end)
    ) {
        total2++
    }
}

console.log(total)
console.log(total2)
