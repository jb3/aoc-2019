package main;

import (
  "bufio"
  "fmt"
  "log"
  "strconv"
  "os"
  "math"
)

func main() {
  masses := getInput()

  part_1 := 0.0
  part_2 := 0.0

  for _, mass := range masses {
	   fuel := calculateFuel(mass)

     part_1 += fuel
     part_2 += fuel

     last := fuel

     for {
       fuel_of_fuel := calculateFuel(last)

       if fuel_of_fuel <= 0 {
         break
       }

       part_2 += fuel_of_fuel
       last = fuel_of_fuel
     }
  }

  fmt.Printf("Part 1: %d\n", int(part_1))
  fmt.Printf("Part 2: %d\n", int(part_2))
}

func calculateFuel(fuel float64) float64 {
  return math.Floor(fuel / 3) - 2
}

func getInput() []float64 {
  file, err := os.Open("../input")
  if err != nil {
    log.Fatal(err)
  }

  defer file.Close()

  scanner := bufio.NewScanner(file)
  var masses []float64

  for scanner.Scan() {
    f, err := strconv.ParseFloat(scanner.Text(), 64)

    if err != nil {
      log.Fatal(err)
    }

    masses = append(masses, f)
  }

  return masses
}
