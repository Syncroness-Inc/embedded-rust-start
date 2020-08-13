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
        sh './scripts/all.py'
      }
    }
    
    stage('Test') {
      steps {
        sh './scripts/test/local.sh'
      }
    }

    stage('Coverage') {
      steps {
        sh './scripts/coverage.sh'

        step([$class: 'CoberturaPublisher', coberturaReportFile: '*.xml'])
      }
    }
  }
}