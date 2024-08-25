package main

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
	"packetware.net/backend/src/db"
	"packetware.net/backend/src/handlers"
)

var router = gin.Default()

func AuthMiddleware() gin.HandlerFunc {
	return func(context *gin.Context) {
		context.Next()
		fmt.Printf("Auth request processed")
	}
}

func main() {
	db := db.Init()
	h := handlers.New(db)

	router.Use(AuthMiddleware())

	router.GET("/ping", func(context *gin.Context) {
		context.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})

	router.GET("/users", h.GetAllUsers)

	router.Run() // listen and serve on 0.0.0.0:8080
}
