pipeline {
    agent {
        docker {
            image  'scorpil/rust:nightly'
            args   '-e HOME=${WORKSPACE}'
        }
    }
    stages {
        stage('Build') {
            steps {
                sh 'cargo version'
                sh 'cargo build --verbose'
            }
        }
        stage('Test') {
            steps {
                sh 'cargo test --verbose'
            }
        }
    }
    post {
       always {
          echo "Does this work?"
       }
    }
}
