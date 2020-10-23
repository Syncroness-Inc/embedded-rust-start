pipeline {
  agent {
    dockerfile {
      dir '.'
      label 'linux'
    }
  }
  
  stages {
    stage('Check') {
      steps {
        sh 'cargo make check-all'
      }
    }
    
    stage('Test') {
      steps {
        sh 'cargo make test-local'
      }
    }

    stage('Coverage') {
      steps {
        sh 'cargo make coverage'

        step([$class: 'CoberturaPublisher', coberturaReportFile: '*.xml'])
      }
    }
  }
}