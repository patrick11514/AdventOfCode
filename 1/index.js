const fs = require('fs')

let elfs = []

let data = fs.readFileSync('input.txt', 'utf8')

let lines = data.split('\n')

let currentCalories = 0

for (let line of lines) {
    if (line != '') {
        currentCalories += parseInt(line)
    } else {
        elfs.push(currentCalories)
        currentCalories = 0
    }
}
//last elf
elfs.push(currentCalories)

console.log('Answer 1: ' + Math.max(...elfs))

let totalCalories = 0
for (let i = 0; i < 3; i++) {
    let mostCalories = 0
    let mostId = 0
    for (let index in elfs) {
        let elf = elfs[index]
        if (elf > mostCalories) {
            mostCalories = elf
            mostId = index
        }
    }

    totalCalories += mostCalories
    elfs[mostId] = 0
}

console.log('Answer 2: ' + totalCalories)
