import { Route, Routes } from 'react-router-dom'
import App from '../App'
import Layout from '../layout'

export const rootPath = '/indexHtml/main.html'

export const writepath = (path: string) => rootPath + path

function RouterComponent() {
  return (
    <Routes>
      <Route path={rootPath} element={<Layout />}>
        <Route index element={<App />} />
        <Route path={writepath('/app1')} element={<App />} />
      </Route>
    </Routes>
  )
}

export default RouterComponent
