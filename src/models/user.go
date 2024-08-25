package models // packetware.net/backend/src/models

import (
	"database/sql"
	"time"
)

// / A user in the Packetware API.
type User struct {
	/// Basic information
	ID    uint `gorm:"primaryKey"`
	Name  string
	Email string `gorm:"size:100;uniqueIndex;not null"`
	// Subsections
	Stripe StripeInfo `gorm:"embedded"`
}

// / A users' information in Stripe
type StripeInfo struct {
	/// Stripe ID
	ID         string
	NextCharge sql.NullTime
	ToCharge   sql.NullInt32
	Plans      []StripePlan
	Payments   []PastPayments
}

// / The plan
type StripePlan struct {
	Domain string
	Plan   uint
}

type PastPayments struct {
	Name   string
	Id     string
	amount uint
	date   time.Time
}
