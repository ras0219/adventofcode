package main

import (
    "os"
    "log"
    "fmt"
    "bufio"
    "strconv"
    "strings"
)
func main() {
    file, err := os.Open("day2/day2.txt")
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()

    var mem_init []int
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        toks := strings.Split(scanner.Text(), ",")
        mem_init = make([]int, len(toks))
        for i,tok := range toks {
        v, err := strconv.Atoi(tok)
        mem_init[i] = v
        if err != nil {
            log.Fatal(err)
        }
    }}
    mem := make([]int, len(mem_init))
    for arg1 := 0; arg1 < 100; arg1++ {
        for arg2 := 0; arg2 < 100; arg2++ {
            copy(mem, mem_init)
            mem[1] = arg1
            mem[2] = arg2

            ip := 0
            loop:
            for {
                switch mem[ip] {
                case 1:
                    mem[mem[ip+3]] = mem[mem[ip+2]] +
                      mem[mem[ip+1]]
                case 2:
                    mem[mem[ip+3]] = mem[mem[ip+2]] *
                      mem[mem[ip+1]]
                default:
                    break loop
                }
                ip += 4
            }
            if mem[0] == 19690720 {
              fmt.Println("hello!", arg1, arg2, mem[0])
            }
        }
    }
}
