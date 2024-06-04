package security

import (
	"sync"
)

// DomainInfo stores information about a domain
type DomainInfo struct {
	Name string
	IP   string
}

// DomainStore holds the last 10 domain entries
type DomainStore struct {
	Domains map[string]string
	Keys    []string
	Lock    sync.Mutex
	MaxSize int
}

var Store *DomainStore

// InitializeStore sets up the domain store with a specified size limit
func InitializeStore(size int) {
	Store = &DomainStore{
		Domains: make(map[string]string),
		Keys:    make([]string, 0, size),
		MaxSize: size,
	}
}

// AddDomain adds a new domain and its IP to the store, maintaining the size limit
func (ds *DomainStore) AddDomain(name, ip string) {
	ds.Lock.Lock()
	defer ds.Lock.Unlock()

	// Check if already exists, if so, just update the IP
	if _, exists := ds.Domains[name]; exists {
		ds.Domains[name] = ip
		return
	}

	// Check if the store is full
	if len(ds.Keys) >= ds.MaxSize {
		// Remove the oldest item
		oldestKey := ds.Keys[0]
		delete(ds.Domains, oldestKey)
		ds.Keys = ds.Keys[1:]
	}

	// Add the new domain to the map and the key to the slice
	ds.Domains[name] = ip
	ds.Keys = append(ds.Keys, name)
}
