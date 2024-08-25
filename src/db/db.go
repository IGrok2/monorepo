package db

import (
	"log"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"packetware.net/backend/src/models"
)

func Init() *gorm.DB {
	dbURL := "postgres://default_user:ZltFe3bUhtI4f9QinmYCKfkbR50VdmLtGRJzuub7mKGWIZg1jP@104.129.132.111:32345/default"

	db, err := gorm.Open(postgres.Open(dbURL), &gorm.Config{})

	if err != nil {
		log.Fatalln(err)
	}

	db.AutoMigrate(&models.User{})

	return db
}
