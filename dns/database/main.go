package database

import (
	"context"
	"log"
	"time"

	"dns/config"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var (
	// Db will hold the reference to the database collection.
	Db     *mongo.Collection
	Client = *&mongo.Client{}
)

func Initialize() {

	conf := config.GetConfig()

	client, err := mongo.NewClient(options.Client().ApplyURI(conf.Mongo))
	if err != nil {
		log.Fatalf("Failed to create client: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	err = client.Connect(ctx)
	if err != nil {
		log.Fatalf("Failed to connect to database: %v", err)
	}

	if err = client.Ping(ctx, nil); err != nil {
		log.Fatalf("Failed to ping database: %v", err)
	}

	Db = client.Database("DNS").Collection("domains")
}

func Disconnect() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	if err := Client.Disconnect(ctx); err != nil {
		log.Fatalf("Failed to disconnect from database: %v", err)
	}
}
