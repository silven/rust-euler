pipeline {
    agent {
        docker {
            image  'scorpil/rust:nightly'
            args '-e HOME=${WORKSPACE}'
        }
    }
    stages {
        stage('Build') {
            steps {
                sh 'export'
                sh 'cargo version'
                sh 'cargo build --verbose'
            }
        }
        stage('Test') {
            steps {
                sh 'export'
                sh 'ls -ahl'
                sh 'cargo test --verbose'
            }
        }
    }
    post {
       always {
          archive "*.strace"
       }
    }
}
