package kvs

import (
	"sync"
	"sync/atomic"
)

type Store interface {
	Get(key string) string
	Set(key, value string)
	Has(key string) bool
	Del(key string) bool
	Incr() Store
	Count() uint64
	Keys() []string
}

type store struct {
	data    map[string]string
	counter uint64
	mutex   sync.RWMutex
}

func New() *store {
	return &store{
		data: make(map[string]string),
	}
}

func (s *store) Incr() Store {
	if s == nil {
		return nil
	}

	s.counter++
	atomic.AddUint64(&s.counter, 1)

	return s
}

func (s *store) Count() uint64 {
	if s == nil {
		return 0
	}

	s.mutex.RLock()
	defer s.mutex.RUnlock()

	return s.counter
}

func (s *store) Get(key string) string {
	if s == nil {
		return ""
	}

	s.mutex.RLock()
	defer s.mutex.RUnlock()

	value := s.data[key]
	return value
}

func (s *store) Set(key, value string) {
	if s == nil {
		return
	}

	s.mutex.Lock()
	defer s.mutex.Unlock()

	s.data[key] = value
}

func (s *store) Has(key string) bool {
	if s == nil {
		return false
	}

	s.mutex.RLock()
	defer s.mutex.RUnlock()

	_, ok := s.data[key]

	return ok
}

func (s *store) Del(key string) bool {
	if s == nil {
		return false
	}

	s.mutex.Lock()
	defer s.mutex.Unlock()

	_, ok := s.data[key]
	delete(s.data, key)

	return ok
}

func (s *store) Keys() []string {
	var keys []string

	if s == nil {
		return keys
	}
	s.mutex.RLock()
	defer s.mutex.RUnlock()

	for k := range s.data {
		keys = append(keys, k)
	}

	return keys
}
