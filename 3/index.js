const fs = require('fs')

let priorities = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('')

let prios = {}

for (let p in priorities) {
    let priority = parseInt(p) + 1
    prios[priorities[p]] = priority
}

let rucksacks = fs.readFileSync('input.txt', 'utf8').split('\n')

let sum = 0

for (let r of rucksacks) {
    let c = [r.substring(0, r.length / 2), r.substring(r.length / 2)]

    let items = c[0].split('')

    let same = []

    for (let i of c[1].split('')) {
        if (items.includes(i)) {
            if (!same.includes(i)) same.push(i)
        }
    }

    for (let s of same) {
        sum += prios[s]
    }
}

console.log(sum)

let i = 0
let items = []
let same = []

sum = 0

for (let r of rucksacks) {
    switch (i) {
        case 0:
            items = r.split('')
            break
        case 1:
            for (let i of r.split('')) {
                if (items.includes(i)) {
                    if (!same.includes(i)) same.push(i)
                }
            }
            break
        case 2:
            let localSame = []
            let finalSame = []
            for (let i of r.split('')) {
                if (items.includes(i)) {
                    if (!localSame.includes(i)) localSame.push(i)
                }
            }

            for (let s of same) {
                if (localSame.includes(s)) finalSame.push(s)
            }

            for (let l of finalSame) {
                sum += prios[l]
            }

            i = -1
            items = []
            same = []
            break
    }

    i++
}

console.log(sum)
