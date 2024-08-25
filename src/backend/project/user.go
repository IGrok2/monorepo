package models // packetware.net/backend/src/models

import (
	"gorm.io/gorm"
)

// / A user in the Packetware API.
type User struct {
	gorm.Model
	ID    uint `gorm:"primaryKey"`
	Name  string
	Email string `gorm:"size:100;uniqueIndex;not null"`

	StripeCustomerId string
}
