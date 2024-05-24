package main

import (
	"fmt"
	"interpreter/repl"
	"os"
	"os/user"
)

func main() {
	user, err := user.Current()
	if err != nil {
		panic(err)
	}
	greetings, err := fmt.Printf("Hello %s! This is the Monkey programming language!", user.Username)
	if err != nil {
		panic(err)
	}
	fmt.Println(greetings)
	fmt.Println("Feel free to type in commands")
	repl.Start(os.Stdin, os.Stdout)
}
