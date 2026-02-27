package main

import (
	"log"

	"records-api/internal/api"

	"github.com/gin-gonic/gin"
)

func main() {
	// 1. Initialize data directory
	if err := api.InitDataDir(); err != nil {
		log.Fatalf("failed to initialize data directory: %v", err)
	}

	// 2. Setup Gin router
	router := gin.Default()

	// 3. Define routes
	v1 := router.Group("/api/v1")
	{
		v1.POST("/records", api.CreateRecord)
		v1.GET("/records/:uuid", api.GetRecordByUUID)
		v1.GET("/openapi.yaml", api.GetOpenAPI)
		v1.GET("/openapi.json", api.GetOpenAPIJSON)
	}

	// 4. Start server
	log.Println("Server starting on :8081...")
	if err := router.Run(":8081"); err != nil {
		log.Fatalf("failed to run server: %v", err)
	}
}
