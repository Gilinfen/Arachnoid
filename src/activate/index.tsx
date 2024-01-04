import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './styles.css'
import AppBorder from '../components/AppBorder'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <AppBorder>
      <App />
    </AppBorder>
  </React.StrictMode>
)
