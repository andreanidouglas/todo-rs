pipeline {
        agent any

        stages  {

                stage("DB_setup") {
                    steps {
                        sh './scripts/init_db.sh && /home/jenkins/.cargo/bin/cargo install sqlx-cli'
                    }
                }
                stage("Test") {
                        steps {
                                sh '/home/jenkins/.cargo/bin/cargo test'
                        }
                }
      
                stage("Clippy") {
                        steps {
                                sh '/home/jenkins/.cargo/bin/cargo clippy -- -D warnings'
                        }
                }

                stage("RustFmt") {
                        steps {
                                sh '/home/jenkins/.cargo/bin/cargo fmt --check'
                        }
                }

                stage("Build") {
                        steps {
                                sh '/home/jenkins/.cargo/bin/cargo build --release'
                        }
                }
               
        }
        post {
            always {
                sh 'docker rm -f pg'
            }
        }
 }
