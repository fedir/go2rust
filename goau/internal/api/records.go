package api

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/goccy/go-yaml"
	"github.com/google/uuid"
)

const DataDir = "./data"

// StoredRecord represents the structure of the JSON file saved on disk
type StoredRecord struct {
	UUID      string      `json:"uuid"`
	TraceID   string      `json:"trace_id"`
	Timestamp time.Time   `json:"timestamp"`
	Payload   interface{} `json:"payload"`
}

// InitDataDir ensures the storage directory exists
func InitDataDir() error {
	if _, err := os.Stat(DataDir); os.IsNotExist(err) {
		return os.MkdirAll(DataDir, 0755)
	}
	return nil
}

// CreateRecord handles POST /api/v1/records
func CreateRecord(c *gin.Context) {
	traceID := c.GetHeader("X-Trace-ID")
	if traceID == "" {
		traceID = uuid.New().String()
	}

	var payload interface{}
	if err := c.ShouldBindJSON(&payload); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "invalid JSON payload"})
		return
	}

	newUUID := uuid.New().String()
	record := StoredRecord{
		UUID:      newUUID,
		TraceID:   traceID,
		Timestamp: time.Now().UTC(),
		Payload:   payload,
	}

	filePath := filepath.Join(DataDir, fmt.Sprintf("%s.json", newUUID))
	fileData, err := json.MarshalIndent(record, "", "  ")
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "failed to encode record"})
		return
	}

	if err := os.WriteFile(filePath, fileData, 0644); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "failed to save record"})
		return
	}

	c.JSON(http.StatusCreated, gin.H{
		"status": "success",
		"uuid":   newUUID,
	})
}

// GetRecordByUUID handles GET /api/v1/records/{uuid}
func GetRecordByUUID(c *gin.Context) {
	id := c.Param("uuid")

	if _, err := uuid.Parse(id); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "invalid uuid format"})
		return
	}

	filePath := filepath.Join(DataDir, fmt.Sprintf("%s.json", id))

	if _, err := os.Stat(filePath); os.IsNotExist(err) {
		c.JSON(http.StatusNotFound, gin.H{"error": "record not found"})
		return
	}

	fileData, err := os.ReadFile(filePath)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "failed to read record"})
		return
	}

	var record StoredRecord
	if err := json.Unmarshal(fileData, &record); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "corrupted record data"})
		return
	}

	c.JSON(http.StatusOK, record)
}

// GetOpenAPI handles GET /api/v1/openapi.yaml
func GetOpenAPI(c *gin.Context) {
	content, err := os.ReadFile("openapi.yaml")
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "failed to read openapi specification"})
		return
	}
	c.Data(http.StatusOK, "text/yaml", content)
}

// GetOpenAPIJSON handles GET /api/v1/openapi.json
func GetOpenAPIJSON(c *gin.Context) {
	yamlContent, err := os.ReadFile("openapi.yaml")
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "failed to read openapi specification"})
		return
	}

	var data interface{}
	if err := yaml.Unmarshal(yamlContent, &data); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "failed to parse yaml"})
		return
	}

	c.JSON(http.StatusOK, data)
}
