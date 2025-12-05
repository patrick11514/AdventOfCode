const fs = require('fs')

const chars = fs.readFileSync('input.txt', 'utf8').split('')

function checkDuplicates(array) {
    let seen = []
    for (let a of array) {
        if (seen.includes(a)) {
            return true
        }
        seen.push(a)
    }
    return false
}

function getAnswer(count) {
    for (let i in chars) {
        i = parseInt(i)
        //let char = chars[i]

        let list = []
        for (let l = i; l < i + count; l++) {
            //console.log(l)
            list.push(chars[l])
        }
        if (!checkDuplicates(list)) {
            //console.log(list)
            console.log(i + count)
            break
        }
    }
}

//silver
getAnswer(4)
//gold
getAnswer(14)
