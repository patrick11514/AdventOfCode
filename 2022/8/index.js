const fs = require('fs')

const treeLine = fs
    .readFileSync('input.txt', 'utf8')
    .trim()
    .split('\n')
    .map((line) => line.split(''))

//silver
function checkVisibility(treeLineIndex, treeIndex, side) {
    if (side === 'left') {
        for (let i = treeIndex - 1; i >= 0; i--) {
            if (getValue(treeLineIndex, i) >= getValue(treeLineIndex, treeIndex)) return false
        }
    } else if (side === 'right') {
        for (let i = treeIndex + 1; i < treeLine[treeLineIndex].length; i++) {
            if (getValue(treeLineIndex, i) >= getValue(treeLineIndex, treeIndex)) return false
        }
    } else if (side === 'top') {
        for (let i = treeLineIndex - 1; i >= 0; i--) {
            if (getValue(i, treeIndex) >= getValue(treeLineIndex, treeIndex)) return false
        }
    } else if (side === 'bottom') {
        for (let i = treeLineIndex + 1; i < treeLine.length; i++) {
            if (getValue(i, treeIndex) >= getValue(treeLineIndex, treeIndex)) return false
        }
    }

    return true
}

//true = visible
function checkSides(treeLineIndex, treeIndex) {
    if (treeIndex - 1 < 0) return true
    if (treeIndex + 1 >= treeLine[treeLineIndex].length) return true
    if (treeLineIndex - 1 < 0) return true
    if (treeLineIndex + 1 >= treeLine.length) return true

    return (
        checkVisibility(treeLineIndex, treeIndex, 'left') ||
        checkVisibility(treeLineIndex, treeIndex, 'right') ||
        checkVisibility(treeLineIndex, treeIndex, 'top') ||
        checkVisibility(treeLineIndex, treeIndex, 'bottom')
    )
}

function getValue(treeLineIndex, treeIndex) {
    return treeLine[treeLineIndex][treeIndex]
}

let visibleTrees = 0

for (let treeLineIndex in treeLine) {
    for (let treeIndex in treeLine[treeLineIndex]) {
        if (checkSides(parseInt(treeLineIndex), parseInt(treeIndex))) {
            visibleTrees++
        }
    }
}

console.log(visibleTrees)

//gold
function getVisibleDistance(treeLineIndex, treeIndex, side) {
    let distance = 0
    if (side === 'left') {
        for (let i = treeIndex - 1; i >= 0; i--) {
            if (getValue(treeLineIndex, i) < getValue(treeLineIndex, treeIndex)) distance++
            else {
                distance++
                break
            }
        }
    } else if (side === 'right') {
        for (let i = treeIndex + 1; i < treeLine[treeLineIndex].length; i++) {
            if (getValue(treeLineIndex, i) < getValue(treeLineIndex, treeIndex)) distance++
            else {
                distance++
                break
            }
        }
    } else if (side === 'top') {
        for (let i = treeLineIndex - 1; i >= 0; i--) {
            if (getValue(i, treeIndex) < getValue(treeLineIndex, treeIndex)) distance++
            else {
                distance++
                break
            }
        }
    } else if (side === 'bottom') {
        for (let i = treeLineIndex + 1; i < treeLine.length; i++) {
            if (getValue(i, treeIndex) < getValue(treeLineIndex, treeIndex)) distance++
            else {
                distance++
                break
            }
        }
    }

    return distance
}

function getScore(treeLineIndex, treeIndex) {
    return (
        getVisibleDistance(treeLineIndex, treeIndex, 'left') *
        getVisibleDistance(treeLineIndex, treeIndex, 'right') *
        getVisibleDistance(treeLineIndex, treeIndex, 'top') *
        getVisibleDistance(treeLineIndex, treeIndex, 'bottom')
    )
}

let maxVisibility = 0

for (let treeLineIndex in treeLine) {
    for (let treeIndex in treeLine[treeLineIndex]) {
        let score = getScore(parseInt(treeLineIndex), parseInt(treeIndex))
        //console.log(treeLineIndex, treeIndex)
        //console.log(score)
        if (score > maxVisibility) {
            maxVisibility = score
        }
    }
}

console.log(maxVisibility)
