import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import RouterComponent from './router/RouterComponent.tsx'

import './styles.css'
import AppBorder from '../components/AppBorder/index.tsx'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <AppBorder>
      <BrowserRouter>
        <RouterComponent />
      </BrowserRouter>
    </AppBorder>
  </React.StrictMode>
)
