const fs = require('fs')

const moves = fs.readFileSync('input.txt', 'utf8').trim().split('\n')

let visitedLocations = []

let headX = 0,
    headY = 0,
    tailX = 0,
    tailY = 0

function includes(array, array2) {
    let incl = false
    for (let i = 0; i < array.length; i++) {
        if (array[i][0] == array2[0] && array[i][1] == array2[1]) {
            incl = true
        }
    }
    return incl
}

//silver
function dbhat() {
    return Math.sqrt(Math.pow(headX - tailX, 2) + Math.pow(headY - tailY, 2))
}

for (let move of moves) {
    let [direction, distance] = move.split(' ')

    switch (direction) {
        case 'R': {
            for (let i = 0; i < parseInt(distance); i++) {
                headX++
                if (dbhat() >= 2) {
                    visitedLocations.push([tailX, tailY])
                    tailX = headX - 1
                    tailY = headY
                }
            }
            break
        }
        case 'U': {
            for (let i = 0; i < parseInt(distance); i++) {
                headY++
                if (dbhat() >= 2) {
                    visitedLocations.push([tailX, tailY])
                    tailX = headX
                    tailY = headY - 1
                }
            }
            break
        }
        case 'D': {
            for (let i = 0; i < parseInt(distance); i++) {
                headY--
                if (dbhat() >= 2) {
                    visitedLocations.push([tailX, tailY])
                    tailX = headX
                    tailY = headY + 1
                }
            }
            break
        }
        case 'L': {
            for (let i = 0; i < parseInt(distance); i++) {
                headX--
                if (dbhat() >= 2) {
                    visitedLocations.push([tailX, tailY])
                    tailX = headX + 1
                    tailY = headY
                }
            }
            break
        }
    }
}
visitedLocations.push([tailX, tailY])

{
    let count = 0
    let seen = []

    for (let vis of visitedLocations) {
        if (!includes(seen, vis)) {
            seen.push(vis)
            count++
        }
    }

    console.log(count)
}

//gold
let points = []
for (let i = 0; i < 10; i++) {
    points.push([11, 5])
}
visitedLocations = []

function dbhatspec(x1, y1, x2, y2) {
    return Math.sqrt(Math.pow(x1 - x2, 2) + Math.pow(y1 - y2, 2))
}

function vizualize() {
    for (let y = 21 - 1; y >= 0; y--) {
        for (let x = 0; x < 26; x++) {
            let found = false
            for (let l = 0; l < 10; l++) {
                if (points[l][0] == x && points[l][1] == y) {
                    process.stdout.write(l.toString())
                    found = true
                    break
                }
            }
            if (!found) {
                process.stdout.write('X')
            }
        }
        process.stdout.write('\n')
    }
    process.stdout.write('\n')
}

let lastX = 0,
    lastY = 0,
    lastX2 = 0,
    lastY2 = 0

vizualize()
for (let move of moves) {
    let [direction, distance] = move.split(' ')

    switch (direction) {
        case 'R': {
            for (let i = 0; i < parseInt(distance); i++) {
                points[0][0]++
                for (let l = 1; l < points.length; l++) {
                    if (dbhatspec(points[l][0], points[l][1], points[l - 1][0], points[l - 1][1]) >= 2) {
                        if (l == 9) {
                            visitedLocations.push([points[l][0], points[l][1]])
                        }

                        if (l == 1) {
                            lastX = points[l][0]
                            lastY = points[l][1]

                            points[l][0] = points[l - 1][0] - 1
                            points[l][1] = points[l - 1][1]
                        } else {
                            lastX2 = points[l][0]
                            lastY2 = points[l][1]

                            points[l][0] = lastX
                            points[l][1] = lastY

                            lastX = lastX2
                            lastY = lastY2
                        }
                    }
                }
            }
            vizualize()
            break
        }
        case 'U': {
            for (let i = 0; i < parseInt(distance); i++) {
                points[0][1]++
                for (let l = 1; l < points.length; l++) {
                    if (dbhatspec(points[l][0], points[l][1], points[l - 1][0], points[l - 1][1]) >= 2) {
                        if (l == 9) {
                            visitedLocations.push([points[l][0], points[l][1]])
                        }
                        if (l == 1) {
                            lastX = points[l][0]
                            lastY = points[l][1]

                            points[l][0] = points[l - 1][0]
                            points[l][1] = points[l - 1][1] - 1
                        } else {
                            lastX2 = points[l][0]
                            lastY2 = points[l][1]

                            points[l][0] = lastX
                            points[l][1] = lastY

                            lastX = lastX2
                            lastY = lastY2
                        }
                    }
                }
            }
            vizualize()
            break
        }
        case 'D': {
            for (let i = 0; i < parseInt(distance); i++) {
                points[0][1]--
                for (let l = 1; l < points.length; l++) {
                    if (dbhatspec(points[l][0], points[l][1], points[l - 1][0], points[l - 1][1]) >= 2) {
                        if (l == 9) {
                            visitedLocations.push([points[l][0], points[l][1]])
                        }
                        if (l == 1) {
                            lastX = points[l][0]
                            lastY = points[l][1]

                            points[l][0] = points[l - 1][0]
                            points[l][1] = points[l - 1][1] + 1
                        } else {
                            lastX2 = points[l][0]
                            lastY2 = points[l][1]

                            points[l][0] = lastX
                            points[l][1] = lastY

                            lastX = lastX2
                            lastY = lastY2
                        }
                    }
                }
            }
            vizualize()
            break
        }
        case 'L': {
            for (let i = 0; i < parseInt(distance); i++) {
                points[0][0]--
                for (let l = 1; l < points.length; l++) {
                    if (dbhatspec(points[l][0], points[l][1], points[l - 1][0], points[l - 1][1]) >= 2) {
                        if (l == 9) {
                            visitedLocations.push([points[l][0], points[l][1]])
                        }
                        if (l == 1) {
                            lastX = points[l][0]
                            lastY = points[l][1]

                            points[l][0] = points[l - 1][0] + 1
                            points[l][1] = points[l - 1][1]
                        } else {
                            lastX2 = points[l][0]
                            lastY2 = points[l][1]

                            points[l][0] = lastX
                            points[l][1] = lastY

                            lastX = lastX2
                            lastY = lastY2
                        }
                    }
                }
            }
            vizualize()
            break
        }
    }
}

console.log(visitedLocations)

{
    let count = 0
    let seen = []

    for (let vis of visitedLocations) {
        if (!includes(seen, vis)) {
            seen.push(vis)
            count++
        }
    }

    console.log(count)
}
