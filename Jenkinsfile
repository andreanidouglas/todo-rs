pipeline {
        agent {
            label 'rust'
        }

        environment {
                DOCKER_BUILD = 'true'
                GITHUB_TOKEN = credentials('github-token')
                IMAGE_NAME='andreanidouglas/todo-rs'
                IMAGE_VERSION='latest'
        }

        stages  {

                stage("DB_setup") {
                    steps {
                        sh 'cargo install sqlx-cli'
                        sh './scripts/init_db.sh'
                        sh 'cargo sqlx prepare --check -- --bin todo-rust'
                    }
                }
                stage("Test") {
                        steps {
                                sh 'cargo test'
                        }
                }
      
                stage("RustFmt") {
                        steps {
                                sh 'cargo fmt --check'
                        }
                }

                stage("Build") {
                        steps {
                                sh 'cargo build --release'
                        }
                }

                stage("Docker prune") {
                        steps {
                            sh 'docker system prune -a --volumes --force'
                        }
                }

                stage("Docker build") {
                        steps {
                            sh 'docker build -t $IMAGE_NAME:$IMAGE_VERSION .'
                        }
                }

                stage("Login to ghcr") {
                        steps {
                            sh 'echo $GITHUB_TOKEN_PSW | docker login ghcr.io -u $GITHUB_TOKEN_USR --password-stdin'
                        }
                }

                stage("Tag docker image") {
                        steps {
                            sh 'docker tag $IMAGE_NAME:$IMAGE_VERSION ghcr.io/$IMAGE_NAME:$IMAGE_VERSION'
                        }
                }
                stage("Push Image to ghcr.io") {
                        steps {
                            sh 'docker push ghcr.io/$IMAGE_NAME:$IMAGE_VERSION'
                        }
                }
                        
               
        }
        post {
            always {
                sh 'docker rm -f ${POSTGRES_HOST} && docker logout'
            }
        }
 }
