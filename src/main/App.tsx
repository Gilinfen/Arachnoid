import { useEffect, useState } from 'react'
import { Button, Input } from 'antd'
import { tyInvoke } from '../invoke'
import Chrome from '../components/chrome'
import LogViewer from '../components/log'
import './App.css'

function App() {
  const [times, setTimes] = useState(0)
  const [settings, setSettings] = useState<any>()
  const [python_path, setpython_path] = useState('')

  const init_fun = async () => {
    await tyInvoke('read_json_command')
      .then((res: any) => setSettings(res))
      .catch(() => setSettings(''))
  }

  useEffect(() => {
    init_fun()
    // 页面加载完成后通知 Tauri 显示窗口
    tyInvoke('app_ready')
  }, [])

  const updateSe = async () => {
    await tyInvoke('update_json_command', {
      data: {
        ...settings,
        python_path,
      },
    })
    await init_fun()
  }

  return (
    <div className="container">
      <h1>Settings</h1>
      <p>{settings?.python_path}</p>
      <p>{settings?.chromedriver}</p>
      <Input
        value={python_path}
        onChange={(e) => setpython_path(e.target.value)}
      />
      <Button onClick={updateSe}>修改 Settings</Button>
      <Chrome />

      <Button
        onClick={async () => {
          const time1 = +new Date()
          await tyInvoke('execute_python_script', {
            cmds: ['app1'],
          })
          const time2 = +new Date()
          setTimes(time2 - time1)
        }}
      >
        测试Python
      </Button>
      <p>耗时：{times / 1000} s</p>
      <LogViewer />
    </div>
  )
}

export default App
