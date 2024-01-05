import React, { useMemo, useState } from 'react'
import { UserOutlined } from '@ant-design/icons'
import type { MenuProps } from 'antd'
import { Layout, Menu, theme } from 'antd'
import { Outlet, useNavigate } from 'react-router-dom'
import { rootPath, writepath } from './router/RouterComponent'
import dayjs from 'dayjs'
import styles from './layout.module.scss'
import clsx from 'clsx'

const { Content, Sider } = Layout

const LayoutCom: React.FC = () => {
  const [collapsed, setCollapsed] = useState(true)

  const nav = useNavigate()
  const itemsObj = useMemo(
    () => [
      {
        icon: UserOutlined,
        label: '主页',
        link: rootPath,
      },
      {
        icon: UserOutlined,
        label: '产品处理',
        link: writepath('/product'),
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
    <Layout hasSider className={styles.root}>
      <Sider
        collapsible
        collapsed={collapsed}
        onCollapse={(value) => setCollapsed(value)}
      >
        <div className="logo-box">
          <div className="demo-logo-vertical" />
          <div className={clsx('logo-info', !collapsed && 'logo-info-flex')}>
            <div className="user-name">XXXXXXXXXXXXXXXXXXXX</div>
            <div className="user-date">
              {dayjs().format('YYYY MM-DDTHH:mm')}
            </div>
          </div>
        </div>
        <Menu
          theme="dark"
          mode="inline"
          defaultSelectedKeys={['1']}
          items={items}
        />
      </Sider>
      <Layout
        style={{
          height: 'calc(100vh - 30px)',
        }}
      >
        <Content style={{ margin: '24px 16px 0', overflow: 'initial' }}>
          <div
            style={{
              padding: 24,
              textAlign: 'center',
              height: '100%',
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
