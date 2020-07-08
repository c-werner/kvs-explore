package kvs

import (
	"sync"
	"sync/atomic"
)

type Store interface {
	Begin() *readyStore
}

type store struct {
	data    map[string]string
	counter uint64
	mutex   sync.RWMutex
}

type readyStore struct {
	*store
}

func New() *store {
	return &store{
		data: make(map[string]string),
	}
}

func (s *store) Begin() *readyStore {
	if s == nil {
		return nil
	}

	atomic.AddUint64(&s.counter, 1)

	return &readyStore{store: s}
}

func (s *readyStore) Count() uint64 {
	if s == nil {
		return 0
	}

	s.mutex.RLock()
	defer s.mutex.RUnlock()

	return s.counter
}

func (s *readyStore) Get(key string) string {
	if s == nil {
		return ""
	}

	s.mutex.RLock()
	defer s.mutex.RUnlock()

	value := s.data[key]
	return value
}

func (s *readyStore) Set(key, value string) {
	if s == nil {
		return
	}

	s.mutex.Lock()
	defer s.mutex.Unlock()

	s.data[key] = value
}

func (s *readyStore) Has(key string) bool {
	if s == nil {
		return false
	}

	s.mutex.RLock()
	defer s.mutex.RUnlock()

	_, ok := s.data[key]

	return ok
}

func (s *readyStore) Del(key string) bool {
	if s == nil {
		return false
	}

	s.mutex.Lock()
	defer s.mutex.Unlock()

	_, ok := s.data[key]
	delete(s.data, key)

	return ok
}

func (s *readyStore) Keys() []string {
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
