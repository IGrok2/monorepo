package models

import (
	"time"

	"gorm.io/gorm"
)

type Role int

const (
	Standard Role = iota
	Admin
	Owner
)

type Member struct {
	gorm.Model
	ID                 uint `gorm:"primaryKey"`
	UserID             uint `gorm:"not null"`
	User               User
	ProjectID          uint `gorm:"not null"`
	Project            Project
	Role               string `gorm:"type:ENUM('MEMBER', 'ADMIN', 'OWNER')"`
	InvitedAt          time.Time
	AcceptedInvitation bool
	AcceptedAt         time.Time
}

type Secret struct {
	gorm.Model
	ID        uint `gorm:"primaryKey"`
	ProjectID uint `gorm:"not null"`
	Project   Project
	Name      string
	Type      string `gorm:"type:ENUM('PLAINTEXT', 'CERTIFICATE', 'KEY')"`
	Value     string
}

type Environment struct {
	gorm.Model
	ID        uint `gorm:"primaryKey"`
	ProjectID uint `gorm:"not null"`
	Project   Project
	Name      string
	Apps      []string `gorm:"type:text[]"`
	Volumes   []string `gorm:"type:text[]"`
	Secrets   []Secret
}

type ContainerRegistry struct {
	gorm.Model
	ID        uint `gorm:"primaryKey"`
	ProjectID uint `gorm:"not null"`
	Project   Project
	Platform  string `gorm:"type:ENUM('DOCKER_HUB', 'GITHUB_REGISTRY', 'PRIVATE_REGISTRY')"`
	URL       string
	Username  string
	Password  string
}

type Notification struct {
	gorm.Model
	ID        uint `gorm:"primaryKey"`
	ProjectID uint `gorm:"not null"`
	Project   Project
	Message   string
	Link      string
	Seen      bool
}

type Activity struct {
	gorm.Model
	ID        uint `gorm:"primaryKey"`
	ProjectID uint `gorm:"not null"`
	Project   Project
	UserID    string
	UserName  string
	Type      string `gorm:"type:ENUM('DOMAIN', 'APP', 'PROJECT', 'VOLUME')"`
	Resource  string
	Message   string
}

type Project struct {
	gorm.Model
	ID                  uint `gorm:"primaryKey"`
	Members             []Member
	Name                string
	Environments        []Environment
	ContainerRegistries []ContainerRegistry
	Notifications       []Notification
	Activity            []Activity
}
