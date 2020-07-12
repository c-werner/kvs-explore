package server

import (
	"encoding/json"
	"log"
	"net/http"
)

type jsonStruct interface{}

func (s *svc) Resp(w http.ResponseWriter, js jsonStruct) {
	b, err := json.Marshal(js)
	if err != nil {
		log.Fatal(err)
	}

	_, err = w.Write(b)
	if err != nil {
		log.Fatal(err)
	}
}

func (s *svc) Err(w http.ResponseWriter, js jsonStruct, code int) {
	b, err := json.Marshal(js)
	if err != nil {
		log.Fatal(err)
	}

	http.Error(w, string(b), code)
}

func (s *svc) ArgError(w http.ResponseWriter, js jsonStruct) {
	s.Err(w, js, http.StatusBadRequest)
}

func (s *svc) NotFoundError(w http.ResponseWriter, js jsonStruct) {
	s.Err(w, js, http.StatusNotFound)
}

type ErrResp struct {
	error string
}

type CountResp struct {
	RequestNumber uint64 `json:"request_number"`
}

type ListResp struct {
	Keys []string `json:"keys"`
}

type GetOrUpdateResp struct {
	Key   string  `json:"key"`
	Value *string `json:"value"`
}

type BoolResp struct {
	Key string `json:"key"`
	Ok  bool   `json:"ok"`
}
