const fs = require('fs')

const lines = fs.readFileSync('input.txt', 'utf8').trim().split('\n')

lines.shift()

let fileSystem = {}
let currentDir = null
let currentObject = null
let addingFiles = false

for (let line of lines) {
    let match = line.match(/^\$ ls$/)
    if (match) {
        addingFiles = true
    } else if (addingFiles) {
        let match2 = line.match(/cd (.*)/)
        if (match2) {
            if (!currentDir) {
                currentDir = match2[1]
                addingFiles = false
            } else {
                if (match2[1] == '..') {
                    currentDir = currentDir.split('/')
                    currentDir.pop()
                    currentDir = currentDir.join('/')
                    continue
                } else {
                    currentDir = currentDir + '/' + match2[1]
                }
            }
            continue
        }

        if (currentDir) {
            let subdirs = currentDir.split('/')
            currentObject = fileSystem
            for (let dir of subdirs) {
                currentObject = currentObject[dir]
            }
        } else {
            currentObject = fileSystem
        }

        let match = line.match(/dir (.*)/)
        if (match) {
            currentObject[match[1]] = {}
        } else {
            let match = line.match(/(\d+) (.*)/)
            if (match) {
                currentObject[match[2]] = match[1]
            }
        }
    }
}

function getSizeOfDir(obj) {
    let size = 0
    for (let key in obj) {
        if (typeof obj[key] == 'object') {
            size += getSizeOfDir(obj[key])
        } else {
            size += parseInt(obj[key])
        }
    }
    return size
}

function getSizeOfDir(obj) {
    let size = 0
    for (let key in obj) {
        if (typeof obj[key] == 'object') {
            size += getSizeOfDir(obj[key])
        } else {
            size += parseInt(obj[key])
        }
    }
    return size
}

//silver
{
    let sum = 0
    function goThroughDir(obj) {
        for (let key in obj) {
            if (typeof obj[key] == 'object') {
                let sz = getSizeOfDir(obj[key])
                if (sz <= 100000) {
                    sum += sz
                }
                goThroughDir(obj[key])
            }
        }
    }

    goThroughDir(fileSystem, sum)
    console.log(sum)
}

//gold
{
    let sizes = []

    let space = 70000000
    let needed = 30000000
    let free = space - getSizeOfDir(fileSystem)

    function goThroughDir(obj) {
        for (let key in obj) {
            if (typeof obj[key] == 'object') {
                let sz = getSizeOfDir(obj[key])
                if (sz + free >= needed) {
                    sizes.push(sz)
                }
                goThroughDir(obj[key])
            }
        }
    }

    goThroughDir(fileSystem)

    console.log(Math.min(...sizes))
}
