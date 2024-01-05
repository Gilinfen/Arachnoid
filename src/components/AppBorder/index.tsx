import { tyInvoke } from '../../invoke'
import { appWindow } from '@tauri-apps/api/window'
import { app } from '@tauri-apps/api'
import { useEffect, useState } from 'react'
import {
  CloseOutlined,
  FullscreenExitOutlined,
  FullscreenOutlined,
  MinusOutlined,
} from '@ant-design/icons'
import clsx from 'clsx'
import style from './index.module.scss'

export default function AppBorder({ children }: any) {
  const [appName, setAppName] = useState('')
  const [isMaximized, setisMaximized] = useState(true)

  useEffect(() => {
    app.getName().then((res: string) => {
      setAppName(res)
    })
  }, [])

  const toggleMaximize = async () => {
    if (await appWindow.isMaximized()) {
      appWindow.unmaximize()
    } else {
      appWindow.maximize()
    }
  }

  return (
    <div className={style.root}>
      <div
        className="header"
        data-tauri-drag-region
        onDoubleClick={toggleMaximize}
      >
        <div className={clsx('controls')} data-tauri-drag-region>
          <div className="items">
            <div className="item clear" onClick={() => tyInvoke('close_app')}>
              <CloseOutlined />
            </div>
            <div
              className="item minification"
              onClick={() => appWindow.minimize()}
            >
              <MinusOutlined />
            </div>
            <div
              className="item blow—up"
              onClick={() => {
                setisMaximized(!isMaximized)
                toggleMaximize()
              }}
            >
              {isMaximized ? (
                <FullscreenExitOutlined />
              ) : (
                <FullscreenOutlined />
              )}
            </div>
          </div>
        </div>
        <div className="title" data-tauri-drag-region>
          {appName}
        </div>
      </div>
      <div className="context">{children}</div>
    </div>
  )
}
