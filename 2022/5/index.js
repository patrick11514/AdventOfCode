const fs = require('fs')

const lines = fs.readFileSync('./input.txt', 'utf8').split('\n')

let data = []
let moves = []

let d = true
for (let l of lines) {
    if (l == '') {
        d = false
        continue
    }

    if (d) {
        data.push(l)
    } else {
        moves.push(l)
    }
}

//remnove last line from data
data.pop()

let rows = {}

for (let d of data) {
    //split every 3rd character
    let split = d.match(/.{1,4}/g)
    //console.log(split)
    //trim split
    let trimmed = split.map((s) => s.trim())
    let i = 1
    trimmed.forEach((t) => {
        if (!rows[i]) {
            rows[i] = []
        }
        rows[i].push(t)
        i++
    })
}

//reverse each array in rows and remove empty values
for (let r in rows) {
    rows[r] = rows[r].reverse()
    rows[r] = rows[r].filter((r) => r != '')
}

//Function to move with blocks by moves
function parseMoves(rows, moves, reverse = true) {
    //parse moves
    for (let move of moves) {
        //match move NUMBER from NUMBER to NUMBER
        let [_, count, from, to] = move.match(/move (\d+) from (\d+) to (\d+)/)

        //remove count from from end of array and put it to to
        let removed = rows[from].splice(-count)
        if (reverse) {
            removed = removed.reverse()
        }
        rows[to] = rows[to].concat(removed)
    }

    let output = ''
    for (let r in rows) {
        let row = rows[r]
        //get last element from array, remove [] and add to output
        let last = row.pop()
        output += last.replace('[', '').replace(']', '')
    }

    return output
}

//silver
console.log(parseMoves(structuredClone(rows), moves))

//gold
console.log(parseMoves(structuredClone(rows), moves, false))
