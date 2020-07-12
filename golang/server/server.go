package server

import (
	"github.com/c-werner/kvs-explore/golang/golang/kvs"
	"github.com/julienschmidt/httprouter"
	"net/http"
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

func (s *svc) Count(w http.ResponseWriter, r *http.Request, p httprouter.Params) {
	s.Resp(w, CountResp{s.k.Begin().Count()})
}

func (s *svc) GetOrUpdate(w http.ResponseWriter, r *http.Request, p httprouter.Params) {
	key := p.ByName(keyName)
	if key == "" {
		s.ArgError(w, ErrResp{"no key specified"})
		return
	}

	value := r.URL.Query().Get("v")
	if value != "" {
		s.k.Begin().Set(key, value)
	} else {
		value = s.k.Begin().Get(key)
	}

	if value == "" {
		s.NotFoundError(w, GetOrUpdateResp{Key: key})
		return
	}

	s.Resp(w, GetOrUpdateResp{Key: key, Value: &value})
}

func (s *svc) Has(w http.ResponseWriter, r *http.Request, p httprouter.Params) {
	key := p.ByName(keyName)
	if key == "" {
		s.ArgError(w, "no key specified")
		return
	}

	ok := s.k.Begin().Has(key)
	s.Resp(w, BoolResp{Key: key, Ok: ok})
}

func (s *svc) Del(w http.ResponseWriter, r *http.Request, p httprouter.Params) {
	key := p.ByName(keyName)
	if key == "" {
		s.ArgError(w, "no key specified")
		return
	}

	ok := s.k.Begin().Del(key)
	s.Resp(w, BoolResp{Key: key, Ok: ok})
}

func (s *svc) List(w http.ResponseWriter, r *http.Request, p httprouter.Params) {
	keys := s.k.Begin().Keys()
	s.Resp(w, ListResp{Keys: keys})
}

func (s *svc) ListenAndServe(addr string) error {
	return http.ListenAndServe(addr, s.router)
}
