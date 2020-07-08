package main

import (
	"log"

	"github.com/c-werner/kvs-explore/golang/golang/kvs"
	"github.com/c-werner/kvs-explore/golang/golang/server"
)

func main() {
	var (
		k = kvs.New()
		s = server.New(k)
	)

	log.Fatal(s.ListenAndServe(":8080"))
}
