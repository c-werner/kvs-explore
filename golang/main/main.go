package main

import (
	"github.com/c-werner/kvs-explore/golang/golang/kvs"
	"github.com/c-werner/kvs-explore/golang/golang/server"
	"log"
)

func main() {
	var (
		k = kvs.New()
		s = server.New(k)
	)

	log.Fatal(s.ListenAndServe(":8080"))
}