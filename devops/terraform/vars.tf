variable "access_key" {
  type = string
  description = "AWS Access Key"
}

variable "secret_key" {
  type = string
  description = "AWS Secret Key"
}

variable "personal_ssh_key" {
  type = string
  description = "SSH Key with root access"
}

variable "personal_private_ssh_key" {
  type = string
  description = "SSH Key with root access"
}

variable "db_username" {
  type = string
  description = "SSH Key with root access"
}

variable "db_name" {
  type = string
  description = "The username of the RDS Database"
}

variable "db_password" {
  type = string
}
