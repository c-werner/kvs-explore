package server

import (
	"fmt"
	"github.com/c-werner/kvs-explore/golang/golang/kvs"
	"log"
	"net/http"
	"strings"

	"github.com/julienschmidt/httprouter"
)

type svc struct {
	k      kvs.Store
	router *httprouter.Router
}

const keyName = "key"

func New(k kvs.Store) *svc {
	s := &svc{
		k:      k,
		router: httprouter.New(),
	}

	s.router.GET("/", s.Count)
	s.router.GET("/list", s.List)
	s.router.GET("/k/:"+keyName, s.GetOrUpdate)
	s.router.GET("/d/:"+keyName, s.Del)
	s.router.GET("/h/:"+keyName, s.Has)

	return s
}

func (s *svc) ArgError(w http.ResponseWriter, msg string) {
	http.Error(w, msg, 400)
}

func (s *svc) NotFoundError(w http.ResponseWriter, msg string) {
	http.Error(w, msg, 404)
}

func (s *svc) Resp(w http.ResponseWriter, message string) {
	_, err := fmt.Fprint(w, message)
	if err != nil {
		log.Fatal(err)
	}
}

func (s *svc) RespBool(w http.ResponseWriter, b bool) {
	s.Resp(w, fmt.Sprintf("%t", b))
}

func (s *svc) Count(w http.ResponseWriter, r *http.Request, p httprouter.Params) {
	s.Resp(w, fmt.Sprintf("Request number: %d", s.k.Begin().Count()))
}

func (s *svc) GetOrUpdate(w http.ResponseWriter, r *http.Request, p httprouter.Params) {
	key := p.ByName(keyName)
	if key == "" {
		s.ArgError(w, "no key specified")
		return
	}

	value := r.URL.Query().Get("v")
	if value != "" {
		s.k.Begin().Set(key, value)
	} else {
		value = s.k.Begin().Get(key)
	}

	if value == "" {
		s.NotFoundError(w, fmt.Sprintf("'%s' not found", key))
		return
	}

	s.Resp(w, value)
}

func (s *svc) Has(w http.ResponseWriter, r *http.Request, p httprouter.Params) {
	key := p.ByName(keyName)
	if key == "" {
		s.ArgError(w, "no key specified")
		return
	}

	ok := s.k.Begin().Has(key)
	s.RespBool(w, ok)
}

func (s *svc) Del(w http.ResponseWriter, r *http.Request, p httprouter.Params) {
	key := p.ByName(keyName)
	if key == "" {
		s.ArgError(w, "no key specified")
		return
	}

	ok := s.k.Begin().Del(key)
	s.RespBool(w, ok)
}

func (s *svc) List(w http.ResponseWriter, r *http.Request, p httprouter.Params) {
	keys := s.k.Begin().Keys()
	s.Resp(w, strings.Join(keys, "\n"))
}

func (s *svc) ListenAndServe(addr string) error {
	return http.ListenAndServe(addr, s.router)
}
