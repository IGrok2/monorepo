package models

type User struct {
	Id    int    `json:"id" gorm:"primaryKey"`
	Email string `json:"email"`
}
