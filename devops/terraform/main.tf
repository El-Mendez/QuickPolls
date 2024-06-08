provider "aws" {
  region     = "us-east-2"
  access_key = var.access_key
  secret_key = var.secret_key
}

resource "aws_key_pair" "personal_ssh_key" {
  key_name   = "personal_ssh_key"
  public_key = var.personal_ssh_key
}


resource "aws_vpc" "vpc" {
  cidr_block           = "10.0.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true
}
resource "aws_internet_gateway" "gateway" {
  vpc_id = aws_vpc.vpc.id
}
resource "aws_subnet" "public_subnet" {
  cidr_block = cidrsubnet(aws_vpc.vpc.cidr_block, 3, 1)
  vpc_id     = aws_vpc.vpc.id
}
resource "aws_route_table" "public_route" {
  vpc_id = aws_vpc.vpc.id
  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.gateway.id
  }
}
resource "aws_route_table_association" "rt_associate_public" {
  subnet_id      = aws_subnet.public_subnet.id
  route_table_id = aws_route_table.public_route.id
}


resource "aws_security_group" "app_sg" {
  name        = "app_sg"
  description = "For the webapp servers"
  vpc_id      = aws_vpc.vpc.id
}
resource "aws_vpc_security_group_egress_rule" "app_egress_all" {
  security_group_id = aws_security_group.app_sg.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "-1"
}
resource "aws_vpc_security_group_ingress_rule" "app_ingress_ssh" {
  security_group_id = aws_security_group.app_sg.id

  cidr_ipv4   = "0.0.0.0/0"
  ip_protocol = "tcp"
  from_port   = 22
  to_port     = 22
}
resource "aws_vpc_security_group_ingress_rule" "app_ingress_http" {
  security_group_id = aws_security_group.app_sg.id

  cidr_ipv4   = "0.0.0.0/0"
  ip_protocol = "tcp"
  from_port   = 80
  to_port     = 80
}
resource "aws_vpc_security_group_ingress_rule" "app_ingress_app_metrics" {
  security_group_id = aws_security_group.app_sg.id

  cidr_ipv4   = cidrsubnet(aws_vpc.vpc.cidr_block, 3, 1)
  ip_protocol = "tcp"
  from_port   = 3000
  to_port     = 3000
}
resource "aws_vpc_security_group_ingress_rule" "app_ingress_node_exporter_metrics" {
  security_group_id = aws_security_group.app_sg.id

  cidr_ipv4   = cidrsubnet(aws_vpc.vpc.cidr_block, 3, 1)
  ip_protocol = "tcp"
  from_port   = 9100
  to_port     = 9100
}

resource "aws_security_group" "monitoring_sg" {
  name        = "monitoring_sg"
  description = "The security group for the monitoring servers"
  vpc_id      = aws_vpc.vpc.id
}
resource "aws_vpc_security_group_egress_rule" "monitoring_egress_all" {
  security_group_id = aws_security_group.monitoring_sg.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "-1"
}
resource "aws_vpc_security_group_ingress_rule" "monitoring_ingress_ssh" {
  security_group_id = aws_security_group.monitoring_sg.id

  cidr_ipv4   = "0.0.0.0/0"
  ip_protocol = "tcp"
  from_port   = 22
  to_port     = 22
}
resource "aws_vpc_security_group_ingress_rule" "monitoring_ingress_grafana" {
  security_group_id = aws_security_group.monitoring_sg.id

  cidr_ipv4   = "0.0.0.0/0"
  ip_protocol = "tcp"
  from_port   = 3000
  to_port     = 3000
}

resource "aws_security_group" "db_sg" {
  name        = "db_sg"
  description = "The security group for the db servers"
}
resource "aws_vpc_security_group_ingress_rule" "db_ingress" {
  security_group_id = aws_security_group.db_sg.id

  cidr_ipv4   = "0.0.0.0/0"
  ip_protocol = "tcp"
  from_port   = 5432
  to_port     = 5432
}
resource "random_password" "db_password" {
  special = false
  length = 20
}


resource "aws_instance" "application_server" {
  ami                         = "ami-0f30a9c3a48f3fa79"
  instance_type               = "t2.micro"
  key_name                    = aws_key_pair.personal_ssh_key.id
  vpc_security_group_ids      = [aws_security_group.app_sg.id]
  subnet_id                   = aws_subnet.public_subnet.id
  associate_public_ip_address = true
  tags = {
    Name = "application_server"
  }
}
resource "null_resource" "prepare_application_server" {
  provisioner "remote-exec" {
    inline = ["echo 'ssh ready'"]
    connection {
      host = aws_instance.application_server.public_ip
      type = "ssh"
      user = "ubuntu"
      private_key = file(var.personal_private_ssh_key)
    }
  }

  provisioner "local-exec" {
    command = "ANSIBLE_HOST_KEY_CHECKING=False ansible-playbook -u ubuntu -i '${aws_instance.application_server.public_ip},' ../playbooks/appserver/main.yml"
  }
}
resource "null_resource" "install_application_server" {
  depends_on = [null_resource.prepare_application_server]
  provisioner "local-exec" {
    command = "ANSIBLE_HOST_KEY_CHECKING=False ansible-playbook -u ubuntu -i '${aws_instance.application_server.public_ip},'  -e 'db_uri=postgres://${var.db_username}:${random_password.db_password.result}@${aws_db_instance.db.address}:5432/${aws_db_instance.db.db_name}' ../playbooks/pipelines/deploy.yml"
  }
}

resource "aws_instance" "monitoring_server" {
  ami                         = "ami-0f30a9c3a48f3fa79"
  instance_type               = "t2.micro"
  key_name                    = aws_key_pair.personal_ssh_key.id
  vpc_security_group_ids      = [aws_security_group.monitoring_sg.id]
  subnet_id                   = aws_subnet.public_subnet.id
  associate_public_ip_address = true
  tags = {
    Name = "monitoring_server"
  }
}
resource "null_resource" "install_monitoring_server" {
  provisioner "remote-exec" {
    inline = ["echo 'ssh ready'"]
    connection {
      host = aws_instance.monitoring_server.public_ip
      type = "ssh"
      user = "ubuntu"
      private_key = file(var.personal_private_ssh_key)
    }
  }

  provisioner "local-exec" {
    command = "ANSIBLE_HOST_KEY_CHECKING=False ansible-playbook -u ubuntu -i '${aws_instance.monitoring_server.public_ip},' -e '{\"app_targets\":[\"${aws_instance.application_server.private_ip}\"]}' ../playbooks/monitoring/main.yml"
  }
}

resource "aws_db_instance" "db" {
  engine            = "postgres"
  instance_class    = "db.t3.micro"
  db_name           = var.db_name
  username          = var.db_username
  password          = random_password.db_password.result
  allocated_storage = 10
  performance_insights_enabled = false
  skip_final_snapshot = true
  publicly_accessible = true
  vpc_security_group_ids = [aws_security_group.db_sg.id]
}