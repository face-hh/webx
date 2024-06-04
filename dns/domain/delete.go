package domain

import (
	"context"
	"dns/database"
	"net/http"

	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/bson"
)

func Delete(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	key := vars["id"]

	result, err := database.Db.DeleteOne(context.Background(), bson.M{"secret_key": key})
	if err != nil || result.DeletedCount == 0 {
		http.Error(w, "Failed to delete domain or domain not found", http.StatusNotFound)
		return
	}

	w.WriteHeader(http.StatusOK)
}
