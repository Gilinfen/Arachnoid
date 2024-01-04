import React, { useMemo } from 'react'
import { UserOutlined } from '@ant-design/icons'
import type { MenuProps } from 'antd'
import { Layout, Menu, theme } from 'antd'
import { Outlet, useNavigate } from 'react-router-dom'
import { rootPath, writepath } from './router/RouterComponent'

const { Content, Sider } = Layout

const LayoutCom: React.FC = () => {
  const nav = useNavigate()
  const itemsObj = useMemo(
    () => [
      {
        icon: UserOutlined,
        label: '签名',
        link: rootPath,
      },
      {
        icon: UserOutlined,
        label: '签名2',
        link: writepath('/app1'),
      },
    ],
    []
  )

  const items: MenuProps['items'] = useMemo(
    () =>
      itemsObj.map((item, index) => ({
        key: String(index + 1),
        icon: React.createElement(item.icon),
        label: item.label,
        onClick() {
          nav(item.link)
        },
      })),
    []
  )

  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken()

  return (
    <Layout hasSider>
      <Sider
        style={{
          overflow: 'auto',
          height: '100vh',
          position: 'fixed',
          left: 0,
          top: 0,
          bottom: 0,
        }}
      >
        <div className="demo-logo-vertical" />
        <Menu
          theme="dark"
          mode="inline"
          defaultSelectedKeys={['1']}
          items={items}
        />
      </Sider>
      <Layout style={{ marginLeft: 200 }}>
        <Content style={{ margin: '24px 16px 0', overflow: 'initial' }}>
          <div
            style={{
              padding: 24,
              textAlign: 'center',
              background: colorBgContainer,
              borderRadius: borderRadiusLG,
            }}
          >
            <Outlet />
          </div>
        </Content>
      </Layout>
    </Layout>
  )
}

export default LayoutCom
