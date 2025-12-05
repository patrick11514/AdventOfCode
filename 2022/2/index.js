const fs = require('fs')

let rows = fs.readFileSync('input.txt', 'utf8').split('\n')

let scores = {
    X: 1,
    Y: 2,
    Z: 3,
}

let translates = {
    X: 'A',
    Y: 'B',
    Z: 'C',
}

//A B C
//R P S

let wins = {
    A: 'C',
    C: 'B',
    B: 'A',
}
function calculateScore(rows) {
    let score = 0
    for (let row of rows) {
        let symbols = row.split(' ')
        score += scores[symbols[1]]
        if (symbols[0] == translates[symbols[1]]) score += 3
        else score += wins[translates[symbols[1]]] == symbols[0] ? 6 : 0
    }
    return score
}

console.log(calculateScore(rows))

let reverseTranslates = {
    A: 'X',
    B: 'Y',
    C: 'Z',
}

let reverseWins = {
    A: 'B',
    B: 'C',
    C: 'A',
}

let newRows = []

for (let row of rows) {
    let symbols = row.split(' ')

    if (symbols[1] == 'X') {
        if (symbols[0] == 'A') {
            symbols[1] = reverseTranslates['C']
        } else if (symbols[0] == 'B') {
            symbols[1] = reverseTranslates['A']
        } else {
            symbols[1] = reverseTranslates['B']
        }
    } else if (symbols[1] == 'Y') {
        symbols[1] = reverseTranslates[symbols[0]]
    } else {
        symbols[1] = reverseTranslates[reverseWins[symbols[0]]]
    }

    newRows.push(symbols.join(' '))
}

console.log(calculateScore(newRows))
