pipeline {
    agent any

    tools {
        nodejs "nodejs"
    }

    environment {
        def TAG = sh(script: "date +%Y.%m.%d-`git log -n 1 --pretty=format:'%h'`", returnStdout: true).trim()
    }

    stages {
        stage('Checkout repo') {
            steps {
                checkout scmGit(
                    branches: [[name: '*/main']],
                    extensions: [],
                    userRemoteConfigs: [[url: 'https://github.com/El-Mendez/QuickPolls/']]
	        )
	    }
        }
        stage('UI Pipeline') {
            when {
                changeset "ui/**/*"
            }
            stages {
                stage("Install Dependencies") {
                    steps {
                        sh 'cd ui && npm install'
                    }
                }
                stage('Build') {
                    steps {
                        sh 'cd ui && npm run build'
                    }
                }
                stage('Test') {
                    steps {
                        sh 'cd ui && npm run test'
                    }
                }
            }
        }
        stage('API Pipeline') {
            when {
                changeset "api/**/*"
            }
            stages {
                stage("Install Dependencies") {
                    steps {
                        sh 'cd api && cargo install --path .'
                    }
                }
                stage('Build') {
                    steps {
                        sh 'cd api && cargo build'
                    }
                }
                stage('Test') {
                    steps {
                        sh 'cd api && cargo test'
                    }
                }
            }
        }

        stage('Deploy') {
            when {
                anyOf { changeset "api/**/*"; changeset "ui/**/*"; changeset "Dockerfile" }
            }
            environment {
                ANSIBLE_HOST_KEY_CHECKING="False"
            }

            steps {
                withCredentials([usernamePassword(credentialsId: 'docker', usernameVariable: 'DOCKER_USR', passwordVariable: 'DOCKER_PSW')]) {
                    sh '''
                        docker build -t $DOCKER_USR/quick-poll:latest -t $DOCKER_USR/quick-poll:$TAG .
                        docker login -u $DOCKER_USR -p $DOCKER_PSW
                        docker push $DOCKER_USR/quick-poll:$TAG
                        docker push $DOCKER_USR/quick-poll:latest
                        docker logout
                        docker image rm $DOCKER_USR/quick-poll:latest $DOCKER_USR/quick-poll:$TAG
                    '''
                }

                withCredentials([
                    usernamePassword(credentialsId: 'docker', usernameVariable: 'DOCKER_USR', passwordVariable: 'DOCKER_PSW'),
                    string(credentialsId: 'db_uri', variable: 'DB_URI')
                ]) {
                    ansiblePlaybook(
                        playbook: './devops/playbooks/pipelines/deploy.yml',
                        credentialsId: 'app_server_ssh',
                        inventory: '$APP_SERVER_IP,',
                        extraVars: [
                            db_uri: '$DB_URI',
                            app_image: '$DOCKER_USR/quick-poll:$TAG'
                        ]
                    )
                }
            }
        }
    }
}