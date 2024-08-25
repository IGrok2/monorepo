package handlers

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"packetware.net/backend/src/models"
)

func (h handler) GetAllUsers(c *gin.Context) {
	var users []models.User

	if result := h.DB.Find(&users); result.Error != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": result.Error.Error()})
		return
	}

	c.JSON(http.StatusOK, users)
}
