package main

import (
	"io"
	"log"
	"net/http"
	"os"

	"dns/config"
	"dns/database"
	"dns/domain"
	"dns/home"
	"dns/lookup"
	"dns/security"

	"github.com/gorilla/mux"
)

func main() {
	defer log.Printf("Shutting down....")
	conf := config.GetConfig()

	logFile, err := os.OpenFile("app.log", os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0666)
	if err != nil {
		log.Fatalf("error opening file: %v", err)
	}
	defer logFile.Close()

	mw := io.MultiWriter(os.Stdout, logFile)
	log.SetOutput(mw)

	database.Initialize()
	go security.Init()
	defer database.Disconnect()

	security.InitializeStore(10)

	r := mux.NewRouter()
	r.HandleFunc("/", home.Handler).Methods("GET")
	r.HandleFunc("/domain", domain.Register(conf.Cloudflare)).Methods("POST")
	r.HandleFunc("/domainapi/{key}", domain.RegisterAPI(conf.APIKeys)).Methods("POST")
	r.HandleFunc("/domain/{name}/{tld}", lookup.Specific).Methods("GET")
	r.HandleFunc("/domain/{key}", domain.Update).Methods("PUT")
	r.HandleFunc("/domain/{id}", domain.Delete).Methods("DELETE")
	r.HandleFunc("/domains", lookup.All).Methods("GET")
	r.HandleFunc("/tlds", lookup.Tlds).Methods("GET")

	log.Printf("Server running at http://%s\n", conf.Bind)
	log.Fatal(http.ListenAndServe(conf.Bind, r))
}
