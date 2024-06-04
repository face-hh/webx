package config

import (
	"encoding/json"
	"io/ioutil"
	"log"
	"sync"
)

type APIKeyConfig struct {
	Owner     string `json:"owner"`
	RateLimit int    `json:"ratelimit"`
}

type Config struct {
	AIKey      string                  `json:"aikey"`
	Bind       string                  `json:"bind"`
	Mongo      string                  `json:"mongo"`
	Cloudflare bool                    `json:"cloudflare"`
	APIKeys    map[string]APIKeyConfig `json:"apiKeys"`
}

var (
	instance *Config
	once     sync.Once
)

func GetConfig() *Config {
	once.Do(func() {
		instance = &Config{}
		err := instance.loadConfig("./config.json")
		if err != nil {
			log.Fatalf("Error loading configuration: %v", err)
		}
	})
	return instance
}

func (c *Config) loadConfig(filePath string) error {
	data, err := ioutil.ReadFile(filePath)
	if err != nil {
		return err
	}
	return json.Unmarshal(data, c)
}
