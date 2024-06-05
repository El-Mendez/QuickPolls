output "app_server_public_ip" {
  description = "The public IP of the app server"
  value = aws_instance.application_server.public_ip
}

output "monitoring_server_public_ip" {
  description = "The public IP of the app server"
  value = aws_instance.monitoring_server.public_ip
}

output "db_address" {
  description = "The password of the RDS"
  value = "postgres://${var.db_username}:${var.db_password}@${aws_db_instance.db.address}:5432/${aws_db_instance.db.db_name}"
}
