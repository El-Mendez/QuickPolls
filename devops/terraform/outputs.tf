output "app_server_public_ip" {
  description = "The public IP of the app server"
  value = aws_instance.application_server.public_ip
}

output "app_server_private_ip" {
  description = "The private IP of the app server"
  value = aws_instance.application_server.private_ip
}

output "monitoring_server_public_ip" {
  description = "The public IP of the app server"
  value = aws_instance.monitoring_server.public_ip
}
