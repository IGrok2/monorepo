package main // packetware.net/backend/src

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

var router = gin.Default()

func main() {
	// db := db.Init()
	// h := handlers.New(db)

	println("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")

	router.GET("/ping", func(context *gin.Context) {
		context.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})

	router.Run() // listen and serve on 0.0.0.0:8080
}
