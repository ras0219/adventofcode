package main

import (
    "os"
    "log"
    "fmt"
    "bufio"
    "strconv"
)

func recfuel(a int) int {
    fuel := a / 3 - 2
    if fuel > 0 {
        return fuel + recfuel(fuel)
    } else {
        return 0
    }
}

func main() {
    file, err := os.Open("day1/day1.txt")
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()

    total_part1 := 0
    total_part2 := 0
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        v, err := strconv.Atoi(scanner.Text())
        if err != nil {
            log.Fatal(err)
        }
        total_part1 += v / 3 - 2
        total_part2 += recfuel(v)
    }
    fmt.Println("Part1 =", total_part1)
    fmt.Println("Part2 =", total_part2)
}
